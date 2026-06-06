# Supervisor Notes — Round 1

**Timestamp**: 2026-06-06T13:00+08:00

## Draft Verdict: STRONG ⚠️ (fixable issues)

## Modification Instructions for Generator (Round 2)

### Mandatory Fixes

1. **Section 1, Reader Roadmap paragraph**: Replace the current paragraph with one that explicitly enumerates all 4 anchor questions from the survey spec:
   - Primary: What are the dominant architectural patterns... → Section 2
   - Secondary 1: How do existing systems evaluate survey quality... → Section 6
   - Secondary 2: What role does citation graph expansion play... → Section 5
   - Secondary 3: What are the key limitations... → Section 8
   Each anchor question should be explicitly called out by number/name and mapped to its section.

2. **Section 8.2, last paragraph**: Replace "The Medical Information Mart for Intensive Care (MIMIC) model in clinical NLP or the GLUE benchmark..." with just the GLUE precedent or use BLUE benchmark (biomedical NLP). MIMIC is a dataset, not an evaluation standardization model. Suggested rewrite: "The GLUE benchmark in general NLP — with its standardized training/leaderboard/scoring protocol — offers a precedent for how such standardization can catalyze a research community."

3. **Section 2.2, first paragraph**: Change "This section-survey addresses coordination patterns" → "This subsection surveys coordination patterns".

4. **Section 4.3**: Add a caveat sentence after the current description (before the concluding paragraph or within it): "We note that these adaptive retrieval methods have been validated primarily on QA and summarization tasks; their transfer to the multi-section survey generation setting, while promising, remains an empirical question."

5. **Section 6.3**: Disambiguate "LitLLMs" from "LitLLM":
   - Where 2402.01788 appears (Sections 2.1, 4.1, 4.2): refer to as "LitLLM toolkit"
   - Where 2412.15249 appears (Section 6.3): refer to as "LitLLMs evaluation study" and note the difference explicitly on first mention

6. **Section 6.5**: After "κ = 0.55–0.75" add "(Cohen's κ)" or specify the type of κ statistic reported in the source paper.

7. **Section 9**: Change "precedents like GLUE in general NLP" → "precedents like GLUE (General Language Understanding Evaluation) in general NLP".

### Recommended Improvements

8. **Section 1, first paragraph**: Split the 4-line sentence after "cannot scale." for readability.

9. **Section 5.4 and 6.4**: Add transitional phrases between method descriptions to reduce the "list-like" presentation. E.g., group methods by approach type (decomposition-based, search-augmented, NLI-based).

10. **Section 7.4**: Remove the duplicate "blur the boundary" statement in the closing paragraph — keep only the first occurrence in the opening paragraph.

11. **Standardize "coverage breadth"** across Sections 3.4, 6.1, and 8.2 where "coverage" is used alone but refers to the same SurveyBench-defined concept.

### Citation Notes

- Papers cited outside the 82-paper candidate pool (6 total) are acceptable as supporting/classic references. No action required unless the Judge flags any.
- DAHL (2411.09255) in Section 6.4 was added by the Generator. Ensure it's documented in the next iteration's candidate pool if it becomes a permanent citation.
- ResearchAgent (2404.07738) in Section 7.4 should ideally be added to the candidate pool if it will remain a primary reference.

### Staleness Check

- `07_survey.md` does not exist yet (will be produced after Generator implements these modifications).
- All input files (`outline.md`, `section_summaries.md`, `05_draft.md`) are current as of Round 1.

### Constraints for Next Agent

- Next agent: Generator (Round 2) — implement the 7 mandatory fixes and 4 recommended improvements listed above.
- After fixes are applied, re-run the supervisor check. If all 🔴 and 🟡 issues are resolved, proceed to Judge evaluation.

---

# Supervisor Notes — Round 3

**Timestamp**: 2026-06-06T13:11+08:00

## Draft Verdict: STRONG ✅ (ready for Judge)

## Fix Verification

All 24 modification items from Round 1–2 were verified against the draft:

**Supervisor Mandatory (7/7)**: ✅ All implemented:
1. Reader roadmap with 4 anchor questions → Section 1 ✅
2. MIMIC → GLUE-only precedent → Section 8.2 ✅
3. "section-survey" → "subsection surveys" → Section 2.2 ✅
4. Adaptive retrieval caveat → Section 4.3 ✅
5. LitLLM toolkit vs LitLLMs eval study → Section 6.3 ✅
6. Cohen's κ specification → Section 6.5 ✅
7. GLUE full name → Section 9 ✅

