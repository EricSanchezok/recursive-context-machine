# LLM Judge Report

## Round: 3

## Dimension Scores

| Dimension | Score | Weight | Weighted |
|-----------|-------|--------|----------|
| Coverage | 4 | 20% | 0.80 |
| Citation Relevance | 3 | 20% | 0.60 |
| Factual Consistency | 3 | 20% | 0.60 |
| Redundancy | 4 | 15% | 0.60 |
| Citation Balance | 5 | 15% | 0.75 |
| Section Balance | 4 | 10% | 0.40 |
| **Total** | — | **100%** | **3.75** |

## Verdict

**Status**: CONTINUE
**Reason**: BELOW_THRESHOLD — Total weighted score (3.75) is below the 4.3 threshold.

### Decision Rules Applied

| Rule | Status | Detail |
|------|--------|--------|
| Single Dimension Check | ✅ **PASS** | Lowest dimension score is 3 (Citation Relevance, Factual Consistency); both ≥ 3.0 |
| Total Score Check | 🔴 **BELOW_THRESHOLD** | Total = 3.75 < 4.3 |
| Convergence Check | ✅ **NOT CONVERGED** | Score decreased from 4.00 (R2) to 3.75 (R3) — regression, not plateau |
| Max Rounds Check | ✅ **NOT MAX** | Round 3 < 5 |
| **Priority verdict** | **→ CONTINUE** | BELOW_THRESHOLD is highest-priority applicable rule |

### Score Progression

| Dimension | Round 1 | Round 2 | Round 3 | Δ R2→R3 |
|-----------|---------|---------|---------|----------|
| Coverage | 4 | 5 | 4 | **−1.0** |
| Citation Relevance | 3 | 4 | 3 | **−1.0** |
| Factual Consistency | 2 | 3 | 3 | ±0.0 |
| Redundancy | 3 | 3 | 4 | **+1.0** |
| Citation Balance | 4 | 5 | 5 | ±0.0 |
| Section Balance | 3 | 4 | 4 | ±0.0 |
| **Total** | **3.15** | **4.00** | **3.75** | **−0.25** |

**Note**: The score regression from 4.00 to 3.75 is not due to draft regression but because Round 3 evaluation was more rigorous — it caught issues that Round 2 missed (AutoSurvey2 characterization still unfixed, missing Table 1 columns, §3.4 cross-reference gap).

## The Single Most Important Issue

### 🔴 Critical — AutoSurvey2 characterization contradicts pool entry (§2.3, line 53)

The draft states that AutoSurvey2 [2510.26012] "lacks the iterative refinement of other hybrid systems." The candidate pool entry (#1) explicitly describes it as having "iterative refinement." This is a direct factual contradiction flagged by **three separate evaluations**:

| Evaluator | Finding |
|-----------|---------|
| Citation Relevance (Score 3) | C1: "direct factual contradiction — the citation does not support the claim made about it" |
| Factual Consistency (Score 3) | C1: "one of the few claims in the draft that can be definitively checked against a pool entry, and it is wrong" |
| Coverage (Score 4) | Noted AutoSurvey2 placement as correctly in §2.3 only, but the characterization issue was not flagged here |

**Suggested fix**: Change "though it lacks the iterative refinement of other hybrid systems" to "though with less pronounced iterative refinement compared to the dedicated refinement loops in IterSurvey's paper-card pipeline."

## When This Issue Is Fixed

If the AutoSurvey2 characterization is corrected, the expected scoring improvement is:

| Dimension | Likely New Score | Rationale |
|-----------|-----------------|-----------|
| Citation Relevance | 3 → **4** | Removes the critical misattribution; 4 is "nearly all citations directly support their claims" |
| Factual Consistency | 3 → **4** | Removes the only direct source contradiction; all ~45 pool-verifiable claims match |
| Projected new total | 3.75 → **4.15** | Still below 4.3 threshold — additional fixes needed |

## Additional Fixes Needed to Reach 4.3

| Fix | Impact | Affected Dimension |
|-----|--------|-------------------|
| 🔴 Fix AutoSurvey2 characterization | +0.20 (Relevance) + 0.20 (Factual) = **+0.40** | Relevance → 4, Factual → 4 |
| 🟡 Add hedging to 8 unverifiable quantitative claims | +0.20 (Factual) | Factual → 4 → 5 unlikely without pool verification |
| 🟡 Fix Table 1 formatting (extra pipe) | Cosmetic | (Formatting, not scoring) |
| 🟡 Add §3.4 → §9.4 cross-reference for PaSa | Minor | Coverage → 4 → 5 likely with all gaps closed |
| 🟡 Group §7.5 hallucination benchmarks | Minor | Coverage → 4 → 5 |
| 🔵 Trim GLUE (§10), OpenScholar (§8.4), coverage breadth (§9) | Minor | Redundancy → 4 (already strong) |

**Minimum path to 4.3**: Fix 🔴 AutoSurvey2 characterization → total reaches **4.15**. That still leaves a 0.15 gap. To close it, fix the unverifiable quantitative claims (hedging language) and the two Coverage gaps (Table 1 columns, §3.4 cross-reference, §7.5 transitions).

## Consolidated Suggestions

### Critical (must fix)

1. **Fix AutoSurvey2 characterization (§2.3, line 53)**: Change the claim to acknowledge iterative refinement. This is the single error preventing a total score of 4.0+; three judges flagged it.

### Important (should fix)

2. **Add hedging to unverifiable quantitative claims** (§1, §6.1, §6.2, §7.2, §9.1, §9.5): At minimum: "reportedly identified ~147K hallucinated citations"; "SciReviewGen-based evaluations report 15–25% hallucination rates"; "estimated $10–50 in API fees."
3. **Add forward cross-reference §3.4 → §9.4** for PaSa's traversal trade-offs.
4. **Add transitional grouping phrases in §7.5** for the 7 hallucination benchmarks.
5. **Add missing Table 1 columns** ("number of agents," "retrieval integration") per outline specification — currently only implicit.

### Nice-to-have

6. **Consolidate duplicates**: OpenScholar phrase (§5.4/§8.4), GLUE analogy (§9.2/§10), coverage breadth definition (§2/§9), SuperWriter (§4.2/§4.3), HierCat statistics (§4.1/§7.3).
7. **Verify SciReviewGen 15–25% hallucination rate** against the actual paper or hedge the attribution.
8. **Consider adding central out-of-pool papers** (STORM, Self-RAG, PaperQA2) to the main candidate pool for traceability.

## Next Action

**CONTINUE** — The draft is structurally complete with good coverage and balance. The main blocker to approval is the AutoSurvey2 factual error. Once fixed and the unverifiable claims hedged, the draft should meet the 4.3 threshold for Polisher processing.
