"""
SciReplicate-Bench evaluation wrapper.
Calls the official SciReplicate-Bench evaluation pipeline.

Official repo: https://github.com/xyzCS/SciReplicate-Bench
Paper: https://arxiv.org/abs/2504.00255

Note: Full evaluation requires Ubuntu + CUDA 12.2 + A100 GPU.
"""

import argparse
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from shared.config import get_repo_dir


BENCHMARK_NAME = "scireplicate"
REPO_DIR = get_repo_dir(BENCHMARK_NAME)


def check_prerequisites() -> bool:
    """Check that the official repo is cloned."""
    repo = Path(REPO_DIR)
    if not repo.exists():
        print(f"ERROR: Official SciReplicate-Bench repo not found at: {REPO_DIR}")
        print("  Run 'setup.ps1' first.")
        return False
    return True


def run_sci_reproducer(model: str = "gpt-4o-mini", root_path: str | None = None):
    """
    Run the official SciReproducer dual-agent framework.
    """
    if not check_prerequisites():
        return False

    root = root_path or REPO_DIR
    script = Path(REPO_DIR) / "SciReproducer.py"
    if not script.exists():
        print(f"ERROR: SciReproducer.py not found in {REPO_DIR}")
        return False

    cmd = [
        sys.executable, str(script),
        "--root_path", root,
        "--model", model,
    ]

    print(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=REPO_DIR)
    return result.returncode == 0


def run_evaluation(
    metric: str,
    model: str = "gpt-4o-mini",
    root_path: str | None = None,
    gpu_id: int = 0,
    reference: bool = False,
):
    """
    Run the official Evaluation.py for a specific metric.

    Args:
        metric: One of "CodeBLEU_Score", "execution_ACC", "Recall", "ReasoningGraph_ACC"
        model: Model name used for code generation
        root_path: Root directory of SciReplicate-Bench
        gpu_id: GPU ID to use
        reference: Run in reference mode (execution_ACC only)
    """
    if not check_prerequisites():
        return False

    eval_script = Path(REPO_DIR) / "Evaluation.py"
    if not eval_script.exists():
        print(f"ERROR: Evaluation.py not found in {REPO_DIR}")
        return False

    root = root_path or REPO_DIR
    result_path = Path(REPO_DIR) / "Result"

    cmd = [
        sys.executable, str(eval_script),
        "--metric", metric,
        "--model", model,
        "--root_path", root,
        "--result_path", str(result_path),
        "--gpu_id", str(gpu_id),
    ]

    if reference:
        cmd.append("--reference")

    print(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=REPO_DIR)
    return result.returncode == 0


def run_all_metrics(model: str = "gpt-4o-mini", root_path: str | None = None):
    """Run all 4 evaluation metrics."""
    metrics = ["CodeBLEU_Score", "execution_ACC", "Recall", "ReasoningGraph_ACC"]
    all_ok = True

    for metric in metrics:
        print(f"\n{'='*60}")
        print(f"  Metric: {metric}")
        print(f"{'='*60}")

        if metric == "execution_ACC":
            # First run reference, then evaluate
            print("  Step 1: Obtain reference output...")
            ok = run_evaluation(metric, model, root_path, reference=True)
            if not ok:
                print("  WARNING: Reference run had issues")

        ok = run_evaluation(metric, model, root_path)
        if not ok:
            all_ok = False

    return all_ok


def main():
    parser = argparse.ArgumentParser(
        description="SciReplicate-Bench evaluation (official SciReplicate-Bench wrapper)"
    )
    parser.add_argument(
        "--run", action="store_true",
        help="Run SciReproducer on all papers"
    )
    parser.add_argument(
        "--evaluate", action="store_true",
        help="Run evaluation metrics on generated results"
    )
    parser.add_argument(
        "--all-metrics", action="store_true",
        help="Run all 4 evaluation metrics"
    )
    parser.add_argument(
        "--metric",
        choices=["CodeBLEU_Score", "execution_ACC", "Recall", "ReasoningGraph_ACC"],
        help="Specific metric to evaluate",
    )
    parser.add_argument(
        "--model", "--model_name", default="gpt-4o-mini",
        help="Model name (default: gpt-4o-mini)"
    )
    parser.add_argument(
        "--root-path",
        help="Root path to SciReplicate-Bench directory"
    )
    parser.add_argument("--gpu-id", type=int, default=0, help="GPU ID (default: 0)")
    parser.add_argument("--reference", action="store_true",
                        help="Run reference mode (execution_ACC)")
    parser.add_argument("--check", action="store_true", help="Check prerequisites")

    args = parser.parse_args()

    if args.check:
        ok = check_prerequisites()
        sys.exit(0 if ok else 1)

    if args.run:
        success = run_sci_reproducer(args.model, args.root_path)
        sys.exit(0 if success else 1)

    if args.all_metrics:
        success = run_all_metrics(args.model, args.root_path)
        sys.exit(0 if success else 1)

    if args.evaluate and args.metric:
        success = run_evaluation(
            metric=args.metric,
            model=args.model,
            root_path=args.root_path,
            gpu_id=args.gpu_id,
            reference=args.reference,
        )
        sys.exit(0 if success else 1)

    parser.print_help()


if __name__ == "__main__":
    main()
