"""
Survey Benchmark Evaluation Framework
======================================

Complete evaluation framework implementing three academic survey generation benchmarks:

  1. SurveyBench (arXiv:2510.03120v2)  — Content-based + Quiz-based evaluation
  2. DeepSurvey-Bench (arXiv:2601.15307v1) — Surface quality + Academic value
  3. DeepScholar-Bench (arXiv:2508.20033v2) — Knowledge synthesis + Retrieval quality + Verifiability

Usage:
  python run_evaluation.py --survey path/to/survey.md --benchmark surveybench --output report.md
  python run_evaluation.py --survey path/to/survey.md --benchmark deepsurvey --reference ref.md
  python run_evaluation.py --survey path/to/survey.md --benchmark deepscholar --topic "LLM"
  python run_evaluation.py --survey path/to/survey.md --benchmark quiz --topic "NLP"
  python run_evaluation.py --survey path/to/survey.md --benchmark verifiability
  python run_evaluation.py --survey generated.md --benchmark nugget --reference ref.md
  python run_evaluation.py --list
"""
import os
import sys
import argparse
import json
from typing import List, Optional

# ── Benchmark directories and their prompt files ──
BENCHMARK_DIRS = {
    "surveybench":  {"dir": "surveybench",  "desc": "SurveyBench: Content-based (arXiv:2510.03120v2)"},
    "deepsurvey":   {"dir": "deepsurvey-bench", "desc": "DeepSurvey-Bench: Academic Value (arXiv:2601.15307v1)"},
    "deepscholar":  {"dir": "deepscholar-bench","desc": "DeepScholar-Bench: Research Synthesis (arXiv:2508.20033v2)"},
    "quiz":         {"dir": None,           "desc": "SurveyBench Quiz-based Evaluation (arXiv:2510.03120v2)"},
    "verifiability":{"dir": None,           "desc": "DeepScholar Verifiability (arXiv:2508.20033v2)"},
    "nugget":       {"dir": None,           "desc": "DeepScholar Nugget Coverage (arXiv:2508.20033v2)"},
    "document-importance": {"dir": None,    "desc": "DeepScholar Document Importance (arXiv:2508.20033v2)"},
    "comparative":  {"dir": None,           "desc": "Comparative evaluation against SciReviewGen reference"},
}


def load_survey(file_path: str) -> str:
    """Load survey text from a file."""
    if not os.path.exists(file_path):
        raise FileNotFoundError(f"Survey file not found: {file_path}")
    with open(file_path, "r", encoding="utf-8") as f:
        return f.read()


def load_prompt(benchmark: dict) -> str:
    """Load the prompt template for a benchmark that uses prompt.txt."""
    base = os.path.dirname(os.path.abspath(__file__))
    if benchmark["dir"] is None:
        raise ValueError(f"Benchmark has no prompt file")
    prompt_path = os.path.join(base, benchmark["dir"], "prompt.txt")
    if not os.path.exists(prompt_path):
        raise FileNotFoundError(f"Prompt not found: {prompt_path}")
    with open(prompt_path, "r", encoding="utf-8") as f:
        return f.read()


def build_prompt_with_reference(prompt_text: str, reference_text: str) -> str:
    """Inject reference survey text into the prompt for comparison mode."""
    placeholder = "[REFERENCE_SURVEY]"
    if placeholder in prompt_text:
        return prompt_text.replace(placeholder, reference_text)
    return prompt_text + f"\n\n## Reference Survey for Comparison\n\n{reference_text}\n"


def evaluate_with_llm(survey_text: str, prompt_text: str,
                      model_name: str = "deepseek-v4-flash") -> str:
    """
    Run evaluation using the 'accelerator' tool (LLM-as-Judge).

    Falls back to a mock evaluation if the tool is unavailable or fails.
    """
    result = try_accelerator_eval(survey_text, prompt_text, model_name)
    if result is not None:
        return result
    return generate_mock_report(survey_text, prompt_text)


def try_accelerator_eval(survey_text: str, prompt_text: str,
                         model_name: str) -> Optional[str]:
    """Try to evaluate using the accelerator tool."""
    import tempfile
    import subprocess

    pilot_cmd = ["accelerator", "--help"]
    try:
        subprocess.run(pilot_cmd, capture_output=True, timeout=5)
    except (subprocess.SubprocessError, FileNotFoundError):
        return None

    survey_file = None
    prompt_file = None
    try:
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".md", delete=False,
            encoding="utf-8"
        ) as sf:
            sf.write(survey_text)
            survey_file = sf.name

        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".txt", delete=False,
            encoding="utf-8"
        ) as pf:
            pf.write(prompt_text)
            prompt_file = pf.name

        cmd = [
            "accelerator", "run",
            "--model", model_name,
            "--prompt", prompt_file,
            "--input", survey_file,
        ]
        result = subprocess.run(
            cmd, capture_output=True, text=True,
            encoding="utf-8", timeout=300
        )
        if result.returncode == 0:
            return result.stdout
        return None
    except Exception:
        return None
    finally:
        for path in [survey_file, prompt_file]:
            if path and os.path.exists(path):
                os.unlink(path)


