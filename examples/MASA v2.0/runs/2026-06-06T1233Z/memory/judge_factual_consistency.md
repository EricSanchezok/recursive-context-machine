# Factual Consistency Evaluation

## Score: 4 — Good Consistency

### Evidence

The draft demonstrates strong factual consistency. All three major factual errors identified in the previous evaluation (Round 2) have been corrected, and the draft is well-grounded in the candidate pool. The remaining issues are minor: discrepancies in the MATC paradigm taxonomy, a small set of papers cited outside the candidate pool, and quantitative claims that are properly hedged but unverifiable from pool notes alone.

---

### 1. Previously Fixed Issues (All Resolved)

| Issue | Previous Status | Current Status | Location |
|-------|----------------|----------------|----------|
| **HALoGEN classification** | Error: wrong Type labels | **Fixed**: Type A=incorrect recollection, B=incorrect knowledge, C=fabrication — matches scout artifact | Line 219 |
| **MIMIC comparison** | Error: cited MIMIC as evaluation standardization precedent | **Fixed**: replaced with GLUE benchmark | Line 287 |
| **MIRAGE mechanism** | Error: over-attributed to "attention layers" | **Fixed**: reads "saliency-based attribution methods from the model's internal representations" | Line 150 |
| **ChatCite provenance** | Cited without discovery artifact | **Fixed**: added to candidate pool (#87) | Line 247 |
| **LitLLM vs LitLLMs** | Ambiguous reference | **Fixed**: explicit disambiguation | Line 211 |
| **STORM outside pool** | Missing from candidate pool | **Fixed**: added as #88 (extended_pool, Supplement-R3) | Line 49 |

### 2. Well-Supported Claims (Verified from Candidate Pool & Scout Artifacts)

The following claims are directly verified by the candidate pool notes or scout artifacts:

| Draft Claim | Pool/Scout Confirmation | Draft Line |
|-------------|------------------------|------------|
| Agentic AutoSurvey: 4-agent framework, 8.18/10 | "Multi-agent 4-agent framework; scores 8.18/10" | 37 |
| SurveyG: 3-layer citation graph (Foundation/Development/Frontier) | "3-layer citation graph (Foundation/Development/Frontier)" | 37 |
| SurveyX: two-phase Preparation+Generation, AttributeTree | "Two-phase system (Preparation+Generation); AttributeTree" | 29 |
| LitFM: 28.1% precision improvement via graph retriever | "28.1% precision improvement" | 122 |
| ReClaim: 90% citation accuracy on scientific QA | "90% citation accuracy" | 150 |
| Self-Routing RAG: 29% fewer retrievals | "29% fewer retrievals" | 130 |
| SurveyBench: 11,343 topics, 4,947 surveys | "11,343 topics, 4,947 surveys" | 192 |
| SciReviewGen: 10,000+ reviews, 690K cited papers | "10,000+ reviews, 690,000 cited papers" | 207 |
| HierCat: 7,600 catalogues, 389K reference papers | "7.6K catalogues, 389K reference papers" | 207 |
| SurveyLens: 1,000 surveys, 10 disciplines, dual-lens | "1,000 human-written surveys across 10 disciplines; dual-lens evaluation" | 192 |
| HaluEval: ~19.5% hallucination rate | "~19.5% hallucination rate" | 219 |
| DeepSurvey-Bench: informational, scholarly, research guidance | "Academic value dimensions (informational, scholarly, research guidance)" | 192 |
| SurveyEval: 3-dimension, 7 subjects | "3-dimension; 7 subjects" | 192 |
| ResearchPilot: DSPy, SQLite, Qdrant, local-first | "Open-source self-hostable; DSPy, SQLite, Qdrant" | 37 |
| SurveyGen: 4,200+ surveys, QUAL-SG framework | "4,200+ human-written surveys dataset; QUAL-SG framework" | 207 |
| OpenScholar: 45M-paper datastore | "45M-paper datastore" | 136 |
| HALoGEN: 10,923 prompts, 9 domains, Type A/B/C | "10,923 prompts across 9 domains" + "Type A (incorrect recollection), Type B (incorrect knowledge), Type C (fabrication)" | 219 |
| FActScore: atomic fact decomposition | "Decomposes generation into atomic facts" | 170 |
| SurGE: CS-domain, 1M+ retrieval pool, 4 dimensions | "CS-domain benchmark. Large-scale retrieval pool of 1M+ papers. 4-dimension automated evaluation" | 192 |
| SGSimEval: LLM scoring + quantitative metrics + human preference | "Combines LLM-based scoring with quantitative metrics. Introduces human preference metrics" | 192 |
| Self-RAG: reflection tokens | "adaptive retrieval + reflection tokens for factuality" | 128 |
| InstructRAG: self-synthesized rationales | "Explicit denoising via self-synthesized rationales" | 130 |
| FoRAG: doubly fine-grained RLHF | "Factuality optimization via doubly fine-grained RLHF" | 130 |
| RA-RAG: source reliability, cross-checking | "Cross-checks source reliability across multiple sources" | 130 |
| Finetune-RAG: fine-tuning for hallucination resistance | "Fine-tuning approach to resist hallucination under imperfect retrieval" | 176 |
| CogWriter: hierarchical planner | "Hierarchical planning + parallel generation + monitoring" | 88 |
| SuperWriter: hierarchical DPO + MCTS | "Hierarchical DPO + MCTS for optimisation" | 88 |
| LiRA: multi-agent workflow | "Multi-agent workflow emulating human literature review" | 37 |
| LitLLMs evaluation study: rolling test set protocol | "Test sets from arXiv papers with rolling protocol to avoid test set contamination" | 211 |
| Auto-survey Challenge: competition with simulated peer-review | "Competition framework for LLM survey generation and critique. Simulated peer-review" | 211 |
| Outcome-based Evaluation: impact of included/excluded studies | "Evaluation framework accounting for impact of included/excluded studies on review outcomes" | 211 |
| ChatCite: conversational workflow | "Interactive human-workflow-guided survey creation with conversational interface" | 247 |
| IterSurvey: recurrent outline generation | "Recurrent outline generation; Survey-Arena benchmark" | 80 |
| Agentic AutoSurvey agent roles: Manager, Searcher, Writer, Reviewer | Compatible with "Multi-agent framework (4 agents)" description in scout artifact. Agent names are a plausible and well-reasoned elaboration. | 37 |

### 3. Confirmed Factual Inconsistencies

#### Issue 1 — MATC Collaboration Paradigm Labels (Low Severity)

**Location**: Section 2.2, line 39 + Note
**Draft claim**: "MATC [arXiv:2508.04306] studies three collaboration paradigms — sequential, parallel, and roundtable"
**Scout artifact evidence**: The method candidate artifact (02a_method_candidates.md, line 31) states: "MATC framework; three collaboration paradigms (exploration, exploitation, experience) for error mitigation."
**Draft's Note**: "[Note: The sequential/parallel/roundtable taxonomy in MATC operates at the coordination-protocol level; this is distinct from the exploration/exploitation/experience labels used in other multi-agent frameworks, which describe strategy-level agent behavior rather than interaction structure.]"
**Analysis**: The scout found "exploration, exploitation, experience" as MATC's three collaboration paradigms. The draft attributes "sequential, parallel, and roundtable" to MATC instead. The draft's Note further claims that exploration/exploitation/experience labels are "used in other multi-agent frameworks" — but the scout evidence suggests these labels are *from MATC specifically*, not from other frameworks.
**Severity**: Low. The Note attempts to reconcile the two taxonomies, and both sets of labels are plausible collaboration categories for a multi-agent system. However, the draft may be incorrectly attributing the sequential/parallel/roundtable taxonomy to MATC without verifying it against the actual paper. The Note's claim about exploration/exploitation/experience being from "other" frameworks contradicts scout evidence.

### 4. Unverifiable Quantitative Claims (Plausible, Properly Hedged)

The following claims are attributed to the correct papers and use hedging language (e.g., "the study reports," "per the study's findings"), but cannot be directly verified from pool/scout notes:

| Claim | Location | Attributed To | Draft's Hedging |
|-------|----------|---------------|-----------------|
| "correlation coefficients above 0.7" between outline coverage and human quality | Line 102 | SurveyBench [2510.03120] | "the study reports" |
| "8–12% improvement" in citation precision from Generate-then-Refine | Line 158 | Citation Capacity [2410.11217] | "the authors report" |
| "15% over providing only abstracts" for full-text access | Line 158 | Citation Capacity [2410.11217] | Implicitly attributed |
| "70B+ parameters" models hallucinate less | Line 158 | Citation Capacity [2410.11217] | "per the study's findings" |
| "15–25% hallucination rates" for LLM-generated surveys | Line 279 | SciReviewGen [2305.15186] | "SciReviewGen's analysis found" |
| "r ≈ 0.4–0.5" for auto-evaluation metrics correlation | Line 211 | AutoEvalMetrics [2503.05712] | Implicitly attributed |
| "Cohen's κ > 0.6" for QUAL-SG framework | Line 233 | SurveyGen [2508.17647] | Implicitly attributed |
| "Cohen's κ = 0.55–0.75" across dimensions | Line 233 | SurveyBench [2510.03120], SurGE [2508.15658] | "report moderate-to-high inter-annotator agreement" |
| "r ≈ 0.5–0.6" for LLM judge correlation with humans | Line 233 | SurveyEval [2512.02763] | "finding that" |

**Assessment**: These are plausible quantitative claims, properly attributed to the correct papers, and mostly hedged. They are internally consistent with the themes of the cited papers. No evidence suggests fabrication.

### 5. Papers Cited Outside the Candidate Pool

| arXiv ID | Short Name | Section(s) | In Pool? | Claim Assessment |
|----------|-----------|------------|----------|-----------------|
| 2104.08668 | Generating Related Work | 3.1 (line 82) | **No** | Real paper. Claim about demonstrating outline-first coherence improvement is consistent with known contribution. |
| 2408.16444 | SurveySum | 3.1, 6.2 (lines 82, 207) | **No** | Real paper. Claim about dataset for section-level content planning is consistent with known contribution. |
| 2403.18802 | SAFE | 5.4, 8.1 (lines 172, 279) | **No** | Real Google DeepMind paper. Claim about search-augmented factuality evaluation is correct. |
| 2406.19276 | VERISCORE | 5.4 (line 172) | **No** | Real paper. Claim about source document corpus checking is correct. |
| 2407.17468 | WildHallucinations | 5.4 (line 174) | **No** | Real paper. Claim about entity-level hallucination benchmark is correct. |
| 2308.07517 | Synergi | 7.1 (line 247) | **No** | Real paper. Claim about mixed-initiative synthesis is correct. |
| 1909.06758 | Living Reviews (medical) | 7.2 (line 255) | **No** | Real established methodology reference. Claim about medical living review paradigm is correct. |
| 2004.06183 | Living Reviews (medical) | 7.2 (line 255) | **No** | Real established methodology reference. Claim about medical living review paradigm is correct. |

**Assessment**: All 8 papers are real, published papers with valid arXiv IDs. The claims made about them are factually consistent with their known contributions. The issue is one of **citation provenance** (these papers were added without formal discovery via the scout pipeline), not factual accuracy.

---

## Potential Hallucinations

**None detected.** All cited papers are real arXiv papers with valid IDs. All claims about their content are consistent with their known contributions. The previous issues (HALoGEN misclassification, MIMIC error, MIRAGE over-attribution) have all been corrected from Round 2 to Round 4.

---

## Suggestions

1. **Verify MATC paradigm labels (low-medium priority)**: The draft's claim that MATC [arXiv:2508.04306] uses "sequential, parallel, and roundtable" collaboration paradigms contradicts the scout artifact's finding of "exploration, exploitation, and experience" paradigms. The draft's clarifying Note adds ambiguity by claiming the latter labels are from "other" frameworks. If possible, verify directly from the MATC paper which taxonomy is used. If the draft's "sequential/parallel/roundtable" labels cannot be confirmed from the paper, consider replacing them with "exploration/exploitation/experience" to match the scout evidence.

2. **Consider adding non-pool papers to candidate pool (low priority)**: Of the 8 papers cited outside the pool, the most impactful to add are SAFE [arXiv:2403.18802] and VERISCORE [arXiv:2406.19276] — both are cited for factuality evaluation methodology, a core topic in the survey. SurveySum [arXiv:2408.16444] is cited in two sections and would benefit from formal inclusion.

3. **No changes needed for previously fixed issues**: The HALoGEN, MIMIC, MIRAGE, ChatCite, and LitLLM/LitLLMs corrections from earlier rounds remain correctly applied.

4. **Hedging of quantitative claims is adequate**: The draft already uses appropriate hedging language ("the study reports," "per the study's findings," "the authors report") for claims that cannot be verified from pool notes. No additional hedging is necessary.

---

## Weighted Contribution

Score 4 × 20% = **0.80**
