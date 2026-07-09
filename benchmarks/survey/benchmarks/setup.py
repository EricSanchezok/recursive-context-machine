"""Benchmark setup helpers for the survey evaluation framework."""

from __future__ import annotations

import logging
import shutil
import subprocess
from pathlib import Path
from typing import Any

logger = logging.getLogger(__name__)

_SURVEYBENCH_REPO = "https://github.com/OpenDataBox/SurveyBench.git"
_DEEPSCHOLAR_DATASET = "deepscholar-bench/DeepScholarBench"


def _default_benchmarks_dir() -> Path:
    """Return the default benchmarks directory relative to this module."""
    return Path(__file__).resolve().parent.parent / "benchmarks"


def _resolve_dir(benchmarks_dir: str | Path | None) -> Path:
    """Return a Path for the benchmarks directory, defaulting to the repo layout."""
    if benchmarks_dir is None:
        return _default_benchmarks_dir()
    return Path(benchmarks_dir)


def setup_surveybench(benchmarks_dir: str | Path | None = None) -> bool:
    """Ensure SurveyBench reference data is available.

    If the expected HumanSurvey directory already exists, this is a no-op.
    Otherwise, it clones the OpenDataBox/SurveyBench repository.
    """
    target_dir = _resolve_dir(benchmarks_dir) / "surveybench"
    human_dir = target_dir / "data" / "HumanSurvey"

    if human_dir.is_dir() and any(human_dir.glob("*.md")):
        logger.info("SurveyBench data already present at %s", human_dir)
        return True

    target_dir.parent.mkdir(parents=True, exist_ok=True)

    if shutil.which("git") is None:
        logger.error("git is not available; cannot clone SurveyBench")
        return False

    try:
        logger.info("Cloning SurveyBench into %s", target_dir)
        subprocess.run(
            ["git", "clone", "--depth", "1", _SURVEYBENCH_REPO, str(target_dir)],
            check=True,
            capture_output=True,
            text=True,
        )
    except subprocess.CalledProcessError as exc:
        logger.error("Failed to clone SurveyBench: %s", exc.stderr)
        return False

    if not human_dir.is_dir():
        logger.error(
            "SurveyBench cloned but HumanSurvey directory not found at %s; "
            "the upstream repository layout may have changed",
            human_dir,
        )
        return False

    return True


def setup_deepscholar(benchmarks_dir: str | Path | None = None) -> bool:
    """Ensure DeepScholar-Bench reference data is available.

    If papers.csv already exists, this is a no-op. Otherwise, it tries to
    download the dataset from HuggingFace (requires the `datasets` package).
    """
    target_dir = _resolve_dir(benchmarks_dir) / "deepscholar"
    papers_csv = target_dir / "papers.csv"

    if papers_csv.is_file():
        logger.info("DeepScholar data already present at %s", target_dir)
        return True

    try:
        from datasets import load_dataset
    except ImportError as exc:
        logger.error(
            "The `datasets` package is required to download DeepScholar: %s", exc
        )
        return False

    target_dir.mkdir(parents=True, exist_ok=True)

    try:
        logger.info("Downloading DeepScholar-Bench from HuggingFace")
        ds = load_dataset(_DEEPSCHOLAR_DATASET)
    except Exception as exc:
        logger.error("Failed to load DeepScholar dataset: %s", exc)
        return False

    # Save the most relevant split. The dataset may contain one or more splits;
    # prefer a split named "test", "validation", or "train", in that order.
    split_name = None
    for candidate in ("test", "validation", "train"):
        if candidate in ds:
            split_name = candidate
            break
    if split_name is None and len(ds) > 0:
        split_name = list(ds.keys())[0]

    if split_name is None:
        logger.error("DeepScholar dataset has no splits")
        return False

    split = ds[split_name]

    # Normalize to the columns the evaluation expects.
    expected_columns = [
        "arxiv_id",
        "title",
        "abstract",
        "categories",
        "published_date",
        "pdf_related_works",
    ]

    # Map common alternative names if present.
    column_map: dict[str, str] = {
        "arxiv_id": "arxiv_id",
        "id": "arxiv_id",
        "paper_id": "arxiv_id",
        "title": "title",
        "abstract": "abstract",
        "categories": "categories",
        "category": "categories",
        "published_date": "published_date",
        "published": "published_date",
        "date": "published_date",
        "pdf_related_works": "pdf_related_works",
        "related_work": "pdf_related_works",
        "related_works": "pdf_related_works",
    }

    available_columns = set(split.column_names)
    selected: dict[str, str] = {}
    for expected in expected_columns:
        for src, dst in column_map.items():
            if dst == expected and src in available_columns and src not in selected:
                selected[src] = dst
                break

    if "arxiv_id" not in [selected.get(src) for src in selected] or "title" not in [
        selected.get(src) for src in selected
    ]:
        logger.error(
            "DeepScholar dataset is missing required columns. Available: %s",
            available_columns,
        )
        return False

    try:
        # Rename columns and write CSV.
        renamed = split.rename_columns({src: dst for src, dst in selected.items()})
        # Ensure all expected columns exist (fill missing with empty string).
        for col in expected_columns:
            if col not in renamed.column_names:
                renamed = renamed.add_column(col, [""] * len(renamed))

        renamed.to_csv(str(papers_csv))
    except Exception as exc:
        logger.error("Failed to write DeepScholar papers.csv: %s", exc)
        return False

    # Try to download important_citations.csv if the dataset provides it.
    try:
        if "important_citations" in ds:
            citations = ds["important_citations"]
            citations.to_csv(str(target_dir / "important_citations.csv"))
    except Exception as exc:
        logger.warning("Could not save important_citations.csv: %s", exc)

    return True


def setup_all(benchmarks_dir: str | Path | None = None) -> dict[str, bool]:
    """Run setup for all supported benchmarks."""
    return {
        "surveybench": setup_surveybench(benchmarks_dir),
        "deepscholar": setup_deepscholar(benchmarks_dir),
    }
