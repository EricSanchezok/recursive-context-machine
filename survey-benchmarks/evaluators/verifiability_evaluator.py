"""
DeepScholar-Bench Verifiability Evaluation Module.
Implements citation precision and claim coverage metrics from arXiv:2508.20033v2.

Pipeline:
  1. Extract claims (factual statements) from the generated survey
  2. Extract citations linked to each claim
  3. Verify whether each cited source actually supports its accompanying claim
  4. Calculate Citation Precision and Claim Coverage as 0-100% scores
"""
import re
import json
from typing import List, Dict, Tuple


class ClaimExtractor:
    """Extracts claims and their citations from survey text."""

    @staticmethod
    def extract_claims(survey_text: str) -> List[Dict]:
        """Extract claims with their associated citations."""
        claims = []
        sentences = re.split(r'(?<=[.!?])\s+', survey_text)

        for sentence in sentences:
            sentence = sentence.strip()
            if not sentence or len(sentence) < 20:
                continue

            citations = ClaimExtractor._extract_citations(sentence)
            if citations:
                claims.append({
                    "claim": sentence[:300],
                    "citations": citations,
                    "has_citation": True
                })
            else:
                # Track un-cited sentences as potential unsupported claims
                if any(kw in sentence.lower() for kw in [
                    "propose", "introduce", "demonstrate", "achieve", "show",
                    "find", "report", "according to", "state-of-the-art", "state of the art",
                    "improve", "outperform", "achieve", "observe", "suggest"
                ]):
                    claims.append({
                        "claim": sentence[:300],
                        "citations": [],
                        "has_citation": False
                    })

        return claims

    @staticmethod
    def _extract_citations(text: str) -> List[str]:
        """Extract citation markers from text."""
        citations = []

        patterns = [
            r'\[(\d+(?:,\s*\d+)*)\]',
            r'\[(\d+)[–-](\d+)\]',
            r'@(\w+(?:\d{4}[a-z]?)?)',
            r'\(([A-Za-z]+\s+et\s+al\.\s*,?\s*\d{4}[a-z]?)\)',
            r'\(([A-Za-z]+\s+and\s+[A-Za-z]+\s*,?\s*\d{4}[a-z]?)\)',
            r'\(([A-Za-z]+(?:,\s*[A-Za-z]+)*,\s*\d{4}[a-z]?)\)',
            r'\[([A-Z][A-Za-z0-9/+_-]+)\]',  # [PaperName] style citations
        ]

        for pattern in patterns:
            matches = re.findall(pattern, text, re.IGNORECASE)
            for match in matches:
                if isinstance(match, tuple):
                    match = match[0]
                if '-' in str(match) and re.match(r'\d+–\d+', str(match)):
                    start, end = str(match).split('–')
                    citations.extend([f"[{i}]" for i in range(int(start), int(end) + 1)])
                elif ',' in str(match) and re.match(r'\d+(?:,\s*\d+)*', str(match)):
                    parts = [p.strip() for p in str(match).split(',')]
                    citations.extend([f"[{p}]" for p in parts if p.isdigit()])
                else:
                    citation_str = str(match).strip()
                    if citation_str:
                        citations.append(citation_str)

        return list(set(citations))


class CitationVerifier:
    """Verifies whether citations actually support their associated claims."""

    def __init__(self, judge_model: str = "deepseek-v4-flash"):
        self.judge_model = judge_model
        self.available = False
        try:
            import subprocess
            result = subprocess.run(["accelerator", "--help"],
                                   capture_output=True, timeout=5)
            self.available = result.returncode == 0
        except Exception:
            self.available = False

    def verify_claim_citation_pair(self, claim: str, citation: str,
                                   survey_context: str) -> Dict:
        """Verify if a specific citation supports its claim."""
        prompt = f"""You are verifying whether a citation supports a claim in a scientific survey.

**Claim:** {claim[:500]}

**Citation:** {citation}

**Survey Context (nearby text):**
{survey_context[:1000]}

**Verification criteria (from DeepScholar-Bench paper):**
1. Does the cited source actually exist and is it correctly attributed?
2. Does the claim accurately represent the findings/conclusions of the cited work?
3. Is the claim fully supported by the cited source, or only partially?

**Output format (JSON only):**
{{
  "is_supported": "Yes|Partial|No",
  "confidence": 0.0-1.0,
  "issues": ["issue1", "issue2"] or null,
  "explanation": "brief explanation"
}}
"""
        try:
            import subprocess
            result = subprocess.run(
                ["accelerator", "run", "--model", self.judge_model,
                 "--prompt", prompt, "--json"],
                capture_output=True, text=True, encoding="utf-8", timeout=30
            )
            if result.returncode == 0:
                return json.loads(result.stdout)
        except Exception:
            pass

        return {
            "is_supported": "Unknown",
            "confidence": 0.5,
            "issues": None,
            "explanation": "Verification skipped"
        }


