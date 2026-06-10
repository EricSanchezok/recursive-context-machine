#!/usr/bin/env python3
"""Survey Benchmark Framework — unified CLI.

Commands:
  run.py setup    <benchmark>        Fetch benchmark data (SurveyBench, DeepScholar)
  run.py generate --pipeline <name> --benchmark <name> [--topics ...]
                                     Run RCM pipeline to generate surveys
  run.py evaluate --pipeline <name> --benchmark <name> [--topics ...]
                                     Evaluate generated surveys
  run.py report   --pipeline <name> --benchmark <name>  [--compare ...]
                                     Aggregate results into summary report
  run.py list     [benchmarks|pipelines|topics]
                                     List available resources
"""

from __future__ import annotations

import argparse
import concurrent.futures
import csv
import json
import logging
import math
import os
import subprocess
import sys
from pathlib import Path
from typing import Any

# ── path setup ───────────────────────────────────────────────────────────────

_SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(_SCRIPT_DIR))

from configs import load_config, list_configs, validate_config
from schemas.topics import (
    load_surveybench_topics,
    load_deepscholar_tasks,
    list_available,
)
from benchmarks.setup import (
    setup_surveybench,
    setup_deepscholar,
    setup_all,
)
from evaluators import content, quiz
from evaluators import deepsurvey
from evaluators import (
    organization,
    nugget,
    reference,
    verifiability,
    document_importance,
)

# ── constants ────────────────────────────────────────────────────────────────

_REPO_ROOT = _SCRIPT_DIR.parent.parent
_BENCHMARKS_DIR = _SCRIPT_DIR / "benchmarks"
_GENERATED_DIR = _SCRIPT_DIR / "generated"
_RESULTS_DIR = _SCRIPT_DIR / "results"

_DEFAULT_TIMEOUT_SECONDS = 7200
_DEFAULT_MAX_WORKERS = 4

logger = logging.getLogger("survey.runner")


# ── topic helpers ────────────────────────────────────────────────────────────


def _load_topics(benchmark: str) -> list[dict[str, Any]]:
    """Load topic dicts for the given benchmark name."""
    benchmarks_dir = str(_BENCHMARKS_DIR)
    if benchmark == "surveybench":
        return load_surveybench_topics(benchmarks_dir)
    if benchmark == "deepscholar":
        return load_deepscholar_tasks(benchmarks_dir)
    print(f"Unknown benchmark: {benchmark}", file=sys.stderr)
    return []


def _topic_key(topic: dict) -> str:
    """Return a stable display identifier for a topic dict."""
    return (
        topic.get("topic")
        or topic.get("title")
        or topic.get("arxiv_id")
        or str(topic.get("id", ""))
    )


def _topic_dir_name(topic: dict) -> str:
    """Return a filesystem-safe directory name for a topic."""
    name = _topic_key(topic)
    safe = "".join(c if c.isalnum() or c in " -_" else "_" for c in name)
    safe = safe.strip().replace(" ", "_")[:120]
    return safe or f"topic_{topic.get('id', 0)}"


def _filter_topics(
    topics: list[dict[str, Any]],
    topic_filters: list[str] | None,
) -> list[dict[str, Any]]:
    """Filter topics by name/id/index if filters are provided."""
    if not topic_filters:
        return topics

    selected: list[dict[str, Any]] = []
    filter_set = {f.lower().strip() for f in topic_filters}

    for idx, topic in enumerate(topics):
        topic_id = str(topic.get("id", idx))
        topic_name = _topic_key(topic).lower()
        if any(
            f in topic_id or f in topic_name or f == topic_id or f == topic_name
            for f in filter_set
        ):
            selected.append(topic)

    return selected


# ── mock detection ───────────────────────────────────────────────────────────


def _is_mock() -> bool:
    return not bool(os.environ.get("EVA_API_KEY") or os.environ.get("OPENAI_API_KEY"))


def _print_mock_warning() -> None:
    print(
        "\n\u26a0  No API key configured \u2014 evaluations will produce MOCK scores.",
        file=sys.stderr,
    )
    print(
        "   Set EVA_API_KEY or OPENAI_API_KEY.",
        file=sys.stderr,
    )


# ── command: list ────────────────────────────────────────────────────────────


