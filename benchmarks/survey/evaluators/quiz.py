"""SurveyBench quiz-based evaluation (arXiv:2510.03120v2).

Implements quiz generation, BM25-based passage retrieval, and LLM grading.
Supports both benchmark mode (predefined questions) and exploratory mode
(LLM-generated).
"""

from __future__ import annotations

import json
import logging
import os
import re
from typing import Any, Optional

from evaluators.llm_judge import judge, judge_json

logger = logging.getLogger(__name__)

_SECTION_SPLIT_RE = re.compile(r"\n#{1,3}\s+", re.MULTILINE)
_TOKENIZE_RE = re.compile(r"\s+")

# ---------------------------------------------------------------------------
# BM25 retrieval (with keyword-overlap fallback when rank_bm25 is missing)
# ---------------------------------------------------------------------------

try:
    from rank_bm25 import BM25Okapi  # type: ignore[import-untyped]

    _BM25_AVAILABLE = True
except ImportError:
    _BM25_AVAILABLE = False
    logger.warning(
        "rank_bm25 is not installed — falling back to keyword-overlap retrieval. "
        "Install with: pip install rank-bm25"
    )


def _tokenize(text: str) -> list[str]:
    return [t.lower() for t in _TOKENIZE_RE.split(text) if t]


def _split_sections(survey_text: str) -> list[str]:
    """Split survey text into markdown sections (headings and bodies)."""
    parts = _SECTION_SPLIT_RE.split(survey_text)
    return [p.strip() for p in parts if p.strip()]


def _bm25_retrieve(query: str, sections: list[str], top_k: int) -> list[str]:
    """Retrieve top-k sections using BM25."""
    tokenized_corpus = [_tokenize(s) for s in sections]
    tokenized_query = _tokenize(query)
    bm25 = BM25Okapi(tokenized_corpus)
    scores = bm25.get_scores(tokenized_query)
    ranked = sorted(enumerate(scores), key=lambda pair: pair[1], reverse=True)
    return [sections[idx] for idx, _ in ranked[:top_k]]


def _keyword_overlap_retrieve(query: str, sections: list[str], top_k: int) -> list[str]:
    """Retrieve top-k sections using simple keyword overlap."""
    query_tokens = set(_tokenize(query))
    if not query_tokens:
        return sections[:top_k]
    scored: list[tuple[int, float]] = []
    for idx, section in enumerate(sections):
        section_tokens = set(_tokenize(section))
        overlap = len(query_tokens & section_tokens)
        norm = len(query_tokens)
        scored.append((idx, overlap / norm if norm else 0.0))
    ranked = sorted(scored, key=lambda pair: pair[1], reverse=True)
    return [sections[idx] for idx, _ in ranked[:top_k]]


def retrieve_passages(
    survey_text: str,
    question: str,
    top_k: int = 3,
) -> list[str]:
    """Retrieve relevant passages from survey using BM25.

    Splits the survey by markdown sections, indexes them, and returns the
    top-*k* sections whose content is most relevant to *question*.

    Falls back to keyword-overlap scoring when the ``rank_bm25`` package is
    not installed.
    """
    sections = _split_sections(survey_text)
    if not sections:
        return []
    if _BM25_AVAILABLE:
        return _bm25_retrieve(question, sections, top_k)
    return _keyword_overlap_retrieve(question, sections, top_k)


# ---------------------------------------------------------------------------
# Question generation
# ---------------------------------------------------------------------------

_QUESTION_GEN_PROMPT = """\
Generate {num} quiz questions to test whether an academic survey on the
topic "{topic}" provides comprehensive coverage. Each question must probe for
specific knowledge that a high-quality survey should include.

Types of questions to mix:
- Factual: Specific technical details, definitions, or proven results
- Comparative: Comparing approaches, trade-offs between methods
- Coverage: Whether the survey discusses a particular sub-topic or paper

Return ONLY a valid JSON array of objects with no markdown fences.  Each
object must have these fields:
  "id": string (unique, e.g. "q1", "q2"),
  "type": one of "factual", "comparative", "coverage",
  "question": string,
  "key_points": [string, ...] (3-5 specific points a good answer should cover),
  "difficulty": one of "easy", "medium", "hard"
"""


