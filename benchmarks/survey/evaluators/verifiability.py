"""DeepScholar-Bench verifiability evaluator.

Checks whether claims in the survey are supported by cited sources.
Uses the Semantic Scholar API to retrieve paper abstracts for real
verification instead of asking the LLM to hallucinate support.

Protocol: V2 (Claim Coverage) from arXiv:2508.20033v2.
"""

from __future__ import annotations

import json
import logging
import time
from typing import Any, Optional
from urllib import request, error as urllib_error

from evaluators.llm_judge import judge, judge_json

logger = logging.getLogger(__name__)

_SEMANTIC_SCHOLAR_SEARCH_URL = "https://api.semanticscholar.org/graph/v1/paper/search"
_SEMANTIC_SCHOLAR_ABSTRACT_URL = (
    "https://api.semanticscholar.org/graph/v1/paper/{paper_id}"
)
_SEMANTIC_SCHOLAR_FIELDS = "title,abstract,year,authors,citationCount"
_REQUEST_TIMEOUT_SECONDS = 15.0
_RATE_LIMIT_PAUSE_SECONDS = 3.1  # ~100 requests per 5 minutes

_EXTRACT_CLAIMS_PROMPT = """\
Extract every substantive claim and its associated citation from the survey text below.

A "claim" is a factual assertion or statement attributed to a cited source.
Exclude generic statements, structural transitions, and author opinions
that are not backed by a citation.

For each claim, identify:
- The claim text
- The citation marker (e.g., "[1]", "Smith et al. 2020")
- The section of the survey where it appears (Introduction, Methods, etc.)

Return a JSON object with a single key "claims" whose value is a list of objects,
each with keys: "claim", "citation", "section".

Return ONLY the JSON object, no other text."""

_VERIFY_CLAIM_PROMPT = """\
You are verifying whether a cited paper supports a specific claim from a survey.

CLAIM: {claim}

CITATION: {citation}

PAPER ABSTRACT: {abstract}

Based on the paper abstract above, does the cited paper support this claim?
Consider:
- The abstract summarizes the paper's contributions and findings
- If the abstract mentions similar methods, findings, or claims → "Yes"
- If the abstract clearly contradicts or is about something different → "No"
- If the abstract is too vague to determine → "Unverified"

Return a JSON object with exactly these keys:
- "supported": one of "Yes", "No", or "Unverified"
- "confidence": integer 0-100
- "reasoning": one sentence explaining your decision

Return ONLY the JSON object, no other text."""


def extract_claims(survey_text: str) -> list[dict]:
    """Extract claims with their citations from survey text.

    Returns a list of ``{claim: str, citation: str, section: str}`` dicts.
    """
    result = judge_json(_EXTRACT_CLAIMS_PROMPT, survey_text)
    claims = result.get("claims", [])

    if not isinstance(claims, list):
        logger.warning("LLM judge returned non-list claims; treating as empty")
        return []

    parsed: list[dict] = []
    for entry in claims:
        if not isinstance(entry, dict):
            continue
        parsed.append(
            {
                "claim": str(entry.get("claim", "")),
                "citation": str(entry.get("citation", "")),
                "section": str(entry.get("section", "")),
            }
        )

    logger.info("Extracted %d claims from survey text", len(parsed))
    return parsed


def lookup_abstract(citation: str) -> Optional[str]:
    """Look up a paper abstract via the Semantic Scholar API.

    Searches by ``citation`` text and returns the abstract of the top match,
    or ``None`` if the lookup fails.
    """
    try:
        encoded_query = request.quote(citation)
        search_url = (
            f"{_SEMANTIC_SCHOLAR_SEARCH_URL}"
            f"?query={encoded_query}"
            f"&limit=1"
            f"&fields={_SEMANTIC_SCHOLAR_FIELDS}"
        )

        req = request.Request(search_url)
        req.add_header("Accept", "application/json")
        req.add_header("User-Agent", "RCM-DeepScholar-Evaluator/1.0")

        with request.urlopen(req, timeout=_REQUEST_TIMEOUT_SECONDS) as resp:
            body = resp.read().decode("utf-8")
            data = json.loads(body)

        papers = data.get("data", [])
        if not papers:
            logger.debug("No Semantic Scholar results for citation: %s", citation)
            return None

        top_paper = papers[0]
        abstract = top_paper.get("abstract")
        title = top_paper.get("title", "Unknown")

        time.sleep(_RATE_LIMIT_PAUSE_SECONDS)

        if abstract:
            logger.debug("Found abstract for '%s' via %s", citation, title)
            return str(abstract)

        logger.debug(
            "Paper found but no abstract available for '%s': %s", citation, title
        )
        return None

    except urllib_error.HTTPError as exc:
        if exc.code == 429:
            logger.warning("Semantic Scholar rate limited; pausing then retrying")
            time.sleep(30.0)
            try:
                return lookup_abstract(citation)
            except Exception:
                return None
        logger.warning("Semantic Scholar HTTP %d for '%s'", exc.code, citation)
        return None
    except Exception as exc:
        logger.warning("Semantic Scholar lookup failed for '%s': %s", citation, exc)
        return None


