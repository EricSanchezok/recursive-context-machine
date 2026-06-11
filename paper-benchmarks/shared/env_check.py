"""
Environment check utility.
Verifies that all prerequisites for each benchmark are available.
"""

import os
import shutil
import subprocess
import sys
from pathlib import Path


def check_python(package: str) -> bool:
    """Check if a Python package is installed."""
    try:
        __import__(package)
        return True
    except ImportError:
        return False


def check_command(cmd: str) -> bool:
    """Check if a command-line tool is available."""
    return shutil.which(cmd) is not None


def check_repo(benchmark: str, repo_dir: str) -> bool:
    """Check if a benchmark repo has been cloned."""
    return Path(repo_dir).exists() and (Path(repo_dir) / "README.md").exists()


def check_docker() -> bool:
    """Check if Docker is available."""
    try:
        subprocess.run(
            ["docker", "--version"],
            capture_output=True,
            timeout=5,
        )
        return True
    except (subprocess.SubprocessError, FileNotFoundError):
        return False


def check_api_key(name: str) -> bool:
    """Check if an API key is set."""
    return bool(os.environ.get(name))


def print_status():
    """Print a full environment status report."""
    from .config import REPO_DIRS

    print("=" * 60)
    print("  Paper Benchmarks - Environment Status")
    print("=" * 60)

    # Python
    print(f"  Python:      {sys.version.split()[0]}")
    print(f"  PyTorch:     {'[OK]' if check_python('torch') else '[NO]'}")

    # Docker
    print(f"  Docker:      {'[OK]' if check_docker() else '[NO]'}")

    # Repos
    print()
    for name, path in REPO_DIRS.items():
        found = check_repo(name, path)
        print(f"  {name:20s} {'[OK]' if found else '[NO] (run setup)'}  {path}")

    # API keys
    print()
    for key in [
        "OPENAI_API_KEY",
        "ANTHROPIC_API_KEY",
        "DEEPSEEK_API_KEY",
        "GEMINI_API_KEY",
        "OPENROUTER_API_KEY",
    ]:
        has = check_api_key(key)
        val = os.environ.get(key, "")
        masked = val[:8] + "..." if val else ""
        print(f"  {key:25s} {'[OK]' if has else '[NO]'}  {masked}")

    print()
