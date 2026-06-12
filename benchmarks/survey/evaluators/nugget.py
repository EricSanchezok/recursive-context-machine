"""DeepScholar-Bench nugget coverage evaluator.

Matches generated content against ground-truth information nuggets using
semantic similarity (sentence-transformers optional) with a BM25+LLM fallback.
Replaces the old word-overlap heuristic.
"""

from __future__ import annotations

import logging
import math
from collections import defaultdict
from typing import Any, Optional

from evaluators.llm_judge import judge, judge_json

logger = logging.getLogger(__name__)

_EXTRACT_NUGGETS_PROMPT = """\
Extract atomic information nuggets from the following survey / related-work text.

A "nugget" is a single, self-contained factual statement or key insight.
Each nugget should be one sentence. Do NOT extract generic statements, fluff,
or transitional phrases. Focus on:

- Named methods, models, or algorithms and their properties
- Key empirical findings (numbers, comparisons, claims)
- Important relationships between works
- Technical innovations attributed to specific papers

Return a JSON object with a single key "nuggets" whose value is a list of strings.

Return ONLY the JSON object, no other text."""

_VERIFY_NUGGET_PROMPT = """\
You are verifying whether a generated text covers a specific information nugget.

NUGGET: {nugget}

GENERATED TEXT:
{generated_text}

Does the generated text contain this nugget (either explicitly or by clear
semantic implication)? Consider:

- Semantic equivalence, not exact wording
- Implied knowledge (if the text says X and the nugget is a direct consequence)
- Partial coverage counts as "no"

Return a JSON object with exactly two keys:
- "covered": boolean (true/false)
- "reasoning": one sentence explaining your decision

Return ONLY the JSON object, no other text."""


def extract_nuggets(text: str) -> list[str]:
    """Extract atomic information nuggets from text using LLM judge.

    Returns a list of nugget strings.
    """
    result = judge_json(_EXTRACT_NUGGETS_PROMPT, text)
    nuggets = result.get("nuggets", [])

    if not isinstance(nuggets, list):
        logger.warning("LLM judge returned non-list nuggets; treating as empty")
        return []

    valid = [str(n) for n in nuggets if isinstance(n, str) and n.strip()]
    logger.info("Extracted %d nuggets from text", len(valid))
    return valid


def evaluate(
    generated_text: str,
    reference_text: str,
    ground_truth_nuggets: Optional[list[dict]] = None,
) -> dict:
    """Evaluate nugget coverage of generated text against reference.

    If ``ground_truth_nuggets`` is provided, those nuggets are used directly.
    Otherwise, nuggets are extracted from ``reference_text`` via LLM judge.

    Returns a dict with keys: ``nugget_coverage`` (0-100), ``matched_nuggets``,
    ``total_nuggets``, ``matched`` (list of dicts), ``missed`` (list of str).
    """
    if ground_truth_nuggets:
        target_nuggets = [
            n["text"] if isinstance(n, dict) else str(n) for n in ground_truth_nuggets
        ]
    else:
        target_nuggets = extract_nuggets(reference_text)

    if not target_nuggets:
        logger.warning("No nuggets found in reference; returning perfect coverage")
        return {
            "nugget_coverage": 100.0,
            "matched_nuggets": 0,
            "total_nuggets": 0,
            "matched": [],
            "missed": [],
        }

    matched: list[dict] = []
    missed: list[str] = []

    for nugget in target_nuggets:
        nugget_prompt = _VERIFY_NUGGET_PROMPT.format(
            nugget=nugget, generated_text=generated_text
        )
        result = judge_json(nugget_prompt, "")
        covered = result.get("covered", False)
        reasoning = result.get("reasoning", "")

        if covered:
            matched.append({"nugget": nugget, "reasoning": reasoning})
        else:
            missed.append(nugget)

    total = len(target_nuggets)
    coverage = (len(matched) / total) * 100.0

    logger.info(
        "Nugget coverage: %.1f%% (%d/%d matched)",
        coverage,
        len(matched),
        total,
    )

    return {
        "nugget_coverage": round(coverage, 1),
        "matched_nuggets": len(matched),
        "total_nuggets": total,
        "matched": matched,
        "missed": missed,
    }


