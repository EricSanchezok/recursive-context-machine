"""
MLR-Bench evaluation wrapper.
Calls the official MLR-Bench evaluation pipeline.

Official repo: https://github.com/chchenhui/mlrbench
Paper: https://arxiv.org/abs/2505.19955
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from shared.config import get_repo_dir


BENCHMARK_NAME = "mlr"
REPO_DIR = get_repo_dir(BENCHMARK_NAME)


def check_prerequisites() -> bool:
    """Check that the official repo is cloned."""
    repo = Path(REPO_DIR)
    if not repo.exists():
        print(f"ERROR: Official MLR-Bench repo not found at: {REPO_DIR}")
        print("  Run 'setup.ps1' first.")
        return False
    return True


def run_official_script(script_name: str, cwd: str, **kwargs) -> bool:
    """Run an official MLR-Bench Python script with arguments."""
    script_path = Path(cwd) / script_name
    if not script_path.exists():
        print(f"ERROR: {script_name} not found in {cwd}")
        return False

    cmd = [sys.executable, str(script_path)]
    for key, value in kwargs.items():
        key_arg = "--" + key
        cmd.extend([key_arg, str(value)])

    print(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=cwd)
    return result.returncode == 0


def run_mlr_agent(model_name: str, coding_agent: str | None = None):
    """
    Run the official MLR-Agent end-to-end.
    Calls run_mlr_agent.py from the official repo.
    """
    if not check_prerequisites():
        return False

    return run_official_script(
        "run_mlr_agent.py",
        REPO_DIR,
        model_name=model_name,
        coding_agent=coding_agent or "",
    )


def run_review(review_type: str):
    """
    Run the official MLR-Judge review for a specific stage.

    Args:
        review_type: "overall", "idea", "proposal", "experiments", "writeup"
    """
    if not check_prerequisites():
        return False

    script_map = {
        "overall": "mlrbench/evals/overall_review.py",
        "idea": "mlrbench/evals/review_idea.py",
        "proposal": "mlrbench/evals/review_proposal.py",
        "experiments": "mlrbench/evals/review_experiments.py",
        "writeup": "mlrbench/evals/review_writeup.py",
    }

    script = script_map.get(review_type)
    if not script:
        print(f"ERROR: Unknown review type: {review_type}")
        return False

    return run_official_script(script, REPO_DIR)


def main():
    parser = argparse.ArgumentParser(
        description="MLR-Bench evaluation (official MLR-Bench wrapper)"
    )
    parser.add_argument(
        "--end-to-end", action="store_true",
        help="Run end-to-end evaluation"
    )
    parser.add_argument(
        "--stepwise", action="store_true",
        help="Run stepwise evaluation"
    )
    parser.add_argument(
        "--review",
        choices=["overall", "idea", "proposal", "experiments", "writeup"],
        help="Run MLR-Judge review for a specific stage",
    )
    parser.add_argument(
        "--model", type=str, default=None,
        help="Model name for evaluation (alias for --model-name)"
    )
    parser.add_argument(
        "--model-name", default="gpt-4o",
        help="Model name for evaluation (default: gpt-4o). Overridden by --model if set."
    )
    parser.add_argument(
        "--coding-agent",
        help="Coding agent name (for end-to-end mode)"
    )
    parser.add_argument(
        "--input-dir",
        help="Input directory with results to review"
    )
    parser.add_argument("--check", action="store_true", help="Check prerequisites")

    args = parser.parse_args()

    # --model overrides --model-name
    if args.model:
        args.model_name = args.model

    if args.check:
        ok = check_prerequisites()
        sys.exit(0 if ok else 1)

    # End-to-end: run full agent pipeline
    if args.end_to_end:
        success = run_mlr_agent(
            model_name=args.model_name,
            coding_agent=args.coding_agent,
        )
        sys.exit(0 if success else 1)

    # Stepwise: run all MLR-Judge reviews
    if args.stepwise:
        stages = ["idea", "proposal", "experiments", "writeup"]
        all_ok = True
        for stage in stages:
            print(f"\n{'='*60}")
            print(f"  Reviewing: {stage}")
            print(f"{'='*60}")
            ok = run_review(stage)
            if not ok:
                all_ok = False
        # Also run overall review
        print(f"\n{'='*60}")
        print(f"  Overall review")
        print(f"{'='*60}")
        ok = run_review("overall")
        sys.exit(0 if all_ok and ok else 1)

    # Single review stage
    if args.review:
        success = run_review(args.review)
        sys.exit(0 if success else 1)

    parser.print_help()


if __name__ == "__main__":
    main()
