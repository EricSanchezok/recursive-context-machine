"""
Paper Benchmarks — Unified Evaluation CLI
==========================================

Orchestrates evaluation across four paper generation benchmarks:

  1. PaperWrite-Bench      —  Paper writing quality
  2. MLR-Bench             —  Machine learning research quality
  3. SciReplicate-Bench    —  Algorithmic reproduction accuracy
  4. DeepResearch-Bench    —  PhD-level deep research report quality (RACE + FACT)

Usage:
  python run_evaluation.py --benchmark paperwrite --paper paper_1 --eval-mode all
  python run_evaluation.py --benchmark mlr --stepwise --model-name gpt-4o
  python run_evaluation.py --benchmark scireplicate --all-metrics
  python run_evaluation.py --benchmark deepresearch --mode simple --pdf report.pdf
  python run_evaluation.py --all
  python run_evaluation.py --check
  python run_evaluation.py --list
"""

import argparse
import subprocess
import sys
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent

BENCHMARKS = {
    "paperwrite": {
        "dir": "paperwrite-bench",
        "desc": "PaperWrite-Bench: Paper writing quality (Rubric + Hallucination + Citation F1)",
    },
    "mlr": {
        "dir": "mlr-bench",
        "desc": "MLR-Bench: Open-ended ML research (Idea + Proposal + Experiment + Writing)",
    },
    "scireplicate": {
        "dir": "scireplicate-bench",
        "desc": "SciReplicate-Bench: Algorithmic reproduction (CodeBLEU + Exec ACC + Recall + Graph ACC)",
    },
    "deepresearch": {
        "dir": "deepresearch-bench",
        "desc": "DeepResearch-Bench: PhD-level deep research reports (RACE + FACT)",
    },
}


def check_environment():
    """Run environment check."""
    sys.path.insert(0, str(BASE_DIR))
    from shared.env_check import print_status
    print_status()


def run_benchmark(benchmark: str, args_list: list[str]) -> bool:
    """Delegate to a benchmark's own run_evaluation.py."""
    info = BENCHMARKS.get(benchmark)
    if not info:
        print(f"Unknown benchmark: {benchmark}")
        return False

    script = BASE_DIR / info["dir"] / "run_evaluation.py"
    if not script.exists():
        print(f"ERROR: {script} not found")
        return False

    cmd = [sys.executable, str(script)] + args_list
    print(f"\n{'='*60}")
    print(f"  [{benchmark}] {' '.join(args_list)}")
    print(f"{'='*60}")
    result = subprocess.run(cmd)
    return result.returncode == 0


def run_all_benchmarks(args_list: list[str]):
    """Run all four benchmarks in sequence."""
    all_ok = True
    for name in BENCHMARKS:
        ok = run_benchmark(name, args_list)
        if not ok:
            all_ok = False
        print()
    sys.exit(0 if all_ok else 1)


def list_benchmarks():
    """Print available benchmarks."""
    print("\nAvailable Benchmarks:")
    print("=" * 60)
    for name, info in sorted(BENCHMARKS.items()):
        print(f"  {name:20s}  {info['desc']}")
    print()
    print("Flags:")
    print("  --all        Run all benchmarks")
    print("  --check      Check environment")
    print("  --list       List available benchmarks")
    print()


def main():
    parser = argparse.ArgumentParser(
        description="Paper Benchmarks — Unified Evaluation CLI",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--benchmark", "-b", choices=list(BENCHMARKS.keys()),
                        help="Benchmark to run")
    parser.add_argument("--all", action="store_true",
                        help="Run all benchmarks")
    parser.add_argument("--check", action="store_true",
                        help="Check environment prerequisites")
    parser.add_argument("--list", action="store_true",
                        help="List available benchmarks")
    parser.add_argument("benchmark_args", nargs=argparse.REMAINDER,
                        help="Arguments forwarded to the benchmark's run_evaluation.py")

    args = parser.parse_args()

    if args.list:
        list_benchmarks()
        return

    if args.check:
        check_environment()
        return

    if args.all:
        run_all_benchmarks(args.benchmark_args)
        return

    if args.benchmark:
        success = run_benchmark(args.benchmark, args.benchmark_args)
        sys.exit(0 if success else 1)

    parser.print_help()


if __name__ == "__main__":
    main()
