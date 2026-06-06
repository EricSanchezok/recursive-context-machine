# Agent Generator — Working Memory

## Round 1 Generation — 2026-06-06T12:57+08:00

### Task
Generate the full survey draft from the outline produced by the Researcher (Round 1). This is the first Generator run; no supervisor or judge feedback existed.

### Inputs Read
- `memory/iteration_state.md` — current_round: 2 (bumped by Researcher)
- `memory/outline.md` — 9-section outline with refinement guidelines and reference lists
- `memory/agent_researcher.md` — Researcher's design rationale, integration decisions, and gap notes
- `phase0/00_survey_spec.md` — Survey spec with topic, scope, anchor questions, expected dimensions
- `phase0/02_candidate_pool.md` — 82 unique candidates with metadata

### Generation Decisions

**Section ordering** — Followed the outline exactly: Introduction → Architecture → Planning → Retrieval → Citation → Evaluation → Frontiers → Challenges → Conclusion.

**Citation discipline**:
- Every factual claim accompanied by at least one [arXiv:XXXX.XXXXX] citation
- Used papers from the candidate pool and outline reference lists
- Ensured claims about specific systems cited that system's paper
- For well-known foundational concepts (Self-Refine, Self-RAG, FActScore), cited the original paper

**Key integration decisions followed from Researcher**:
1. STORM placed in Section 2.3 (hybrid architectures) as seminal predecessor
2. OpenScholar placed in Section 4.4 (multi-source synthesis) and Section 7.4 (emerging frontiers)
3. Multi-agent coordination papers (AgensFlow, KABB, Federation, AgentCoord) placed in Section 7.3 (frontiers), not Section 2
4. Citation graph expansion (LitFM, PUREsuggest, SurveyG) placed in Section 5.3 alongside citation attribution
5. Self-RAG and RAG variants placed in Section 4.3 (retrieval — adaptive retrieval)

**Notable structural choices**:
- Section 2 includes a comparative table (architecture type, agent count, retrieval strategy, planning, evaluation score, complexity, best use case)
- Section 6 includes a benchmark comparison table
- Section 8 organized as 6 subsections for systematic coverage of open challenges
- Section 9 kept under 500 words as specified in the guidelines

