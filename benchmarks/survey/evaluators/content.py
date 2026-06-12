"""SurveyBench content-based evaluation.

Implements the 5-dimension evaluation from arXiv:2510.03120v2.
Weights: Outline 30% (2×15%), Content 50% (2×25%), Non-textual 20% (2×10%).
All dimensions scored 1-5.
"""

from __future__ import annotations

import logging
import os
import re
from pathlib import Path
from typing import Any, Optional


from evaluators.llm_judge import judge_json

logger = logging.getLogger(__name__)

_PROMPT_DIR = Path(__file__).resolve().parent.parent / "prompts"

_CONTENT_WEIGHTS = {
    "coverage_breadth": 0.15,
    "logical_coherence": 0.15,
    "synthesis_granularity": 0.25,
    "clarity_of_insights": 0.25,
    "reference_relevance": 0.10,
    "nontextual_elements": 0.10,
}

_OUTLINE_WEIGHTS = {
    "coverage": 0.34,
    "relevance": 0.33,
    "structure": 0.33,
}

_FIGURE_RE = re.compile(r"!\[.*?\]\(.*?\)", re.DOTALL)
_TABLE_SEPARATOR_RE = re.compile(r"^\|[:\-\s|]+\|\s*$", re.MULTILINE)
_EQUATION_RE = re.compile(r"\$\$.*?\$\$|\$[^$]+?\$", re.DOTALL)


def _load_prompt(name: str) -> str:
    path = _PROMPT_DIR / name
    return path.read_text(encoding="utf-8")


def _content_prompt(reference_text: str | None) -> str:
    base = _load_prompt("surveybench_content.txt")
    if reference_text:
        base = base.replace("[REFERENCE_SURVEY]", reference_text)
    else:
        base = base.replace(
            "[REFERENCE_SURVEY]",
            "No reference survey provided — evaluate independently.",
        )
    base += (
        "\n\n---\n"
        "Return ONLY a valid JSON object with no markdown fences. Fields:\n"
        '  "coverage_breadth": int 1-5,\n'
        '  "coverage_breadth_justification": string,\n'
        '  "logical_coherence": int 1-5,\n'
        '  "logical_coherence_justification": string,\n'
        '  "synthesis_granularity": int 1-5,\n'
        '  "synthesis_granularity_justification": string,\n'
        '  "clarity_of_insights": int 1-5,\n'
        '  "clarity_of_insights_justification": string,\n'
        '  "reference_relevance": int 1-5,\n'
        '  "reference_relevance_justification": string,\n'
        '  "nontextual_elements": int 1-5,\n'
        '  "nontextual_elements_justification": string,\n'
        '  "strengths": [string, ...],\n'
        '  "weaknesses": [string, ...],\n'
        '  "summary": string\n'
    )
    return base


def _outline_prompt(reference_outline: str | None) -> str:
    ref_section = ""
    if reference_outline:
        ref_section = (
            f"\n\nReference outline for comparison:\n{reference_outline}\n"
            "Compare the survey's outline against this reference."
        )
    return (
        "Evaluate the outline quality of this survey on 3 dimensions "
        "(each scored 1-5):\n\n"
        "1. **Coverage**: How thoroughly does the outline cover the topic "
        "landscape? Are major sub-areas represented?\n"
        "2. **Relevance**: Are the included topics genuinely important to the "
        "domain? Are there irrelevant sections?\n"
        "3. **Structure**: Is the organization logical and progressive? "
        "Do sections build on each other naturally?\n"
        f"{ref_section}\n\n"
        "Return ONLY a valid JSON object with no markdown fences. Fields:\n"
        '  "coverage": int 1-5,\n'
        '  "coverage_justification": string,\n'
        '  "relevance": int 1-5,\n'
        '  "relevance_justification": string,\n'
        '  "structure": int 1-5,\n'
        '  "structure_justification": string\n'
    )


def _compute_weighted(dimensions: dict[str, Any], weights: dict[str, float]) -> float:
    total = 0.0
    for key, weight in weights.items():
        score = dimensions.get(key, 3)
        if isinstance(score, (int, float)):
            total += score * weight
    return round(total, 2)


def _is_mock() -> bool:
    return (
        bool(os.environ.get("EVA_API_KEY") or os.environ.get("OPENAI_API_KEY")) is False
    )


def evaluate_content(
    survey_text: str,
    reference_text: Optional[str] = None,
) -> dict[str, Any]:
    """Evaluate survey content quality on 5 SurveyBench dimensions.

    If *reference_text* is provided the LLM compares the generated survey
    against it (with_ref mode); otherwise it scores independently
    (without_ref mode).
    """
    mode = "with_ref" if reference_text else "without_ref"
    prompt = _content_prompt(reference_text)
    result = judge_json(prompt, survey_text)

    dimensions = {
        "coverage_breadth": result.get("coverage_breadth", 3),
        "coverage_breadth_justification": result.get(
            "coverage_breadth_justification", ""
        ),
        "logical_coherence": result.get("logical_coherence", 3),
        "logical_coherence_justification": result.get(
            "logical_coherence_justification", ""
        ),
        "synthesis_granularity": result.get("synthesis_granularity", 3),
        "synthesis_granularity_justification": result.get(
            "synthesis_granularity_justification", ""
        ),
        "clarity_of_insights": result.get("clarity_of_insights", 3),
        "clarity_of_insights_justification": result.get(
            "clarity_of_insights_justification", ""
        ),
        "reference_relevance": result.get("reference_relevance", 3),
        "reference_relevance_justification": result.get(
            "reference_relevance_justification", ""
        ),
        "nontextual_elements": result.get("nontextual_elements", 3),
        "nontextual_elements_justification": result.get(
            "nontextual_elements_justification", ""
        ),
    }

    overall_score = _compute_weighted(dimensions, _CONTENT_WEIGHTS)

    return {
        "dimensions": dimensions,
        "overall_score": overall_score,
        "mode": mode,
        "strengths": result.get("strengths", []),
        "weaknesses": result.get("weaknesses", []),
        "summary": result.get("summary", ""),
    }


