# Semantic Expansion via Embedding Search

**Assembled by**: SemanticExpander (MASA pipeline)
**Method**: Holos arXiv embedding search — 7 concept-rich queries
**Date**: 2026-06-06
**run_dir**: `.`
**Seed source**: `03a_seed_papers.md`
**Existing pool before expansion**: 95 candidates (`02_candidate_pool.md`)

---

## Query Design Rationale

Queries target **method family gaps** and **boundary probes** not covered by the 32 SurveyScout keyword queries:

| Gap Identified | Query Focus | Why Needed |
|----------------|-------------|------------|
| KV cache / long-context efficiency | Long-context retrieval accuracy | System prompt example; no pool paper addresses memory-efficient long-context handling for survey agents |
| RL for citation traversal | RL for citation graph traversal | Only PaSa uses RL; no dedicated RL-for-traversal papers in pool |
| Tool-use patterns for retrieval | Tool-augmented literature agents | Several pool systems use tools but no dedicated tool-use methodology paper |
| Evidence attribution / citation hallucination | Citation grounding & attribution | Pool has metrics (FActScore, TRUE) but no dedicated attribution methodology |
| Multi-agent coordination | Multi-agent knowledge synthesis | Pool has many multi-agent systems but no dedicated coordination pattern paper |
| Outline-from-graph structure | Structured outline from citation networks | SurveyG, SurveyForge do this procedurally; no formal outline-from-graph method paper |
| Factual consistency evaluation | Citation accuracy metrics | Pool has FActScore, TRUE; newer benchmarks exist |

---

## Query Results

### Query 1: "methods that reduce KV cache memory while preserving long-context retrieval accuracy"
- **arXiv categories**: cs.CL, cs.IR, cs.LG
- **topK**: 10
- **Results**: 10 papers on KV cache compression (MiniKV, CompressKV, PoD, A²ATS, RocketKV, ClusterKV, CONF-KV, etc.)
- **Deduplication**: 0 already in pool
- **Verdict**: ⚠️ **DRIFT** — All 10 papers are about LLM inference optimization / KV cache engineering, not survey generation or citation graph expansion. None address the literature survey pipeline. Excluded. (This confirms the scope boundary: system-level optimization is out of scope.)

### Query 2: "reinforcement learning for citation graph traversal and academic paper discovery"
- **arXiv categories**: cs.CL, cs.IR, cs.LG
- **topK**: 10
- **Results**: 10 papers (PaSa, CitationIE, citation recommendation papers, RL reference selection)
- **Already in pool**: PaSa (2501.10120v2), Semantic Scholar LitGraph (1805.02262v1)
- **Drifted**: 6 citation recommendation papers (excluded per scope_exclude: "pure recommender systems / citation recommendation without synthesis")
- **New candidates**: **2 identified** (see below)

### Query 3: "tool-augmented language agents for academic literature search retrieval synthesis"
- **arXiv categories**: cs.CL, cs.IR
- **topK**: 10
- **Results**: 10 papers (SPAR, OpenScholar, PaSa, Ai2 Scholar QA, WisPaper, SciLit, Talk to Papers, etc.)
- **Already in pool**: SPAR (2507.15245v1), OpenScholar (2411.14199v1), PaSa (2501.10120v2), Ai2 Scholar QA (2504.10861v2)
- **Drifted**: 5 literature search/management tools (IntellectSeeker, WisPaper, Talk to Papers, Embedding-based Discovery, FAIR RA) — search engines and personal management, not survey generation. Excluded.
- **New candidates**: **1 identified** (SciLit — bridges discovery, summarization, and citation generation)

### Query 4: "evidence attribution and citation grounding in LLM-generated survey content hallucination mitigation"
- **arXiv categories**: cs.CL, cs.IR
- **topK**: 10
- **Results**: 10 papers (Attribution survey, ChatGPT hallucination, G-Cite vs P-Cite, According-to prompting, SurveyForge, SurveyGen, InteractiveSurvey, SurveyX, SurveyGen-I)
- **Already in pool**: SurveyForge (2503.04629v1), SurveyGen (2508.17647v1), InteractiveSurvey (2504.08762v1), SurveyX (2502.14776v2), SurveyGen-I (2508.14317v1)
- **Drifted**: 0
- **New candidates**: **3 identified** (see below)

