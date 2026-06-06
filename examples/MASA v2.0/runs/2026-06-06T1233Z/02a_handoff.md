# Handoff: MethodScout → Next Scout

**run_dir**: `.`
**artifact**: `02a_method_candidates.md`
**status**: ok
**candidate_count**: 37

**ids** (up to 5 representative arXiv IDs):
- `2509.18661v1` — Agentic AutoSurvey (multi-agent framework, 4 specialist agents)
- `2510.07733` — SurveyG (hierarchical citation graph for survey generation)
- `2510.05138` — LiRA (multi-agent collaborative workflow for literature reviews)
- `2410.11217v1` — Citation Generation Capacity (systematic analysis + Generate-then-Refine)
- `2506.04180v1` — SuperWriter (reflection-driven long-form generation with hierarchical DPO)

**risks**:
- Citation graph expansion query (M-05) returned mostly infrastructure papers (2018–2021) with low direct relevance to survey generation; supplemented by LitFM (2024) and PUREsuggest (2024).
- Some adjacent paper-review agent papers (ReviewAgents, MARG) were caught; flagged as boundary candidates rather than core method.
- CogWriter (2502.12568v3) and SuperWriter (2506.04180v1) are about constrained/reflective long-form generation more broadly, not specifically survey generation — relevance judgement needed downstream.