def _cmd_list(args: argparse.Namespace) -> None:
    resource = args.resource

    if resource == "benchmarks":
        info = list_available()
        for name, details in info.items():
            status = (
                "\u2713 ready" if not details["setup_needed"] else "\u2717 needs setup"
            )
            print(f"  {name:20s}  {status:14s}  ({details['topic_count']} topics)")

    elif resource == "pipelines":
        configs = list_configs()
        if not configs:
            print("  No pipeline configs found.")
            return
        for cfg_name in configs:
            config = load_config(cfg_name)
            desc = ""
            if config:
                pip = config.get("pipeline", {})
                desc = pip.get("description", "")
            print(f"  {cfg_name:20s}  {desc}")

    elif resource == "topics":
        if not args.benchmark:
            print("--benchmark is required for `list topics`", file=sys.stderr)
            print("  Choices: surveybench, deepscholar", file=sys.stderr)
            return
        topics = _load_topics(args.benchmark)
        if not topics:
            print(
                f"  No topics for {args.benchmark}. Run `run.py setup {args.benchmark}` first."
            )
            return
        for t in topics:
            print(f"  [{t.get('id', '?')}] {_topic_key(t)}")


# ── command: setup ───────────────────────────────────────────────────────────


def _cmd_setup(args: argparse.Namespace) -> None:
    benchmark = args.benchmark
    benchmarks_dir = str(_BENCHMARKS_DIR)

    if benchmark == "surveybench":
        ok = setup_surveybench(benchmarks_dir)
    elif benchmark == "deepscholar":
        ok = setup_deepscholar(benchmarks_dir)
    elif benchmark == "all":
        results = setup_all(benchmarks_dir)
        ok = all(results.values())
        for name, success in results.items():
            status = "\u2713" if success else "\u2717"
            print(f"  {status} {name}")
    else:
        print(f"Unknown benchmark: {benchmark}", file=sys.stderr)
        return

    if ok:
        print(f"  \u2713 {benchmark} setup complete")
    else:
        print(f"  \u2717 {benchmark} setup failed", file=sys.stderr)


# ── command: generate ────────────────────────────────────────────────────────