def generate_questions(topic: str, num_questions: int = 5) -> list[dict[str, Any]]:
    """Generate topic-specific quiz questions using the LLM.

    Returns a list of dicts each containing *id*, *type*, *question*,
    *key_points*, and *difficulty*.
    """
    prompt = _QUESTION_GEN_PROMPT.format(num=num_questions, topic=topic)
    raw = judge(prompt, topic)

    try:
        parsed = json.loads(raw)
        if isinstance(parsed, list):
            return [
                {
                    "id": q.get("id", f"q{i}"),
                    "type": q.get("type", "factual"),
                    "question": q.get("question", ""),
                    "key_points": q.get("key_points", []),
                    "difficulty": q.get("difficulty", "medium"),
                }
                for i, q in enumerate(parsed)
            ]
    except (json.JSONDecodeError, TypeError):
        logger.warning("Question generation returned non-JSON, using fallback")
    # Fallback: return a basic question
    return [
        {
            "id": f"q{i}",
            "type": "coverage",
            "question": f"Does the survey cover key aspects of {topic}?",
            "key_points": [
                "Major research directions",
                "Key methods and approaches",
                "Recent advances",
            ],
            "difficulty": "medium",
        }
        for i in range(1, num_questions + 1)
    ]


# ---------------------------------------------------------------------------
# Answer grading
# ---------------------------------------------------------------------------

_GRADING_PROMPT = """\
You are evaluating whether an academic survey can answer a specific question.

**Question**: {question}
**Question type**: {question_type}
**Key points a good answer should cover**: {key_points}

**Retrieved passages from the survey**:
{passages}

Determine:
1. Can the survey answer this question based on the retrieved passages?
   - "Yes": The passages contain a clear, complete answer
   - "Partial": Some information is present but key details are missing
   - "No": The question cannot be answered from these passages

2. Correctness score (0-100):
   - 90-100: Excellent, comprehensive answer with evidence
   - 70-89: Good answer with minor gaps
   - 50-69: Adequate but significant gaps
   - 30-49: Weak coverage, mostly superficial
   - 0-29: Little to no relevant information

3. How many of the key points are covered in the passages?

Return ONLY a valid JSON object with no markdown fences.  Fields:
  "can_answer": "Yes" | "Partial" | "No",
  "correctness_score": int 0-100,
  "key_points_covered": int,
  "justification": string
"""


def grade_answer(
    question: dict[str, Any],
    survey_text: str,
    passages: list[str],
) -> dict[str, Any]:
    """Grade whether the survey can answer *question* using retrieved passages.

    Uses the LLM judge to evaluate answerability.  Returns a dict with
    *can_answer*, *correctness_score*, *key_points_covered*, and
    *key_points_total*.
    """
    prompt = _GRADING_PROMPT.format(
        question=question.get("question", ""),
        question_type=question.get("type", "factual"),
        key_points=", ".join(question.get("key_points", [])),
        passages="\n\n---\n\n".join(passages)
        if passages
        else "(no passages retrieved)",
    )
    result = judge_json(prompt, survey_text)

    key_points_total = len(question.get("key_points", []))
    return {
        "can_answer": result.get("can_answer", "Partial"),
        "correctness_score": result.get("correctness_score", 50),
        "key_points_covered": result.get("key_points_covered", 0),
        "key_points_total": key_points_total,
        "justification": result.get("justification", ""),
    }


# ---------------------------------------------------------------------------
# Full evaluation pipeline
# ---------------------------------------------------------------------------


