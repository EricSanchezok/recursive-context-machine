# Supervisor Review Report — Round 2

## Verdict: CONTINUE (estimated post-fix score: ~4.20–4.25, below 4.3 threshold)

## Status of Round 2 Fixes

All 11 fix items from Round 1 supervisor review + judge suggestions have been verified as applied:

| Fix Item | Status | Location |
|----------|--------|----------|
| AutoSurvey2 added to §4.1 | ✅ | Table 4 + 2-paragraph mechanism description |
| ReClaim cost quantified | ✅ | §4.3: ~10 API calls/sentence, ~1,000 calls/100-sentence survey, ~$50-100 |
| ResearchPilot trade-off expanded | ✅ | §5: quantization specifics (4/8-bit, 7B-13B, 60-75% estimate), manual curation |
| §6.2 aggregated statistics | ✅ | mean ~20 topics, median ~15, max ~100 across 12 systems |
| Table 9 Claim Source column | ✅ | §6.1: column with specific paper + section citations |
| Header style consistency | ✅ | ## headers throughout §5 and §6 |
| §7 direction 6 merged into 3 | ✅ | Non-textual content folded as structured output extension |
| Cross-cutting trade-offs in §7 | ✅ | 3 tensions analyzed |
| "77 papers" softened | ✅ | Changed to "most surveyed papers" |
| DOVA cross-reference to §4.1 | ✅ | §5.3: links deliberation quality to rubric-quality problem |
| Prompt-deep falsifiability | ✅ | §4.1: controlled experiment proposal (same base model, different prompts) |

## Remaining Issues

### 馃敶 CRITICAL (1 item)

1. **§4.2 — Multi-hop "why" analysis missing**. The critique that "all graph traversal is single-hop BFS" is stated but not analyzed. Missing: what specific barriers prevent multi-hop reasoning? Is it computational cost, relevance degradation per hop, lack of claim-level provenance data, or missing graph infrastructure? The SciAtlas paper (2605.22878, retrieved this round) explicitly identifies that current tools lack topological reasoning capabilities — this should be cited to substantiate the critique.

### 馃煛 MODERATE (2 items)

2. **§6 — Reproducibility gap missing**. None of the 12+ compared systems provide publicly available, reproducible code for survey generation. This is a critical methodological weakness not addressed. The Reproducible Pipeline paper (2508.04612, retrieved this round) demonstrates F1>0.85 for literature synthesis tasks, providing evidence that reproducibility is achievable — making the gap in ASG systems more salient.

3. **§5 — No direct comparison of quality control paradigms**. Three fundamentally different quality control paradigms exist across the surveyed systems: deliberation-first (DOVA §5), rubric-guided (ARISE §4.1), and reflect-when-you-write (SciSage §4.1). These are never directly compared. Adding a brief comparative analysis in §5.3 would deepen the critical assessment.

### 馃數 NICE-TO-HAVE (1 item)

4. **§7.4 — Domain adaptation concrete example**. The domain adaptation direction mentions OrchMAS as preliminary work but could strengthen its argument with a concrete example of how citation density norms, argumentation structures, and evaluation standards differ across disciplines (e.g., CS vs. biomedicine vs. social sciences).

## Coverage Check
- All 7 sections from outline are present with correct structure.
- All 4 narrative threads developed across designated sections.
- 63 unique papers cited (43 core method, 9 benchmark, 2 Phase 1, 3 frontier, 4 reference surveys, 2 infrastructure papers retrieved this round).
- All Round 2 fixes verified against source subsection files.

## Next Steps
- Round 3 Generator should address CRITICAL + MODERATE items.
- Estimated impact: adding "why multi-hop is hard" analysis (~+0.02 overall), reproducibility gap (~+0.015), quality control comparison (~+0.0125) = ~+0.05 total, bringing estimated score to ~4.25-4.30.