def _run_generation(
    topic: dict[str, Any],
    config: dict[str, Any],
    output_dir: Path,
    topic_index: int,
    total: int,
) -> bool:
    """Generate a single survey for one topic. Returns True on success."""
    topic_name = _topic_key(topic)
    gen_cfg = config["generation"]
    entry_graph = gen_cfg["entry_graph"]
    working_dir = _REPO_ROOT / gen_cfg.get("working_dir", ".")
    final_output = gen_cfg["final_output"]
    target_name = config.get("output", {}).get("target_name", "output.md")
    timeout = gen_cfg.get("timeout_seconds", _DEFAULT_TIMEOUT_SECONDS)
    env_vars = gen_cfg.get("env", [])

    output_dir.mkdir(parents=True, exist_ok=True)

    # Build environment
    subprocess_env = os.environ.copy()
    for var in env_vars:
        if isinstance(var, str) and var in os.environ:
            subprocess_env[var] = os.environ[var]

    purpose = topic.get("topic") or topic.get("title") or topic_name

    cmd = [
        "accelerator",
        "run",
        entry_graph,
        "--purpose",
        purpose,
        "--speed",
        "0",
        "--context",
    ]

    status = f"[{topic_index}/{total}] {topic_name}"

    try:
        result = subprocess.run(
            cmd,
            cwd=str(working_dir),
            env=subprocess_env,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        if result.returncode != 0:
            stderr_tail = result.stderr.strip()[-200:] if result.stderr else ""
            print(
                f"{status} \u2717 (exit {result.returncode}) {stderr_tail}",
                file=sys.stderr,
            )
            return False

        # Copy final output
        generated_file = working_dir / final_output
        if generated_file.is_file():
            dest = output_dir / target_name
            dest.write_text(generated_file.read_text(encoding="utf-8"))
            print(f"{status} \u2713")
            return True

        print(
            f"{status} \u2717 (final_output not found: {final_output})", file=sys.stderr
        )
        return False

    except subprocess.TimeoutExpired:
        print(f"{status} \u2717 (timeout after {timeout}s)", file=sys.stderr)
        return False
    except FileNotFoundError:
        print(
            f"{status} \u2717 (accelerator not found — install it first)",
            file=sys.stderr,
        )
        return False
    except Exception as exc:
        logger.error("Generation failed for %s: %s", topic_name, exc)
        print(f"{status} \u2717 ({exc})", file=sys.stderr)
        return False


def _cmd_generate(args: argparse.Namespace) -> None:
    pipeline = args.pipeline
    benchmark = args.benchmark
    topic_filters = args.topics
    max_workers = args.max_workers or _DEFAULT_MAX_WORKERS

    config = load_config(pipeline)
    if config is None:
        print(f"Pipeline config '{pipeline}' not found.", file=sys.stderr)
        return

    errors = validate_config(config)
    if errors:
        for e in errors:
            print(f"  Config error: {e}", file=sys.stderr)
        return

    topics = _load_topics(benchmark)
    if not topics:
        print(
            f"No topics loaded for benchmark '{benchmark}'. Run setup first.",
            file=sys.stderr,
        )
        return

    topics = _filter_topics(topics, topic_filters)
    if not topics:
        print("No topics matched the filter.", file=sys.stderr)
        return

    total = len(topics)
    print(
        f"Generating {total} surveys with pipeline '{pipeline}' on benchmark '{benchmark}'"
    )
    print(f"  Working dir: {_REPO_ROOT / config['generation'].get('working_dir', '.')}")
    print(f"  Entry graph: {config['generation']['entry_graph']}")
    print(
        f"  Timeout: {config['generation'].get('timeout_seconds', _DEFAULT_TIMEOUT_SECONDS)}s"
    )
    print(f"  Workers: {max_workers}")
    print()

    succeeded = 0
    failed = 0

    futures: list[concurrent.futures.Future] = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
        for idx, topic in enumerate(topics, start=1):
            dir_name = _topic_dir_name(topic)
            output_dir = _GENERATED_DIR / pipeline / dir_name
            future = executor.submit(
                _run_generation, topic, config, output_dir, idx, total
            )
            futures.append(future)

        for future in concurrent.futures.as_completed(futures):
            if future.result():
                succeeded += 1
            else:
                failed += 1

    print()
    print(f"  Summary: {succeeded}/{total} succeeded, {failed} failed")


# ── command: evaluate ────────────────────────────────────────────────────────


def _run_evaluation(
    topic: dict[str, Any],
    pipeline: str,
    benchmark: str,
    output_dir: Path,
    topic_index: int,
    total: int,
) -> dict[str, Any] | None:
    """Evaluate one generated survey. Returns the result dict or None on failure."""
    topic_name = _topic_key(topic)
    dir_name = _topic_dir_name(topic)
    survey_path = _GENERATED_DIR / pipeline / dir_name / "output.md"

    if not survey_path.is_file():
        print(
            f"  [{topic_index}/{total}] {topic_name} \u2717 (no output.md)",
            file=sys.stderr,
        )
        return None

    survey_text = survey_path.read_text(encoding="utf-8")
    output_dir.mkdir(parents=True, exist_ok=True)

    try:
        if benchmark == "surveybench":
            report = _evaluate_surveybench(topic, survey_text)
        elif benchmark == "deepscholar":
            report = _evaluate_deepscholar(topic, survey_text)

        # Write JSON report
        json_path = output_dir / "report.json"
        json_path.write_text(
            json.dumps(report, indent=2, default=str), encoding="utf-8"
        )

        # Write Markdown report
        md_path = output_dir / "report.md"
        md_text = _format_eval_report(report, benchmark, topic_name)
        md_path.write_text(md_text, encoding="utf-8")

        print(f"  [{topic_index}/{total}] {topic_name} \u2713")
        return report

    except Exception as exc:
        logger.error("Evaluation failed for %s: %s", topic_name, exc)
        print(f"  [{topic_index}/{total}] {topic_name} \u2717 ({exc})", file=sys.stderr)
        return None


def _evaluate_surveybench(topic: dict, survey_text: str) -> dict[str, Any]:
    """Run SurveyBench (content + outline + quiz) evaluation."""
    reference_text = None
    reference_path = topic.get("reference_path")
    if reference_path and Path(reference_path).is_file():
        reference_text = Path(reference_path).read_text(encoding="utf-8")

    reference_outline = topic.get("outline") or None

    content_result = content.evaluate_content(survey_text, reference_text)
    outline_result = content.evaluate_outline(survey_text, reference_outline)
    quiz_result = quiz.evaluate(survey_text, topic.get("topic", ""), num_questions=5)

    return {
        "benchmark": "surveybench",
        "content": content_result,
        "outline": outline_result,
        "quiz": quiz_result,
        "mock": _is_mock(),
    }


def _evaluate_deepsurvey(survey_text: str) -> dict[str, Any]:
    """Run DeepSurvey-Bench evaluation."""
    result = deepsurvey.evaluate(survey_text)
    return {
        "benchmark": "deepsurvey",
        "deepsurvey": result,
        "mock": _is_mock(),
    }


def _evaluate_deepscholar(topic: dict, survey_text: str) -> dict[str, Any]:
    """Run DeepScholar-Bench evaluation (5 dimensions + geometric mean).

    Loads reference text (pdf_related_works) and important_citations from the
    benchmark dataset. When data is unavailable (setup not run), missing
    metrics are marked as N/A.
    """
    arxiv_id = topic.get("arxiv_id", "")

    # Reference text for nugget coverage: ground-truth Related Work section
    reference_text = topic.get("pdf_related_works", "")
    has_reference = bool(reference_text and reference_text.strip())

    # Important citations for reference coverage
    important_citations = _load_important_citations(arxiv_id)
    has_citations = bool(important_citations)

    org_result = organization.evaluate(survey_text)

    if has_reference:
        nugget_result = nugget.evaluate(survey_text, reference_text)
    else:
        nugget_result = {
            "nugget_coverage": "N/A",
            "matched_nuggets": 0,
            "total_nuggets": 0,
            "matched": [],
            "missed": [],
        }

    if has_citations:
        ref_cov_result = reference.evaluate_reference_coverage(
            survey_text, important_citations
        )
    else:
        ref_cov_result = {
            "coverage": "N/A",
            "found": [],
            "missing": [],
        }

    cite_prec_result = reference.evaluate_citation_precision(survey_text)
    verif_result = verifiability.evaluate(survey_text)
    docimp_result = document_importance.evaluate(survey_text)

    report = {
        "benchmark": "deepscholar",
        "organization": org_result,
        "nugget_coverage": nugget_result,
        "reference_coverage": ref_cov_result,
        "citation_precision": cite_prec_result,
        "verifiability": verif_result,
        "document_importance": docimp_result,
        "mock": _is_mock(),
    }

    # Geometric mean overall score per arXiv:2508.20033v2 §3
    # KS = (KS1 + KS2) / 2, RQ = (RQ2 + RQ3) / 2, V = (V1 + V2) / 2
    # Overall = (KS^0.4 × RQ^0.3 × V^0.3)^(1/(0.4+0.3+0.3))
    overall = _compute_deepscholar_overall(report)
    report["overall_score"] = overall

    return report


def _load_important_citations(arxiv_id: str) -> list[dict]:
    """Load important citations for a given arxiv_id from the benchmark dataset.

    Reads important_citations.csv from benchmarks/survey/benchmarks/deepscholar/
    and filters by ``source_arxiv_id``. Returns a list of dicts with ``title`` keys.
    """
    csv_path = _BENCHMARKS_DIR / "deepscholar" / "important_citations.csv"
    if not csv_path.is_file():
        return []

    try:
        citations: list[dict] = []
        with open(csv_path, encoding="utf-8", newline="") as fh:
            reader = csv.DictReader(fh)
            for row in reader:
                if row.get("source_arxiv_id", "").strip() == arxiv_id.strip():
                    citations.append({"title": row.get("title", "").strip()})
        return citations
    except Exception as exc:
        logger.warning("Failed to load important_citations for %s: %s", arxiv_id, exc)
        return []


def _compute_deepscholar_overall(report: dict) -> dict:
    """Compute the geometric mean overall score from DeepScholar sub-scores.

    Formula from arXiv:2508.20033v2 §3:
      KS = (KS1 + KS2) / 2     — Organization (KS1) + Coverage (KS2 ≈ nugget coverage)
      RQ = (RQ2 + RQ3) / 2     — Document Importance (RQ2) + Reference Coverage (RQ3)
      V  = (V1 + V2) / 2       — Citation Precision (V1) + Verifiability (V2)
      Overall = (KS^0.4 × RQ^0.3 × V^0.3)^(1/(0.4+0.3+0.3))

    When any sub-score is N/A, the corresponding dimension is excluded from
    the geometric mean and the weight is re-normalized.

    Returns ``{score, ks, rq, v, ks1, ks2, rq2, rq3, v1, v2}``.
    """

    def _numeric(value: Any) -> float:
        if isinstance(value, (int, float)):
            return float(value)
        return 0.0

    org_score = _numeric(report.get("organization", {}).get("score", 0))
    nugget_cov = _numeric(report.get("nugget_coverage", {}).get("nugget_coverage", 0))
    ref_cov = _numeric(report.get("reference_coverage", {}).get("coverage", 0))
    cite_prec = _numeric(report.get("citation_precision", {}).get("cite_p", 0))
    verif_cov = _numeric(report.get("verifiability", {}).get("claim_coverage", 0))
    docimp_avg = _numeric(
        report.get("document_importance", {}).get("avg_importance", 0)
    )

    # All sub-scores are 0-100 in the DeepScholar protocol
    ks1 = org_score
    ks2 = nugget_cov
    ks = (ks1 + ks2) / 2.0

    rq2 = docimp_avg
    rq3 = ref_cov
    rq = (rq2 + rq3) / 2.0

    v1 = cite_prec
    v2 = verif_cov
    v = (v1 + v2) / 2.0

    epsilon = 1e-6
    ks = max(epsilon, ks)
    rq = max(epsilon, rq)
    v = max(epsilon, v)

    w_ks = 0.4
    w_rq = 0.3
    w_v = 0.3

    log_sum = w_ks * math.log(ks) + w_rq * math.log(rq) + w_v * math.log(v)
    weight_sum = w_ks + w_rq + w_v
    overall = math.exp(log_sum / weight_sum)

    return {
        "score": round(overall, 1),
        "ks": round(ks, 1),
        "rq": round(rq, 1),
        "v": round(v, 1),
        "ks1": round(ks1, 1),
        "ks2": round(ks2, 1),
        "rq2": round(rq2, 1),
        "rq3": round(rq3, 1),
        "v1": round(v1, 1),
        "v2": round(v2, 1),
    }


def _format_eval_report(report: dict, benchmark: str, topic_name: str) -> str:
    """Format evaluation results as a human-readable markdown snippet."""
    mock_banner = ""
    if report.get("mock"):
        mock_banner = (
            "\n> \u26a0\ufe0f **MOCK EVALUATION** \u2014 no EVA_API_KEY or OPENAI_API_KEY "
            "configured.\n"
        )

    lines = [f"# Evaluation Report — {topic_name}", mock_banner]

    if benchmark == "surveybench":
        content_result = report.get("content", {})
        outline_result = report.get("outline", {})
        quiz_res = report.get("quiz", {})

        lines.append(
            f"\n**Content Score**: {content_result.get('overall_score', 'N/A')}/5.0"
        )
        lines.append(
            f"**Outline Score**: {outline_result.get('overall_score', 'N/A')}/5.0"
        )
        lines.append(
            f"**Quiz Answerability**: {quiz_res.get('overall_answerability', 'N/A')}/100"
        )

    elif benchmark == "deepsurvey":
        ds = report.get("deepsurvey", {})
        lines.append(f"\n**Surface Score**: {ds.get('surface_score', 'N/A')}/5.0")
        lines.append(f"**Academic Score**: {ds.get('academic_score', 'N/A')}/5.0")
        lines.append(f"**Overall Score**: {ds.get('overall_score', 'N/A')}/5.0")

    elif benchmark == "deepscholar":
        org = report.get("organization", {})
        nug = report.get("nugget_coverage", {})
        ref_cov = report.get("reference_coverage", {})
        cite_prec = report.get("citation_precision", {})
        verif = report.get("verifiability", {})
        docimp = report.get("document_importance", {})

        lines.append(f"\n**Organization (KS1)**: {org.get('score', 'N/A')}/100")
        lines.append(f"**Nugget Coverage**: {nug.get('nugget_coverage', 'N/A')}%")
        lines.append(f"**Reference Coverage**: {ref_cov.get('coverage', 'N/A')}%")
        lines.append(f"**Citation Precision**: {cite_prec.get('cite_p', 'N/A')}%")
        lines.append(f"**Verifiability**: {verif.get('claim_coverage', 'N/A')}%")
        lines.append(
            f"**Document Importance**: {docimp.get('avg_importance', 'N/A')}/100"
        )
        overall_score = report.get("overall_score", {})
        lines.append(
            f"**Overall Score (geom. mean)**: {overall_score.get('score', 'N/A')}/100"
        )
        lines.append(
            "  (KS: "
            + str(overall_score.get("ks", "N/A"))
            + ", RQ: "
            + str(overall_score.get("rq", "N/A"))
            + ", V: "
            + str(overall_score.get("v", "N/A"))
            + ")"
        )
        lines.append("")
        lines.append(
            "> **Note**: RQ1 (Relevance Rate) is not yet implemented. "
            "See arXiv:2508.20033v2 §3 for the metric definition."
        )

    return "\n".join(lines)


def _cmd_evaluate(args: argparse.Namespace) -> None:
    pipeline = args.pipeline
    benchmark = args.benchmark
    topic_filters = args.topics
    max_workers = args.max_workers or _DEFAULT_MAX_WORKERS

    if _is_mock():
        _print_mock_warning()
        print()

    config = load_config(pipeline)
    if config is None:
        print(f"Pipeline config '{pipeline}' not found.", file=sys.stderr)
        return

    topics = _load_topics(benchmark)
    if not topics:
        print(
            f"No topics loaded for benchmark '{benchmark}'. Run setup first.",
            file=sys.stderr,
        )
        return

    topics = _filter_topics(topics, topic_filters)
    if not topics:
        print("No topics matched the filter.", file=sys.stderr)
        return

    total = len(topics)
    print(
        f"Evaluating {total} surveys from pipeline '{pipeline}' on benchmark '{benchmark}'"
    )
    print()

    succeeded = 0
    failed = 0
    all_scores: list[dict[str, Any]] = []

    with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures_map: dict[concurrent.futures.Future, int] = {}
        for idx, topic in enumerate(topics, start=1):
            dir_name = _topic_dir_name(topic)
            output_dir = _RESULTS_DIR / pipeline / benchmark / dir_name
            future = executor.submit(
                _run_evaluation, topic, pipeline, benchmark, output_dir, idx, total
            )
            futures_map[future] = idx

        for future in concurrent.futures.as_completed(futures_map):
            result = future.result()
            if result is not None:
                succeeded += 1
                all_scores.append(result)
            else:
                failed += 1

    print()
    print(f"  Summary: {succeeded}/{total} succeeded, {failed} failed")

    if all_scores:
        _print_average_scores(all_scores, benchmark)


def _print_average_scores(all_results: list[dict], benchmark: str) -> None:
    """Compute and print average scores across all evaluated topics."""
    print()

    if benchmark == "surveybench":
        content_scores = [
            r.get("content", {}).get("overall_score", 0) for r in all_results
        ]
        outline_scores = [
            r.get("outline", {}).get("overall_score", 0) for r in all_results
        ]
        quiz_scores = [
            r.get("quiz", {}).get("overall_answerability", 0) for r in all_results
        ]
        print(f"  Avg Content Score:  {_safe_mean(content_scores):.2f}/5.0")
        print(f"  Avg Outline Score:  {_safe_mean(outline_scores):.2f}/5.0")
        print(f"  Avg Quiz Score:     {_safe_mean(quiz_scores):.1f}/100")

    elif benchmark == "deepsurvey":
        surface = [r.get("deepsurvey", {}).get("surface_score", 0) for r in all_results]
        academic = [
            r.get("deepsurvey", {}).get("academic_score", 0) for r in all_results
        ]
        overall = [r.get("deepsurvey", {}).get("overall_score", 0) for r in all_results]
        print(f"  Avg Surface Score:  {_safe_mean(surface):.2f}/5.0")
        print(f"  Avg Academic Score: {_safe_mean(academic):.2f}/5.0")
        print(f"  Avg Overall Score:  {_safe_mean(overall):.2f}/5.0")

    elif benchmark == "deepscholar":
        org_scores = [r.get("organization", {}).get("score", 0) for r in all_results]
        nug_scores = [
            r.get("nugget_coverage", {}).get("nugget_coverage", 0) for r in all_results
        ]
        ref_scores = [
            r.get("reference_coverage", {}).get("coverage", 0) for r in all_results
        ]
        cite_scores = [
            r.get("citation_precision", {}).get("cite_p", 0) for r in all_results
        ]
        verif_scores = [
            r.get("verifiability", {}).get("claim_coverage", 0) for r in all_results
        ]
        docimp_scores = [
            r.get("document_importance", {}).get("avg_importance", 0)
            for r in all_results
        ]
        overall_scores = [
            r.get("overall_score", {}).get("score", 0) for r in all_results
        ]
        print(f"  Avg Organization:        {_safe_mean(org_scores):.1f}/100")
        print(f"  Avg Nugget Coverage:     {_safe_mean(nug_scores):.1f}%")
        print(f"  Avg Reference Coverage:  {_safe_mean(ref_scores):.1f}%")
        print(f"  Avg Citation Precision:  {_safe_mean(cite_scores):.1f}%")
        print(f"  Avg Verifiability:       {_safe_mean(verif_scores):.1f}%")
        print(f"  Avg Document Importance: {_safe_mean(docimp_scores):.1f}/100")
        print(f"  Avg Overall (geom.):     {_safe_mean(overall_scores):.1f}/100")


def _safe_mean(values: list[float]) -> float:
    return sum(values) / len(values) if values else 0.0


# ── command: report ──────────────────────────────────────────────────────────


def _cmd_report(args: argparse.Namespace) -> None:
    pipeline = args.pipeline
    benchmark = args.benchmark
    compare_methods = args.compare or []

    results_dir = _RESULTS_DIR / pipeline / benchmark
    if not results_dir.is_dir():
        print(f"No results found at {results_dir}", file=sys.stderr)
        return

    # Collect all report.json files
    all_reports: list[dict[str, Any]] = []
    topic_names: list[str] = []
    for topic_dir in sorted(results_dir.iterdir()):
        if not topic_dir.is_dir():
            continue
        report_path = topic_dir / "report.json"
        if not report_path.is_file():
            continue
        try:
            report = json.loads(report_path.read_text(encoding="utf-8"))
            all_reports.append(report)
            topic_names.append(topic_dir.name)
        except Exception as exc:
            logger.warning("Failed to read %s: %s", report_path, exc)

    if not all_reports:
        print("No report.json files found.", file=sys.stderr)
        return

    # Build summary markdown
    lines = _build_summary_table(all_reports, topic_names, benchmark, pipeline)

    # Add comparison if requested
    if compare_methods:
        lines.append("")
        lines.append("## Comparison with Baselines")
        lines.append("")
        lines.append(
            "_Baseline comparison requires benchmark official baselines. Not yet implemented._"
        )
        # TODO: load official baseline scores from benchmark data directory

    summary_path = results_dir / "summary.md"
    summary_path.write_text("\n".join(lines), encoding="utf-8")

    print(f"  Summary written to {summary_path}")
    print()
    print("\n".join(lines))


def _build_summary_table(
    reports: list[dict[str, Any]],
    names: list[str],
    benchmark: str,
    pipeline: str,
) -> list[str]:
    """Build a summary markdown string from evaluation reports."""
    lines = [
        f"# Evaluation Summary — {pipeline} / {benchmark}",
        "",
        f"**Topics evaluated**: {len(reports)}",
        "",
    ]

    if benchmark == "surveybench":
        lines.append("| Topic | Content (5) | Outline (5) | Quiz (100) |")
        lines.append("|-------|-------------|-------------|------------|")
        for name, report in zip(names, reports):
            c = report.get("content", {}).get("overall_score", "—")
            o = report.get("outline", {}).get("overall_score", "—")
            q = report.get("quiz", {}).get("overall_answerability", "—")
            lines.append(f"| {name} | {c} | {o} | {q} |")
        c_avg = _safe_mean(
            [r.get("content", {}).get("overall_score", 0) for r in reports]
        )
        o_avg = _safe_mean(
            [r.get("outline", {}).get("overall_score", 0) for r in reports]
        )
        q_avg = _safe_mean(
            [r.get("quiz", {}).get("overall_answerability", 0) for r in reports]
        )
        lines.append(
            f"| **Average** | **{c_avg:.2f}** | **{o_avg:.2f}** | **{q_avg:.1f}** |"
        )

    elif benchmark == "deepsurvey":
        lines.append("| Topic | Surface (5) | Academic (5) | Overall (5) |")
        lines.append("|-------|-------------|-------------|-------------|")
        for name, report in zip(names, reports):
            ds = report.get("deepsurvey", {})
            s = ds.get("surface_score", "—")
            a = ds.get("academic_score", "—")
            ov = ds.get("overall_score", "—")
            lines.append(f"| {name} | {s} | {a} | {ov} |")
        s_avg = _safe_mean(
            [r.get("deepsurvey", {}).get("surface_score", 0) for r in reports]
        )
        a_avg = _safe_mean(
            [r.get("deepsurvey", {}).get("academic_score", 0) for r in reports]
        )
        ov_avg = _safe_mean(
            [r.get("deepsurvey", {}).get("overall_score", 0) for r in reports]
        )
        lines.append(
            f"| **Average** | **{s_avg:.2f}** | **{a_avg:.2f}** | **{ov_avg:.2f}** |"
        )

    elif benchmark == "deepscholar":
        lines.append(
            "| Topic | Org (100) | Nugget (%) | Ref (%) | CiteP (%) | Verif (%) | DocImp (100) | Overall (100) |"
        )
        lines.append(
            "|-------|-----------|------------|---------|-----------|-----------|--------------|---------------|"
        )
        for name, report in zip(names, reports):
            org = report.get("organization", {}).get("score", "—")
            ncov = report.get("nugget_coverage", {}).get("nugget_coverage", "—")
            rcov = report.get("reference_coverage", {}).get("coverage", "—")
            cprec = report.get("citation_precision", {}).get("cite_p", "—")
            verif = report.get("verifiability", {}).get("claim_coverage", "—")
            docimp = report.get("document_importance", {}).get("avg_importance", "—")
            overall = report.get("overall_score", {}).get("score", "—")
            lines.append(
                f"| {name} | {org} | {ncov} | {rcov} | {cprec} | {verif} | {docimp} | {overall} |"
            )

        org_avg = _safe_mean(
            [r.get("organization", {}).get("score", 0) for r in reports]
        )
        ncov_avg = _safe_mean(
            [r.get("nugget_coverage", {}).get("nugget_coverage", 0) for r in reports]
        )
        rcov_avg = _safe_mean(
            [r.get("reference_coverage", {}).get("coverage", 0) for r in reports]
        )
        cprec_avg = _safe_mean(
            [r.get("citation_precision", {}).get("cite_p", 0) for r in reports]
        )
        verif_avg = _safe_mean(
            [r.get("verifiability", {}).get("claim_coverage", 0) for r in reports]
        )
        dic_avg = _safe_mean(
            [r.get("document_importance", {}).get("avg_importance", 0) for r in reports]
        )
        overall_avg = _safe_mean(
            [r.get("overall_score", {}).get("score", 0) for r in reports]
        )
        lines.append(
            f"| **Average** | **{org_avg:.1f}** | **{ncov_avg:.1f}** | **{rcov_avg:.1f}** | **{cprec_avg:.1f}** | **{verif_avg:.1f}** | **{dic_avg:.1f}** | **{overall_avg:.1f}** |"
        )

    return lines


# ── CLI entry point ──────────────────────────────────────────────────────────


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="run.py",
        description="Survey Benchmark Framework — unified CLI",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    # --- list ---
    list_parser = subparsers.add_parser("list", help="List available resources")
    list_parser.add_argument(
        "resource",
        choices=["benchmarks", "pipelines", "topics"],
        help="What to list",
    )
    list_parser.add_argument("--benchmark", help="Benchmark name (for topics)")

    # --- setup ---
    setup_parser = subparsers.add_parser("setup", help="Fetch benchmark data")
    setup_parser.add_argument(
        "benchmark",
        help="Benchmark to set up: surveybench, deepscholar, or all",
    )

    # --- generate ---
    gen_parser = subparsers.add_parser(
        "generate", help="Run RCM pipeline to generate surveys"
    )
    gen_parser.add_argument(
        "--pipeline", required=True, help="Pipeline config name (without .toml)"
    )
    gen_parser.add_argument(
        "--benchmark", required=True, help="Benchmark: surveybench, deepscholar"
    )
    gen_parser.add_argument(
        "--topics", nargs="*", help="Topic names/IDs to filter (default: all)"
    )
    gen_parser.add_argument("--max-workers", type=int, help="Max parallel workers")

    # --- evaluate ---
    eval_parser = subparsers.add_parser("evaluate", help="Evaluate generated surveys")
    eval_parser.add_argument("--pipeline", required=True, help="Pipeline config name")
    eval_parser.add_argument(
        "--benchmark",
        required=True,
        help="Benchmark: surveybench, deepsurvey, deepscholar",
    )
    eval_parser.add_argument(
        "--topics", nargs="*", help="Topic names/IDs to filter (default: all)"
    )
    eval_parser.add_argument("--model", help="Override LLM judge model")
    eval_parser.add_argument("--max-workers", type=int, help="Max parallel workers")

    # --- report ---
    rep_parser = subparsers.add_parser(
        "report", help="Aggregate results into summary report"
    )
    rep_parser.add_argument("--pipeline", required=True, help="Pipeline config name")
    rep_parser.add_argument("--benchmark", required=True, help="Benchmark name")
    rep_parser.add_argument(
        "--compare", nargs="*", help="Methods to compare against baselines"
    )

    args = parser.parse_args()

    logging.basicConfig(
        level=logging.INFO,
        format="%(levelname)s [%(name)s] %(message)s",
        stream=sys.stderr,
    )

    command_map = {
        "list": _cmd_list,
        "setup": _cmd_setup,
        "generate": _cmd_generate,
        "evaluate": _cmd_evaluate,
        "report": _cmd_report,
    }

    handler = command_map.get(args.command)
    if handler:
        handler(args)


if __name__ == "__main__":
    main()
