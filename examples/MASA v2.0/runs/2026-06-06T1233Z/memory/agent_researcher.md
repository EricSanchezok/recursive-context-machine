# Agent Researcher — Working Memory

## Round 1 — 2026-06-06T12:54+08:00

### Task
Generate the initial survey outline for "Automated literature survey generation using large language models: agent architectures, retrieval-augmented pipelines, and evaluation methodologies".

### Inputs Read
- `phase0/00_survey_spec.md` — Survey spec with topic, scope, anchor questions, expected dimensions
- `phase0/02_candidate_pool.md` — 82 unique candidates (12 core_method, 34 mechanism, 11 benchmark, 10 evaluation, 7 survey, 3 frontier, 2 citation_seed, 3 boundary)
- `phase0/03a_seed_papers.md` — 10 seed papers for citation graph expansion
- `phase0/02b_candidate_pool_extended.md` — 88 extended candidates (50 high relevance)
- `memory/iteration_state.md` — current_round: 1

### Outline Design Rationale

**Section ordering logic**: Systems → Planning → Retrieval → Attribution → Evaluation → Frontiers → Challenges → Conclusion
- This follows the natural progression of how a survey generation system works: first understand the architecture (Section 2), then how it plans (Section 3), retrieves (Section 4), cites (Section 5), and gets evaluated (Section 6), before looking at emerging directions (Section 7) and open problems (Section 8).

