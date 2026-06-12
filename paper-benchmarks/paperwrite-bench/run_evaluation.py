"""
PaperWrite-Bench evaluation wrapper.
Calls the official PaperRecon evaluation pipeline.

Official repo: https://github.com/Agent4Science-UTokyo/PaperRecon
Paper: https://arxiv.org/pdf/2604.01128

Supports any evaluation model via --model (e.g., gpt-4o, claude-sonnet-4,
deepseek-chat); dynamically generates a config file for the chosen model.
"""

import argparse
import os
import subprocess
import sys
import yaml
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from shared.config import get_repo_dir


BENCHMARK_NAME = "paperwrite"
REPO_DIR = get_repo_dir(BENCHMARK_NAME)
DEFAULT_CONFIG = "configs/cc_sonnet4.yaml"


def check_prerequisites() -> bool:
    """Check that the official repo is cloned."""
    repo = Path(REPO_DIR)
    if not repo.exists():
        print(f"ERROR: Official PaperRecon repo not found at: {REPO_DIR}")
        print("  Run 'setup.ps1' first.")
        return False
    readme = repo / "README.md"
    if not readme.exists():
        print(f"ERROR: Incomplete repo at: {REPO_DIR}")
        return False
    return True


def resolve_config(
    config_path: str | None,
    model: str | None,
) -> str:
    """
    Resolve the final config path.

    - If --model is given, copy the default config, override the model
      field, write to a temp file, and return that path.
    - Otherwise return the user-specified or default config path.
    """
    base_path = Path(REPO_DIR) / (config_path or DEFAULT_CONFIG)
    if not base_path.exists():
        print(f"ERROR: Config not found: {base_path}")
        sys.exit(1)

    if model is None:
        return str(base_path)

    # Load default config, override evaluation_llm.model
    with open(base_path, encoding="utf-8") as f:
        cfg = yaml.safe_load(f)

    cfg["evaluation_llm"] = cfg.get("evaluation_llm", {})
    cfg["evaluation_llm"]["model"] = model

    cfg["writeup"] = cfg.get("writeup", {})
    cfg["writeup"]["agent"] = cfg["writeup"].get("agent", "ClaudeCode")
    cfg["writeup"]["model"] = cfg["writeup"].get("model", model)

    # Write to a temp file next to the original config
    safe_name = model.replace("/", "_").replace(":", "_")
    tmp_path = base_path.parent / f"cc_custom_{safe_name}.yaml"
    with open(tmp_path, "w", encoding="utf-8") as f:
        yaml.dump(cfg, f, default_flow_style=False)

    print(f"  Generated config: {tmp_path} (eval model: {model})")
    return str(tmp_path)


def run_evaluation_batch(
    config_path: str,
    paper_names: list[str] | None = None,
    all_papers: bool = False,
    eval_mode: str | None = None,
    force: bool = False,
) -> bool:
    """Run the official PaperRecon run_evaluation.py script (not launch_writing.py).

    launch_writing.py handles writing AND evaluation together, but does NOT
    accept --eval-mode or --force flags. The correct evaluation-only entry
    point is run_evaluation.py.
    """
    if not check_prerequisites():
        return False

    eval_script = Path(REPO_DIR) / "run_evaluation.py"
    if not eval_script.exists():
        print(f"ERROR: run_evaluation.py not found in {REPO_DIR}")
        return False

    cmd = [sys.executable, str(eval_script), "--config-path", config_path]

    if all_papers:
        cmd.append("--all")
    elif paper_names:
        for name in paper_names:
            cmd.extend(["--paper", name])

    if eval_mode:
        cmd.extend(["--eval-mode", eval_mode])

    if force:
        cmd.append("--force")

    print(f"  {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=REPO_DIR)
    return result.returncode == 0


def run_evaluation(
    paper_name: str,
    config_path: str,
    eval_mode: str = "all",
    force: bool = False,
) -> bool:
    """Run evaluation for a specific paper via the official run_evaluation.py."""
    if not check_prerequisites():
        return False

    eval_script = Path(REPO_DIR) / "run_evaluation.py"
    if not eval_script.exists():
        print(f"ERROR: run_evaluation.py not found in {REPO_DIR}")
        return False

    cmd = [
        sys.executable, str(eval_script),
        "--config-path", config_path,
        "--paper", paper_name,
        "--eval-mode", eval_mode,
    ]
    if force:
        cmd.append("--force")

    print(f"  {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=REPO_DIR)
    return result.returncode == 0


def list_papers() -> list[str]:
    """List available papers."""
    if not check_prerequisites():
        return []
    papers_dir = Path(REPO_DIR) / "PaperWrite-Bench"
    if not papers_dir.exists():
        print("WARNING: PaperWrite-Bench dataset not found")
        return []
    return sorted(
        d.name for d in papers_dir.iterdir()
        if d.is_dir() and d.name.startswith("paper_")
    )


def main():
    parser = argparse.ArgumentParser(
        description="PaperWrite-Bench evaluation (official PaperRecon wrapper)"
    )
    parser.add_argument("--paper", help="Paper name to evaluate (e.g., paper_1)")
    parser.add_argument("--all", action="store_true", help="Evaluate all papers")
    parser.add_argument(
        "--config", default=DEFAULT_CONFIG,
        help=f"Agent config file (default: {DEFAULT_CONFIG})",
    )
    parser.add_argument(
        "--model",
        help="Evaluation model name (e.g., gpt-4o, claude-sonnet-4, "
             "deepseek-chat). Overrides config's evaluation_llm.model.",
    )
    parser.add_argument(
        "--eval-mode", choices=["rubric", "hallucination", "citation", "all"],
        default="all",
        help="Evaluation mode (default: all)",
    )
    parser.add_argument("--skip-evaluation", action="store_true",
                        help="Skip evaluation step")
    parser.add_argument("--force", action="store_true",
                        help="Re-evaluate existing results")
    parser.add_argument("--list", action="store_true",
                        help="List available papers")
    parser.add_argument("--check", action="store_true",
                        help="Check prerequisites")

    args = parser.parse_args()

    if args.check:
        ok = check_prerequisites()
        sys.exit(0 if ok else 1)

    if args.list:
        papers = list_papers()
        for p in papers or ["(no papers -- run setup.ps1 first)"]:
            print(f"  {p}")
        return

    # Resolve model into config
    config_path = resolve_config(args.config, args.model)

    if args.all:
        success = run_evaluation_batch(
            config_path=config_path, all_papers=True,
            eval_mode=args.eval_mode,
        )
        sys.exit(0 if success else 1)

    if args.paper:
        if args.skip_evaluation:
            success = run_evaluation_batch(
                config_path=config_path, paper_names=[args.paper],
            )
        else:
            success = run_evaluation(
                paper_name=args.paper, config_path=config_path,
                eval_mode=args.eval_mode, force=args.force,
            )
        sys.exit(0 if success else 1)

    parser.print_help()


if __name__ == "__main__":
    main()