class VerifiabilityEvaluator:
    """Full verifiability evaluation following DeepScholar-Bench protocol."""

    def __init__(self, judge_model: str = "deepseek-v4-flash"):
        self.extractor = ClaimExtractor()
        self.verifier = CitationVerifier(judge_model)

    def evaluate(self, survey_text: str, sample_size: int = 30) -> Dict:
        """
        Run full verifiability evaluation.

        Returns:
            citation_precision: % of citations that support their claims (0-100)
            claim_coverage: % of claims that have supporting citations (0-100)
        """
        claims = self.extractor.extract_claims(survey_text)
        cited_claims = [c for c in claims if c["has_citation"]]
        uncited_claims = [c for c in claims if not c["has_citation"]]

        # Claim Coverage = claims with citations / total claims
        total_claims = len(claims) or 1
        claim_coverage = (len(cited_claims) / total_claims) * 100

        # Citation Precision: verify a sample of claim-citation pairs
        citation_verifications = []
        verified_pairs = 0
        supported_pairs = 0

        for claim in cited_claims[:sample_size]:
            for citation in claim["citations"][:3]:
                result = self.verifier.verify_claim_citation_pair(
                    claim["claim"], citation, survey_text
                )
                is_supported = result.get("is_supported", "Unknown")
                citation_verifications.append({
                    "claim": claim["claim"][:200],
                    "citation": citation,
                    "is_supported": is_supported,
                    "issues": result.get("issues"),
                })
                if is_supported != "Unknown":
                    verified_pairs += 1
                    if is_supported == "Yes":
                        supported_pairs += 1
                elif self.verifier.available:
                    verified_pairs += 1

        if verified_pairs > 0:
            citation_precision = (supported_pairs / verified_pairs) * 100
        else:
            citation_precision = float("nan")

        return {
            "citation_precision": None if verified_pairs == 0 else round(citation_precision, 2),
            "claim_coverage": round(claim_coverage, 2),
            "total_claims": total_claims,
            "cited_claims": len(cited_claims),
            "uncited_claims": len(uncited_claims),
            "verified_pairs": verified_pairs,
            "supported_pairs": supported_pairs,
            "sample_verifications": citation_verifications[:10],
        }

    def generate_report(self, survey_text: str) -> str:
        """Generate a full verifiability evaluation report."""
        result = self.evaluate(survey_text)
        cp = result['citation_precision']
        cp_str = f"{cp:.1f}%" if cp is not None else "N/A (LLM judge unavailable)"
        lines = [
            "# DeepScholar-Bench Verifiability Evaluation Report",
            "",
            f"**Citation Precision:** {cp_str}",
            f"**Claim Coverage:** {result['claim_coverage']:.1f}%",
            "",
            "## Overview",
            f"- Total Claims: {result['total_claims']}",
            f"- Claims with Citations: {result['cited_claims']}",
            f"- Claims without Citations: {result['uncited_claims']}",
            f"- Verified Citation-Claim Pairs: {result['verified_pairs']}",
            f"- Supported Pairs: {result['supported_pairs']}",
            "",
            "## Sample Verifications",
        ]
        for v in result.get("sample_verifications", []):
            lines.extend([
                f"- Claim: \"{v['claim'][:100]}...\"",
                f"  Citation: {v['citation']}",
                f"  Supported: {v['is_supported']}",
                f"  Issues: {v.get('issues', 'None')}",
                "",
            ])

        lines.extend([
            "## Score Summary",
            "| Metric | Score |",
            "|--------|:-----:|",
            f"| Citation Precision | {cp_str} |",
            f"| Claim Coverage | {result['claim_coverage']:.1f}% |",
        ])

        return "\n".join(lines)
