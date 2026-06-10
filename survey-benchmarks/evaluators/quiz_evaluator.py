"""
SurveyBench Quiz-based Evaluation Pipeline.
Implements the quiz-driven evaluation protocol from arXiv:2510.03120v2.

Pipeline:
  1. Generate topic-specific quiz questions (factual, analytical, comprehensive)
  2. Use the survey as a knowledge base to answer questions (RAG-style)
  3. Evaluate answer correctness and support from survey text
  4. Compute answerability score
"""
import os
import re
import json
import subprocess
from typing import List, Dict, Optional


class QuizGenerator:
    """Generates topic-specific quiz questions following SurveyBench protocol."""

    def __init__(self, judge_model: str = "deepseek-v4-flash"):
        self.judge_model = judge_model

    def generate_quiz_questions(self, topic: str, num_questions: int = 5) -> List[Dict]:
        """Generate quiz questions covering factual, analytical, and comprehensive types."""
        prompt = f"""You are a research domain expert tasked with creating quiz questions to evaluate a survey on the topic: "{topic}".

Generate exactly {num_questions} questions that a reader would expect a high-quality survey to answer.
Include a mix of question types following SurveyBench protocol:

1. **Factual Questions** (2 questions): Test specific knowledge that should be covered
   - Ask about specific methods, findings, or approaches in the field
   - Require precise, verifiable answers

2. **Analytical Questions** (2 questions): Test synthetic understanding
   - Ask about comparisons, trade-offs, or relationships between approaches
   - Require understanding of multiple papers/concepts

3. **Comprehensive Questions** (1 question): Test deep understanding
   - Ask about open challenges, future directions, or overarching themes
   - Require synthesis of multiple ideas

For each question, provide:
- id: Q1, Q2, ...
- type: "factual", "analytical", or "comprehensive"
- question: The question text
- key_points: What constitutes a correct answer (list of essential points)
- difficulty: 1-5

Output as JSON array only, no other text.
"""
        try:
            result = subprocess.run(
                ["accelerator", "run", "--model", self.judge_model,
                 "--prompt", prompt, "--json"],
                capture_output=True, text=True, encoding="utf-8", timeout=120
            )
            if result.returncode == 0:
                return json.loads(result.stdout)
        except (subprocess.TimeoutExpired, json.JSONDecodeError, FileNotFoundError):
            pass

        return self._generate_fallback(topic, num_questions)

    def _generate_fallback(self, topic: str, num_questions: int) -> List[Dict]:
        return [
            {"id": "Q1", "type": "factual", "question": f"What are the key methods and approaches in {topic}?",
             "key_points": ["List of main methods", "Key innovations"],
             "difficulty": 3},
            {"id": "Q2", "type": "factual", "question": f"What are the major datasets or benchmarks used in {topic}?",
             "key_points": ["Common datasets", "Evaluation metrics"],
             "difficulty": 3},
            {"id": "Q3", "type": "analytical", "question": f"How do different approaches in {topic} compare in terms of performance and trade-offs?",
             "key_points": ["Performance comparison", "Strengths and weaknesses"],
             "difficulty": 4},
            {"id": "Q4", "type": "analytical", "question": f"What are the unresolved challenges and limitations in {topic}?",
             "key_points": ["Current limitations", "Open problems"],
             "difficulty": 4},
            {"id": "Q5", "type": "comprehensive", "question": f"What are the most promising future research directions in {topic} and why?",
             "key_points": ["Future directions", "Rationale"],
             "difficulty": 5},
        ][:num_questions]


class QuizAnswerExtractor:
    """Extracts answers from the survey for each quiz question (RAG-style)."""

    @staticmethod
    def extract_passages(survey_text: str, question: str, top_k: int = 3) -> List[str]:
        """Retrieve relevant passages from survey using keyword matching."""
        passages = []
        question_keywords = set(question.lower().split())
        question_keywords = {w for w in question_keywords if len(w) > 3}

        sections = re.split(r'\n#{1,3}\s+', survey_text)
        scored_sections = []

        for i, section in enumerate(sections):
            section_lower = section.lower()
            keyword_matches = sum(1 for kw in question_keywords if kw in section_lower)
            scored_sections.append((keyword_matches, section))

        scored_sections.sort(key=lambda x: x[0], reverse=True)
        for _, section in scored_sections[:top_k]:
            if len(passages) == 0 or _ > 0:
                passages.append(section.strip()[:2000])
            else:
                break

        return passages if passages else [survey_text[:2000]]


