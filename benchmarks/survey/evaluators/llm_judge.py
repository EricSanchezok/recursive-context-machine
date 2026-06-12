"""
Shared LLM judge client for survey benchmark evaluators.

Wraps the OpenAI-compatible API with retry, timeout, resource logging,
and a mock fallback when no API key is configured.
"""

from __future__ import annotations

import json
import logging
import os
import time
from typing import Any

from openai import OpenAI

logger = logging.getLogger(__name__)

_API_KEY = os.environ.get("EVA_API_KEY") or os.environ.get("OPENAI_API_KEY")
_MODEL = os.environ.get("EVA_MODEL", "gpt-4o")
_ENDPOINT = os.environ.get("EVA_ENDPOINT", "https://api.openai.com/v1")
_TIMEOUT_SECONDS = 120.0
_MAX_RETRIES = 3
_INITIAL_BACKOFF_SECONDS = 1.0
_TEMPERATURE = 0.0


def judge(prompt: str, text: str) -> str:
    """Send a survey prompt + text to the LLM and return the raw response string.

    Falls back to mock responses when no API key is available.
    """
    if _API_KEY is None:
        logger.warning(
            "No EVA_API_KEY or OPENAI_API_KEY set — returning mock judge result"
        )
        return _mock_response(prompt, text)

    client = OpenAI(api_key=_API_KEY, base_url=_ENDPOINT, timeout=_TIMEOUT_SECONDS)
    user_message = f"{prompt}\n\n--- SURVEY TEXT ---\n{text}"

    return _call_with_retry(client, user_message)


def judge_json(prompt: str, text: str) -> dict[str, Any]:
    """Send a prompt + text to the LLM and return the response parsed as JSON.

    On JSON parse failure, wraps the raw string in a fallback dictionary so
    callers always receive a dict.
    """
    raw = judge(prompt, text)
    try:
        parsed = json.loads(raw)
        if not isinstance(parsed, dict):
            return {"raw": raw, "error": "response is not a JSON object"}
        return parsed
    except json.JSONDecodeError:
        logger.warning("LLM judge response is not valid JSON, returning raw fallback")
        return {"raw": raw, "error": "invalid JSON"}


def _call_with_retry(client: OpenAI, user_message: str) -> str:
    last_exception: Exception | None = None
    backoff = _INITIAL_BACKOFF_SECONDS

    for attempt in range(_MAX_RETRIES + 1):
        try:
            started = time.monotonic()
            completion = client.chat.completions.create(
                model=_MODEL,
                messages=[{"role": "user", "content": user_message}],
                temperature=_TEMPERATURE,
            )
            elapsed = time.monotonic() - started
            content = completion.choices[0].message.content or ""
            usage = completion.usage
            tokens_used = usage.total_tokens if usage else 0
            logger.info(
                "judge call completed",
                extra={
                    "model": _MODEL,
                    "tokens_used": tokens_used,
                    "duration_seconds": round(elapsed, 3),
                },
            )
            return content

        except Exception as exc:
            last_exception = exc
            if not _is_retryable(exc) or attempt == _MAX_RETRIES:
                break
            logger.warning(
                "judge call retry %d/%d after error: %s",
                attempt + 1,
                _MAX_RETRIES,
                exc,
                extra={"model": _MODEL},
            )
            time.sleep(backoff)
            backoff *= 2

    logger.error(
        "judge call failed after %d attempts: %s",
        _MAX_RETRIES + 1,
        last_exception,
        extra={"model": _MODEL},
    )
    raise last_exception  # type: ignore[misc]


def _is_retryable(exc: Exception) -> bool:
    error_str = str(exc)
    if "rate_limit" in error_str.lower():
        return True
    if "rate limit" in error_str.lower():
        return True
    status = getattr(exc, "status_code", None)
    if status is not None and (status == 429 or status >= 500):
        return True
    if hasattr(exc, "http_status"):
        http_status = getattr(exc, "http_status", None)
        if http_status is not None and (http_status == 429 or http_status >= 500):
            return True
    return False


def _mock_response(prompt: str, text: str) -> str:
    _ = (prompt, text)
    return (
        '{"mock": true, "score": 3, "reason": "mock judge — '
        'no EVA_API_KEY or OPENAI_API_KEY configured"}'
    )
