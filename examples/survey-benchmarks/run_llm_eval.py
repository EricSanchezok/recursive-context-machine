"""Run all three benchmark evaluations using LLM-as-Judge.
Usage:
    python run_llm_eval.py <survey_path> <output_dir> [--topic <topic>]
"""
import os
import sys
import time

# Ensure we can import evaluators
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

# Set the API key from user
os.environ["CROSS_JUDGE_API_KEY"] = "sk-73f1b9dcf4d6ca55e987cddcac4ce1a313b7269e37d6e3658b7ed6c28f9e8de3"

from evaluators.llm_judge import run_benchmark_evaluation, extract_scores

# Config
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
BENCHMARKS = {
    "surveybench": {
        "prompt": os.path.join(BASE_DIR, "surveybench", "prompt.txt"),
    },
    "deepsurvey": {
        "prompt": os.path.join(BASE_DIR, "deepsurvey-bench", "prompt.txt"),
    },
    "deepscholar": {
        "prompt": os.path.join(BASE_DIR, "deepscholar-bench", "prompt.txt"),
    },
}


def main():
    if len(sys.argv) < 2:
        print("Usage: python run_llm_eval.py <survey_path> [output_dir] [--topic <topic>]")
        sys.exit(1)
    
    survey_path = os.path.abspath(sys.argv[1])
    if not os.path.exists(survey_path):
        print(f"ERROR: Survey not found: {survey_path}")
        sys.exit(1)
    
    # Output dir
    script_dir = os.path.dirname(os.path.abspath(__file__))
    if len(sys.argv) >= 3 and not sys.argv[2].startswith("--"):
        output_dir = os.path.abspath(sys.argv[2])
    else:
        run_name = "eval_" + time.strftime("%Y%m%dT%H%M%S")
        output_dir = os.path.join(script_dir, "reports", run_name)
    os.makedirs(output_dir, exist_ok=True)
    
    # Topic
    topic = None
    for i, arg in enumerate(sys.argv):
        if arg == "--topic" and i + 1 < len(sys.argv):
            topic = sys.argv[i + 1]
    
    print(f"Survey: {survey_path}")
    print(f"Output: {output_dir}")
    print(f"Topic: {topic or '(not specified)'}")
    print()
    
    # Run evaluations
    results = {}
    for name, config in BENCHMARKS.items():
        print(f"\n{'='*60}")
        print(f"  {name.upper()} Evaluation")
        print(f"{'='*60}")
        
        # Check prompt
        prompt_path = config["prompt"]
        if not os.path.exists(prompt_path):
            print(f"  ERROR: Prompt not found: {prompt_path}")
            continue
        
        output_path = os.path.join(output_dir, f"{name}_report.md")
        
        result = run_benchmark_evaluation(survey_path, prompt_path, output_path)
        results[name] = result
        
        if result["status"] == "ok":
            print(f"  Scores: {result.get('scores', {})}")
        else:
            print(f"  Error: {result.get('message', 'Unknown')}")
    
    # Generate summary
    print(f"\n{'='*60}")
    print(f"  SUMMARY")
    print(f"{'='*60}")
    
    summary_path = os.path.join(output_dir, "00_summary.md")
    with open(summary_path, "w", encoding="utf-8") as f:
        f.write(f"# 全面评估汇总报告\n\n")
        f.write(f"**Survey**: {survey_path}\n")
        f.write(f"**Topic**: {topic or 'N/A'}\n")
        f.write(f"**Model**: GPT-5.5 (via gmncode)\n\n")
        f.write(f"---\n\n")
        f.write(f"## 评分总览\n\n")
        f.write(f"| Benchmark | Key Scores | 状态 |\n")
        f.write(f"|-----------|-----------|:----:|\n")
        
        for name, result in results.items():
            scores = result.get("scores", {})
            score_str = "; ".join([f"{k}: {v:.1f}%" for k, v in list(scores.items())[:4]])
            status = "✅" if result["status"] == "ok" else "❌"
            f.write(f"| **{name.upper()}** | {score_str or 'N/A'} | {status} |\n")
        
        f.write(f"\n---\n\n")
        f.write(f"## 详细报告\n\n")
        for name, result in results.items():
            output_path = result.get("output", "")
            f.write(f"- [{name.upper()}报告]({os.path.basename(output_path)})\n")
    
    print(f"\nSummary: {summary_path}")
    print(f"Reports: {output_dir}")


if __name__ == "__main__":
    main()
