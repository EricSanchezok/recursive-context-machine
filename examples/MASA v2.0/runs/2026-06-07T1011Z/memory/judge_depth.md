# Analysis Depth Judge — Round 3 (Revised)

## Per-Section Scores

| Section | Score (1–5) | Key Strength | Key Weakness |
|---------|-------------|--------------|--------------|
| Section 2: Foundations — RAG Paradigm and Dataset Infrastructure (2020–2023) | 4 | Mechanism-level breakdown of RAG (mathematical formulation, passage-level limitation); clear analytical transition §2.3 identifying the phase boundary; dataset limitations tied to the evaluation comparability crisis | Quantitative results from Phase 1 papers (e.g., specific ROUGE/BLEU scores on QA benchmarks, SciFact verification F1) are absent — only dataset scale numbers provided |
| Section 3: The Single-Agent Pipeline Emerges — Task Decomposition (2024) | 4 | Clear architectural template with two comparison tables (Tables 2, 3); genuine advances vs. unfulfilled claims separated; fine-tuned vs. zero-shot paradigm trade-off analyzed; quantitative anchor (~40% human win rate) | No ablation-style analysis of which architectural choices drive quality differences; the "single-LLM bottleneck" identified but not analyzed functionally (what specific multi-task conflicts arise?) |
| Section 4: Architectural Proliferation — Multi-Agent, Graph, and Human-in-the-Loop (2025) | 5 | Four comparison tables across subsections; multi-hop citation barrier decomposed into 4 distinct categories (engineering, relevance degradation, infrastructural, evaluation) — genuine novel synthesis; ReClaim computational cost quantified ($50–100/survey) anchoring Depth–Breadth Trade-Off; proposed controlled experiment to test prompt-deep specialization | The excellent cross-paradigm quality-control comparison (deliberation-first vs. rubric-guided vs. reflect-when-you-write) in §5.3 could have been placed or referenced in §4 to strengthen cross-subsection synthesis |
| Section 5: Current Frontier — Deliberation, Domain Expertise, and Local Deployment (2026) | 4 | Detailed mechanism descriptions for DOVA, OrchMAS, ResearchPilot; cross-phase comparison table (Table 8); three-paradigm quality-control analysis in §5.3 is excellent analytical synthesis; DOVA critique cross-references ARISE rubric-quality problem | ResearchPilot quality estimate ("60–75% of benchmark performance") lacks a supporting citation; Section is noticeably shorter per-system than §4's treatment; deliberation quality critique stated but less deeply developed than §4.1's prompt-deep analysis |
| Section 6: Critical Assessment — Claims, Gaps, and Blind Spots | 5 | Claim vs. Evidence table (Table 9) with explicit source tracing is exemplary; field-wide aggregate statistics (mean/median/max evaluation scope across 12 systems); systematic identification of 7 blind spots with convergence argument; all four narrative threads converge; reproducibility crisis and cost blindness added as new weaknesses | Missing a concrete named-system example of an unreproducible claim to make the reproducibility argument more actionable; otherwise fully meets Level-5 criteria |
| Section 7: Future Directions — Toward Next-Generation Survey Generation | 4 | Each direction grounded in a specific §6 gap with solution sketch; cross-cutting trade-off analysis (multi-hop vs. Depth–Breadth, domain adaptation vs. evaluation comparability, local deployment vs. evaluation protocol) is a Level-5 indicator | Directions 5 (temporal grounding) and 6 (uncertainty communication) lack even preliminary work references, making them feel speculative; no implementation-cost estimation for any direction |

## Overall Depth Score: **4.3** / 5.0

Weighted average: (4 + 4 + 5 + 4 + 5 + 4) / 6 = 4.33 → **4.3**

## Depth Issues

### Critical (score 1–2)
- None. No section scores below 4. This is a consistently strong draft with analytical treatment throughout.

### Moderate (score 3–4)
- **Section 5 (§5.3 — ResearchPilot)**: The claim that quantized 7B–13B models achieve "60–75% of benchmark performance" needs a citation (e.g., from Llama/Mistral model cards or a systematic quantization survey). Without it, this reads as a plausible but unsupported estimate in an otherwise well-evidenced section.
- **Section 7 (§7, Directions 5–6)**: Temporal grounding and uncertainty communication lack even preliminary work references. CRUISE-Screening [2309.01684] (already in the draft) could ground Direction 5. A representative QA calibration paper could ground Direction 6.

### Strengths at Level 5
- **Section 4**: The four-category decomposition of the multi-hop citation barrier (§4.2) — engineering scalability, relevance degradation per hop, infrastructural gap (no claim-level graph), evaluation gap — is exactly the kind of novel analytical synthesis that distinguishes Level-5 writing from mere summarization. A reader comes away understanding *why* the field is stuck, not just *that* it is stuck.
- **Section 6**: The Claim vs. Evidence table with source-column traceability is a model for critical survey writing. The field-wide scope aggregation (mean ~20, median ~15, max ~100 topics across 12 systems) is a novel data point no other survey has produced.

### Previous-Round Improvements Observed
- **§4.2** — The previous round flagged the absence of multi-hop barrier analysis. This has been fully addressed with the four-category decomposition.
- **§5.3** — The previous round flagged the absence of quality-control paradigm comparison. This has been added with the deliberation-first vs. rubric-guided vs. reflect-when-you-write analysis.
- **§6.2** — The previous round flagged missing reproducibility crisis and computational cost blindness. These have been added with the Reproducible Pipeline [2508.04612] contrast case.

## Recommendations for Depth Improvement

1. **Section 2 — Add Phase 1 quantitative results**: Cite specific RAG SOTA scores (e.g., "64.0 EM on Open-domain QA, +5.4 over previous SOTA" from [2005.11401]) and SciFact verification F1 or FActScore human-correlation numbers. This anchors the foundation narrative with empirical grounding and strengthens the §2.3 transition argument.

2. **Section 3 — Deepen single-LLM bottleneck analysis**: Add 2–3 sentences identifying specific multi-task conflicts: does optimizing for drafting fluency degrade citation accuracy? Does the outline-generation task compete with the drafting task for the model's limited attention budget? Evidence from multi-task learning literature or from the systems' own failure analyses could contextualize this.

3. **Section 5 — Anchor quality claims with citations**: Replace "60–75%" with a citation to a specific quantization benchmark. Add a quantitative column to Table 8 with whatever comparable metrics exist across these systems (even if the table shows mostly gaps — that would itself be a finding).

4. **Section 7 — Ground speculative directions**: For Direction 5 (temporal grounding), cite CRUISE-Screening [2309.01684] as a living-review precedent. For Direction 6 (uncertainty communication), cite a representative QA calibration paper demonstrating the approach in a related task. This moves these directions from "entirely new" to "adaptation of existing work."

5. **Cross-cutting synthesis opportunity**: The three-paradigm quality-control comparison currently in §5.3 is excellent analytical synthesis. Consider adding a condensed version to §4 (perhaps as §4.1's conclusion) so the Phase 3 analysis already surfaces this framework, and §5 can extend it rather than introduce it fresh. Alternatively, make it the opening frame of §6 to unify the two sections under a single analytical lens.
