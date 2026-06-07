# Agent Researcher Memory

## Round 1 (2026-06-07)

### Work Summary
Generated the initial survey outline and section plan for "Automated Literature Survey Agents with Citation Graph Expansion" (140+ papers).

### Key Synthesis Decisions
1. **Four narrative threads** identified from the evolution narrative's Critical Assessment: Semantic–Structural Tension, Evaluation Comparability Crisis, Bottleneck Transfer Problem, Critical-Analytic Blind Spot. These threads interleave across all sections.

2. **7-section structure**: Introduction → Chronological Arc (5 phases) → Architectural Deep-Dive (graph vs. pipeline vs. multi-agent) → Beyond Pipeline (interaction, iteration, RL) → Critical Assessment → Future Directions → Conclusion.

3. **Section 3.4 (Bottleneck Transfer)**: Added as a dedicated subsection to crystallize Thread 3—the untested assumption that retrieval gains linearly transfer to survey quality. This is the survey's most actionable gap finding.

4. **Section 5.5 (Root Cause)**: Argues that the evaluation crisis and critical-analytic blind spot are causally linked—the field cannot address what it cannot measure. This is the survey's core critical argument.

5. **Taxonomy unchanged**: The 6-category taxonomy (graph_enhanced_retrieval, single_agent_pipeline, multi_agent_pipeline, hybrid_interactive, iterative_refinement, benchmark_evaluation) with cross-cutting dimensions is adequate for Round 1.

### Papers Deep-Read (PDF)
None this round—paper profiles were sufficient for outline generation.

### Open Questions
- Whether the taxonomy should be revised to split "single_agent_pipeline" which is very broad (includes core systems, benchmarks, analysis papers, and boundary SLR tools)
- Whether a dedicated "benchmark_evaluation" category would improve clarity for the 10+ evaluation benchmarks

## Round 1 Refinement (2026-06-07)

### Work Summary
Refined the Round 1 outline with three targeted improvements based on review of the full candidate pool.

### Key Changes
1. **Taxonomy split**: Added `benchmark_evaluation` as 6th primary category, moving 12 benchmark/dataset papers from single_agent_pipeline. Updated cross-category matrices and boundary rationale.

2. **Cost dimension**: Added computational cost/token efficiency as a comparison dimension across all deep subsections and as a 6th methodological weakness. This addresses a practical gap: no system reports standardized cost metrics.

3. **Related work positioning**: Added positioning against 3 existing surveys to the Introduction, distinguishing this survey's unique focus on citation graph expansion as a retrieval strategy.

4. **Sharpened narrative threads**: Extended Thread 3 to include cost–efficiency trade-off. All thread connections now explicitly mentioned in writing instructions.

5. **Updated section_plan.md**: 15+ subsections received refined writing instructions incorporating cost dimensions, sharper mechanism explanations, and improved thread connections.

## Round 2 (2026-06-07) — Feedback-Driven Refinement

### Work Summary
Processed 13 feedback items from Supervisor Notes and Round 1 Judge Report (score 4.15). All critical and important items addressed.

### Key Changes
1. **Cross-phase comparison table (§2)**: Made explicit with dedicated `s2.cross` subsection. All required columns specified.
2. **Cross-domain context (§1)**: Added PRISMA/SummEval/scientometrics paragraph.
3. **Metric consolidation**: Introduced "first-mention-only" rule across all subsections to eliminate metric redundancy (judge's M1).
4. **New §4.4 (Cross-Approach Synthesis)**: Compares HITL vs procedural iteration vs RL on scalability, quality ceiling, cost.
5. **§6 prioritization table**: Added `s6.prio` with impact/feasibility comparison.
6. **Paper count fixed**: 140+ → 135+; citation added for "2M+ papers"; "every system" → "nearly every system"; vague attribution replaced with specific cross-refs.
7. **Baseline absolute values**: Required in bottleneck transfer table.
8. **Taxonomy unchanged**: 6-category structure remains adequate for Round 2.

### Papers Deep-Read (PDF)
None this round — existing profiles and feedback were sufficient.