def generate_mock_report(survey_text: str, prompt_text: str) -> str:
    """Generate a placeholder report when LLM evaluation is unavailable."""
    return (
        "# Evaluation Report (Mock — LLM judge unavailable)\n\n"
        "**Note:** This is a placeholder report. Install the `accelerator`\n"
        "tool and configure a model to run the actual LLM-as-Judge evaluation.\n\n"
        "## Summary\n\n"
        "All evaluation dimensions and scoring criteria have been defined.\n"
        "Run with a configured LLM judge to obtain actual scores.\n"
    )


# ── Specialized evaluators (not using prompt.txt) ──

def evaluate_quiz(survey_text: str, topic: str, output_path: str) -> str:
    """Run SurveyBench quiz-based evaluation."""
    from evaluators.quiz_evaluator import QuizEvaluator
    evaluator = QuizEvaluator()
    report = evaluator.generate_report(survey_text, topic)
    if output_path:
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(report)
    print(f"Quiz evaluation report → {output_path}")
    return report


def evaluate_verifiability(survey_text: str, output_path: str) -> str:
    """Run DeepScholar-Bench verifiability evaluation."""
    from evaluators.verifiability_evaluator import VerifiabilityEvaluator
    evaluator = VerifiabilityEvaluator()
    report = evaluator.generate_report(survey_text)
    if output_path:
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(report)
    print(f"Verifiability report → {output_path}")
    return report


def evaluate_nugget_coverage(generated_text: str, reference_text: str,
                             output_path: str) -> str:
    """Run DeepScholar-Bench nugget coverage evaluation."""
    from evaluators.nugget_evaluator import NuggetEvaluator
    evaluator = NuggetEvaluator()
    report = evaluator.generate_report(generated_text, reference_text)
    if output_path:
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(report)
    print(f"Nugget coverage report → {output_path}")
    return report


def evaluate_document_importance(survey_text: str, output_path: str) -> str:
    """Run DeepScholar-Bench document importance evaluation."""
    from evaluators.nugget_evaluator import DocumentImportanceEvaluator
    evaluator = DocumentImportanceEvaluator()
    report = evaluator.generate_report(survey_text)
    if output_path:
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(report)
    print(f"Document importance report → {output_path}")
    return report


def evaluate_comparative(generated_text: str, topic: str,
                         reference_path: Optional[str], output_path: str) -> str:
    """Run comparative evaluation against SciReviewGen reference."""
    from evaluators.comparative_evaluator import ComparativeEvaluator, ReferenceSurvey
    evaluator = ComparativeEvaluator.__new__(ComparativeEvaluator)
    evaluator.loader = __import__(
        "loaders.scireviewgen_loader", fromlist=["SciReviewGenLoader"]
    ).SciReviewGenLoader()

    if reference_path:
        with open(reference_path, "r", encoding="utf-8") as f:
            content = f.read()
            evaluator.reference_survey = ReferenceSurvey(
                title="Custom Reference",
                abstract="",
                sections={"Full Text": content},
                references=[]
            )
    elif topic:
        from evaluators.comparative_evaluator import ReferenceSurvey
        data = evaluator.loader.get_review_by_topic(topic)
        if data:
            evaluator.reference_survey = ReferenceSurvey.from_dict(data)
        else:
            return f"# Comparative Evaluation\n\nNo reference found for topic: {topic}"

    evaluator.generated_survey_path = None
    evaluator._survey_text = generated_text

    report_lines = [
        "# Comparative Evaluation Report\n",
        "## Key Point Coverage\n",
    ]

    if evaluator.reference_survey:
        reference_text = evaluator.reference_survey.get_full_text()

        from evaluators.nugget_evaluator import NuggetEvaluator
        nugget_eval = NuggetEvaluator()
        nugget_result = nugget_eval.evaluate(generated_text, reference_text)

        report_lines.extend([
            f"- Reference: {evaluator.reference_survey.title}",
            f"- Nugget Coverage: {nugget_result['nugget_coverage']:.1f}%",
            f"- Matched / Total Nuggets: {nugget_result['matched_nuggets']}/{nugget_result['total_nuggets']}",
            f"- Average Confidence: {nugget_result['average_confidence']:.1f}%",
        ])

    report = "\n".join(report_lines)
    if output_path:
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(report)
    print(f"Comparative report → {output_path}")
    return report


