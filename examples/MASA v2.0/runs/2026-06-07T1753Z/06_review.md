# Supervisor Review Report — Round 2

**Draft**: `05_draft.md` (689 lines, ~116 KB)
**Review Date**: 2026-06-07
**Verdict**: ACCEPTABLE
**Score Trend**: Round 1 → Round 2: 4.15 → Est. ≥4.3 (all critical and major issues resolved)

---

## Round 1 Issue Resolution

| ID | Issue | Priority | Status | Evidence |
|----|-------|----------|--------|----------|
| C1 | Cross-phase comparison table | 馃敶 CRITICAL | ✅ FIXED | §2 end (L170-185): 8-column table with all required dimensions |
| C2 | Unsourced "2M papers" claim | 馃敶 CRITICAL | ✅ FIXED | L5: cites [STM Global Brief 2023; UNESCO Science Report 2021] |
| C3 | Paper count "over 140" | 馃敶 CRITICAL | ✅ FIXED | L13: "135+ papers" |
| M1 | Cross-domain context gap | 馃煛 MODERATE | ✅ FIXED | §1 L11: ¶ with PRISMA, SummEval, scientometric citations |
| M2 | Claim boundary "every system" | 馃煛 MODERATE | ✅ FIXED | L443: "nearly every system — sole exception is PaperQA/PaperQA2" |
| M3 | Paper count mismatch | 馃煛 MODERATE | ✅ FIXED | Now "135+" (taxonomy updated to 139 post-retrieval) |
| M4 | Baseline values in §3.4 | 馃煛 MODERATE | ✅ FIXED | L291-296: "Baseline Absolute Value" column added |
| N1 | Cross-references | 馃數 NICE | ✅ ADDRESSED | §3.3 cross-refs §2.3; §5.3→§6.4; §6.1→§5.4 |
| N2 | Evaluation weight justification | 馃數 NICE | ✅ ADDRESSED | §6.3 weight table with rationale (already present in Generator output) |
| N3 | Conclusion expansion | 馃數 NICE | ✅ FIXED | §7: ~400 words with concrete 2-year vision |
| N4 | Cross-domain context (sup. note) | 馃數 NICE | ✅ FIXED | Folded into M1 fix |

---

## Round 2 New Assessment

### 馃敶 CRITICAL: None

All Round 1 critical issues are resolved. No new critical issues emerge.

### 馃煛 MODERATE

**M5 — §2.5 paper coverage is thin** (Section 2.5)
The current frontier section covers only 4 systems (SurveyG, Graphs of Research, Science Hierarchography, LitFM). DeepSurvey (2605.29522, May 2026) represents a significant advance: it combines citation-graph expansion with multi-granularity agentic refinement, achieving 8.644/10 content score and 83.3% expert preference over human-written surveys. Adding this paper would directly strengthen the "open problems" analysis — DeepSurvey partially addresses two of the four identified gaps (shallow integration and no graph-aware multi-agent system).

**M6 — §3.4 bottleneck transfer lacks direct empirical evidence** (Section 3.4)
The three structural reasons (selection vs recall, recall-coherence trade-off, unmeasured quality dimensions) are logically sound but no cited study directly measures the correlation between retrieval quality and generation quality. Paper 2512.20854 ("How important is Recall for Measuring Retrieval Quality?") provides exactly this: it evaluates the correlation between retrieval metrics and LLM-judged generation quality, finding metric-dependent relationships. Integrating this finding would transform §3.4 from a theoretical argument into an empirically-grounded critique.

**M7 — §4.1/§4.3 tables overlap with §4.4 synthesis** (Section 4)
The individual subsection tables in §4.1 (L328-333) and §4.3 (L381-386) contain comparison dimensions that are re-summarized in §4.4's synthesis table. The tables could be trimmed to focus on within-category comparisons only, with cross-category synthesis delegated to §4.4.

**M8 — §5.1 claim table lacks cross-references to critiques** (Section 5.1)
The 7-claim table (L431-439) includes citations for each claimed performance metric but does not reference the specific critical sources (SurGE, SGSimEval, DeepSurvey-Bench) in the table cells themselves. These critiques are mentioned in the analysis paragraph (L441-443) but the table would be more impactful with direct cross-references.

### 馃數 NICE-TO-HAVE

**N5 — §6.2 thin reference set** (Section 6.2)
Currently cites only PaSa (2501.10120) and Temporal GNN (2408.15371). RL-based sparse reference selection (2509.05874) and citation evolution DAGs (2605.14790) provide relevant prior work.

**N6 — Paper count update** (Section 1)
L13 says "135+ papers" which was correct before retrieval. After adding 2 papers to the taxonomy (now 139), consider updating to "137+" or keeping "135+" as an approximate bound.

---

## Depth Assessment (Round 2)

