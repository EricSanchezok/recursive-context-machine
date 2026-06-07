# Supervisor Notes — Round 1

## Verdict: ACCEPTABLE
Draft is structurally complete, all 15 subsections present, 9 tables, 4 narrative threads woven.

## Key Issues Found

### 馃敶 CRITICAL (1 item)
1. **AutoSurvey2 (2510.26012) missing from §4.1** — Direct successor to foundational AutoSurvey, in candidate pool but not discussed. Add to Table 4 with description.

### 馃煛 MODERATE (4 items)
2. **ResearchPilot trade-off underdeveloped in §5** — Quality vs. privacy trade-off stated in one sentence; needs expansion with metrics quantification.
3. **§6.2 needs aggregated statistics** — "Small topical samples" listed as weakness with examples but no field-wide aggregation (mean/median evaluation scope).
4. **ReClaim cost not quantified in §4.3** — "Computational cost prohibitive" stated but no estimate of calls per survey.
5. **Table 9 missing "Claim Source" column** — Would improve readability and cross-referencing.

### 馃數 NICE-TO-HAVE (3 items)
6. Header style inconsistency in §5 and §6 (### used without preceding ##).
7. "preprocessing" vs "pre-processing" inconsistency.
8. §7 direction 6 (Non-textual content) lacks any grounding — consider merging with direction 3.

## Coverage Check
- All 7 sections from outline are present.
- All 4 narrative threads are developed across their designated sections.
- 61 unique papers cited (43 core method, 9 benchmark, 2 Phase 1, 3 frontier, 4 reference surveys).
- All factual claims checked against available pool — no hallucination detected.

## Recommendations
- Prioritize CRITICAL fix (AutoSurvey2) before next generation round.
- Address MODERATE items 2-5 in §5, §6, §4.3 edits before Judge evaluation.
- NICE-TO-HAVE items are optional polish.

---

# Supervisor Notes — Round 2

## Verdict: CONTINUE

Estimated post-fix score: ~4.20–4.25. Gap to 4.3: ~0.05-0.10.

## Round 2 Fixes Verified (11/11 applied)
All Round 1 feedback items confirmed present in current draft. Details in 06_review.md.

## New Issues (Round 2)
1. **CRITICAL — §4.2**: "Why multi-hop is hard" analysis missing. SciAtlas (2605.22878) provides evidence that current tools lack topological reasoning — cite it.
2. **MODERATE — §6**: Missing reproducibility gap. Reproducible Pipeline (2508.04612) shows F1>0.85 is achievable — contrast with ASG systems.
3. **MODERATE — §5**: No comparison of quality control paradigms (deliberation-first vs. rubric-guided vs. reflect-when-you-write). Add brief analysis to §5.3.

## Papers Retrieved (2)
- 2605.22878 (SciAtlas) — KG infrastructure, supports §4.2 "why multi-hop is hard" analysis
- 2508.04612 (Reproducible Pipeline) — Reproducibility demonstration, supports §6 reproducibility gap

## Focus Sections for Round 3
See memory/focus_sections.md for detailed gap analysis. Sections: §4.2, §6, §5 (in priority order).
