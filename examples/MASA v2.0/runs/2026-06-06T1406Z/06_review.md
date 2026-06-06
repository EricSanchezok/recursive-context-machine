# Supervisor Review Report — Round 3

**Reviewer**: MASA Supervisor
**Run**: `2026-06-06T1406Z`
**Draft**: `05_draft.md` (62.4 KB, 359 lines, ~9,600 words)
**Round**: 3 (reviewing Round 2 draft state)
**Previous Review**: `06_review.md` (Round 2 — dated 2026-06-06T06:49)

---

## Verdict: **STRONG** ⚠️ — fix 1 critical issue, then proceed to Judge

The draft is substantively complete with strong coverage of all 10 sections and 36 subsections. All 24 fix items from Round 1 (8 supervisor + 16 judge) were correctly applied. However, one unresolved **🔴 factual contradiction** remains (AutoSurvey2 claim §2.3), and two **🟡** structural issues from Round 2 were not yet addressed (Table 1 formatting, §7.5 transitions). The Judge Round 2 scored **4.00** (below 4.3 threshold), with the AutoSurvey2 contradiction as a primary downgrade factor.

---

## 🔴 CRITICAL — Must Fix

### C1: AutoSurvey2 characterization contradicts pool entry (§2.3, line 53)

**Problem**: The draft states: "AutoSurvey2 [arXiv:2510.26012] also falls in this category [hybrid] due to its parallel section generation with real-time retrieval, **though it lacks the iterative refinement** of other hybrid systems."

**Pool entry (#1)**: "Multi-stage pipeline with parallel section generation, **iterative refinement**, real-time retrieval of recent publications, multi-LLM evaluation."

The claim directly contradicts the pool entry, which explicitly mentions "iterative refinement." The Judge Round 2 flagged this (score reduction from 4→3 on factual consistency). The Supervisor Round 2 review missed this issue.

**Fix**: Replace the clause with: "...though with less pronounced iterative refinement compared to the dedicated refinement loops in IterSurvey's paper-card pipeline."

---

## 🟡 MODERATE — Should Fix

### M1: Table 1 formatting bug (§2.4, lines 63–71)

**Problem**: Each data row has a leading `| |` (empty first cell), creating 8 columns for a 7-column header. The table will render with a blank first column and shifted content.

**Fix**: Remove the leading `| |` from each data row.

```
Current:   | | Single-agent | AutoSurvey [2406.10252] | None | N/A | N/A | Semantic search | 4.77/10
Fix:       | Single-agent | AutoSurvey [2406.10252] | None | N/A | N/A | Semantic search | 4.77/10
```

### M2: §7.5 — Missing transitional phrases between method groups

**Problem**: The outline requires "Use transitional phrases between method groups." The current §7.5 (line 263) is:

> "HaluEval... offers a large-scale hallucination benchmark... HALoGEN provides 10K prompts... SelfCheckGPT uses sampling-based consistency checking... TRUE provides a meta-evaluation... Provenance uses NLI-based attribution... DAHL focuses on domain-specific... ReFACT targets scientific confabulation..."

This is a flat list of 7 benchmarks with no grouping transitions. Compare with §6.4 which correctly uses grouping (Decomposition-based / Search-augmented / Claim-verification / Entity-grounded).

**Fix**: Group into categories with transitional phrases. Example grouping:
- **Large-scale benchmarks**: HaluEval (general), HALoGEN (fine-grained type labels)
- **Sampling-based methods**: SelfCheckGPT (no reference needed)
- **Meta-evaluation frameworks**: TRUE (11-dataset meta-evaluation)
- **NLI-based attribution**: Provenance (attribution verification)
- **Domain-specific**: DAHL (biomedical), ReFACT (scientific confabulation)

### M3: AutoSurvey2 fix may need cross-reference alignment

**Problem**: If C1 is fixed to acknowledge AutoSurvey2's iterative refinement, verify consistency with any other mentions of AutoSurvey2 elsewhere in the draft (search for "26012" occurrences to confirm all references align).

---

## 🔵 NICE-TO-HAVE — Polish

### N1: Agentic AutoSurvey BFS claim (§2.2, line 37)

**Problem**: "The Paper Search agent performs citation-aware retrieval by starting from seed papers and expanding through reference lists, though the specific traversal strategy is BFS-based rather than learned."

The pool entry (#9) does not mention BFS. While BFS is a plausible default for reference-list expansion, this detail cannot be verified.

**Fix**: Change "BFS-based" to "reference-list expansion" or "breadth-first style expansion."

### N2: §4.4 — SurveyBench correlation lacks specific number

**Problem**: Says "outline quality is a significant predictor" without providing the actual correlation value.

**Fix**: If available in SurveyBench paper, add: "Pearson's r = 0.72 between outline coherence and overall quality scores."

### N3: Unverifiable quantitative claims — consider hedging

Several specific numerical claims in the draft cannot be verified from the candidate pool (noted by Judge):
- §7.2: CiteME "4.2–18.5% vs 69.7%" [2407.12861] — paper not in pool
- §9.1: SciReviewGen "15–25% hallucination rates" [2305.15186] — paper not in pool
- §9.5: "$10–50 in API fees" [2509.18661] — pool lacks cost data
- §6.2: "15–20% full-text advantage, 70B+ scale" [2410.11217] — paper not in pool
- §1: "147,000 hallucinated citations" [2605.07723] — paper not in pool

**Fix**: Either (a) verify each against the actual paper, (b) add hedges ("reportedly," "the authors claim"), or (c) remove specific figures. Given the draft's stage, option (b) is recommended for non-critical claims.

### N4: §8.4 duplicate "blur" phrasing

Line 299 contains both "blur the line" and "blurring" in the same and following sentence about convergence. Remove the second occurrence.

---

## Quality Assessment

| Dimension | Score (1–5) | Comment |
|-----------|-------------|---------|
| Coverage | 5 | All 10 sections, 36 subsections, ~130 papers cited |
| Citation Relevance | 4 | Good alignment with outline; pool-verifiable claims match |
| Factual Consistency | 3 | ⚠️ AutoSurvey2 contradiction lowers this; 1 remaining 🔴 issue |
| Redundancy | 4 | Good cross-referencing; minor duplicate phrasing in §8.4 |
| Citation Balance | 4 | Well-distributed across subsections |
| Section Balance | 4 | Good word distribution; §8 slightly thinner than others |
| **Overall** | **4.0** | **Below 4.3 threshold — one 🔴 issue needs fixing** |

---

## Summary

Total issues: **1 🔴 CRITICAL** + **3 🟡 MODERATE** + **4 🔵 NICE-TO-HAVE**

The draft is nearly ready for final Judge evaluation. The AutoSurvey2 characterization contradiction (§2.3, line 53) is the sole blocking issue — it directly contradicts the pool entry and was identified by both the Judge and this review. Once fixed, along with the table formatting and §7.5 transitional phrases, the factual consistency score should rise to ≥4, putting the total above the 4.3 threshold.

**Next action**: Generator Round 3 — apply C1, M1, M2, M3 fixes. After fixes, re-run Supervisor check. If all 🔴 and 🟡 issues resolved, proceed to Judge evaluation.