**Supervisor Recommended (4/4)**: ✅ All implemented:
8. Long sentence split → Section 1 ✅
9. Method grouping + transitions → Sections 5.4, 6.4 ✅
10. Duplicate "blur the boundary" removed ✅
11. "Coverage breadth" standardized ✅

**Judge Critical (3/3)**: ✅ All implemented:
1. HALoGEN labels corrected ✅
2. MIMIC fix (covered by #2) ✅
3. 6 quantitative claims hedged ✅

**Judge Important (5/5)**: ✅ All implemented:
4. Discovery provenance (pool additions) ✅
5. FActScore cross-reference in 8.1 ✅
6. InteractiveSurvey cross-reference in 7.1 ✅
7. Section 8 expanded (~650→~1,250 words) ✅
8. Section 3.4 expanded (~95→~230 words) ✅

**Judge Nice-to-Have (5/5)**: ✅ Implemented (partial on SurveyBench consolidation)

## Remaining Issues

3 moderate issues remain — none blocking for Judge evaluation:

| # | Issue | Location | Severity |
|---|-------|----------|----------|
| M1 | Missing cross-reference: Section 3.3 → Section 5.2 (Generate-then-Refine) | Section 3.3 | 🟡 |
| M2 | SurveyBench citation markers still ~10 (target ≤4) | Throughout | 🟡 |
| M3 | STORM (2402.14207) not in candidate pool | Section 2.3 | 🟡 |

## Staleness Check

- `07_survey.md` does not exist yet (will be produced after Judge approves).
- `05_draft.md` is current as of Round 3 Generator update.
- `memory/section_summaries.md` is current as of Round 3.
- `memory/iteration_state.md`: current_round=3, last_total_score=3.75.

## Constraints for Next Agent

- Next agent: **Judge** — proceed to evaluation of current `05_draft.md`.
- No generator fixes needed before Judge evaluation; remaining M1–M3 issues can be addressed in Round 4 if Judge scores are below threshold.
- If Judge approves (score ≥ 4.3), proceed to `07_survey.md` production.

---

# Supervisor Notes — Round 4

**Timestamp**: 2026-06-06T13:25+08:00

## Draft Verdict: STRONG ✅ (ready for Judge)

## Round 4 Change Verification

All 12 modification items from Round 4 were verified against the draft:

**Generator Changes (12/12)**: ✅ All implemented:
1. MATC paradigm clarification note → Section 2.2 ✅
2. STORM cross-reference as architectural bridge → Section 2.4 ✅
3. Self-Refine/EIPE-text → Generate-then-Refine cross-ref → Section 3.3 ✅
4. SurveyBench citations removed from 3.4, 6.3 ✅
5. SurveyBench removed from 6.3 ROUGE/BLEU → Section 6.3 ✅
6. LitLLM toolkit arXiv ID removed from 6.3 disambiguation → Section 6.3 ✅
7. InteractiveSurvey replaced with cross-refs → Sections 7.1, 8.6 ✅
8. LitFM graph-based vs LitLLM cross-encoder comparison → Section 4.2 ✅
9. OpenScholar datastore vs DimInd facet-based comparison → Section 4.4 ✅
10. Section 8.3 temporal-ordering failure example → Section 8.3 ✅
11. Section 8.5 content-type list + multi-modal LLM refs → Section 8.5 ✅
12. Section 7.4 connection to Section 7.3 coordination patterns → Section 7.4 ✅

## Remaining Issues

**None.** All issues from all previous rounds resolved. Draft is clean.

## Staleness Check

- `07_survey.md` does not exist yet (will be produced after Judge approves).
- `05_draft.md` is current as of Round 4 Generator update.
- `memory/section_summaries.md` is current as of Round 4.
- `memory/iteration_state.md`: current_round=4, last_total_score=3.95, verdict=CONTINUE.

## Constraints for Next Agent

- Next agent: **Judge** — proceed to evaluation of current `05_draft.md`.
- No generator fixes needed; draft is clean and ready for scoring.
- If Judge approves (score ≥ 4.3), proceed to `07_survey.md` production.
- If Judge score is below threshold, investigate remaining issues and handoff to Generator for Round 5.