| Section | Depth Rating | Assessment |
|---------|-------------|------------|
| §1 Introduction | STANDARD | Clear positioning, cross-domain context added. All three contributions stated. |
| §2 Evolution Arc | DEEP | 5 phases with mechanism detail, performance tables, critical analysis. Cross-phase table adds synthesis. 馃煛 §2.5 needs more papers. |
| §3 Architectural Deep Dive | DEEP | Mechanism-level analysis for each paradigm. Bottleneck transfer argument is novel and well-structured. 馃煛 §3.4 needs empirical support. |
| §4 Cross-Cutting Strategies | DEEP | Detailed mechanisms + critical analysis per approach. §4.4 synthesis adds cross-category comparison. 馃數 Trim §4.1/§4.3 tables to reduce overlap with §4.4. |
| §5 Critical Assessment | EXCELLENT | Claim-evidence table, 6 weaknesses, 11-benchmark landscape, 5 blind spots, root cause argument. Strong analysis throughout. |
| §6 Future Directions | STANDARD | Well-grounded proposals with existing work references. 馃煛 §6.2 thinner than §6.1/§6.3. |
| §7 Conclusion | STANDARD | ~400 words with 4-thread synthesis and concrete 2-year vision. |

---

## Claim Boundary Scan

| Location | Claim | Scope Check | Verdict |
|----------|-------|-------------|---------|
| L67 | PaperQA2 "superhuman" — critiqued by SurveyLens and DeepSurvey-Bench | Citations provided for both critiques | ✅ PASS |
| L113 | "No controlled ablation studies exist" [for multi-agent] | Scoped to survey generation literature; supported by analysis in §2.3 | ✅ PASS |
| L168 | "No system uses the graph simultaneously for retrieval, organization, validation, and narrative tracing" | Scoped to systems in pool; accurate | ✅ PASS |
| L443 | "nearly every system invents its own evaluation protocol — the sole exception is PaperQA and PaperQA2" | Properly scoped with exception | ✅ PASS |
| L514 | "No system or benchmark evaluates whether a generated survey provides original analysis" | Hedged by "take partial steps" for DeepSurvey-Bench, SGSimEval | ✅ PASS |
| L546 | "the single most important missing capability" | Opinion/argument, not factual claim | ✅ PASS |

---

## Self-Contradiction Check

| Cross-Section Comparison | Result |
|--------------------------|--------|
| §3.4: "no single system reports both retrieval and quality metrics in a way that allows causal linking" vs §5.1 claim table showing SciSage reports both | ✅ Consistent — the table notes SciSage's metrics are "conflated" not causally linked |
| §4: "no approach addresses critical-analytic depth" vs §5.4: DeepSurvey-Bench/SGSimEval take "partial steps" | ✅ Consistent — "partial steps" ≠ measures critical-analytic depth |
| §5.2: "no system reports computational cost in a standardized way" vs §6.4: "require standardized cost reporting" | ✅ Consistent — current gap / future direction |
| §6.1 unified architecture proposal vs §6.4 community benchmarking — potential overlap | ✅ Different scopes: architecture is technical design, benchmarking is social coordination |

---

## Evidence Retrieval Summary (Round 2)

| Paper | arXiv ID | Section Gap | Download | Profile | Pool | Taxonomy |
|-------|----------|-------------|----------|---------|------|----------|
| DeepSurvey | 2605.29522 | §2.5 (thin coverage), §3.4 (missing evidence) | ✅ pdfs/ | ✅ profile/2605.29522.md | ✅ 02_candidate_pool.md | ✅ Category 3 |
| Recall Quality | 2512.20854 | §3.4 (no empirical correlation evidence) | ✅ pdfs/ | ✅ profile/2512.20854.md | ✅ 02_candidate_pool.md | ✅ Category 6 |

---

## Files Modified This Round

- `06_review.md` — Written (this file)
- `memory/supervisor_notes.md` — Appended with Round 2 notes
- `memory/focus_sections.md` — Written with 3 focus sections + retrieved papers
- `phase0/02_candidate_pool.md` — Added 2 new papers
- `phase0/paper_taxonomy.md` — Count 137→139, added 2 papers to categories 3 and 6
- `phase0/paper_profiles/2605.29522.md` — Created
- `phase0/paper_profiles/2512.20854.md` — Created
- `pdfs/2605.29522.pdf` — Downloaded
- `pdfs/2512.20854.pdf` — Downloaded

---

## Recommendations for Generator (Round 3)

| Priority | Action | Section | Evidence Source |
|----------|--------|---------|-----------------|
| HIGH | Integrate DeepSurvey (2605.29522) findings into §2.5 — citation-graph expansion + multi-granularity refinement partially closes the "shallow integration" open problem | §2.5 | profile/2605.29522.md |
| HIGH | Integrate paper 2512.20854 into §3.4 — its empirical correlation analysis transforms the bottleneck transfer argument from theoretical to evidence-based | §3.4 | profile/2512.20854.md |
| MEDIUM | Add RL-based sparse reference selection (2509.05874) and citation evolution DAGs (2605.14790) to §6.2 | §6.2 | Search results above |
| MEDIUM | Tighten §4.1 and §4.3 comparison tables to avoid overlap with §4.4 synthesis | §4.1, §4.3 | Internal consistency |
| LOW | Add cross-reference citations to §5.1 claim table cells pointing to critique papers | §5.1 | L431-439 |
| LOW | Update paper count in §1 from "135+" to "137+" if taxonomy finalized | §1 | taxonomy.md (139) |

---

## Staleness

| Artifact | Status |
|----------|--------|
| `07_survey.md` | Does not exist — not stale |
| `05_draft.md` | Current (Round 2) |
| `memory/focus_sections.md` | Newly written (Round 2) |
| `memory/supervisor_notes.md` | Updated |
