"""DeepScholar-Bench organization evaluator.

LLM-as-judge scoring of structure, flow, and coherence (0-100 scale),
mapped to the KS1 metric defined in the DeepScholar-Bench protocol (arXiv:2508.20033v2).
"""

from __future__ import annotations

import logging

from evaluators.llm_judge import judge, judge_json

logger = logging.getLogger(__name__)

_ORGANIZATION_PROMPT = """\
Evaluate the organization and coherence of the following survey / related-work text.

Score the text on a 0-100 scale where:
0-20 - Disorganized, incoherent, difficult to follow
21-40 - Poorly organized with disjointed sections
41-60 - Moderately organized with reasonable flow
61-80 - Well-organized with clear structure and logical flow
81-100 - Exceptionally well-organized with coherent narrative and smooth transitions

Return a JSON object with exactly two keys:
- "score": integer from 0 to 100
- "reasoning": a concise paragraph explaining the score with specific examples

Return ONLY the JSON object, no other text."""

_ORGANIZATION_REPORT_PROMPT = """\
Provide a detailed organization and coherence assessment of the following survey text.

Structure your report with these sections:
1. Overall structure analysis (are sections logically ordered?)
2. Flow between sections (transitions, narrative thread)
3. Coherence within sections (logical paragraphing, topic sentences)
4. Strengths and weaknesses with specific examples
5. Specific recommendations for improvement

Write in Markdown format."""


def evaluate(survey_text: str) -> dict:
    """Score the organization quality of a survey text on a 0-100 scale.

    Returns ``{score: int, reasoning: str}``.
    """
    result = judge_json(_ORGANIZATION_PROMPT, survey_text)

    score = result.get("score")
    if not isinstance(score, (int, float)):
        logger.warning("LLM judge did not return a numeric score; defaulting to 50")
        score = 50

    score = max(0, min(100, int(score)))
    reasoning = result.get("reasoning", result.get("raw", "No reasoning provided"))

    logger.info("Organization score: %d/100", score)
    return {"score": score, "reasoning": reasoning}


def generate_report(survey_text: str) -> str:
    """Generate a Markdown organization assessment report."""
    return judge(_ORGANIZATION_REPORT_PROMPT, survey_text)
