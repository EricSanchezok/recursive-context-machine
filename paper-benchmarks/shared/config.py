"""
Shared configuration for paper benchmark evaluations.
Reads API keys and paths from environment variables with sensible defaults.
"""

import os
from pathlib import Path

BENCHMARKS_DIR = Path(__file__).resolve().parent.parent

# ── Official repo paths (set after running setup scripts) ──
REPO_DIRS = {
    "paperwrite": os.environ.get(
        "PAPERWRITE_REPO_DIR",
        str(BENCHMARKS_DIR / "paperwrite-bench" / "PaperRecon"),
    ),
    "mlr": os.environ.get(
        "MLR_REPO_DIR",
        str(BENCHMARKS_DIR / "mlr-bench" / "mlrbench"),
    ),
    "scireplicate": os.environ.get(
        "SCIREPLICATE_REPO_DIR",
        str(BENCHMARKS_DIR / "scireplicate-bench" / "SciReplicate-Bench"),
    ),
    "deepresearch": os.environ.get(
        "DEEPRESEARCH_REPO_DIR",
        str(BENCHMARKS_DIR / "deepresearch-bench" / "data"),
    ),
}

# ── API keys (priority: env var > .env file) ──
API_KEYS = {}

for key_name in [
    "OPENAI_API_KEY",
    "ANTHROPIC_API_KEY",
    "DEEPSEEK_API_KEY",
    "GEMINI_API_KEY",
    "OPENROUTER_API_KEY",
    "CROSS_JUDGE_API_KEY",
    "EVA_API_KEY",
]:
    value = os.environ.get(key_name)
    if value:
        API_KEYS[key_name] = value

# Try loading from .env file
env_file = BENCHMARKS_DIR / ".env"
if env_file.exists():
    with open(env_file, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                k, v = line.split("=", 1)
                k = k.strip()
                v = v.strip().strip('"').strip("'")
                if k not in API_KEYS:
                    API_KEYS[k] = v


def get_api_key(name: str) -> str:
    """Get API key by name. Raises if not found."""
    value = API_KEYS.get(name)
    if not value:
        raise ValueError(
            f"API key '{name}' not found. "
            f"Set it as an environment variable or add to .env file."
        )
    return value


def get_repo_dir(benchmark: str) -> str:
    """Get the cloned repo directory for a benchmark."""
    return REPO_DIRS.get(benchmark, "")
