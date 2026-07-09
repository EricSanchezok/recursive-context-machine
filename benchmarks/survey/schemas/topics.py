"""Schema loaders for survey benchmark topics and tasks."""

import csv
import json
import logging
from pathlib import Path
from typing import Any

logger = logging.getLogger(__name__)


def load_surveybench_topics(benchmarks_dir: str) -> list[dict[str, Any]]:
    """Load the 20 SurveyBench topics from the cloned repo's HumanSurvey directory.

    Assumes setup_surveybench() has been called first. Each topic is read from
    the .md files under data/HumanSurvey/, cross-referenced with outlines.json.

    Returns list of {id, topic, reference_path, outline} dicts.
    """
    human_dir = Path(benchmarks_dir) / "surveybench" / "data" / "HumanSurvey"

    if not human_dir.is_dir():
        logger.warning(
            "SurveyBench HumanSurvey directory not found at %s — run setup_surveybench() first",
            human_dir,
        )
        return []

    outlines_file = human_dir / "outlines.json"
    outlines: dict[str, str] = {}
    if outlines_file.is_file():
        data = json.loads(outlines_file.read_text(encoding="utf-8"))
        if isinstance(data, dict):
            outlines = data
        elif isinstance(data, list):
            for item in data:
                if isinstance(item, dict) and "topic" in item:
                    outlines[item["topic"]] = item.get("outline", "")

    topics: list[dict[str, Any]] = []
    for idx, md_path in enumerate(sorted(human_dir.glob("*.md"))):
        topic_name = md_path.stem
        topics.append(
            {
                "id": idx,
                "topic": topic_name,
                "reference_path": str(md_path.resolve()),
                "outline": outlines.get(topic_name, ""),
            }
        )

    logger.info("Loaded %d SurveyBench topics from %s", len(topics), human_dir)
    return topics


def load_deepscholar_tasks(benchmarks_dir: str) -> list[dict[str, Any]]:
    """Load the 63 DeepScholar-Bench test cases.

    Reads papers.csv from the downloaded dataset. Assumes setup_deepscholar()
    has been called first.

    Returns list of {arxiv_id, title, abstract, categories, published_date} dicts.
    """
    papers_csv = Path(benchmarks_dir) / "deepscholar" / "papers.csv"

    if not papers_csv.is_file():
        logger.warning(
            "DeepScholar papers CSV not found at %s — run setup_deepscholar() first",
            papers_csv,
        )
        return []

    tasks: list[dict[str, Any]] = []
    with open(papers_csv, encoding="utf-8", newline="") as fh:
        reader = csv.DictReader(fh)
        for row in reader:
            tasks.append(
                {
                    "arxiv_id": row["arxiv_id"],
                    "title": row["title"],
                    "abstract": row["abstract"],
                    "categories": row.get("categories", ""),
                    "published_date": row.get("published_date", ""),
                    "pdf_related_works": row.get("pdf_related_works", ""),
                }
            )
    return tasks


def list_available() -> dict[str, dict[str, Any]]:
    """Return {benchmark_name: {setup_needed, topic_count} dict} for each benchmark.

    Uses the default benchmarks directory: benchmarks/survey/benchmarks/
    """
    default_dir = Path(__file__).resolve().parent.parent / "benchmarks"

    human_dir = default_dir / "surveybench" / "data" / "HumanSurvey"
    papers_csv = default_dir / "deepscholar" / "papers.csv"

    topic_count = 0
    if human_dir.is_dir():
        topic_count = len(list(human_dir.glob("*.md")))

    task_count = 0
    if papers_csv.is_file():
        task_count = _count_csv_rows(papers_csv)

    return {
        "surveybench": {
            "setup_needed": topic_count == 0,
            "topic_count": topic_count,
        },
        "deepscholar": {
            "setup_needed": task_count == 0,
            "topic_count": task_count,
        },
    }


def _count_csv_rows(path: Path) -> int:
    """Count data rows in a CSV file (excluding header).

    Returns 0 if the file cannot be read.
    """
    try:
        with open(path, encoding="utf-8", newline="") as fh:
            reader = csv.reader(fh)
            next(reader, None)  # skip header
            return sum(1 for _ in reader)
    except Exception:
        return 0
