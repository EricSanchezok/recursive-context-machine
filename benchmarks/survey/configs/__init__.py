"""Pipeline configuration loader.

Config files are TOML and live next to this module.  Each config describes how
RCM should run a survey generation pipeline for a particular benchmark.
"""

import tomllib
from pathlib import Path
from typing import Any, Dict, Optional, List

_CONFIG_DIR = Path(__file__).resolve().parent


def load_config(config_name: str) -> Optional[Dict[str, Any]]:
    """Load a pipeline config by name (without ``.toml`` extension).

    Returns the parsed dict on success, or ``None`` when the file does not
    exist.
    """
    path = _CONFIG_DIR / f"{config_name}.toml"
    if not path.is_file():
        return None
    with open(path, "rb") as fh:
        return tomllib.load(fh)


def list_configs() -> list[str]:
    """List available pipeline config names (stem of each ``.toml`` file)."""
    return sorted(p.stem for p in _CONFIG_DIR.glob("*.toml") if p.is_file())


def validate_config(config: dict) -> list[str]:
    """Validate a pipeline config dict.

    Returns a list of human-readable error messages.  An empty list means the
    config is valid.
    """
    errors: List[str] = []

    # -- top-level tables --
    if "pipeline" not in config or not isinstance(config.get("pipeline"), dict):
        errors.append("missing or invalid [pipeline] table")
    else:
        pip = config["pipeline"]
        if not isinstance(pip.get("name"), str) or not pip["name"].strip():
            errors.append("pipeline.name must be a non-empty string")
        if (
            not isinstance(pip.get("description"), str)
            or not pip["description"].strip()
        ):
            errors.append("pipeline.description must be a non-empty string")

    if "generation" not in config or not isinstance(config.get("generation"), dict):
        errors.append("missing or invalid [generation] table")
    else:
        gen = config["generation"]
        if (
            not isinstance(gen.get("entry_graph"), str)
            or not gen["entry_graph"].strip()
        ):
            errors.append("generation.entry_graph must be a non-empty string")
        if (
            not isinstance(gen.get("final_output"), str)
            or not gen["final_output"].strip()
        ):
            errors.append("generation.final_output must be a non-empty string")

        working_dir = gen.get("working_dir")
        if working_dir is not None and not isinstance(working_dir, str):
            errors.append("generation.working_dir must be a string when set")

        timeout = gen.get("timeout_seconds")
        if timeout is not None and not isinstance(timeout, int):
            errors.append("generation.timeout_seconds must be an integer when set")
        elif isinstance(timeout, int) and timeout < 1:
            errors.append("generation.timeout_seconds must be >= 1")

        env = gen.get("env")
        if env is not None:
            if not isinstance(env, list) or not all(isinstance(v, str) for v in env):
                errors.append("generation.env must be a list of strings when set")

    if "output" in config:
        out = config["output"]
        if not isinstance(out, dict):
            errors.append("[output] must be a table when present")
        else:
            fmt = out.get("format")
            if fmt is not None:
                if fmt not in ("markdown", "json"):
                    errors.append('output.format must be "markdown" or "json"')

            target = out.get("target_name")
            if target is not None and not isinstance(target, str):
                errors.append("output.target_name must be a string when set")

            artifacts = out.get("copy_artifacts")
            if artifacts is not None and not isinstance(artifacts, bool):
                errors.append("output.copy_artifacts must be a boolean when set")

    return errors