**Key integration decisions**:
1. **STORM (2402.14207)** and **OpenScholar (2411.14199)** were pulled from the extended pool to fill gaps — STORM as the seminal Wikipedia-article system that inspired survey generators, OpenScholar as the most recent scientific literature synthesis system.
2. **Multi-agent coordination papers** (AgensFlow, KABB, Federation of Agents, AgentCoord) were placed in the Frontiers section rather than Section 2, because they are not yet deployed in survey systems but represent transferable coordination patterns.
3. **Citation graph expansion** (LitFM, PUREsuggest, SurveyG's citation graph) was placed in Section 5 alongside citation attribution, rather than Section 4 (retrieval), because the primary purpose is citation fidelity and coverage breadth rather than evidence retrieval.
4. **Self-RAG and RAG variants** were placed in Section 4 (retrieval) because they are retrieval mechanisms, not full survey architectures.

**Gaps I noticed**:
- No paper specifically about **domain adaptation** of survey systems — this is noted as an open challenge.
- **Longitudinal/update evaluation** is thin (only vitaLITy 2 and Evolving Literature Analysis).
- **Multi-modal survey generation** is essentially absent — noted as an open challenge.
- The boundary between **related work generation** (single-section) and **full survey generation** (multi-section) could be sharper.

### Papers Added from Extended Pool
- arXiv:2402.14207 — STORM — Knowledge-grounded Wikipedia-like article generation (Section 2)
- arXiv:2411.14199 — OpenScholar — Retrieval-augmented scientific literature synthesis (Section 4, 7)
- arXiv:2504.18496 — DimInd — Facet-based LLM-assisted literature review (Section 7)
- arXiv:2403.02574 — ChatCite — Human workflow guidance for comparative summaries (Section 7)
- arXiv:2308.07517 — Synergi — Mixed-initiative scholarly synthesis (Section 7)
- arXiv:2403.18802 — LongFact + SAFE — Long-form factuality evaluation (Section 5, 6)
- arXiv:2406.19276 — VERISCORE — Factuality of verifiable claims (Section 5, 6)
- arXiv:2407.17468 — WildHallucinations — Entity-grounded factuality (Section 5, 6)
- arXiv:2408.16444 — SurveySum — Multi-document summarization into survey sections (Section 6)

---

## Round 2 — 2026-06-06T13:05+08:00

### Task
Update the survey outline based on Judge (Round 2) and Supervisor (Round 1) feedback. Since this is Round 2, structural changes are permitted but were unnecessary — the 9-section structure was maintained.

### Inputs Read
- `memory/iteration_state.md` — current_round: 2, score: 3.75, threshold: 4.3, verdict: CONTINUE
- `memory/outline.md` — previous outline (Version 1)
- `memory/supervisor_notes.md` — 7 mandatory fixes + 4 recommended improvements (addressed to Generator for draft text)
- `memory/judge_suggestions.md` — 14 items (3 critical, 5 important, 6 nice-to-have)
- `memory/judge_report_round2.md` — Full judge report with dimension scores
- `memory/judge_coverage.md` — Score 5
- `memory/judge_citation_relevance.md` — Score 4, detailed analysis of 84 citations
- `memory/judge_section_balance.md` — Score 3, identified Section 8 (650w, 6 subsections) and Section 3.4 (95w) as thin
- `phase0/02_candidate_pool.md` — 82 candidates (updated to 87)
- `phase0/02b_candidate_pool_extended.md` — 88 extended candidates
- `phase0/03a_seed_papers.md` — 10 seed papers

### Paper Supplement Protocol — Round 2
Added 5 papers from the extended pool to `phase0/02_candidate_pool.md`:

| arXiv ID | Title | Reason for Adding |
|----------|-------|-------------------|
| 2411.14199 | OpenScholar | Judge Item #13 — major system, used in Sections 4.4 and 7.4 |
| 2504.18496 | DimInd | Judge Item #13 — facet-based synthesis, used in Sections 4.4 and 7.1 |
| 2404.07738 | ResearchAgent | Judge Item #13 + Supervisor citation note — used in Section 7.4 |
| 2411.09255 | DAHL | Supervisor citation note — used in Section 6.4 |
| 2403.02574 | ChatCite | Judge Item #4 — missing provenance, used in Sections 7.1 and 8.6 |

Source: All from `phase0/02b_candidate_pool_extended.md` (extended pool).

### Outline Changes Summary

**Citation replacements:**
- Section 3.1: Replaced [1905.10039] (Plan-and-Write, story generation) with [2104.08668] (Generating Related Work)
- Section 3.1: Replaced [1911.08836] (Template NLG) with [2408.16444] (SurveySum)

**Refinement guideline expansions (key ones):**
- Section 3.4: Target ~200 words (was ~95)
- Section 8 (all 6 subsections): Target ~180-250 words each (was ~108); total ~1,100-1,300 words
- Sections 5.4 and 6.4: Added method-grouping guidelines to reduce list-like presentation
- Section 7.4: Added redundancy-avoidance directive

**Precision fixes applied to guidelines:**
- Section 5.1: Softened MIRAGE to "saliency-based attribution methods"
- Section 6.4: Added HALoGEN label verification note
- Section 6.5: Added Cohen's κ specification
- Section 8.2: Replaced MIMIC with GLUE
- Section 8.5: Added caution about non-textual richness dimension verification
- Section 9: Added GLUE full name

**Terminology standardization:**
- Standardized "coverage breadth" across Sections 3.4, 5.2, 5.3, 6.1, and 8.2

**Cross-reference additions:**
- Section 7.1: Use cross-reference to Section 2.3 for InteractiveSurvey
- Section 7.3: Add LiRA cross-reference from Section 2.2

**Five papers added to candidate pool** (from extended pool) to address provenance gaps.

### Coverage Gaps Noticed
- "Non-textual richness" dimension in SurveyBench (2510.03120) needs verification — the pool notes don't confirm this dimension exists. If it doesn't exist, Section 8.5 needs an alternative citation.
- MIRAGE mechanism (attention-layer saliency maps vs general saliency methods) needs verification against the actual paper.
- The 6 unverifiable quantitative claims (r>0.7, 8-12%, 15%, 70B+, 15-25%, r≈0.4-0.5) remain a verification task for the Generator.

---

## Round 3 — 2026-06-06T13:20+08:00

### Task
Update the survey outline based on Judge Round 3 feedback (total score 3.95 < 4.3 threshold) and Supervisor Round 3 remaining issues (M1-M3). Since this is Round 3+, only refinement guidelines and reference paper lists can be updated; section structure must be preserved.

### Inputs Read
- `memory/iteration_state.md` — current_round: 3, score: 3.95, verdict: CONTINUE
- `memory/outline.md` — previous outline (Version 2)
- `memory/supervisor_notes.md` — 3 remaining issues: M1 (3.3→5.2 cross-ref), M2 (SurveyBench ~10 mentions), M3 (STORM not in pool)
- `memory/judge_suggestions.md` — 9 items (2 critical, 3 important, 4 nice-to-have)
- `memory/judge_report_round3.md` — Full report with dimension scores: Coverage 5, Citation Relevance 4, Factual Consistency 4, Redundancy 4, Citation Balance 3, Section Balance 3. Total 3.95.
- `phase0/02_candidate_pool.md` — 87 candidates (updated to 88)
- `phase0/02b_candidate_pool_extended.md` — 88 extended candidates

### Paper Supplement Protocol — Round 3
Added 1 paper from the extended pool:

| arXiv ID | Title | Reason for Adding |
|----------|-------|-------------------|
| 2402.14207 | STORM | Supervisor M3 — seminal hybrid system, used in Section 2.3, not in candidate pool |

### Outline Changes Summary

**SurveyBench consolidation (9→4 mentions target):**
- Removed arXiv:2510.03120 from Section 3.4 and Section 8.2 reference lists
- Added cross-reference instructions to Section 6.1 in both sections
- Preserved canonical reference in Section 6.1

**Section 8 expansion (~760w→1,100-1,300w target):**
- 8.3: Added concrete temporal-ordering failure example + temporal reasoning benchmark guidance
- 8.4: Added social-science discourse structure expansion with literature references
- 8.5: Added multi-modal LLM references (GPT-4V, LLaVA) + concrete content-type examples
- All 6 subsections retargeted to ~200 words each

**Citation reduction (LitLLM 6→3-4, InteractiveSurvey 6→3):**
- Section 6.3: Removed LitLLM arXiv ID from disambiguation
- Sections 7.1 and 8.6: Replaced InteractiveSurvey citations with cross-references; removed arXiv:2504.08762 from reference lists

**Cross-references added:**
- 3.3→5.2 (Self-Refine↔Generate-then-Refine) — resolves Judge #6 and Supervisor M1
- 2.4→STORM — resolves Judge #8 and Supervisor M3
- 7.4→7.3 (OpenScholar↔coordination patterns) — resolves Judge #7

**Comparative sentences added:**
- 4.2: LitFM graph-based vs LitLLM cross-encoder
- 4.4: OpenScholar datastore-centric vs DimInd facet-based

**MATC paradigm verification note** added to 2.2 (Judge #9).

### Key Decisions
- Kept SurveyBench in Section 6 reference list (canonical section) but removed from all non-canonical sections.
- Used cross-references rather than citations for systems described in detail elsewhere (InteractiveSurvey, STORM, SurveyBench).
- All changes are refinement guidelines or reference list updates; no sections added, removed, or reordered.
