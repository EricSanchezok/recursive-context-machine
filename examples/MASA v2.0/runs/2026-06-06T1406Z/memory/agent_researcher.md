# Agent Researcher — Working Memory

## Round 1 — 2026-06-06T14:29+08:00

### Run Context
- **Run**: 2026-06-06T1406Z (new run, different from the 2026-06-06T1233Z run that reached final survey)
- **Topic**: automated literature survey agents with citation graph expansion
- **current_round**: 1
- **Candidate pool**: 95 unique candidates across 9 roles (17 core_method, 11 mechanism, 12 benchmark, 6 metric, 5 frontier, 12 citation_seed, 9 survey_reference, 9 related_system, 7 boundary)
- **Extended pool**: 107 additional candidates (4 core_method, 25 mechanism, 16 benchmark, 14 metric, 11 survey_reference, 5 citation_seed, 14 related_system)
- **Seed papers**: 8 seeds (AutoSurvey, PaperQA, OpenScholar, SurveyG, SurveyForge, Agentic AutoSurvey, SurveyBench, LitFM)

### Key Differences from Previous Run (2026-06-06T1233Z)
The previous run (1233Z) completed successfully with a final score of 4.55 and produced `07_survey.md`. This new run (1406Z) re-initiates the pipeline with:
1. A refined emphasis on **citation graph expansion** as the central theme (vs. the previous run's broader "survey generation" framing)
2. A 10-section structure (vs. previous 9-section) that elevates citation graph expansion to its own dedicated Section 3
3. Updated candidate pool with 95 unique papers (vs. previous 82/87/88) — including newly discovered papers from extended scouts
4. Different section ordering: Citation Graph Expansion (Section 3) appears right after Architecture taxonomy and before Planning/Retrieval

### Outline Design Rationale

**Section ordering logic**: Architecture → Citation Graph Expansion → Planning → Retrieval → Attribution → Evaluation → Frontiers → Challenges → Conclusion
- This ordering directly prioritizes the primary anchor question: "How do current automated survey agents incorporate citation graph structure to guide paper discovery, relevance ranking, and content synthesis?" by placing the citation graph expansion section (Section 3) immediately after the architectural overview (Section 2).
- The flow follows: understand what survey agents exist (Section 2) → how they traverse citation graphs (Section 3) → how they plan content (Section 4) → how they retrieve evidence (Section 5) → how they attribute and verify (Section 6) → how they are evaluated (Section 7) → emerging paradigms (Section 8) → open problems (Section 9) → conclusion (Section 10).

**Key integration decisions**:
1. **Citation Graph Expansion as Section 3**: Unlike the previous run where citation graph content was distributed across Sections 5.3 and 4.2, this outline dedicates a full section (4 subsections) to citation graph expansion, covering classical methods, graph-based retrieval augmentation, hierarchical traversal, and agent-driven RL optimization.
2. **Citation-specific benchmarks in Section 7.2**: A dedicated subsection for CiteEval, ALCE, CiteME, REASONS, and Survey-Arena, reflecting the expanded emphasis on citation quality evaluation.
3. **Citation graph trade-offs in Section 9.4**: A new open challenge subsection specifically addressing the exploration-exploitation trade-off in citation traversal, optimal depth parameters, and adaptive stopping criteria.
4. **Graph-Aware Deep Research in Section 8.4**: A new subsection bridging survey generation with the deep research paradigm (PaperQA2, OpenScholar, DeepSearch Agents survey).
5. **Section 10 (Conclusion)**: Expanded to include a research agenda explicitly tied to citation graph expansion.

**Papers pulled from extended pool** (not in main 95-candidate pool but referenced in outline):
- arXiv:2402.14207 — STORM — Wikipedia-like article generation (Sections 2.3)
- arXiv:2403.02574 — ChatCite — Human workflow guidance (Section 8.1)
- arXiv:2308.07517 — Synergi — Mixed-initiative synthesis (Section 8.1)
- arXiv:2504.18496 — DimInd — Facet-based synthesis (Sections 5.4, 8.1)
- arXiv:2403.18802 — SAFE/LongFact — Long-form factuality (Sections 6.4, 9.1)
- arXiv:2406.19276 — VERISCORE — Verifiable claims (Section 6.4)
- arXiv:2407.17468 — WildHallucinations — Entity-grounded (Section 6.4)
- arXiv:2408.16444 — SurveySum — Multi-document summarization (Section 4.1)
- arXiv:2005.11401 — RAG foundational paper (Section 5.4)
- arXiv:2402.05629 — D-FActScore — Entity-ambiguity-aware (Section 6.4)
- arXiv:2309.12455 — LongDocFACTScore — Long document factuality (Section 6.4)
- arXiv:2402.14207 — STORM (already listed above)
- arXiv:2305.14627 — ALCE — Citation evaluation benchmark (Section 7.2)
- arXiv:2407.12861 — CiteME — Citation identification (Section 7.2)
- arXiv:2405.02228 — REASONS — Citation attribution (Section 7.2)
- arXiv:2509.25868 — ReFACT — Confabulation detection (Section 7.5)
- arXiv:2409.13740 — PaperQA2 — Superhuman synthesis (Section 8.4)

**Coverage gaps I noticed**:
- No paper specifically benchmarks citation graph expansion effectiveness (how well different traversal strategies perform) — this is noted as an open challenge in Section 9.4.
- Temporal reasoning for citation graphs (time-aware citation ranking, stale citation detection) is under-explored.
- The interaction between citation graph traversal strategy and survey outline structure (whether graph topology should inform section organization) lacks empirical study.
- Cross-domain citation graph traversal (different citation behaviors in CS vs. biomed vs. social sciences) remains uncharacterized.

### Paper Supplement
No supplement needed in Round 1 — this is the initial outline generation.

---

## Round 2 — 2026-06-06T14:42+08:00

### Run Context
- **Run**: 2026-06-06T1406Z (Round 2)
- **current_round**: 2
- **Candidate pool**: 97 unique papers (added 2 from extended pool: arXiv:2503.21460, arXiv:2508.05668)
- **Extended pool**: 107 candidates (unchanged)

### Feedback Summary
**Supervisor Notes** addressed:
- **C1** (AutoSurvey2 contradiction): Verified outline correctly places AutoSurvey2 only in §2.3. Added explicit prohibition in §2.1 guideline.
- **C2** (exclusion citations): Removed requirement to cite [arXiv:2002.06961] and [arXiv:2306.14905] in exclusion context. Added explicit writing note not to cite them.
- **M4** (SurveyScope claim): Verified SciSage [2506.12689] does introduce SurveyScope (46 papers, 11 CS domains). Added SurveyScope to §7.1 benchmark list and cross-referenced in §2.3.
- **M1** (Section 8 expansion): Added minimum 150-word requirement per subsection to Writing Requirements.
- **M2** (automation-vs-control trade-offs): Added 2–3 sentence spectrum analysis to §2.3.
- **M3** (graph partitioning detail): Added substantive coordination-graph-partitioning examples (AgensFlow, KABB, Federation of Agents) to §8.3.
- **N1** (community detection): Added Louvain/spectral clustering discussion to §4.4.
- **N2** (attribution example): Added method-X-SOTA attribution example to §6.3.
- **N3** (coverage breadth standardization): Added explicit definition to §2 and §9.2.

**Judge Suggestions** addressed:
- Item 1 (PRISMA-DFLLM): Covered by C2 fix (no exclusion citations).
- Item 2 (arXiv statistic): Motivating data point now uses [arXiv:2605.07723] citation hallucination audit.
- Item 3 (AutoSurvey2): Same as C1 fix.
- Item 4 (evaluation scores table): Added to §2.4 comparison axes.
- Item 5 (Section 9 expansion): Target 1,100–1,300 words already in guidelines; added more paper references to support content.
- Item 6 (Section 9.6 citations): Added PaperArena [2510.10909] and Deep Search Agents Survey [2508.05668].
- Item 7 (Section 8 expansion): Same as M1 — added 150-word minimum.
- Item 8 (cross-references): Added HierCat (§4.1↔§7.3) and LiRA (§2.2↔§8.3) cross-references.
- Item 9 (unsubstantiated claims): Addressed by adding concrete data sources and cross-references.
- Item 10 (automation-vs-control): Same as M2.
- Item 11 (SurveyBench finding): Added to §4.4 guideline.
- Item 12 (survey spec quality bar): Added to §1 Writing Requirements and guideline #4.
- Item 13 (§6.2 strengthening): Added Generate-then-Refine empirical analysis detail.
- Item 14 (arXiv:2503.21460): Added to main pool from extended pool; included in §1 references.
- Item 15 (PaSa cross-reference): Added to §9.4.
- Item 16 (coverage breadth standardization): Same as N3.

### Paper Supplement — Round 2
- Added: arXiv:2503.21460 (LLM Agent Survey) for broader agent-context framing in Section 1
- Added: arXiv:2508.05668 (Deep Search Agents Survey) for deep research paradigm in Sections 1, 8.4, 9.6
- Source: extended_pool
- Search keyword: N/A (already present in extended pool)

### Key Coverage Gaps Still Open
Same as Round 1:
- No paper specifically benchmarks citation graph expansion effectiveness
- Temporal reasoning for citation graphs is under-explored
- Interaction between citation graph traversal and survey outline structure lacks empirical study
- Cross-domain citation graph traversal remains uncharacterized
