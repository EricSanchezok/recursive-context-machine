import os
import json
import re
from typing import Dict, List, Optional
from loaders.scireviewgen_loader import SciReviewGenLoader, ReferenceSurvey


class ComparativeEvaluator:
    def __init__(self, generated_survey_path: str,
                 reference_survey: Optional[ReferenceSurvey] = None):
        self.generated_survey_path = generated_survey_path
        self.reference_survey = reference_survey
        self.loader = SciReviewGenLoader()

    def load_generated_survey(self) -> str:
        """Load the generated survey text from file."""
        if os.path.exists(self.generated_survey_path):
            with open(self.generated_survey_path, 'r', encoding='utf-8') as f:
                return f.read()
        return ""

    def get_reference_by_topic(self, topic: str) -> Optional[ReferenceSurvey]:
        """Fuzzy topic matching — tries multiple strategies."""
        dataset = self.load_dataset()
        if not dataset:
            return None

        topic_lower = topic.lower()
        words = topic_lower.split()

        best_match = None
        best_score = 0

        for item in dataset:
            title = item.get("title", "").lower()

            # Strategy 1: exact substring match
            if topic_lower in title:
                return ReferenceSurvey.from_dict(item)

            # Strategy 2: word overlap score
            title_words = set(title.split())
            overlap = len(set(words) & title_words)
            score = overlap / max(len(words), 1)

            if score > best_score:
                best_score = score
                best_match = item

        # Return best match if score is reasonable
        if best_match and best_score >= 0.3:
            print(f"  Fuzzy matched: '{topic}' -> "
                  f"'{best_match.get('title', '')}' (score={best_score:.2f})")
            return ReferenceSurvey.from_dict(best_match)

        return None

    def load_dataset(self) -> List[Dict]:
        """Load SciReviewGen dataset; return fallback samples if unavailable."""
        dataset = self.loader.load_dataset()
        if dataset and len(dataset) > 2:
            return dataset

        # Check for sample JSON files
        sample_data_dir = ("datasets/SciReviewGen/samples"
                           if os.path.exists("datasets/SciReviewGen/samples")
                           else None)
        if sample_data_dir:
            samples = []
            for fname in os.listdir(sample_data_dir):
                if fname.endswith(".json"):
                    with open(os.path.join(sample_data_dir, fname)) as f:
                        samples.append(json.load(f))
            if samples:
                return samples

        return self.loader._generate_fallback_samples()

    def extract_key_points(self, text: str) -> List[str]:
        """Extract key points with better sentence splitting."""
        key_points = []

        # Split by markdown headings
        sections = re.split(r'(?:^|\n)(#{1,3})\s+', text, flags=re.MULTILINE)

        current_level = ""
        current_title = ""

        for i, part in enumerate(sections):
            part = part.strip()
            if not part:
                continue

            if i % 2 == 1:
                current_level = part
                continue
            elif i % 2 == 0 and current_level:
                lines = part.split('\n')
                current_title = lines[0].strip()
                content = '\n'.join(lines[1:])
            else:
                content = part
                current_title = "Preamble"

            # Extract sentences from content
            sentences = re.split(r'(?<=[.!?])\s+', content)
            for sentence in sentences:
                sentence = sentence.strip()
                sentence = re.sub(r'^[-*\s]+', '', sentence)
                sentence = re.sub(r'\*\*([^*]+)\*\*', r'\1', sentence)
                if 40 < len(sentence) < 500:
                    key_points.append(f"[{current_title}] {sentence[:200]}")

        return key_points[:30]

    def compare_key_points(self) -> Dict:
        """Compare generated and reference key points for coverage."""
        if not self.reference_survey:
            return {"error": "No reference survey provided"}

        generated_text = self.load_generated_survey()
        reference_text = self.reference_survey.get_full_text()

        generated_points = self.extract_key_points(generated_text)
        reference_points = self.extract_key_points(reference_text)

        matched_points = []
        missed_points = []

        for ref_point in reference_points:
            found = False
            for gen_point in generated_points:
                if self._has_overlap(ref_point, gen_point):
                    matched_points.append({
                        "reference": ref_point,
                        "generated": gen_point
                    })
                    found = True
                    break
            if not found:
                missed_points.append(ref_point)

        return {
            "generated_key_points": generated_points,
            "reference_key_points": reference_points,
            "matched_count": len(matched_points),
            "missed_count": len(missed_points),
            "coverage_ratio": (len(matched_points) /
                               max(len(reference_points), 1)),
            "matched_points": matched_points,
            "missed_points": missed_points
        }

    def _has_overlap(self, text1: str, text2: str) -> bool:
        """Check whether two text spans share enough content words."""
        words1 = set(text1.lower().split())
        words2 = set(text2.lower().split())
        common = words1.intersection(words2)
        return len(common) >= 5

    def evaluate_rouge(self) -> Dict:
        """Compute ROUGE scores (requires `rouge` library)."""
        try:
            from rouge import Rouge
            generated_text = self.load_generated_survey()
            reference_text = (self.reference_survey.get_full_text()
                              if self.reference_survey else "")
            if not reference_text:
                return {"error": "No reference survey for ROUGE"}
            rouge = Rouge()
            return rouge.get_scores(generated_text, reference_text,
                                    avg=True)
        except ImportError:
            return {"error": "rouge library not installed"}
        except Exception as e:
            return {"error": str(e)}

    def generate_comparative_report(self, topic: str = "") -> str:
        """Generate a full comparative evaluation report."""
        if not self.reference_survey:
            self.reference_survey = self.get_reference_by_topic(topic)

        comparison = self.compare_key_points()
        rouge_scores = self.evaluate_rouge()

        report = []
        report.append("# Comparative Evaluation Report\n")
        report.append("## Overview")
        report.append(f"- Generated Survey: {self.generated_survey_path}")
        report.append(
            f"- Reference Survey: "
            f"{self.reference_survey.title if self.reference_survey else 'Not provided'}"
        )
        report.append("")

        report.append("## Key Point Coverage")
        report.append(
            f"- Generated key points: "
            f"{len(comparison.get('generated_key_points', []))}"
        )
        report.append(
            f"- Reference key points: "
            f"{len(comparison.get('reference_key_points', []))}"
        )
        report.append(f"- Matched: {comparison.get('matched_count', 0)}")
        report.append(f"- Missed: {comparison.get('missed_count', 0)}")
        report.append(
            f"- Coverage Ratio: "
            f"{comparison.get('coverage_ratio', 0):.2%}"
        )
        report.append("")

        if comparison.get('missed_points'):
            report.append("### Missing Key Points from Reference")
            for i, point in enumerate(comparison['missed_points'][:5], 1):
                report.append(f"{i}. {point}")
            report.append("")

        if isinstance(rouge_scores, dict) and 'rouge-1' in rouge_scores:
            report.append("## ROUGE Scores")
            report.append("| Metric | F1 | Precision | Recall |")
            report.append("|--------|----|-----------|--------|")
            for metric, scores in rouge_scores.items():
                report.append(
                    f"| {metric} | {scores['f']:.4f} | "
                    f"{scores['p']:.4f} | {scores['r']:.4f} |"
                )

        return '\n'.join(report)


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Comparative evaluation of generated surveys")
    parser.add_argument("--generated", required=True,
                        help="Path to generated survey")
    parser.add_argument("--topic",
                        help="Topic to find reference survey")
    parser.add_argument("--reference",
                        help="Path to reference survey file")
    parser.add_argument("--output", help="Output report path")

    args = parser.parse_args()

    evaluator = ComparativeEvaluator(args.generated)

    if args.reference:
        with open(args.reference, 'r', encoding='utf-8') as f:
            content = f.read()
            evaluator.reference_survey = ReferenceSurvey(
                title="Custom Reference",
                abstract="",
                sections={"Full Text": content},
                references=[]
            )
    elif args.topic:
        evaluator.reference_survey = evaluator.get_reference_by_topic(
            args.topic)

    report = evaluator.generate_comparative_report(args.topic)

    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            f.write(report)
        print(f"Report saved to {args.output}")
    else:
        print(report)


if __name__ == "__main__":
    main()