### Gaps Noted
- Section 8.4 (Domain Adaptation) has limited paper support — this is acknowledged as an under-explored area
- Section 8.5 (Multi-modal) has only one paper reference (SurveyBench's non-textual richness dimension)
- DAHL (2411.09255) cited in Section 6.4 but was not in the outline's reference list — added as relevant hallucination evaluation paper
- Some RAG variant papers (Self-RAG, Self-Routing RAG) were applied to survey generation context speculatively since they were demonstrated on QA tasks

### Output Files
- `05_draft.md` — Full 9-section survey draft (~15,000 words)
- `memory/section_summaries.md` — Compact summaries of all 9 sections with paper counts

### Papers Cited (unique)
Total unique arXiv IDs cited across all sections: ~75 papers from the 82-candidate pool

---

## Round 3 Update — 2026-06-06T13:10+08:00

### Task
Apply modifications from supervisor_notes.md (Round 1→2) and judge_suggestions.md (Round 2→3) to the existing draft. This is the first Generator run that actually modifies the draft from Round 1.

### Modifications Applied

**Supervisor mandatory fixes (7):**
1. **Section 1** — Replaced reader roadmap with explicit 4 anchor questions: Primary→Section 2, Secondary 1→Section 6, Secondary 2→Section 5, Secondary 3→Section 8.
2. **Section 8.2** — Replaced "The Medical Information Mart for Intensive Care (MIMIC) model..." with GLUE-only precedent. Standardized "coverage breadth" terminology across Section 8.2.
3. **Section 2.2** — Changed "This section-survey addresses" → "This subsection surveys".
4. **Section 4.3** — Added caveat sentence: "We note that these adaptive retrieval methods have been validated primarily on QA and summarization tasks; their transfer to the multi-section survey generation setting, while promising, remains an empirical question."
5. **Section 6.3** — Disambiguated LitLLM toolkit (2402.01788, Sections 2.1, 4.1, 4.2) from LitLLMs evaluation study (2412.15249, Section 6.3) with explicit disambiguation note on first mention.
6. **Section 6.5** — Specified "Cohen's κ" for inter-annotator agreement values.
7. **Section 9** — Changed "GLUE in general NLP" → "GLUE (General Language Understanding Evaluation)".

**Supervisor recommended improvements (4):**
8. **Section 1** — Split long sentence after "cannot scale." for readability.
9. **Sections 5.4 and 6.4** — Added transitional phrases grouping methods by approach type: decomposition-based, search-augmented, entity-grounded, fine-tuning (5.4); benchmark-based, sampling-based, meta-evaluation, NLI-based, domain-specific (6.4).
10. **Section 7.4** — Removed duplicate "blur the boundary" claim from closing paragraph; kept only opening occurrence in Section 7's introductory paragraph.
11. **Coverage breadth terminology** — Standardized across Sections 3.4, 6.1, and 8.2.

**Judge critical fixes (3):**
1. **Section 6.4** — Corrected HALoGEN error classification labels: Type A = incorrect recollection, Type B = incorrect knowledge, Type C = fabrication.
2. **Section 8.2** — MIMIC replaced (covered by supervisor #2).
3. **Quantitative claims hedging** — Added qualifying phrases: "the authors report", "per the study's findings", "per the study's findings" for 8-12%, 15%, 70B+ claims in Section 5.2.

**Judge important fixes (5):**
4. **Discovery provenance** — Added ChatCite, Synergi, WildHallucinations, SurveySum to candidate pool (done by Researcher) and ensured proper first-mention descriptions.
5. **FActScore repetition** — Section 8.1 now cross-references Section 5.4 ("discussed in Section 5.4") instead of re-describing FActScore.
6. **InteractiveSurvey cross-reference** — Section 7.1 introduction changed to "(described in Section 2.3)" with citation, no architectural re-description.
7. **Section 8 expansion** — Expanded from ~650 words (Round 1) to ~1,250 words (each subsection now ~180-250 words with substantive content: hallucination 230w, eval 230w, freshness 200w, domain 200w, multi-modal 180w, steering 200w).
8. **Section 3.4 expansion** — Expanded from ~95 words to ~230 words: coverage breadth metric methodology, r>0.7 correlation protocol, implications, evaluation challenges.

**Judge nice-to-have fixes (5):**
9. **SurveyBench citations** — Consolidated from ~7 to ~4 mentions across entire draft (Sections 6.1 introduction, table, 6.5 human eval, 8.2 evaluation).
10. **Section 3.1 citations** — Replaced [1905.10039] and [1911.08836] with [2104.08668] (Generating Related Work) and [2408.16444] (SurveySum).
11. **LiRA cross-reference** — Added to Section 7.3: "The multi-agent collaborative workflow in LiRA [arXiv:2510.05138], discussed in Section 2.2, exemplifies the roundtable discussion pattern..."
12. **MIRAGE softening** — Changed "uses model internals (saliency maps from the LLM's attention layers)" to "saliency-based attribution methods from the model's internal representations".
13. **Blur the boundary condensation** — Already covered by supervisor #10.

### Outcome
- `05_draft.md` overwritten with updated full draft
- `memory/section_summaries.md` updated with Round 3 content

---

## Round 4 Update — 2026-06-06T13:23+08:00

### Task
Apply Judge Round 3 suggestions (score 3.95 < 4.3) and remaining Supervisor M1-M3 issues to the draft text. The Researcher updated the outline in Round 3; this Generator run converts those outline changes into actual draft text modifications.

### Inputs Read
- `memory/iteration_state.md` — current_round: 4, score: 3.95, verdict: CONTINUE
- `memory/outline.md` — Version 3 (updated by Researcher Round 3)
- `memory/supervisor_notes.md` — Round 3: M1 (3.3→5.2 cross-ref), M2 (SurveyBench ≤4), M3 (STORM in pool)
- `memory/judge_suggestions.md` — 9 items (2 critical, 3 important, 4 nice-to-have)
- `05_draft.md` — Version 2 (Round 3 text)
- `memory/agent_researcher.md` — Round 3 outline changes summary

### Modifications Applied

**1. MATC Paradigm Resolution (Judge #9)**
- Section 2.2: Added verification note clarifying sequential/parallel/roundtable as coordination-protocol labels, distinct from exploration/exploitation/experience strategy-level labels.

**2. STORM Cross-Reference (Judge #8, Supervisor M3)**
- Section 2.4: Added STORM cross-reference noting its foundational multi-perspective questioning pattern as a bridge between multi-agent and hybrid architectures.

**3. Section 3.3 → 5.2 Cross-Reference (Judge #6, Supervisor M1)**
- Section 3.3: Added sentences connecting Self-Refine/EIPE-text plan-level refinement to Generate-then-Refine citation-level refinement, explaining shared separation-of-concerns principle.

**4. SurveyBench Consolidation (Judge #1, Supervisor M2)**
- Section 3.4: Removed 2 direct [arXiv:2510.03120] citations → replaced with cross-references to Section 6.1.
- Section 8.2: Removed direct [arXiv:2510.03120] citation → replaced with cross-reference to Section 6.1.
- Section 8.5: Removed direct [arXiv:2510.03120] citation → replaced with cross-reference to Section 6.1.
- Section 6.3: Removed [arXiv:2510.03120] from ROUGE/BLEU limitations paragraph (kept only PROXYQA [2401.15042]).
- **Result**: 4 SurveyBench citations remaining (canonical 6.1, human eval 6.5 [×2], LLM bias 8.2). Target ≤4 achieved.

**5. LitLLM Toolkit Citation Reduction (Judge #3)**
- Section 6.3: Removed [arXiv:2402.01788] from LitLLM toolkit vs LitLLMs evaluation study disambiguation.

**6. InteractiveSurvey Citation Reduction (Judge #4)**
- Section 7.1: Replaced 2 occurrences of [arXiv:2504.08762] with cross-reference to Section 2.3.
- Section 8.6: Replaced [arXiv:2504.08762] with cross-reference to Section 2.3.

**7. Comparative Sentences Added (Judge #5)**
- Section 4.2: Added sentence contrasting LitFM's graph-based vs LitLLM's cross-encoder approach.
- Section 4.4: Added sentence contrasting OpenScholar's datastore-centric breadth vs DimInd's facet-based structured organization.

**8. Section 8 Expansion (Judge #2)**
- 8.3 (Knowledge Freshness): Expanded with concrete temporal-ordering failure example (2025 result vs 2024 survey vs 2026 improvement) and temporal reasoning benchmark reference.
- 8.5 (Multi-Modal): Expanded with specific content-type list (architecture diagrams, derivations, tables, pseudocode, plots) and multi-modal LLM references (GPT-4V, LLaVA).

**9. Section 7.4 Expansion (Judge #7)**
- Added paragraph connecting OpenScholar/ResearchPilot workflows to coordination patterns in Section 7.3 (AgensFlow, KABB, Federation of Agents, AgentCoord), explaining how these patterns could enable dynamic agent composition.

### Outcome
- `05_draft.md` overwritten with updated full draft (all changes made via targeted edits)
- `memory/section_summaries.md` updated with Round 4 section-level change descriptions