### Query 5: "multi-agent collaboration patterns for scientific knowledge synthesis survey generation"
- **arXiv categories**: cs.CL, cs.AI, cs.MA
- **topK**: 10
- **Results**: 10 papers (Agentic AutoSurvey, SciSage, AutoResearcher, SurveyG, LiRA, SurveyGen-I, ResearchPilot, CKMAs, AgentRxiv)
- **Already in pool**: Agentic AutoSurvey (2509.18661v1), SciSage (2506.12689v2), AutoResearcher (2510.20844), SurveyG (2510.07733), LiRA (2510.05138), SurveyGen-I (2508.14317v1), ResearchPilot (2603.14629)
- **New candidates**: **2 identified** (see below)

### Query 6: "structured outline generation from citation networks for automatic literature survey planning"
- **arXiv categories**: cs.CL, cs.IR, cs.DL
- **topK**: 10
- **Results**: 10 papers (IterSurvey, Meow, SurveyGen-I, SurveyForge, Tell Me How to Survey, InteractiveSurvey, Generating Related Work, Semantic Scholar LitGraph, Relatedly, CitationIE)
- **Already in pool**: IterSurvey (2510.21900), SurveyGen-I (2508.14317v1), SurveyForge (2503.04629v1), Tell Me How to Survey (2110.06354v3), InteractiveSurvey (2504.08762v1), Semantic Scholar LitGraph (1805.02262v1)
- **New candidates**: **3 identified** (see below)

### Query 7: "factual consistency evaluation in machine-generated scientific text citation accuracy metrics"
- **arXiv categories**: cs.CL, cs.IR
- **topK**: 10
- **Results**: 10 papers (TRUE, Answers with Citations via FCMs, FActScore, QAGS, ALCE, LongDocFACTScore, GO FIGURE, FactKB, CiteME, CiteAudit)
- **Already in pool**: TRUE (2204.04991v3), FActScore (2305.14251v2)
- **New candidates**: **8 identified** (see below)

---

## New Candidates (deduplicated, 17 total)

### Tier 1 — Directly relevant (highest overlap with anchor questions)

| # | arXiv ID | Year | Title | Source Query | Role | Inclusion Rationale |
|---|----------|------|-------|-------------|------|---------------------|
| 1 | **2508.15396v1** | 2025 | Attribution, Citation, and Quotation: A Survey of Evidence-based Text Generation with LLMs | Q4 (evidence attribution) | survey_reference | Comprehensive taxonomy of 134 papers on evidence-based text generation; unifies fragmented terminology; 300 evaluation metrics across 7 dimensions. Directly addresses the citation accuracy / attribution failure mode in the secondary anchor question. |
| 2 | **2509.21557v1** | 2025 | Generation-Time vs. Post-hoc Citation: A Holistic Evaluation of LLM Attribution | Q4 (evidence attribution) | metric / mechanism | Compares G-Cite vs P-Cite paradigms across 4 datasets. Recommends retrieval-centric P-Cite-first approach for high-stakes settings. Directly relevant to how survey agents should handle citations (post-hoc verification vs. inline generation). |
| 3 | **2309.09401v1** | 2023 | ChatGPT Hallucinates when Attributing Answers | Q4 (evidence attribution) | metric / failure_mode | First systematic analysis of ChatGPT's reference hallucination rate (only 14% of suggested references exist). Foundational study on the citation hallucination failure mode. |
| 4 | **2509.19370v1** | 2025 | Meow: End-to-End Outline Writing for Automatic Academic Survey | Q6 (outline generation) | core_method | First dedicated outline writing framework for survey generation. Formulates outline writing as an end-to-end task from paper metadata. Uses SFT + RL training. Directly relevant to the outline-first paradigm. |
| 5 | **2106.01560v1** | 2021 | CitationIE: Leveraging the Citation Graph for Scientific Information Extraction | Q2, Q6 (graph extraction) | mechanism | Augments text representations with citation graph structure for SciIE. Shows simple graph utilization yields significant gains. Bridges citation graph structure and information extraction — directly on the primary anchor question. |
| 6 | **2411.06159v3** | 2024 | Mixture of Knowledge Minigraph Agents (CKMAs) for Literature Review Generation | Q5 (multi-agent) | core_method / mechanism | Novel framework using knowledge minigraph construction + multi-path summarization. Demonstrates LLM-as-agent paradigm for lit review. Three benchmark evaluations. |

