# Judge Suggestions — Extracted for Supervisor

## Round 1 — Total Score: 4.15 (Below Threshold)

---

### CRITICAL (must fix)

1. **C1 — Cross-phase comparison table** (Content Judge, Depth Judge, Supervisor)
   - Missing at end of §2 per outline spec
   - Columns needed: phase name, time period, representative systems, graph awareness level, iteration strategy, claimed metric + value, evaluation benchmark, cost profile, paper count
   - Highest-impact fix for next iteration

2. **C2 — Unsourced claim** (Accuracy Judge)
   - "over 2 million papers published annually" (§1 L5) has no citation
   - Source needed: STM Global Brief, UNESCO Science Report, or equivalent

3. **C3 — Paper count correction** (Accuracy Judge, Supervisor)
   - "over 140 papers" (§1 L11) conflicts with taxonomy's 137
   - Correct to "over 135 papers" or the precise count

---

### IMPORTANT (should fix)

4. **M1 — Metric redundancy** (Accuracy Judge)
   - SciSage +32% (5×), LitFM +28.1% (5×), PaSa +37.78% (4×) repeated excessively
   - Consolidate: keep §2 first mention + §3.4 synthesis table; cross-reference elsewhere

5. **M2 — Cross-approach synthesis** (Depth Judge)
   - §4 presents HITL vs iterative vs RL in silos
   - Add synthesis paragraph comparing scalability, quality ceiling, cost per survey

6. **M3 — Claim boundary wording** (Accuracy Judge, Supervisor)
   - "every system invents its own evaluation protocol" → "nearly every system"
   - PaperQA/PaperQA2 share LitQA/LitQA2

7. **M4 — Vague attribution** (Accuracy Judge)
   - "claims that would later attract scrutiny" (§2.2 L65) needs a citation or rephrasing

8. **M5 — Baseline values** (Supervisor)
   - §3.4 bottleneck transfer table lacks absolute scale context for quality scores

---

### NICE-TO-HAVE

9. **N1 — Section 6 prioritization** (Depth Judge)
   - Compare four proposals on impact, feasibility, cost

10. **N2 — Candidate pool cleanup** (Accuracy Judge)
    - `phase0/02_candidate_pool.md` has 18 irrelevant code-generation papers

11. **N3 — Conclusion expansion** (Content Judge)
    - §7 at ~7 lines needs more synthesis of narrative threads

12. **N4 — Cross-domain context** (Supervisor)
    - PRISMA/scientometrics/SummEval connection paragraph

---

### Score Breakdown for Supervisor

| Dimension | Score | Weight | Weighted | Contribution to gap |
|-----------|-------|--------|----------|---------------------|
| Coverage | 4 | 15% | 0.60 | — |
| Citation Relevance | 5 | 15% | 0.75 | — |
| Section Balance | 4 | 10% | 0.40 | — |
| Factual Consistency | 4 | 15% | 0.60 | — |
| Citation Balance | 4 | 10% | 0.40 | — |
| **Redundancy** | **3** | **10%** | **0.30** | **−0.10 from max** |
| **Analysis Depth** | **4.4** | **25%** | **1.10** | **−0.15 from max** |
| **Total** | — | **100%** | **4.15** | **−0.15 below 4.3** |

**Gap analysis**: The 0.15 gap to 4.3 is primarily driven by Redundancy (0.10 below max) and Analysis Depth (0.15 below max). If Redundancy moves from 3→4 and Analysis Depth from 4.4→4.5, total would reach 4.30. These are achievable with the M1 (metric consolidation) and M2 (cross-approach synthesis) fixes.
