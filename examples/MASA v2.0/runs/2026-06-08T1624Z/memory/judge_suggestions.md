# Judge Suggestions — Round 2

### Critical (must fix)
1. **Restore Section 6.4** as a separate subsection ("Ablation Studies and Citation Hallucination Auditing"). Use [2605.07723] for hallucination prevalence evidence and [2605.14790] as an example of graded graph integration levels that could be ablated.
2. **Incorporate all 3 supervisor-retrieved papers**: [2605.07723] in §5.4 (blind spot #1) and §6.4; [2604.22750] in §5.4 (blind spot #4) with the 1000× token multiplier; [2605.14790] in §4.3 as evidence of feasible learned graph representations for generation.

### Important (should fix)
3. **Deepen Section 4.3** with specific technical barrier analysis: (a) GNN training data requirements, (b) task-specific retraining cost, (c) representation alignment between recommendation GNNs and generative organization tasks.
4. **Add PRISMA/systematic review methodology paragraph** to Section 5.2 or 5.3 connecting ASG evaluation to established systematic review methodology.
5. **Source or adjust the "35 core method papers" count** — either explain filtering criteria or replace with "the papers surveyed in this work."
6. **Reduce redundancy**: Trim "single controlled comparison" from 6 occurrences to 3; eliminate verbatim Phase 4 irony duplicate in §5.3; consolidate §3 intro and §3.4.

### Nice-to-have
7. **Add cross-phase comparison table to Section 2** — the data is already in the text; a table would make the metric-dispersion pattern visually concrete.
8. **Attribute "71% improvement" as a derived number** — change to "a ~71% improvement over AutoSurvey's 4.77/10 baseline (computed from the two reported scores)."
9. **Add prioritization sentence to Section 7** identifying citation hallucination auditing as the most immediately actionable blind spot.
10. **Add cost ballpark to Section 5.4** using [2604.22750]'s 1000× token multiplier estimate.
