"""Evaluate a single survey file using DeepSurvey-Bench metrics."""

import json
import os
import sys
from pathlib import Path

# Make survey benchmark imports available.
script_dir = Path(__file__).resolve().parent
sys.path.insert(0, str(script_dir))

from evaluators import deepsurvey


def main() -> int:
    survey_path = script_dir / "output" / "survey.md"
    if not survey_path.is_file():
        print(f"Survey not found: {survey_path}", file=sys.stderr)
        return 1

    survey_text = survey_path.read_text(encoding="utf-8")
    print(f"Evaluating {survey_path} ({len(survey_text)} characters)")
    print()

    result = deepsurvey.evaluate(survey_text)

    print(f"Surface Score:  {result.get('surface_score', 'N/A')}/5.0")
    print(f"Academic Score: {result.get('academic_score', 'N/A')}/5.0")
    print(f"Overall Score:  {result.get('overall_score', 'N/A')}/5.0")
    print(f"Dimensions:     {json.dumps(result.get('dimensions', {}), indent=2)}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
