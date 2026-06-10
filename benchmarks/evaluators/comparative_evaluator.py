import os
import json
from typing import Dict, List, Optional
from benchmarks.loaders.scireviewgen_loader import SciReviewGenLoader, ReferenceSurvey

class ComparativeEvaluator:
    def __init__(self, generated_survey_path: str, reference_survey: Optional[ReferenceSurvey] = None):
        self.generated_survey_path = generated_survey_path
        self.reference_survey = reference_survey
        self.loader = SciReviewGenLoader()
    
    def load_generated_survey(self) -> str:
        """加载生成的综述"""
        if os.path.exists(self.generated_survey_path):
            with open(self.generated_survey_path, 'r', encoding='utf-8') as f:
                return f.read()
        return ""
    
    def get_reference_by_topic(self, topic: str) -> Optional[ReferenceSurvey]:
        """根据主题获取参考综述"""
        data = self.loader.get_review_by_topic(topic)
        if data:
            return ReferenceSurvey.from_dict(data)
        return None
    
    def extract_key_points(self, text: str) -> List[str]:
        """从文本中提取关键点"""
        import re
        
        key_points = []
        
        sections = re.split(r'^#{1,3}\s+', text, flags=re.MULTILINE)
        for section in sections[1:]:
            lines = section.strip().split('\n')
            if lines:
                section_title = lines[0]
                content = '\n'.join(lines[1:])
                
                sentences = re.split(r'[.!?]+', content)
                for sentence in sentences:
                    sentence = sentence.strip()
                    if len(sentence) > 30:
                        key_points.append(f"{section_title}: {sentence[:100]}...")
        
        return key_points[:20]
    
    def compare_key_points(self) -> Dict:
        """比较生成综述和参考综述的关键点覆盖"""
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
                    matched_points.append({"reference": ref_point, "generated": gen_point})
                    found = True
                    break
            if not found:
                missed_points.append(ref_point)
        
        return {
            "generated_key_points": generated_points,
            "reference_key_points": reference_points,
            "matched_count": len(matched_points),
            "missed_count": len(missed_points),
            "coverage_ratio": len(matched_points) / len(reference_points) if reference_points else 0,
            "matched_points": matched_points,
            "missed_points": missed_points
        }
    
    def _has_overlap(self, text1: str, text2: str) -> bool:
        """检查两段文本是否有内容重叠"""
        words1 = set(text1.lower().split())
        words2 = set(text2.lower().split())
        common = words1.intersection(words2)
        return len(common) >= 5
    
    def evaluate_rouge(self) -> Dict:
        """计算ROUGE指标（需要rouge库）"""
        try:
            from rouge import Rouge
            
            generated_text = self.load_generated_survey()
            reference_text = self.reference_survey.get_full_text() if self.reference_survey else ""
            
            if not reference_text:
                return {"error": "No reference survey for ROUGE calculation"}
            
            rouge = Rouge()
            scores = rouge.get_scores(generated_text, reference_text, avg=True)
            return scores
        except ImportError:
            return {"error": "rouge library not installed"}
        except Exception as e:
            return {"error": str(e)}
    
    def generate_comparative_report(self, topic: str = "") -> str:
        """生成对比评估报告"""
        if not self.reference_survey:
            self.reference_survey = self.get_reference_by_topic(topic)
        
        comparison = self.compare_key_points()
        rouge_scores = self.evaluate_rouge()
        
        report = []
        report.append("# Comparative Evaluation Report")
        report.append("")
        report.append("## Overview")
        report.append(f"- Generated Survey: {self.generated_survey_path}")
        report.append(f"- Reference Survey: {self.reference_survey.title if self.reference_survey else 'Not provided'}")
        report.append("")
        
        report.append("## Key Point Coverage")
        report.append(f"- Generated key points: {len(comparison.get('generated_key_points', []))}")
        report.append(f"- Reference key points: {len(comparison.get('reference_key_points', []))}")
        report.append(f"- Matched: {comparison.get('matched_count', 0)}")
        report.append(f"- Missed: {comparison.get('missed_count', 0)}")
        report.append(f"- Coverage Ratio: {comparison.get('coverage_ratio', 0):.2%}")
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
                report.append(f"| {metric} | {scores['f']:.4f} | {scores['p']:.4f} | {scores['r']:.4f} |")
        
        return '\n'.join(report)

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Comparative evaluation of generated surveys")
    parser.add_argument("--generated", required=True, help="Path to generated survey")
    parser.add_argument("--topic", help="Topic to find reference survey")
    parser.add_argument("--reference", help="Path to reference survey file")
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
        evaluator.reference_survey = evaluator.get_reference_by_topic(args.topic)
    
    report = evaluator.generate_comparative_report(args.topic)
    
    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            f.write(report)
        print(f"Report saved to {args.output}")
    else:
        print(report)

if __name__ == "__main__":
    main()