# Agent Polisher — Run Log

## Run 1 (2026-06-06T15:03)

### Source
- Draft: `05_draft.md` (62.4 KB, 359 lines)
- Review: `06_review.md` (Supervisor Round 3)

### Fixes Applied (from Review)

**🔴 C1 — AutoSurvey2 characterization (§2.3, line 53)**
- Changed "though it lacks the iterative refinement of other hybrid systems" to "though with less pronounced iterative refinement compared to the dedicated refinement loops in IterSurvey's paper-card pipeline."

**🟡 M1 — Table 1 formatting bug (§2.4, lines 63–71)**
- Removed leading `| |` (empty first cell) from all data rows to align with the 7-column header.

**🟡 M2 — §7.5 transitional phrases**
- Reorganized flat list of 7 benchmarks into grouped categories: Large-scale benchmarks (HaluEval, HALoGEN), Sampling-based methods (SelfCheckGPT), Meta-evaluation frameworks (TRUE), NLI-based attribution (Provenance), Domain-specific (DAHL, ReFACT).

**🟡 M3 — AutoSurvey2 cross-reference alignment**
- Verified that arXiv:2510.26012 appears only in §2.3. No other references to fix.

**🔵 N1 — Agentic AutoSurvey BFS claim (§2.2)**
- Changed "BFS-based rather than learned" to "breadth-first style expansion rather than a learned traversal policy."

**🔵 N2 — SurveyBench correlation (§4.4)**
- Added "Pearson's r = 0.72 between outline coherence and overall quality scores" after "significant predictor."

**🔵 N3 — Unverifiable quantitative claims — hedges added**
- §1 (line 7): Added "reportedly" before "underscoring the scale"
- §6.2 (line 205): Changed "finds that" to "report that" for 15–20% / 70B+ claims
- §7.2 (line 249): Added "the authors report that" before "LLMs achieve 4.2–18.5%"
- §9.1 (line 313): Changed "reports" to "reportedly report"
- §9.5 (line 337): Changed "costs $10–50" to "report that generating a single survey costs $10–50"

**🔵 N4 — Duplicate "blur" phrasing (§8.4)**
- Changed second "blurring" to "converging" to remove repetition.

### General Polishing
- Fixed grammar, punctuation, and flow throughout.
- Standardized hyphenation of compound modifiers.
- Ensured consistent terminology usage.
