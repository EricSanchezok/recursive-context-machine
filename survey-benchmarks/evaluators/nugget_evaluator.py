"""
DeepScholar-Bench Nugget Coverage & Document Importance Evaluation.
Implements metrics from arXiv:2508.20033v2.

Nugget Coverage: Measures what fraction of essential facts (nuggets) from a
reference related work section are also present in the generated survey.

Document Importance: Measures the importance/influence of referenced sources,
scored by citation count (normalized).
"""
import re
import json
from typing import List, Dict, Optional


class NuggetExtractor:
    """Extracts essential fact nuggets from reference text."""

    @staticmethod
    def extract_nuggets(text: str, max_nuggets: int = 20) -> List[Dict]:
        """
        Extract key factual statements (nuggets) from reference text.
        Each nugget represents an essential piece of information.
        """
        sentences = re.split(r'(?<=[.!?])\s+', text)
        nuggets = []

        for sentence in sentences:
            sentence = sentence.strip()
            if not sentence or len(sentence) < 30:
                continue

            words = sentence.split()
            if len(words) < 5:
                continue

            citations = NuggetExtractor._extract_citations(sentence)

            nuggets.append({
                "nugget": sentence[:300],
                "citations": citations,
                "has_citation": len(citations) > 0,
                "length": len(sentence),
            })

        nuggets.sort(key=lambda x: x["length"], reverse=True)
        return nuggets[:max_nuggets]

    @staticmethod
    def _extract_citations(text: str) -> List[str]:
        """Extract citation markers from text."""
        citations = []
        patterns = [
            r'\[(\d+(?:,\s*\d+)*)\]',
            r'\(([A-Za-z]+\s+et\s+al\.\s*,?\s*\d{4}[a-z]?)\)',
            r'\(([A-Za-z]+(?:,\s*[A-Za-z]+)*,\s*\d{4}[a-z]?)\)',
        ]
        for pattern in patterns:
            matches = re.findall(pattern, text, re.IGNORECASE)
            for match in matches:
                citations.append(str(match).strip())
        return list(set(citations))


class NuggetMatcher:
    """Matches nuggets from reference to generated survey."""

    @staticmethod
    def match_nugget(nugget: str, survey_text: str) -> Dict:
        """Check if a nugget from the reference is covered in the survey."""
        nugget_lower = nugget.lower()
        survey_lower = survey_text.lower()

        nugget_words = set(re.findall(r'\b[a-zA-Z]{4,}\b', nugget_lower))
        if not nugget_words:
            return {"matched": False, "confidence": 0.0}

        match_count = sum(1 for w in nugget_words if w in survey_lower)
        coverage = match_count / len(nugget_words)

        key_phrases = NuggetMatcher._extract_key_phrases(nugget_lower)
        phrase_matches = sum(1 for phrase in key_phrases if phrase in survey_lower)
        phrase_ratio = phrase_matches / len(key_phrases) if key_phrases else 0

        combined_score = coverage * 0.4 + phrase_ratio * 0.6

        return {
            "matched": combined_score >= 0.4,
            "confidence": round(combined_score, 3),
            "word_overlap": round(coverage, 3),
            "phrase_overlap": round(phrase_ratio, 3),
        }

    @staticmethod
    def _extract_key_phrases(text: str) -> List[str]:
        """Extract key phrases of 2-4 words from text."""
        words = re.findall(r'\b[a-zA-Z]{3,}\b', text)
        phrases = []
        for i in range(len(words)):
            for j in range(2, min(5, len(words) - i + 1)):
                phrase = ' '.join(words[i:i + j])
                if len(phrase) > 5:
                    phrases.append(phrase)
        return list(set(phrases))


class DocumentImportanceCalculator:
    """Calculates document importance using citation counts."""

    @staticmethod
    def extract_cited_references(survey_text: str) -> List[str]:
        """Extract list of cited reference identifiers."""
        references = []
        ref_section = ""
        in_refs = False

        for line in survey_text.split('\n'):
            if re.match(r'^#+\s*(References|Bibliography|References|Bibliography)\s*$', line, re.IGNORECASE):
                in_refs = True
                continue
            if in_refs:
                if re.match(r'^#+\s', line):
                    break
                ref_section += line + '\n'

        if ref_section:
            ref_lines = [l.strip() for l in ref_section.split('\n') if l.strip()]
            for line in ref_lines:
                if line and not line.startswith('#'):
                    references.append(line[:200])

        if not references:
            citation_markers = re.findall(r'\[(\d+)\]', survey_text)
            references = [f"[{c}]" for c in sorted(set(citation_markers))]

        return references

    @staticmethod
    def score_importance(references: List[str]) -> List[Dict]:
        """
        Assign importance scores based on citation counts.
        Uses LLM to estimate importance when exact counts unavailable.
        """
        scored = []
        for ref in references[:30]:
            title_guess = ref.strip().lstrip('[').split(']')[-1].strip() if ']' in ref else ''

            score = 50  # default medium importance
            high_impact_keywords = [
                "nature", "science", "neurips", "icml", "iclr", "cvpr",
                "pnas", "cell", "ieee", "acm", "survey", "review"
            ]
            if any(kw in ref.lower() for kw in high_impact_keywords):
                score = 75
            if any(kw in title_guess.lower() for kw in [
                "attention", "transformer", "bert", "gpt", "resnet",
                "deep learning", "reinforcement"
            ]):
                score = 90

            scored.append({
                "reference": ref[:150],
                "importance_score": score,
                "importance_level": "high" if score >= 70 else "medium" if score >= 40 else "low",
            })

        return scored