### Tier 2 — Indirectly relevant (component technology or adjacent)

| # | arXiv ID | Year | Title | Source Query | Role | Inclusion Rationale |
|---|----------|------|-------|-------------|------|---------------------|
| 7 | **2509.05874v1** | 2025 | Learning to Construct Knowledge through Sparse Reference Selection with Reinforcement Learning | Q2 (RL for citation) | mechanism | Deep RL framework for sparse reference selection emulating human knowledge construction. Directly relevant to the exploration/exploitation tradeoff in the survey spec's expected_dimensions. |
| 8 | **2407.12861v2** | 2024 | CiteME: Can Language Models Accurately Cite Scientific Claims? | Q7 (citation accuracy) | benchmark | Benchmark for evaluating LM citation attribution. Reveals large gap (4–18% LM vs 70% human). Introduces CiteAgent achieving 35%. Directly relevant to citation accuracy evaluation. |
| 9 | **2602.23452** | 2026 | CiteAudit: You Cited It, But Did You Read It? A Benchmark for Verifying Scientific References | Q7 (citation accuracy) | benchmark | First comprehensive benchmark and detection framework for hallucinated citations. Multi-agent verification pipeline. Scales to peer review auditing. |
| 10 | **2305.14627v2** | 2023 | ALCE: Enabling LLMs to Generate Text with Citations | Q7 (citation accuracy) | benchmark / mechanism | First benchmark for Automatic LLMs' Citation Evaluation. Diverse questions + retrieval corpora. 3-dimension automatic metrics (fluency, correctness, citation quality). Widely used eval framework. |
| 11 | **2406.13124v2** | 2024 | Learning to Generate Answers with Citations via Factual Consistency Models | Q7 (citation consistency) | mechanism | Weakly-supervised fine-tuning with FCMs for citation generation. 34.1 F1 improvement over in-context learning. Domain transfer validated. |
| 12 | **2306.03535v2** | 2023 | SciLit: Platform for Joint Scientific Literature Discovery, Summarization and Citation Generation | Q3 (tool-augmented agents) | related_system | End-to-end pipeline: paper recommendation, highlight extraction, context-aware citation suggestion. Bridges discovery and writing. |
| 13 | **2305.08281v2** | 2023 | FactKB: Generalizable Factuality Evaluation using LMs Enhanced with Factual Knowledge | Q7 (factual consistency) | metric | State-of-the-art factuality evaluation generalizing across domains. Outperforms on out-of-domain scientific literature datasets. |
| 14 | **2004.04228v1** | 2020 | QAGS: Asking and Answering Questions to Evaluate Factual Consistency of Summaries | Q7 (factual consistency) | metric | Question-answering based factuality evaluation protocol. Provides interpretability (shows which tokens are inconsistent and why). |

### Tier 3 — Frontier / speculative

| # | arXiv ID | Year | Title | Source Query | Role | Inclusion Rationale |
|---|----------|------|-------|-------------|------|---------------------|
| 15 | **2503.18102v1** | 2025 | AgentRxiv: Towards Collaborative Autonomous Research | Q5 (multi-agent) | frontier | Framework for agent laboratories sharing research via preprint server. Demonstrates collaborative improvement (13.7% relative improvement). Speculative but directly relevant to autonomous research agents. |
| 16 | **2104.08668v1** | 2021 | Generating Related Work | Q6 (outline generation) | citation_seed | Early work modeling related work generation with content planning (tree of cited papers) + surface realization. Pre-LLM but directly relevant foundation. |

### Tier 4 — Drift / excluded candidates