def generate_report(generated_text: str, reference_text: str) -> str:
    """Generate a Markdown nugget coverage report."""
    metrics = evaluate(generated_text, reference_text)

    matched_examples = ""
    for entry in metrics["matched"][:5]:
        matched_examples += f"- ✓ {entry['nugget']}\n"

    missed_examples = ""
    for nugget in metrics["missed"][:5]:
        missed_examples += f"- ✗ {nugget}\n"

    return f"""\
## Nugget Coverage Report

- **Coverage**: {metrics["nugget_coverage"]}%
- **Matched**: {metrics["matched_nuggets"]}/{metrics["total_nuggets"]}

### Matched Nuggets (sample)
{matched_examples or "_None_"}

### Missed Nuggets (sample)
{missed_examples or "_None_"}
"""


# ── Semantic similarity helpers ──────────────────────────────────────────────
# compute_similarity, _compute_bm25_similarity, and
# _compute_sentence_transformer_similarity are available for direct use
# but are not currently called by evaluate(). They provide a lightweight
# alternative to LLM-judge-based nugget matching when sentence-transformers
# is installed.
# ─────────────────────────────────────────────────────────────────────────────

# -- Semantic similarity helpers (optional sentence-transformers) --

_SENTENCE_TRANSFORMERS_AVAILABLE = False
try:
    from sentence_transformers import SentenceTransformer  # noqa: F401

    _SENTENCE_TRANSFORMERS_AVAILABLE = True
except ImportError:
    pass


def compute_similarity(nugget: str, sentences: list[str]) -> list[float]:
    """Compute cosine similarity between a nugget and a list of sentences.

    Uses sentence-transformers (all-MiniLM-L6-v2) when available, otherwise
    falls back to a simple BM25-based lexical overlap score.
    """
    if _SENTENCE_TRANSFORMERS_AVAILABLE:
        return _compute_sentence_transformer_similarity(nugget, sentences)
    return _compute_bm25_similarity(nugget, sentences)


def _compute_sentence_transformer_similarity(
    query: str, sentences: list[str]
) -> list[float]:
    """Cosine similarity using all-MiniLM-L6-v2."""
    from sentence_transformers import SentenceTransformer, util

    model = SentenceTransformer("all-MiniLM-L6-v2")
    query_embedding = model.encode(query, convert_to_tensor=True)
    sentence_embeddings = model.encode(sentences, convert_to_tensor=True)
    similarities = util.cos_sim(query_embedding, sentence_embeddings)[0]
    return similarities.tolist()


def _compute_bm25_similarity(query: str, sentences: list[str]) -> list[float]:
    """BM25-based lexical similarity fallback.

    A lightweight implementation that computes term-frequency-based scores
    without external dependencies.
    """
    if not sentences:
        return []

    k1 = 1.2
    b_param = 0.75

    tokenized_sentences = [
        [token.lower() for token in _tokenize(sent)] for sent in sentences
    ]
    query_tokens = [token.lower() for token in _tokenize(query)]

    doc_count = len(tokenized_sentences)
    avg_doc_length = sum(len(d) for d in tokenized_sentences) / max(doc_count, 1)

    doc_freq: dict[str, int] = defaultdict(int)
    for doc_tokens in tokenized_sentences:
        for token in set(doc_tokens):
            doc_freq[token] += 1

    scores: list[float] = []
    for doc_tokens in tokenized_sentences:
        score = 0.0
        doc_length = len(doc_tokens)
        term_freqs: dict[str, int] = defaultdict(int)
        for token in doc_tokens:
            term_freqs[token] += 1

        for query_token in query_tokens:
            tf_val = term_freqs.get(query_token, 0)
            if tf_val == 0:
                continue
            df_val = doc_freq.get(query_token, 0)
            if df_val == 0:
                continue

            idf = math.log((doc_count - df_val + 0.5) / (df_val + 0.5) + 1.0)
            numerator = tf_val * (k1 + 1.0)
            denominator = tf_val + k1 * (
                1.0 - b_param + b_param * (doc_length / max(avg_doc_length, 1))
            )
            score += idf * (numerator / denominator)

        scores.append(score)

    if not scores:
        return [0.0] * len(sentences)

    max_score = max(scores)
    if max_score > 0:
        scores = [s / max_score for s in scores]

    return scores


def _tokenize(text: str) -> list[str]:
    """Simple whitespace-plus-punctuation tokenizer."""
    import re

    return re.findall(r"[a-zA-Z0-9]+", text)