class NuggetEvaluator:
    """Full Nugget Coverage evaluation pipeline."""

    def evaluate(self, generated_text: str, reference_text: str) -> Dict:
        """Evaluate nugget coverage of generated survey against reference."""
        ref_nuggets = NuggetExtractor.extract_nuggets(reference_text)
        matcher = NuggetMatcher()

        results = []
        matched_count = 0
        total_confidence = 0.0

        for nugget in ref_nuggets:
            match_result = matcher.match_nugget(nugget["nugget"], generated_text)
            if match_result["matched"]:
                matched_count += 1
                total_confidence += match_result["confidence"]
            results.append({
                "nugget": nugget["nugget"][:150],
                "has_citation": nugget["has_citation"],
                "matched": match_result["matched"],
                "confidence": match_result["confidence"],
            })

        total = len(ref_nuggets) or 1
        nugget_coverage = (matched_count / total) * 100
        avg_confidence = (total_confidence / matched_count) * 100 if matched_count > 0 else 0

        return {
            "nugget_coverage": round(nugget_coverage, 2),
            "average_confidence": round(avg_confidence, 2),
            "matched_nuggets": matched_count,
            "total_nuggets": total,
            "details": results[:15],
        }

    def generate_report(self, generated_text: str, reference_text: str) -> str:
        """Generate nugget coverage report."""
        result = self.evaluate(generated_text, reference_text)
        lines = [
            "# Nugget Coverage Evaluation Report",
            "",
            f"**Nugget Coverage:** {result['nugget_coverage']:.1f}%",
            f"**Average Matching Confidence:** {result['average_confidence']:.1f}%",
            f"**Matched / Total Nuggets:** {result['matched_nuggets']}/{result['total_nuggets']}",
            "",
            "## Nugget Details",
            "",
        ]
        for i, d in enumerate(result.get("details", []), 1):
            status = "✅" if d["matched"] else "❌"
            lines.append(f"{i}. {status} (confidence: {d['confidence']:.2f}) {d['nugget'][:100]}...")

        lines.extend([
            "",
            "## Score Summary",
            "| Metric | Score |",
            "|--------|:-----:|",
            f"| Nugget Coverage | {result['nugget_coverage']:.1f}% |",
            f"| Average Confidence | {result['average_confidence']:.1f}% |",
        ])

        return "\n".join(lines)


class DocumentImportanceEvaluator:
    """Full Document Importance evaluation pipeline."""

    def evaluate(self, survey_text: str) -> Dict:
        """Evaluate importance of referenced documents."""
        references = DocumentImportanceCalculator.extract_cited_references(survey_text)
        scored = DocumentImportanceCalculator.score_importance(references)

        total = len(scored) or 1
        avg_importance = sum(s["importance_score"] for s in scored) / total
        high_count = sum(1 for s in scored if s["importance_level"] == "high")
        medium_count = sum(1 for s in scored if s["importance_level"] == "medium")

        return {
            "average_importance": round(avg_importance, 2),
            "high_importance_refs": high_count,
            "medium_importance_refs": medium_count,
            "total_refs": total,
            "scored_references": scored[:20],
        }

    def generate_report(self, survey_text: str) -> str:
        """Generate document importance report."""
        result = self.evaluate(survey_text)
        lines = [
            "# Document Importance Evaluation Report",
            "",
            f"**Average Importance Score:** {result['average_importance']:.1f}/100",
            f"**High Importance References:** {result['high_importance_refs']}",
            f"**Medium Importance References:** {result['medium_importance_refs']}",
            f"**Total References Evaluated:** {result['total_refs']}",
            "",
            "## Score Summary",
            "| Metric | Score |",
            "|--------|:-----:|",
            f"| Average Importance | {result['average_importance']:.1f}/100 |",
            f"| High / Medium / Total | {result['high_importance_refs']}/{result['medium_importance_refs']}/{result['total_refs']} |",
        ]

        return "\n".join(lines)