def evaluate_outline(
    survey_text: str,
    reference_outline: Optional[str] = None,
) -> dict[str, Any]:
    """Evaluate survey outline quality on 3 dimensions.

    Dimensions: coverage, relevance, structure.  Each scored 1-5.
    """
    prompt = _outline_prompt(reference_outline)
    result = judge_json(prompt, survey_text)

    dimensions = {
        "coverage": result.get("coverage", 3),
        "coverage_justification": result.get("coverage_justification", ""),
        "relevance": result.get("relevance", 3),
        "relevance_justification": result.get("relevance_justification", ""),
        "structure": result.get("structure", 3),
        "structure_justification": result.get("structure_justification", ""),
    }

    overall_score = _compute_weighted(dimensions, _OUTLINE_WEIGHTS)

    return {
        "dimensions": dimensions,
        "overall_score": overall_score,
    }


def evaluate_richness(survey_text: str) -> dict[str, int]:
    """Count figures, tables, and equations in the survey text."""
    figures = len(_FIGURE_RE.findall(survey_text))
    tables = len(_TABLE_SEPARATOR_RE.findall(survey_text))
    equations = len(_EQUATION_RE.findall(survey_text))
    return {
        "figures": figures,
        "tables": tables,
        "equations": equations,
    }


def generate_report(
    survey_text: str,
    reference_text: Optional[str] = None,
    reference_outline: Optional[str] = None,
) -> str:
    """Generate a full markdown evaluation report combining all evaluations."""
    mock_notice = ""
    if _is_mock():
        mock_notice = (
            "\n> ⚠️ **MOCK EVALUATION** — no EVA_API_KEY or OPENAI_API_KEY "
            "configured. Scores are placeholder values and should not be "
            "used for research conclusions.\n"
        )

    content_result = evaluate_content(survey_text, reference_text)
    outline_result = evaluate_outline(survey_text, reference_outline)
    richness = evaluate_richness(survey_text)

    lines: list[str] = []
    lines.append("# SurveyBench Content Evaluation Report")
    if mock_notice:
        lines.append(mock_notice)

    lines.append(f"\n**Mode**: {content_result['mode']}")
    lines.append(f"**Overall Content Score**: {content_result['overall_score']}/5.0")
    lines.append(f"**Overall Outline Score**: {outline_result['overall_score']}/5.0")

    lines.append("\n## Content Quality Dimensions\n")
    for dim_key, label in [
        ("coverage_breadth", "A1. Coverage Breadth"),
        ("logical_coherence", "A2. Logical Coherence"),
        ("synthesis_granularity", "B1. Synthesis Granularity"),
        ("clarity_of_insights", "B2. Clarity of Insights"),
        ("reference_relevance", "C1. Reference Relevance"),
        ("nontextual_elements", "C2. Non-textual Elements"),
    ]:
        score = content_result["dimensions"].get(dim_key, "—")
        justification = content_result["dimensions"].get(f"{dim_key}_justification", "")
        weight = _CONTENT_WEIGHTS[dim_key]
        weighted = score * weight if isinstance(score, (int, float)) else 0
        lines.append(f"### {label}: {score}/5 (weight: {weight:.0%})")
        if justification:
            lines.append(f"\n{justification}\n")

    lines.append("\n## Outline Quality Dimensions\n")
    for dim_key in ("coverage", "relevance", "structure"):
        score = outline_result["dimensions"].get(dim_key, "—")
        justification = outline_result["dimensions"].get(f"{dim_key}_justification", "")
        lines.append(f"### {dim_key.title()}: {score}/5")
        if justification:
            lines.append(f"\n{justification}\n")

    lines.append("\n## Non-textual Richness\n")
    lines.append(f"- Figures: {richness['figures']}")
    lines.append(f"- Tables: {richness['tables']}")
    lines.append(f"- Equations: {richness['equations']}")

    strengths = content_result.get("strengths", [])
    if strengths:
        lines.append("\n## Strengths\n")
        for s in strengths:
            lines.append(f"- {s}")

    weaknesses = content_result.get("weaknesses", [])
    if weaknesses:
        lines.append("\n## Weaknesses / Gaps\n")
        for w in weaknesses:
            lines.append(f"- {w}")

    summary = content_result.get("summary", "")
    if summary:
        lines.append(f"\n## Summary\n\n{summary}")

    return "\n".join(lines)
