# Handoff: FrontierScout → CandidateScorer

**run_dir**: `.`
**artifact**: `02d_frontier_candidates.md`
**status**: ok

**candidate count**: 14 unique in-scope papers (from 6 queries × 10 topK = 60 raw hits, ~45 unique after dedup)

**query source**: 01_query_plan.md — frontier (fr-01..fr-03) + cross-domain (cd-01..cd-03)

**representative candidates**:
- **SurveyG** (2510.07733) — hierarchical 3-layer citation graph for survey generation
- **Agentic AutoSurvey** (2509.18661v1) — multi-agent with 4 specialists, 12-dim eval
- **SciSage** (2506.12689v2) — reflect-when-you-write paradigm, SurveyScope benchmark
- **CG-RAG** (2501.15067v1) — citation graph RAG with lexical-semantic graph retrieval
- **LitFM** (2409.12177v1) — graph retriever foundation model for citation graphs
- **IterSurvey** (2510.21900) — recurrent outline generation, iterative retrieval loop
- **SurveyBench** (2510.03120v1) — quiz-driven eval benchmark, 11K arXiv papers

**risks**:
1. **cd-02 and cd-03 are pure discriminators** — all 20 results from exploration/exploitation recommender and multi-hop QA queries were out of scope. These queries serve no candidate-harvesting purpose; consider replacing with more targeted cross-domain queries (e.g., "knowledge distillation for survey summarization" or "taxonomy induction for paper categorization") to increase candidate yield.
2. **Heavy clustering around 2025 multi-agent systems** — the frontier queries strongly converged on the same set of ~7 papers (AutoSurvey2, SciSage, SurveyG, SurveyX, SurveyForge, Agentic AutoSurvey, IterSurvey). This suggests the community is currently concentrated; the CandidateScorer should prioritize methods that are genuinely different in architecture (e.g., CG-RAG's graph RAG vs. SurveyG's hierarchical graph vs. LitFM's foundation model).
3. **run_dir recovered from cwd** — no upstream run_dir was provided; all artifacts are in `.`. Verify this is correct before downstream stages consume paths.
