"""DeepScholar-Bench reference coverage and citation precision evaluator.

Evaluates RQ3 (reference coverage) and V1 (citation precision) from the
DeepScholar-Bench protocol (arXiv:2508.20033v2).
"""

from __future__ import annotations

import logging
import re
from typing import Any

from evaluators.llm_judge import judge, judge_json

logger = logging.getLogger(__name__)

_CITATION_PATTERN = re.compile(
    r"\[\d+(?:,\s*\d+)*\]"  # [1], [1,2,3]
    r"|\b\w+\s+et\s+al\.?\s*\(?\d{4}[a-z]?\)?"  # Smith et al. 2020
    r"|\b\w+\s+and\s+\w+.*?\(?\d{4}[a-z]?\)?"  # Smith and Jones 2020
    r"|\(\w+\s+et\s+al\.?,?\s*\d{4}[a-z]?\)"  # (Smith et al., 2020)
)

_CITATION_PRECISION_PROMPT = """\
Evaluate the citation precision of the following survey text.

For each claim-citation pair in the text, determine whether the cited source
actually supports the accompanying claim. A citation is "supported" when the
cited paper is reasonably likely to contain evidence for the claim based on
what can be inferred from the text.

Return a JSON object with these keys:
- "cite_p": integer 0-100 representing the percentage of citations that properly support their claims
- "per_citation": a list of objects, each with:
    - "claim": the claim text
    - "citation": the citation marker
    - "supported": boolean
    - "reasoning": one sentence

Return ONLY the JSON object, no other text."""

_REFERENCE_COVERAGE_PROMPT = """\
Check how many of the important reference titles listed below appear in the
generated survey text. A reference "appears" if it is cited by name, title word,
author, or year — fuzzy matching is acceptable.

IMPORTANT REFERENCES:
{references}

GENERATED SURVEY TEXT:
{text}

Return a JSON object with exactly these keys:
- "coverage": integer 0-100 representing the percentage found
- "found": list of reference titles (exact strings from the important list) that appear
- "missing": list of reference titles (exact strings from the important list) that do not appear

Return ONLY the JSON object, no other text."""


def evaluate_reference_coverage(
    generated_text: str, important_citations: list[dict]
) -> dict:
    """Check which important ground-truth citations appear in generated text.

    ``important_citations`` is a list of dicts, each with at least a ``title`` key.

    Returns ``{coverage: float, found: list[str], missing: list[str]}``.
    """
    if not important_citations:
        logger.warning("No important citations provided; returning 100% coverage")
        return {"coverage": 100.0, "found": [], "missing": []}

    ref_titles = [
        ref["title"] if isinstance(ref, dict) else str(ref)
        for ref in important_citations
    ]
    ref_string = "\n".join(f"- {t}" for t in ref_titles)

    prompt = _REFERENCE_COVERAGE_PROMPT.format(
        references=ref_string, text=generated_text
    )

    result = judge_json(prompt, "")
    coverage = result.get("coverage", 0)
    found = result.get("found", [])
    missing = result.get("missing", [])

    if not isinstance(coverage, (int, float)):
        coverage = 0
    coverage = max(0.0, min(100.0, float(coverage)))

    logger.info(
        "Reference coverage: %.1f%% (%d/%d found)",
        coverage,
        len(found),
        len(ref_titles),
    )

    return {
        "coverage": round(coverage, 1),
        "found": found if isinstance(found, list) else [],
        "missing": missing if isinstance(missing, list) else [],
    }


def evaluate_citation_precision(survey_text: str) -> dict:
    """LLM judge evaluates whether each citation properly supports its claim.

    Returns ``{cite_p: float, per_citation: list[dict]}``.
    """
    result = judge_json(_CITATION_PRECISION_PROMPT, survey_text)

    cite_p = result.get("cite_p", 0)
    if not isinstance(cite_p, (int, float)):
        cite_p = 0
    cite_p = max(0.0, min(100.0, float(cite_p)))

    per_citation = result.get("per_citation", [])
    if not isinstance(per_citation, list):
        per_citation = []

    supported_count = sum(
        1 for c in per_citation if isinstance(c, dict) and c.get("supported", False)
    )

    logger.info(
        "Citation precision: %.1f%% (%d/%d supported)",
        cite_p,
        supported_count,
        len(per_citation),
    )

    return {
        "cite_p": round(cite_p, 1),
        "per_citation": per_citation,
    }


def generate_report(survey_text: str, important_citations: list[dict]) -> str:
    """Generate a Markdown report combining coverage and precision."""
    coverage = evaluate_reference_coverage(survey_text, important_citations)
    precision = evaluate_citation_precision(survey_text)

    found_section = (
        "\n".join(f"- {t}" for t in coverage.get("found", [])[:10]) or "_None_"
    )

    missing_section = (
        "\n".join(f"- {t}" for t in coverage.get("missing", [])[:10]) or "_None_"
    )

    per_cite_section = ""
    for entry in precision.get("per_citation", [])[:5]:
        if not isinstance(entry, dict):
            continue
        marker = "✓" if entry.get("supported") else "✗"
        per_cite_section += (
            f"- {marker} **Claim**: {entry.get('claim', '?')} "
            f"→ **Citation**: {entry.get('citation', '?')}\n"
            f"  _{entry.get('reasoning', '')}_\n"
        )

    return f"""\
## Reference Evaluation Report

### RQ3. Reference Coverage
- **Coverage**: {coverage["coverage"]}%
- **Found**: {len(coverage.get("found", []))} references

#### Found
{found_section}

#### Missing
{missing_section}

### V1. Citation Precision
- **Precision**: {precision["cite_p"]}%

#### Per-Citation Analysis
{per_cite_section or "_No citations analyzed_"}
"""