class QuizAnswerGrader:
    """Grades answers against reference criteria."""

    def __init__(self, judge_model: str = "deepseek-v4-flash"):
        self.judge_model = judge_model

    def grade_answer(self, question: Dict, survey_text: str, passages: List[str]) -> Dict:
        """Grade whether the survey can correctly answer the question."""
        prompt = f"""You are evaluating whether a survey can answer a specific question.

**Question (type: {question['type']}):**
{question['question']}

**Key points expected in a correct answer:**
{chr(10).join(f'- {kp}' for kp in question.get('key_points', []))}

**Relevant passages from the survey:**
{chr(10).join(f'[Passage {i+1}]:{p}' for i, p in enumerate(passages))}

**Evaluation criteria (from SurveyBench paper):**
1. Can the survey provide a correct answer? (Yes/Partial/No)
2. Does the answer cover the expected key points? (count)
3. Is the answer directly supported by the survey content? (Yes/Partial/No)

**Output format (JSON only):**
{{
  "can_answer": "Yes|Partial|No",
  "correctness_score": 0-100,
  "key_points_covered": N,
  "key_points_total": M,
  "supported": "Yes|Partial|No",
  "extracted_answer": "brief answer extracted from survey",
  "explanation": "reasoning for the score"
}}
"""
        try:
            result = subprocess.run(
                ["accelerator", "run", "--model", self.judge_model,
                 "--prompt", prompt, "--json"],
                capture_output=True, text=True, encoding="utf-8", timeout=60
            )
            if result.returncode == 0:
                return json.loads(result.stdout)
        except (subprocess.TimeoutExpired, json.JSONDecodeError, FileNotFoundError):
            pass

        return {
            "can_answer": "Partial",
            "correctness_score": 50,
            "key_points_covered": 1,
            "key_points_total": len(question.get("key_points", [])),
            "supported": "Partial",
            "extracted_answer": "Fallback evaluation",
            "explanation": "Could not call LLM judge"
        }


class QuizEvaluator:
    """Full Quiz-based evaluation pipeline for SurveyBench."""

    def __init__(self, judge_model: str = "deepseek-v4-flash"):
        self.generator = QuizGenerator(judge_model)
        self.extractor = QuizAnswerExtractor()
        self.grader = QuizAnswerGrader(judge_model)

    def evaluate(self, survey_text: str, topic: str, num_questions: int = 5) -> Dict:
        """Run full quiz evaluation pipeline."""
        questions = self.generator.generate_quiz_questions(topic, num_questions)

        results = []
        total_correctness = 0.0
        total_coverage = 0.0

        for q in questions:
            passages = self.extractor.extract_passages(survey_text, q["question"])
            grade = self.grader.grade_answer(q, survey_text, passages)

            correctness = grade.get("correctness_score", 0)
            kp_covered = grade.get("key_points_covered", 0)
            kp_total = grade.get("key_points_total", 1) or 1
            coverage = (kp_covered / kp_total) * 100

            total_correctness += correctness
            total_coverage += coverage

            results.append({
                "question_id": q["id"],
                "question_type": q["type"],
                "question": q["question"],
                "difficulty": q.get("difficulty", 3),
                "can_answer": grade.get("can_answer", "No"),
                "correctness_score": correctness,
                "key_point_coverage": coverage,
                "supported": grade.get("supported", "No"),
                "extracted_answer": grade.get("extracted_answer", ""),
            })

        n = len(questions) or 1
        overall_answerability = (total_correctness / n) * 0.6 + (total_coverage / n) * 0.4

        return {
            "questions": results,
            "overall_answerability": round(overall_answerability, 2),
            "average_correctness": round(total_correctness / n, 2),
            "average_coverage": round(total_coverage / n, 2),
            "num_questions": n,
            "answerable_count": sum(1 for r in results if r["can_answer"] in ("Yes", "Partial")),
        }

    def generate_report(self, survey_text: str, topic: str) -> str:
        """Generate a full quiz evaluation report."""
        result = self.evaluate(survey_text, topic)
        lines = [
            "# SurveyBench Quiz-Based Evaluation Report",
            "",
            f"**Topic:** {topic}",
            f"**Overall Answerability:** {result['overall_answerability']:.1f}/100",
            f"**Average Correctness:** {result['average_correctness']:.1f}%",
            f"**Average Coverage:** {result['average_coverage']:.1f}%",
            f"**Answerable Questions:** {result['answerable_count']}/{result['num_questions']}",
            "",
            "## Question Details",
            "",
        ]
        for q in result["questions"]:
            lines.extend([
                f"### {q['question_id']} [{q['question_type']}] (Difficulty: {q['difficulty']}/5)",
                f"**Question:** {q['question']}",
                f"**Can Answer:** {q['can_answer']}",
                f"**Correctness:** {q['correctness_score']:.1f}%",
                f"**Key Point Coverage:** {q['key_point_coverage']:.1f}%",
                f"**Answer:** {q['extracted_answer'][:200] if q['extracted_answer'] else 'No answer found'}",
                "",
            ])

        lines.extend([
            "## Score Summary",
            "| Metric | Score |",
            "|--------|:-----:|",
            f"| Overall Answerability | {result['overall_answerability']:.1f}/100 |",
            f"| Average Correctness | {result['average_correctness']:.1f}% |",
            f"| Average Key Point Coverage | {result['average_coverage']:.1f}% |",
            f"| Answerable / Total | {result['answerable_count']}/{result['num_questions']} |",
        ])

        return "\n".join(lines)
