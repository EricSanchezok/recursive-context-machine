"""
DeepResearch-Bench configuration.

Model-agnostic design: no hardcoded vendor defaults.
Users configure everything via environment variables.

Basic setup:
  $env:API_KEY = 'sk-...'
  $env:API_BASE_URL = 'https://api.openai.com/v1'
  $env:RACE_MODEL = 'gpt-4o'
  $env:FACT_MODEL = 'gpt-4o-mini'

Advanced setup:
  $env:API_KEY = 'sk-...'                         # fallback for all LLM calls
  $env:RACE_API_KEY = 'sk-...'                    # override for RACE judge
  $env:FACT_API_KEY = 'sk-...'                    # override for FACT judge
  $env:RACE_API_BASE = 'https://api.openai.com/v1'
  $env:FACT_API_BASE = 'https://api.deepseek.com/v1'
  $env:RACE_MODEL = 'gpt-4o'
  $env:FACT_MODEL = 'deepseek-chat'

Supported providers (any OpenAI-compatible API):
  OpenAI, DeepSeek, OpenRouter, Groq, Together AI, Anthropic (via proxy), etc.
"""

import os
from pathlib import Path

BENCH_DIR = Path(__file__).resolve().parent

# ── Provider-agnostic API configuration ─────────────────────────
# Priority: <PREFIX>_KEY > API_KEY
# Priority: <PREFIX>_BASE_URL > API_BASE_URL

def _get_first(*keys: str, default: str = "") -> str:
    """Return the first non-empty environment variable value."""
    for k in keys:
        v = os.environ.get(k)
        if v:
            return v
    return default

# Fallback API credentials (used when no per-model override is set)
API_KEY = os.environ.get("API_KEY", "")
API_BASE_URL = os.environ.get("API_BASE_URL", "")

# ── RACE judge model ────────────────────────────────────────────
# Override order (highest priority first):
#   1. RACE_API_KEY / RACE_API_BASE / RACE_MODEL
#   2. API_KEY / API_BASE_URL / RACE_MODEL (or RACE_MODEL via OPENAI_MODEL compat)
RACE_API_KEY = _get_first("RACE_API_KEY", "API_KEY")
RACE_API_BASE = _get_first("RACE_API_BASE", "RACE_API_BASE_URL", "API_BASE_URL")
RACE_MODEL = os.environ.get("RACE_MODEL") or os.environ.get("OPENAI_MODEL") or ""

# ── FACT judge model ────────────────────────────────────────────
FACT_API_KEY = _get_first("FACT_API_KEY", "API_KEY")
FACT_API_BASE = _get_first("FACT_API_BASE", "FACT_API_BASE_URL", "API_BASE_URL")
FACT_MODEL = os.environ.get("FACT_MODEL") or os.environ.get("OPENAI_MODEL") or ""

# ── Jina AI (for web scraping in FACT, optional) ────────────────
JINA_API_KEY = os.environ.get("JINA_API_KEY", "")

# ── Generation params ───────────────────────────────────────────
TEMPERATURE = float(os.environ.get("TEMPERATURE", "0.0"))
MAX_TOKENS = int(os.environ.get("MAX_TOKENS", "64000"))
MAX_WORKERS = int(os.environ.get("MAX_WORKERS", "5"))

# ── RACE dimension weights (default equal) ──────────────────────
DEFAULT_DIM_WEIGHTS = {
    "comprehensiveness": 0.25,
    "insight": 0.25,
    "instruction_following": 0.25,
    "readability": 0.25,
}

# ── Paths for official benchmark dataset (after setup.ps1) ──────
DATA_DIR = BENCH_DIR / "data"
PROMPT_FILE = DATA_DIR / "prompt_data" / "query.jsonl"
CRITERIA_FILE = DATA_DIR / "criteria_data" / "criteria.jsonl"
REFERENCE_FILE = DATA_DIR / "test_data" / "cleaned_data" / "reference.jsonl"
TEST_DATA_DIR = DATA_DIR / "test_data"
RAW_DATA_DIR = TEST_DATA_DIR / "raw_data"
CLEANED_DATA_DIR = TEST_DATA_DIR / "cleaned_data"