# ── Entry point ──

def list_benchmarks() -> List[str]:
    return sorted(BENCHMARK_DIRS.keys())


def main():
    parser = argparse.ArgumentParser(
        description="Survey Benchmark Evaluation Framework\n\n"
                    "Implements SurveyBench, DeepSurvey-Bench, and DeepScholar-Bench",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--list", action="store_true",
                        help="List available benchmarks")
    parser.add_argument("--survey", "-s",
                        help="Path to the generated survey file")
    parser.add_argument("--benchmark", "-b",
                        choices=list(BENCHMARK_DIRS.keys()) + ["all"],
                        help="Benchmark to use")
    parser.add_argument("--output", "-o",
                        help="Output report path")
    parser.add_argument("--topic", "-t",
                        help="Topic for quiz generation or reference lookup")
    parser.add_argument("--reference", "-r",
                        help="Path to reference survey (for comparative eval)")
    parser.add_argument("--model",
                        default="deepseek-v4-flash",
                        help="LLM model for evaluation (default: deepseek-v4-flash)")
    parser.add_argument("--all", action="store_true",
                        help="Run all benchmarks and produce a combined report")

    args = parser.parse_args()

    if args.list:
        print("\nAvailable Benchmarks:")
        print("=" * 60)
        for name, info in sorted(BENCHMARK_DIRS.items()):
            print(f"  {name:25s}  {info['desc']}")
        print()
        return

    if not args.survey:
        parser.error("--survey is required unless using --list")

    survey_text = load_survey(args.survey)
    output_dir = args.output or os.path.dirname(args.survey)
    base_name = os.path.splitext(os.path.basename(args.survey))[0]

    runs = []

    if args.benchmark == "all":
        # Run everything
        for name in BENCHMARK_DIRS:
            if name in ("surveybench", "deepsurvey", "deepscholar"):
                runs.append(name)
            elif name == "quiz":
                runs.append(name)
            elif name == "verifiability":
                runs.append(name)
        specials = ["quiz", "verifiability"]
    elif args.benchmark == "quiz":
        if not args.topic:
            parser.error("--topic is required for quiz evaluation")
        evaluate_quiz(survey_text, args.topic,
                      args.output or f"{base_name}_quiz_report.md")
        return
    elif args.benchmark == "verifiability":
        evaluate_verifiability(survey_text,
                               args.output or f"{base_name}_verifiability_report.md")
        return
    elif args.benchmark == "nugget":
        if not args.reference and not args.topic:
            parser.error("--reference or --topic required for nugget evaluation")
        ref_text = load_survey(args.reference) if args.reference else ""
        evaluate_nugget_coverage(survey_text, ref_text,
                                 args.output or f"{base_name}_nugget_report.md")
        return
    elif args.benchmark == "document-importance":
        evaluate_document_importance(survey_text,
                                      args.output or f"{base_name}_importance_report.md")
        return
    elif args.benchmark == "comparative":
        evaluate_comparative(survey_text, args.topic or "", args.reference,
                             args.output or f"{base_name}_comparative_report.md")
        return
    else:
        runs = [args.benchmark.name if hasattr(args.benchmark, 'name') else args.benchmark]

    # Run core benchmark evaluations (LLM-as-Judge)
    for benchmark_name in runs:
        if benchmark_name not in BENCHMARK_DIRS:
            print(f"Unknown benchmark: {benchmark_name}")
            continue

        info = BENCHMARK_DIRS[benchmark_name]
        if info["dir"] is None:
            continue

        print(f"\nRunning {benchmark_name}...")
        prompt_text = load_prompt(info)

        if args.reference:
            ref_text = load_survey(args.reference)
            prompt_text = build_prompt_with_reference(prompt_text, ref_text)

        report = evaluate_with_llm(survey_text, prompt_text, args.model)

        report_path = args.output or f"{base_name}_{benchmark_name}_report.md"
        with open(report_path, "w", encoding="utf-8") as f:
            f.write(report)
        print(f"Report saved → {report_path}")

    # Special evaluations
    if args.benchmark == "all":
        if args.topic:
            quiz_path = f"{base_name}_quiz_report.md"
            evaluate_quiz(survey_text, args.topic, quiz_path)

        ver_path = f"{base_name}_verifiability_report.md"
        evaluate_verifiability(survey_text, ver_path)

        imp_path = f"{base_name}_importance_report.md"
        evaluate_document_importance(survey_text, imp_path)

    print("\nDone!")


if __name__ == "__main__":
    main()
