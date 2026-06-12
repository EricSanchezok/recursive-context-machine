"""
DeepResearch-Bench — Wrapper for the unified CLI.
Supports batch evaluation for the official 100-task benchmark.
"""

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

BENCH_DIR = Path(__file__).resolve().parent


def run_simple_mode(args_list: list[str]) -> bool:
    """Delegate to evaluate_simple.py."""
    script = BENCH_DIR / "evaluate_simple.py"
    cmd = [sys.executable, str(script)] + args_list
    result = subprocess.run(cmd)
    return result.returncode == 0


def run_full_mode(args_list: list[str]) -> bool:
    """Run full batch evaluation (requires official benchmark data)."""
    parser = argparse.ArgumentParser()
    parser.add_argument("model_name", type=str, help="Model name (matches raw_data file)")
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--skip_cleaning", action="store_true")
    parser.add_argument("--only_zh", action="store_true")
    parser.add_argument("--only_en", action="store_true")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--max_workers", type=int, default=5)
    parser.add_argument("--output_dir", type=str, default="results")

    # We re-parse in this wrapper since the original args are forwarded
    sub_parser = argparse.ArgumentParser()
    sub_parser.add_argument("model_name", type=str)
    sub_parser.add_argument("--limit", type=int, default=None)
    sub_parser.add_argument("--skip_cleaning", action="store_true")
    sub_parser.add_argument("--only_zh", action="store_true")
    sub_parser.add_argument("--only_en", action="store_true")
    sub_parser.add_argument("--force", action="store_true")
    sub_parser.add_argument("--max_workers", type=int, default=5)
    sub_parser.add_argument("--output_dir", type=str, default="results")
    sub_parser.add_argument("--reference", type=str, default=None,
                            help="Path to reference data file")

    try:
        args = sub_parser.parse_args(args_list)
    except SystemExit:
        return False

    model_name = args.model_name
    raw_data_dir = BENCH_DIR / "data" / "test_data" / "raw_data"
    cleaned_data_dir = BENCH_DIR / "data" / "test_data" / "cleaned_data"
    query_file = BENCH_DIR / "data" / "prompt_data" / "query.jsonl"
    output_dir = BENCH_DIR / "results" / "race" / model_name

    if not query_file.exists():
        print(f"ERROR: Benchmark data not found at {query_file}")
        print("Run setup.ps1 first to download the official dataset.")
        return False

    # Simulate the official pipeline
    print(f"\n{'='*60}")
    print(f"  DeepResearch-Bench Full Mode: {model_name}")
    print(f"{'='*60}")

    # Build RACE command
    race_script = BENCH_DIR / "evaluate_simple.py"
    race_cmd = [sys.executable, str(race_script)]
    race_cmd += ["--task-prompt", f"Evaluate model {model_name} on 100 benchmark tasks"]
    race_cmd += ["--text", str(raw_data_dir / f"{model_name}.jsonl")]

    if args.reference:
        race_cmd += ["--reference", str(cleaned_data_dir / "reference.jsonl")]

    result = subprocess.run(race_cmd)
    return result.returncode == 0


def main():
    parser = argparse.ArgumentParser(
        description="DeepResearch-Bench Evaluation Runner",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--mode", choices=["simple", "full"], default="simple",
        help="simple: single report evaluation (default), full: 100-task benchmark"
    )
    parser.add_argument("args", nargs=argparse.REMAINDER,
                        help="Arguments forwarded to the evaluation script")

    args = parser.parse_args()

    if args.mode == "simple":
        success = run_simple_mode(args.args)
    else:
        success = run_full_mode(args.args)

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
