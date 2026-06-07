# Outline History

## Round 1 (2026-06-07) — Initial Generation

### Summary
Initial outline generation from Round 1. Synthesized paper_taxonomy.md (137 papers, 6 categories), evolution_narrative.md (5 phases, critical assessment), and paper_profiles/ into a 7-section survey outline structured around the evolution arc.

### Sections
1. Introduction and Scope
2. The Evolution Arc — Five Phases (2.1–2.5)
3. Architectural Deep Dive — Graph vs. Pipeline (3.1–3.4)
4. Beyond Pipeline Design — Interaction, Iteration, Learning (4.1–4.3)
5. Critical Assessment (5.1–5.5)
6. Future Directions (6.1–6.4)
7. Conclusion

### Narrative Threads
1. The Semantic–Structural Tension
2. The Evaluation Comparability Crisis
3. The Bottleneck Transfer Problem
4. The Critical-Analytic Blind Spot

### Key Design Decisions
- Section 2 is chronological (5 phases) to establish the evolution arc
- Section 3 is thematic (3 paradigms + bottleneck analysis) for deep comparison
- Section 4 covers non-architectural approaches (interaction, iteration, RL)
- Section 5 moves from claim-evidence gaps → weaknesses → evaluation crisis → blind spots → root cause
- Section 6 proposes future directions grounded in gaps from Section 5

## Round 1 Refinement (2026-06-07) — Taxonomy Split and Outline Refinement

### Summary
Refined the Round 1 outline and section plan with targeted improvements:
1. **Taxonomy split**: Added `benchmark_evaluation` as a 6th primary category, moving 12 benchmark/dataset papers from single_agent_pipeline into their own category. This gives benchmarks appropriate visibility and clarifies the evaluation comparability crisis.
2. **Related work positioning**: Added positioning against 3 existing surveys (When LLMs Meet Citation, citation recommendation survey, Emergence of LLM as a Tool) to the Introduction.
3. **Computational cost dimension**: Added standardized cost/token reporting as a comparison dimension across all deep subsections (s3.1–s3.4, s4.1–s4.3, s6.1, s6.3). Added it as a 6th methodological weakness in s5.2.
4. **Sharper cross-phase comparison**: Added explicit dimensions (computational cost profile, number of papers per phase) to Section 2's cross-phase comparison table.
5. **Thread sharpening**: Extended Thread 3 to include the cost–efficiency trade-off (massive datastores and multi-agent coordination costs).

### Changes Made
- phase0/paper_taxonomy.md: Added Category 6 (Benchmark Evaluation), updated dimension matrix tables
- memory/outline.md: Refined narrative threads, added related work positioning, added cost dimensions, refined refinement guidelines
- memory/section_plan.md: Updated 15+ subsection writing instructions to include cost dimensions and sharper mechanism explanations

## Round 2 (2026-06-07) — Feedback-Driven Refinement (Judge Score 4.15)

### Summary
Refined outline and section plan to address 13 feedback items from Supervisor Notes and Judge Suggestions (total Round 1 score: 4.15, threshold: 4.3).

### Key Changes
1. **Cross-phase comparison table (§2 end, C1)**: Made explicit with all required columns. Added `s2.cross` subsection in section plan. This was the highest-priority fix.
2. **Cross-domain context (§1, M1/N4)**: Added paragraph connecting to PRISMA, SummEval, and scientometric citation analysis. Added 2004.05904, 2203.17239, 1501.05462 to reference papers.
3. **Paper count (C3/M3)**: Changed "140+" to "135+" throughout to match taxonomy's 137 papers.
4. **Unsourced claim (C2)**: Added requirement to cite STM Global Brief 2023 or UNESCO Science Report for "2M+ papers annually."
5. **Metric consolidation (M1 Judge)**: Introduced "first-mention-only" discipline — each core metric (SciSage +32%, LitFM +28.1%, PaSa +37.78%, Agentic AutoSurvey 8.18/10) is introduced in its canonical phase subsection and cross-referenced by later sections. Only §3.4 may synthesize metrics from multiple sources.
6. **Claim boundary (M2/M3)**: Changed "every system invents" → "nearly every system" throughout. Noted PaperQA/PaperQA2 share LitQA/LitQA2.
7. **Cross-approach synthesis (§4.4, M2 Judge)**: Added NEW subsection 4.4 comparing HITL, procedural iteration, and RL on scalability, quality ceiling, and cost per survey.
8. **Vague attribution (§2.2, M4 Judge)**: Replaced "claims that would later attract scrutiny" with specific cross-references to critique papers (2508.15658, 2601.15307).
9. **Baseline absolute values (§3.4, M5/M4)**: Added requirement to include baseline absolute values alongside improvement metrics in bottleneck transfer table.
10. **Section 6 prioritization (N1)**: Added `s6.prio` minimal subsection with impact vs. feasibility comparison table.
11. **Conclusion expansion (N3)**: Expanded §7 target from 300 to 400 words with concrete vision paragraph and deeper thread synthesis.
12. **Benchmark count updated**: Changed "10+ benchmarks" to "11+ benchmarks" (adding SurveyEval 2512.02763).
13. **Evaluation crisis mapping**: Updated to include SurveyEval (2512.02763) in the 11+ benchmark enumeration.

### Sections (unchanged structure)
1. Introduction and Scope (with cross-domain context)
2. The Evolution Arc — Five Phases (2.1–2.5 + cross-phase table)
3. Architectural Deep Dive — Graph vs. Pipeline (3.1–3.4)
4. Beyond Pipeline Design — Interaction, Iteration, Learning (4.1–4.4 NEW)
5. Critical Assessment (5.1–5.5)
6. Future Directions (6.prio NEW + 6.1–6.4)
7. Conclusion (expanded)