def evaluate(
    survey_text: str,
    topic: str,
    questions: Optional[list[dict[str, Any]]] = None,
    num_questions: int = 5,
) -> dict[str, Any]:
    """Run full quiz evaluation pipeline.

    If *questions* are provided (benchmark mode), they are used directly.
    Otherwise questions are generated dynamically (exploratory mode).

    Returns a dict with *questions*, *overall_answerability*, and per-question
    grades.
    """
    if questions is None:
        questions = generate_questions(topic, num_questions)

    graded: list[dict[str, Any]] = []
    correctness_scores: list[int] = []
    coverages: list[float] = []

    for q in questions:
        passages = retrieve_passages(survey_text, q["question"])
        grade = grade_answer(q, survey_text, passages)

        q_result = dict(q)
        q_result["passages_retrieved"] = len(passages)
        q_result["grade"] = grade
        graded.append(q_result)

        correctness_scores.append(grade["correctness_score"])
        if grade["key_points_total"] > 0:
            coverages.append(grade["key_points_covered"] / grade["key_points_total"])
        else:
            coverages.append(0.0)

    avg_correctness = (
        sum(correctness_scores) / len(correctness_scores) if correctness_scores else 0.0
    )
    avg_coverage = sum(coverages) / len(coverages) if coverages else 0.0
    overall_answerability = round(avg_correctness * 0.6 + avg_coverage * 40.0, 1)
    # Coverage is a ratio 0-1, remap to 0-100 scale: coverage * 100 * 0.4 = coverage * 40

    yes_count = sum(1 for g in graded if g["grade"]["can_answer"] == "Yes")
    partial_count = sum(1 for g in graded if g["grade"]["can_answer"] == "Partial")
    no_count = sum(1 for g in graded if g["grade"]["can_answer"] == "No")

    return {
        "questions": graded,
        "overall_answerability": overall_answerability,
        "avg_correctness": round(avg_correctness, 1),
        "avg_coverage_ratio": round(avg_coverage, 3),
        "summary": {
            "can_answer_yes": yes_count,
            "can_answer_partial": partial_count,
            "can_answer_no": no_count,
            "total": len(graded),
        },
    }


# ---------------------------------------------------------------------------
# Report generation
# ---------------------------------------------------------------------------


def generate_report(
    survey_text: str,
    topic: str,
    questions: Optional[list[dict[str, Any]]] = None,
) -> str:
    """Generate a markdown quiz evaluation report.

    If *questions* are provided (benchmark mode), they are used directly;
    otherwise questions are generated dynamically.
    """
    result = evaluate(survey_text, topic, questions=questions)

    mock_notice = ""
    if bool(os.environ.get("EVA_API_KEY") or os.environ.get("OPENAI_API_KEY")) is False:
        mock_notice = (
            "\n> ⚠️ **MOCK EVALUATION** — no EVA_API_KEY or OPENAI_API_KEY "
            "configured. Scores are placeholder values and should not be "
            "used for research conclusions.\n"
        )

    lines: list[str] = []
    lines.append("# SurveyBench Quiz Evaluation Report")
    if mock_notice:
        lines.append(mock_notice)

    lines.append(f"\n**Topic**: {topic}")
    lines.append(
        f"**Overall Answerability**: {result['overall_answerability']:.1f}/100"
    )
    lines.append(f"**Average Correctness**: {result['avg_correctness']:.1f}/100")
    lines.append(f"**Average Key-Point Coverage**: {result['avg_coverage_ratio']:.1%}")

    summary = result["summary"]
    lines.append(
        f"\n**Verdicts**: {summary['can_answer_yes']} yes, "
        f"{summary['can_answer_partial']} partial, "
        f"{summary['can_answer_no']} no "
        f"(out of {summary['total']})"
    )

    lines.append("\n## Per-Question Results\n")
    for qr in result["questions"]:
        grade = qr["grade"]
        lines.append(f"### Q: {qr['question']}")
        lines.append(f"- **Type**: {qr['type']} | **Difficulty**: {qr['difficulty']}")
        lines.append(
            f"- **Verdict**: {grade['can_answer']} | "
            f"Correctness: {grade['correctness_score']}/100 | "
            f"Key-points covered: {grade['key_points_covered']}/{grade['key_points_total']}"
            f"  ({qr['passages_retrieved']} passages retrieved)"
        )
        justification = grade.get("justification", "")
        if justification:
            lines.append(f"\n  {justification}")
        lines.append("")

    lines.append("---")
    lines.append(f"*Generated by SurveyBench quiz evaluator (arXiv:2510.03120v2)*")

    return "\n".join(lines)