def verify_claim(
    claim: str,
    citation: str,
    paper_abstract: Optional[str] = None,
) -> dict:
    """Verify whether a citation supports the claim.

    If ``paper_abstract`` is provided, the LLM judge checks alignment
    between the claim and the abstract. Otherwise, the claim is marked
    as ``"Unverified"`` — this function does NOT hallucinate.

    Returns ``{supported: str, confidence: int, reasoning: str}``.
    """
    if paper_abstract is None:
        return {
            "supported": "Unverified",
            "confidence": 0,
            "reasoning": "Paper abstract not available for verification",
        }

    prompt = _VERIFY_CLAIM_PROMPT.format(
        claim=claim, citation=citation, abstract=paper_abstract
    )

    result = judge_json(prompt, "")
    supported = result.get("supported", "Unverified")
    confidence = result.get("confidence", 0)
    reasoning = result.get("reasoning", "")

    if supported not in ("Yes", "No", "Unverified"):
        supported = "Unverified"

    if not isinstance(confidence, (int, float)):
        confidence = 0
    confidence = max(0, min(100, int(confidence)))

    return {
        "supported": supported,
        "confidence": confidence,
        "reasoning": str(reasoning),
    }


def evaluate(survey_text: str) -> dict:
    """Full verifiability evaluation.

    Extracts claims, looks up supporting paper abstracts via Semantic Scholar,
    and verifies each claim-citation pair.

    Returns a dict with ``claim_coverage`` (0-100), ``verified_claims``,
    ``total_claims``, and ``results`` (list of per-claim dicts).
    """
    claims = extract_claims(survey_text)

    if not claims:
        logger.warning("No claims extracted; returning perfect score")
        return {
            "claim_coverage": 100.0,
            "verified_claims": 0,
            "total_claims": 0,
            "results": [],
        }

    results: list[dict] = []
    verified_count = 0

    for entry in claims:
        citation = entry["citation"]
        abstract = lookup_abstract(citation) if citation else None
        verification = verify_claim(
            claim=entry["claim"],
            citation=citation,
            paper_abstract=abstract,
        )

        supported = verification["supported"] == "Yes"
        if supported:
            verified_count += 1

        results.append(
            {
                "claim": entry["claim"],
                "citation": citation,
                "section": entry["section"],
                "abstract_available": abstract is not None,
                **verification,
            }
        )

    total = len(claims)
    claim_coverage = (verified_count / total) * 100.0

    logger.info(
        "Verifiability: %.1f%% (%d/%d claims verified)",
        claim_coverage,
        verified_count,
        total,
    )

    return {
        "claim_coverage": round(claim_coverage, 1),
        "verified_claims": verified_count,
        "total_claims": total,
        "results": results,
    }


def generate_report(survey_text: str) -> str:
    """Generate a Markdown verifiability report."""
    results = evaluate(survey_text)

    claim_rows = ""
    for entry in results["results"][:10]:
        abstract_marker = "✓" if entry.get("abstract_available") else "✗"
        status_icon = {
            "Yes": "✓",
            "No": "✗",
            "Unverified": "?",
        }.get(entry.get("supported", "Unverified"), "?")

        claim_rows += (
            f"| {status_icon} | {abstract_marker} | "
            f"{entry.get('claim', '?')[:80]}... | "
            f"{entry.get('citation', '?')} | "
            f"{entry.get('confidence', 0)}% |\n"
        )

    with_abstract = sum(1 for r in results["results"] if r.get("abstract_available"))

    return f"""\
## Verifiability Report

- **Claim Coverage**: {results["claim_coverage"]}%
- **Verified Claims**: {results["verified_claims"]}/{results["total_claims"]}
- **Abstracts Available**: {with_abstract}/{results["total_claims"]}

### Per-Claim Verification

| Status | Abstract | Claim | Citation | Confidence |
|--------|----------|-------|----------|------------|
{claim_rows or "| — | — | _No claims extracted_ | — | — |"}
"""
