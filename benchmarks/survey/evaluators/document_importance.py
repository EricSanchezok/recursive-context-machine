"""DeepScholar-Bench document importance evaluator.

Uses the Semantic Scholar API for real citation counts instead of the old
keyword heuristics (e.g., ``"transformer" in title → importance=90``).

Evaluates RQ2 (Document Importance) from arXiv:2508.20033v2.
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
_SEMANTIC_SCHOLAR_BATCH_URL = (
    "https://api.semanticscholar.org/graph/v1/paper/search/bulk"
)
_SEMANTIC_SCHOLAR_FIELDS = "title,citationCount,influentialCitationCount,venue"
_REQUEST_TIMEOUT_SECONDS = 15.0
_RATE_LIMIT_PAUSE_SECONDS = 3.1

_HIGH_TIER_VENUES = frozenset(
    {
        "nature",
        "science",
        "cell",
        "pnas",
        "nature communications",
        "science advances",
        "nature methods",
        "nature genetics",
        "nature neuroscience",
        "nature immunology",
        "neuron",
        "the lancet",
        "jama",
        "new england journal of medicine",
        "icml",
        "neurips",
        "iclr",
        "cvpr",
        "iccv",
        "eccv",
        "acl",
        "emnlp",
        "naacl",
        "aaai",
        "ijcai",
        "acl",
        "sigmod",
        "vldb",
        "sosp",
        "osdi",
        "nsdi",
        "sigcomm",
        "isca",
        "micro",
        "hpca",
        "asplos",
        "pldi",
        "popl",
        "chi",
        "ubicomp",
        "cscw",
        "uist",
    }
)

_MID_TIER_VENUES = frozenset(
    {
        "arxiv preprint",
        "arxiv",
        "corr",
    }
)


def get_citation_counts(paper_titles: list[str]) -> list[dict]:
    """Look up real citation counts via Semantic Scholar API.

    Returns a list of ``{title, citation_count, influential_citation_count,
    venue}`` dicts. Papers that cannot be found get ``citation_count=0``.
    """
    if not paper_titles:
        return []

    results: list[dict] = []

    for title in paper_titles:
        result = _lookup_single_paper(title)
        results.append(result)

    logger.info(
        "Looked up %d papers; %d with citation data",
        len(paper_titles),
        sum(1 for r in results if r.get("citation_count", 0) > 0),
    )

    return results


def _lookup_single_paper(title: str) -> dict:
    """Look up a single paper title via Semantic Scholar.

    Returns ``{title, citation_count, influential_citation_count, venue}``.
    Fields default to 0/empty when unavailable.
    """
    try:
        encoded_query = request.quote(title)
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
        time.sleep(_RATE_LIMIT_PAUSE_SECONDS)

        if not papers:
            logger.debug("No Semantic Scholar result for: %s", title[:80])
            return {
                "title": title,
                "citation_count": 0,
                "influential_citation_count": 0,
                "venue": "",
            }

        paper = papers[0]
        citation_count = paper.get("citationCount") or 0
        influential_count = paper.get("influentialCitationCount") or 0
        venue = paper.get("venue", "")
        venue_text = venue if isinstance(venue, str) else ""

        return {
            "title": title,
            "citation_count": int(citation_count),
            "influential_citation_count": int(influential_count),
            "venue": venue_text,
        }

    except urllib_error.HTTPError as exc:
        if exc.code == 429:
            logger.warning("Semantic Scholar rate limited; pausing then retrying")
            time.sleep(30.0)
            try:
                return _lookup_single_paper(title)
            except Exception:
                pass
        logger.warning("Semantic Scholar HTTP %d for '%s'", exc.code, title[:80])
        return _empty_result(title)
    except Exception as exc:
        logger.warning("Semantic Scholar lookup failed for '%s': %s", title[:80], exc)
        return _empty_result(title)


def _empty_result(title: str) -> dict:
    return {
        "title": title,
        "citation_count": 0,
        "influential_citation_count": 0,
        "venue": "",
    }


def score_importance(
    paper_title: str,
    citation_count: int,
    venue: Optional[str] = None,
) -> float:
    """Score document importance on a 0-100 scale.

    Based on citation count (log-scaled) and venue tier.
    Top-tier venues get a boost; preprint-only papers get a penalty.

    Returns a float in [0, 100].
    """
    citation_score = _citation_count_to_score(citation_count)
    venue_bonus = _venue_bonus(venue or "")

    combined = min(100.0, citation_score + venue_bonus)
    return max(0.0, combined)


def _citation_count_to_score(citation_count: int) -> float:
    """Map citation count to a 0-85 score using log scaling."""
    if citation_count == 0:
        return 0.0
    if citation_count < 10:
        return 15.0
    if citation_count < 50:
        return 30.0
    if citation_count < 100:
        return 45.0
    if citation_count < 500:
        return 60.0
    if citation_count < 1000:
        return 70.0
    if citation_count < 5000:
        return 78.0
    return 85.0


def _venue_bonus(venue: str) -> float:
    """Return a venue-tier bonus in [-10, 15].

    Top-tier venues receive +15, mid/low (preprints) receive -10,
    and unrecognized venues receive 0.
    """
    venue_lower = venue.strip().lower()

    for top_venue in _HIGH_TIER_VENUES:
        if top_venue in venue_lower:
            return 15.0

    for preprint_venue in _MID_TIER_VENUES:
        if preprint_venue in venue_lower:
            return -10.0

    return 0.0


def evaluate(survey_text: str) -> dict:
    """Evaluate document importance of papers cited in the survey.

    Extracts citation titles from survey text, looks up real citation
    counts via Semantic Scholar, and scores each paper's importance.

    Returns ``{avg_importance: float, results: list[dict]}``.
    """
    titles = _extract_cited_titles(survey_text)

    if not titles:
        logger.warning("No cited paper titles extracted from survey")
        return {
            "avg_importance": 0.0,
            "results": [],
        }

    lookup_results = get_citation_counts(titles)

    results: list[dict] = []
    total_score = 0.0

    for entry in lookup_results:
        importance = score_importance(
            paper_title=entry["title"],
            citation_count=entry["citation_count"],
            venue=entry.get("venue"),
        )
        total_score += importance
        results.append(
            {
                "title": entry["title"],
                "citation_count": entry["citation_count"],
                "influential_citation_count": entry["influential_citation_count"],
                "venue": entry.get("venue", ""),
                "importance_score": round(importance, 1),
            }
        )

    avg_importance = total_score / len(results) if results else 0.0

    logger.info(
        "Document importance: avg %.1f across %d papers",
        avg_importance,
        len(results),
    )

    return {
        "avg_importance": round(avg_importance, 1),
        "results": results,
    }


_EXTRACT_TITLES_PROMPT = """\
Extract all academic paper titles that are cited in the following survey text.

Return a JSON object with a single key "titles" whose value is a list of strings.
Each string should be a paper title as it appears in the text.

Return ONLY the JSON object, no other text."""


def _extract_cited_titles(survey_text: str) -> list[str]:
    """Extract cited paper titles using LLM judge."""
    result = judge_json(_EXTRACT_TITLES_PROMPT, survey_text)
    titles = result.get("titles", [])

    if not isinstance(titles, list):
        return []

    return [str(t) for t in titles if isinstance(t, str) and t.strip()]


def generate_report(survey_text: str) -> str:
    """Generate a Markdown document importance report."""
    metrics = evaluate(survey_text)

    rows = ""
    for entry in metrics["results"][:15]:
        rows += (
            f"| {entry['title'][:60]} | "
            f"{entry['citation_count']} | "
            f"{entry.get('venue', '?')[:30]} | "
            f"{entry['importance_score']} |\n"
        )

    return f"""\
## Document Importance Report (RQ2)

- **Average Importance**: {metrics["avg_importance"]}/100
- **Papers Analyzed**: {len(metrics["results"])}

### Top Papers by Importance

| Title | Citations | Venue | Score |
|-------|-----------|-------|-------|
{rows or "| — | — | — | — |"}
"""
