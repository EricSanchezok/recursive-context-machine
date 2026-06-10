"""DeepSurvey-Bench evaluator (arXiv:2601.15307v1).

Two categories: Surface Quality (40%) and Academic Value (60%).
Each has 3 sub-dimensions, all scored 1-5.
Surface: SQ1 (outline quality), SQ2 (content quality), SQ3 (reference quality)
Academic: AV1 (informational value), AV2 (scholarly communication), AV3 (research guidance)
Uses LLM-as-judge with the prompt template from prompts/deepsurvey.txt.
"""

from __future__ import annotations

import logging
from pathlib import Path

from evaluators.llm_judge import judge, judge_json

logger = logging.getLogger(__name__)

_PROMPTS_DIR = Path(__file__).resolve().parent.parent / "prompts"

_EVALUATE_PROMPT = """\
You are an expert evaluator conducting a **DeepSurvey-Bench** assessment following
the protocol described in arXiv:2601.15307v1.

Evaluate the survey text below on 6 dimensions, each scored 1-5. Provide specific
evidence from the text to justify each score.

## Layer 1: Surface Quality (40% Weight)

**SQ1. Outline Quality** (1-5)
- 1: No clear outline; content is disorganized
- 2: Basic structure but lacks logical progression
- 3: Generally well-organized with clear sections
- 4: Well-structured with logical flow and appropriate granularity
- 5: Expert-level organization with insightful taxonomy and seamless transitions

**SQ2. Content Quality** (1-5)
- 1: Superficial or inaccurate content
- 2: Basic descriptions but lacks depth
- 3: Accurate content with reasonable coverage
- 4: Detailed, accurate content with good coverage and some critical analysis
- 5: Comprehensive, accurate, and deeply analytical content

**SQ3. Reference Quality** (1-5)
- 1: Sparse or irrelevant references
- 2: Some relevant references but key works missing
- 3: Adequate reference coverage with most key works cited
- 4: Comprehensive references with good balance of seminal and recent works
- 5: Expert-curated references; appropriate coverage, relevance, and recency

## Layer 2: Academic Value (60% Weight)

**AV1. Informational Value** (1-5)
- 1: Contains factual errors or misleading claims; significant gaps in coverage
- 2: Mostly accurate but missing important information
- 3: Accurate with reasonable breadth; some depth in key areas
- 4: Highly informative with comprehensive coverage and accurate detail
- 5: Exceptional informational value; serves as a definitive reference on the topic

**AV2. Scholarly Communication Value** (1-5)
- 1: No contextualization; papers listed without positioning or comparison
- 2: Basic contextualization but lacks balanced treatment of competing approaches
- 3: Reasonable contextualization with some comparison; open problems identified
- 4: Strong scholarly framing; balanced comparison of approaches; clearly identified research gaps
- 5: Expert-level scholarly communication; nuanced positioning; identifies tensions and unresolved debates

**AV3. Research Guidance Value** (1-5)
- 1: No guidance for future work
- 2: Vague or generic future work suggestions
- 3: Concrete future directions that follow from the analysis
- 4: Specific, well-motivated research directions with actionable suggestions
- 5: Insightful future directions that would genuinely advance the field; prioritization and trade-offs explained

## Output Format

Return a JSON object with exactly these keys:
- "sq1": integer 1-5
- "sq2": integer 1-5
- "sq3": integer 1-5
- "av1": integer 1-5
- "av2": integer 1-5
- "av3": integer 1-5
- "reasoning_sq1": one sentence justifying the SQ1 score
- "reasoning_sq2": one sentence justifying the SQ2 score
- "reasoning_sq3": one sentence justifying the SQ3 score
- "reasoning_av1": one sentence justifying the AV1 score
- "reasoning_av2": one sentence justifying the AV2 score
- "reasoning_av3": one sentence justifying the AV3 score

Return ONLY the JSON object, no other text."""


def _load_prompt() -> str:
    """Load the full DeepSurvey-Bench evaluation prompt template."""
    prompt_path = _PROMPTS_DIR / "deepsurvey.txt"
    return prompt_path.read_text(encoding="utf-8")


def evaluate(survey_text: str) -> dict:
    """Run DeepSurvey-Bench evaluation using LLM-as-judge.

    Returns:
        dimensions: {surface: {sq1, sq2, sq3}, academic: {av1, av2, av3}}
        surface_score: float (average of sq1-sq3, 1-5 scale)
        academic_score: float (average of av1-av3, 1-5 scale)
        overall_score: float (surface x 0.4 + academic x 0.6, 1-5 scale)
    """
    result = judge_json(_EVALUATE_PROMPT, survey_text)

    sq1 = _clamp_score(result.get("sq1"))
    sq2 = _clamp_score(result.get("sq2"))
    sq3 = _clamp_score(result.get("sq3"))
    av1 = _clamp_score(result.get("av1"))
    av2 = _clamp_score(result.get("av2"))
    av3 = _clamp_score(result.get("av3"))

    surface_score = round((sq1 + sq2 + sq3) / 3, 2)
    academic_score = round((av1 + av2 + av3) / 3, 2)
    overall_score = round(surface_score * 0.4 + academic_score * 0.6, 2)

    logger.info(
        "DeepSurvey-Bench: surface=%.2f, academic=%.2f, overall=%.2f",
        surface_score,
        academic_score,
        overall_score,
    )

    return {
        "dimensions": {
            "surface": {"sq1": sq1, "sq2": sq2, "sq3": sq3},
            "academic": {"av1": av1, "av2": av2, "av3": av3},
        },
        "surface_score": surface_score,
        "academic_score": academic_score,
        "overall_score": overall_score,
    }


def generate_report(survey_text: str) -> str:
    """Generate a Markdown DeepSurvey-Bench evaluation report.

    Uses the full prompt template from prompts/deepsurvey.txt which includes
    the markdown report format with evidence, justification, and recommendations.
    """
    prompt = _load_prompt()
    return judge(prompt, survey_text)


def _clamp_score(raw: object) -> int:
    """Clamp a raw score value to the 1-5 range, defaulting to 3 on invalid input."""
    if not isinstance(raw, (int, float)):
        logger.warning("LLM judge returned non-numeric score %r; defaulting to 3", raw)
        return 3
    return max(1, min(5, int(raw)))