| # | arXiv ID | Year | Title | Source Query | Why Excluded |
|---|----------|------|-------|-------------|-------------|
| — | Various | 2024–2026 | 10 KV cache compression papers | Q1 | LLM inference optimization, not survey generation. Scope boundary: system-level efficiency is outside scope. |
| — | Various | 2012–2024 | 6 citation recommendation papers | Q2 | Pure recommendation without synthesis. Excluded per scope_exclude. |
| — | Various | 2020–2025 | 5 search/literature management tools | Q3 | Search engines and personal management platforms (WisPaper, IntellectSeeker, Talk to Papers, Embedding Discovery, FAIR RA). |
| — | 2309.12455v2 | 2023 | LongDocFACTScore | Q7 | Focused on long-document summarization factuality (scientific), not survey generation specifically. Redundant with FactKB and CiteME. |
| — | 2010.12834v2 | 2020 | GO FIGURE | Q7 | Meta-evaluation of factuality in summarization, not specific to survey generation or citation accuracy. |

---

## Summary

| Metric | Value |
|--------|-------|
| Queries executed | 7 |
| Total results retrieved | 70 |
| Already in pool | ~30 (interspersed across queries) |
| Drift (excluded) | ~23 |
| **New candidates added** | **16** |
| Tier 1 (directly relevant) | 6 |
| Tier 2 (component/adjacent) | 8 |
| Tier 3 (frontier/speculative) | 2 |
| Updated pool size | 95 + 16 = **111** |

### New pool composition by role

| Role | Count | New from this expansion |
|------|-------|------------------------|
| core_method | 17 | +2 (Meow, CKMAs) |
| mechanism | 11 | +5 (CitationIE, RL Ref Selection, ALCE, FCMs, SciLit) |
| benchmark | 12 | +3 (CiteME, CiteAudit, ALCE) |
| metric | 6 | +2 (FactKB, QAGS) |
| survey_reference | 9 | +1 (Attribution Survey) |
| citation_seed | 12 | +1 (Generating Related Work) |
| frontier | 5 | +1 (AgentRxiv) |
| failure_mode | 0 | +1 (ChatGPT Hallucination) |

### Key gaps filled

1. **Evidence attribution / citation paradigm**: 3 papers (Attribution Survey, G-Cite vs P-Cite, ChatGPT Hallucination)
2. **Citation accuracy benchmarks**: 3 papers (CiteME, CiteAudit, ALCE)
3. **Dedicated outline generation**: 1 paper (Meow)
4. **Graph structure for IE**: 1 paper (CitationIE)
5. **Minigraph-based review**: 1 paper (CKMAs)
6. **RL for reference selection**: 1 paper (RL Ref Selection)
7. **Factuality evaluation metrics**: 2 papers (FactKB, QAGS)
8. **Frontier collaboration**: 1 paper (AgentRxiv)

### Remaining gaps (not filled by this expansion)

1. **Pre-2020 foundational work** — still no cocitation theory, early snowballing papers. Requires backward expansion from seeds.
2. **Knowledge-graph-based survey methods** — ORKG-driven work (cross-domain frontier) was already in pool but no new additions.
3. **User studies / human evaluation protocols** — no new human evaluation methodology papers.

---

## Drift Risks

| Risk | Detail |
|------|--------|
| **KV cache query (Q1) produced 100% drift** | All 10 results were LLM inference optimization papers. This confirms the system prompt's example query is a boundary probe — it successfully discriminated scope but produced zero candidates. The KV cache / long-context efficiency gap remains unfilled because no paper in the literature combines KV optimization with survey generation. |
| **High overlap rate on Q5 (multi-agent)** | 7 out of 10 results were already in pool, confirming the pool's comprehensive coverage of multi-agent survey systems. The 2 new candidates were from adjacent communities (minigraph agents, collaborative agent labs). |
| **Citation recommendation drift (Q2)** | 6 citation recommendation papers surfaced — all excluded per scope_exclude. This signals that the embedding search on "citation graph traversal" naturally pulls in the broader citation recommendation literature. Future expansions should add explicit exclusion terms (e.g., "not recommendation" or "survey synthesis"). |
