# Merged Candidate Pool — DiscoveryAgent

**run_dir**: `.`
**generated**: 2026-06-07T10:15:37+08:00
**merger_agent**: DiscoveryMerger

**Total unique candidates**: 102 (after deduplication across 4 scout outputs)

---

## Source Summary

| Scout | Source File | Unique Candidates | Notable Contributions |
|-------|------------|------------------|----------------------|
| MethodScout | `02a_method_candidates.md` | 48 | Core method/mechanism papers |
| BenchmarkScout | `02b_benchmark_candidates.md` | 24 | Evaluation benchmarks, datasets, metrics |
| SurveyScout | `02c_survey_candidates.md` | 26 | Existing surveys, taxonomies, additional method papers |
| FrontierScout | `02d_frontier_candidates.md` | ~20 in-scope + ~7 cross-over | Frontier signals, KG traversal, multi-agent orchestration |

**Cross-source overlap**: 21 papers appear in ≥2 scouts (detailed below).

---

## Merged Candidate Entries

### Category A: Core Survey Generation Methods (32 papers)

These are the primary architectural contributions — each proposes a new automated survey pipeline. Primary source: MethodScout and SurveyScout.

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 1 | 2406.10252 | AutoSurvey | 2024 | method-scout, survey-scout | CM-01, CM-05, S-01 | **method** |
| 2 | 2510.26012 | AutoSurvey2 | 2025 | method-scout, survey-scout | CM-01, CM-05, M-02, S-01 | **method** |
| 3 | 2509.18661 | Agentic AutoSurvey | 2025 | method-scout, survey-scout | CM-01, CM-05, S-01 | **method** |
| 4 | 2510.07733 | SurveyG (Hierarchical Citation Graph) | 2025 | method-scout, survey-scout | CM-01, S-01 | **method** |
| 5 | 2503.04629 | SurveyForge | 2025 | method-scout, benchmark-scout, survey-scout | CM-05, M-02, B-01, S-01 | **method** |
| 6 | 2502.14776 | SurveyX (AttributeTree) | 2025 | method-scout, survey-scout | M-02, S-01 | **method** |
| 7 | 2508.17647 | SurveyGen | 2025 | method-scout, benchmark-scout, survey-scout, frontier-scout | M-02, B-01, S-01, F-02 | **method** |
| 8 | 2508.14317 | SurveyGen-I | 2025 | method-scout, survey-scout, frontier-scout | M-02, S-01, F-02 | **method** |
| 9 | 2506.12689 | SciSage / SurveyScope | 2025 | method-scout, benchmark-scout | M-02, B-01 | **method** |
| 10 | 2510.21900 | IterSurvey / Survey-Arena | 2025 | method-scout, benchmark-scout, survey-scout | CM-05, M-02, B-01, S-01 | **method** |
| 11 | 2504.08762 | InteractiveSurvey | 2025 | method-scout, survey-scout, frontier-scout | CM-01, CM-05, M-02, S-01, F-02 | **method** |
| 12 | 2511.17689 | ARISE (Rubric-Guided Iterative) | 2025 | method-scout | CM-05 | **method** |
| 13 | 2504.18496 | DimInd (Facets/Taxonomies/Syntheses) | 2025 | method-scout, survey-scout | CM-05, S-01 | **method** |
| 14 | 2402.01788 | LitLLM | 2024 | method-scout, survey-scout | CM-01, S-01 | **method** |
| 15 | 2408.07884 | Instruct LLMs Step by Step | 2024 | method-scout, survey-scout | CM-01, S-01 | **method** |
| 16 | 2412.15249 | LitLLMs (evaluation protocol) | 2024 | method-scout, benchmark-scout | CM-01, B-02 | **method** |
| 17 | 2407.01796 | ReClaim (Interleaved Reference-Claim) | 2024 | method-scout | CM-04 | **method** |
| 18 | 2504.00824 | ScholarCopilot | 2025 | method-scout | CM-04 | **method** |
| 19 | 2402.16063 | Citation-Enhanced Generation | 2024 | method-scout | CM-04 | **method** |
| 20 | 2411.06159 | Mixture of Knowledge Minigraph Agents | 2024 | method-scout | CM-01 | **mechanism** |
| 21 | 2403.02574 | ChatCite (Reflective Incremental) | 2024 | method-scout | CM-01 | **mechanism** |
| 22 | 2507.15245 | SPAR (Scholar Paper Retrieval) | 2025 | method-scout | M-01 | **mechanism** |
| 23 | 2402.08339 | Interleaved Snowballing | 2024 | method-scout | M-01, M-04 | **mechanism** |
| 24 | 2408.02508 | PUREsuggest | 2024 | method-scout | M-01 | **mechanism** |
| 25 | 2306.03535 | SciLit | 2023 | method-scout | M-01, M-03 | **mechanism** |
| 26 | 2209.13243 | IdeaReader | 2022 | method-scout | M-01 | **mechanism** |
| 27 | 2106.01560 | CitationIE | 2021 | method-scout | M-01, M-04 | **mechanism** |
| 28 | 2306.11832 | QuOTeS (Query-Oriented Technical Summ.) | 2023 | method-scout | CM-05 | **mechanism** |
| 29 | 2406.11460 | TRACE (Knowledge-Grounded Reasoning) | 2024 | method-scout | CM-04 | **mechanism** |
| 30 | 2403.05313 | RAT (Retrieval Augmented Thoughts) | 2024 | method-scout | CM-04 | **mechanism** |
| 31 | 2304.14732 | Search-in-the-Chain | 2023 | method-scout | CM-04 | **mechanism** |
| 32 | 2410.11217 | Citation Generation Capacity | 2024 | method-scout | CM-04 | **mechanism** |

---

### Category B: Existing Surveys & Reference Works (4 papers)

These are surveys *about* the field of automated literature review / survey generation. Primary source: SurveyScout.

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 33 | 2402.08565 | AI for Literature Reviews: Opportunities and Challenges | 2024 | survey-scout | S-01 | **survey** |
| 34 | 2401.10917 | AI to automate systematic review of scientific literature | 2024 | survey-scout | S-01 | **survey** |
| 35 | 2501.04306 | LLM4SR: LLMs for Scientific Research | 2025 | survey-scout | S-01 | **survey** |
| 36 | 2409.04600 | LLMs as tool in literature reviews (auto sys. review) | 2024 | survey-scout | S-01 | **review** |

---

### Category C: Additional Method Papers from SurveyScout (8 papers)

These were surfaced by the survey-oriented queries and represent additional automated survey / SLR pipelines not captured by method-scout queries.

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 37 | 2509.19370 | Meow (Metadata-Driven Outline Writing) | 2025 | survey-scout | S-01 | **method** |
| 38 | 2410.15978 | PROMPTHEUS (Human-Centered SLR Pipeline) | 2024 | survey-scout | S-01 | **method** |
| 39 | 2504.14822 | InsightAgent (Systematic Review in Hours) | 2025 | survey-scout | S-01 | **method** |
| 40 | 2403.08399 | System for SLR using multiple AI agents | 2024 | survey-scout | S-01 | **method** |
| 41 | 2411.18583 | Automated Lit Review using NLP + RAG | 2024 | survey-scout | S-01 | **method** |
| 42 | 2407.20906 | Auto Review Generation (PDH catalysis) | 2024 | survey-scout | S-01 | **method** |
| 43 | 2312.09948 | GEAR-Up (Query Expansion + KG) | 2023 | survey-scout | S-01 | **method** |
| 44 | 2510.13095 | Retrieval-in-the-Chain | 2025 | method-scout | CM-04 | **mechanism** |

---

### Category D: Theory & Infrastructure (8 papers)

These provide foundational theory, datasets, or infrastructure for citation graphs and survey automation.

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 45 | 1805.02262 | Construction of the Literature Graph in Semantic Scholar | 2018 | method-scout, frontier-scout | M-01, F-01 | **theory** |
| 46 | 2110.06595 | Refcat: Internet Archive Scholar Citation Graph | 2021 | method-scout, frontier-scout | M-04, F-01 | **theory** |
| 47 | 1208.5464 | Finding Communities in Citation Graphs | 2012 | method-scout | M-04 | **theory** |
| 48 | 1501.04894 | CITEX: Citation Index for Authors/Papers | 2015 | method-scout | M-04 | **theory** |
| 49 | 1708.00247 | Query Expansion Techniques: A Survey | 2017 | method-scout | M-03 | **theory** |
| 50 | 2311.06785 | Depth and Breadth of Research Area Coverage | 2023 | method-scout | M-04 | **theory** |
| 51 | 1806.00089 | Cascading Citation Expansion | 2018 | method-scout | M-04 | **mechanism** |
| 52 | 2512.22159 | Oignon: Citation Graph Tool | 2025 | method-scout | M-04 | **mechanism** |

---

### Category E: Citation Graph Mechanisms (6 papers)

These are citation graph traversal, retrieval, and analysis mechanisms from the method scout.

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 53 | 1205.1143 | Direction Aware Citation Analysis | 2012 | method-scout | M-01, M-04 | **mechanism** |
| 54 | 2302.07302 | CiteSee (Persistent Citation Context) | 2023 | method-scout, frontier-scout | M-01, F-01 | **mechanism** |
| 55 | 1812.11252 | Non-obvious Paper Recommendation | 2018 | method-scout | M-04 | **mechanism** |
| 56 | 2301.11069 | QeBERT (BERT + Citation Network QE) | 2023 | method-scout | M-03 | **mechanism** |
| 57 | 2005.11401 | RAG for Knowledge-Intensive NLP Tasks | 2020 | method-scout, frontier-scout | CM-04, BX-01 | **method** |
| 58 | 2408.16444 | SurveySum (Multi-Article Summarization Dataset) | 2024 | method-scout | CM-05, M-02 | **benchmark** |

---

### Category F: Benchmarks & Evaluation (17 papers)

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 59 | 2510.03120 | SurveyBench | 2025 | benchmark-scout, survey-scout | B-01, S-01 | **benchmark** |
| 60 | 2508.15658 | SurGE | 2025 | benchmark-scout | B-01 | **benchmark** |
| 61 | 2512.02763 | SurveyEval | 2025 | benchmark-scout | B-01 | **benchmark** |
| 62 | 2601.15307 | DeepSurvey-Bench | 2026 | benchmark-scout | B-01 | **benchmark** |
| 63 | 2602.11238 | SurveyLens | 2026 | benchmark-scout | B-01 | **benchmark** |
| 64 | 2508.11310 | SGSimEval | 2025 | benchmark-scout | B-01 | **benchmark** |
| 65 | 2503.08506 | ReviewBench / Review-CoT | 2025 | benchmark-scout | B-01 | **benchmark** |
| 66 | 2601.14949 | CiteRAG (Citation Prediction Benchmark) | 2026 | method-scout | CM-04 | **benchmark** |
| 67 | 2407.12861 | CiteME (Citation Attribution) | 2024 | benchmark-scout | B-03 | **benchmark** |
| 68 | 2305.14251 | FActScore (Atomic Factuality) | 2023 | benchmark-scout | B-03 | **benchmark** |
| 69 | 2403.18802 | LongFact + SAFE | 2024 | benchmark-scout | B-03 | **benchmark** |
| 70 | 2509.25868 | ReFACT (Scientific Confabulation) | 2025 | benchmark-scout | B-03 | **benchmark** |
| 71 | 2204.04991 | TRUE (Factual Consistency Meta-Eval) | 2022 | benchmark-scout | B-03 | **benchmark** |
| 72 | 2412.13612 | LLMs for Auto Lit Review (Multi-dim eval) | 2024 | benchmark-scout | B-02 | **benchmark** |
| 73 | 2310.04480 | Auto-survey Challenge | 2023 | benchmark-scout | B-02 | **benchmark** |
| 74 | 2308.10410 | Wikipedia-style Survey Eval | 2023 | benchmark-scout | B-02 | **benchmark** |
| 75 | 2411.16638 | Factuality Metrics Critique | 2024 | benchmark-scout | B-02 | **benchmark** |

---

### Category G: Evaluation Datasets (3 papers)

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 76 | 2305.15186 | SciReviewGen (10K reviews, 690K cited) | 2023 | benchmark-scout | B-01 | **dataset** |
| 77 | 2509.00496 | ResearchQA (21K queries, 160K rubric items) | 2025 | benchmark-scout | B-01 | **dataset** |

---

### Category H: Frontier Signals — Live Updating & Interactive (3 papers)

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 78 | 2502.00881 | Toward Living Narrative Reviews | 2025 | frontier-scout | F-01 | **frontier_signal** |
| 79 | 1502.01329 | Perpetual Reviews (Regularly Updated) | 2015 | frontier-scout | F-01 | **frontier_signal** |
| 80 | 2510.15624 | Build Your Personalized Research Group | 2025 | frontier-scout | F-02, CD-02 | **method** |

---

### Category I: Frontier Signals — KG Traversal & Retrieval (7 papers)

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 81 | 2605.23753 | SeedER (Seed-and-Expand KG Retrieval) | 2026 | frontier-scout | CD-01 | **mechanism** |
| 82 | 2601.13969 | ARK (Adaptive KG Exploration) | 2026 | frontier-scout | CD-01 | **mechanism** |
| 83 | 2504.02112 | PolyG (Adaptive GraphRAG Traversal) | 2025 | frontier-scout | CD-01 | **mechanism** |
| 84 | 2410.13765 | Knowledge-Aware Query Expansion (KG+LLM) | 2024 | frontier-scout | CD-01 | **mechanism** |
| 85 | 1911.03868 | Knowledge Guided Text Retrieval for QA | 2019 | frontier-scout | CD-01 | **mechanism** |
| 86 | 1910.03262 | CONVEX (Conversational QA over KGs) | 2019 | frontier-scout | CD-01 | **mechanism** |
| 87 | 2212.05189 | Expanding KGs with Humans in the Loop | 2022 | frontier-scout | CD-01 | **mechanism** |

---

### Category J: Frontier Signals — Multi-Agent Orchestration (5 papers)

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 88 | 2603.13327 | DOVA (Deliberation-First Multi-Agent) | 2026 | frontier-scout | CD-02 | **method** |
| 89 | 2603.03005 | OrchMAS (Multi-Agent Scientific Experts) | 2026 | frontier-scout | CD-02 | **method** |
| 90 | 2507.07257 | Open Source Planning & Control (30 agents) | 2025 | frontier-scout | CD-02 | **method** |
| 91 | 2509.20175 | Federation of Agents (Semantics-Aware) | 2025 | frontier-scout | CD-02 | **method** |
| 92 | 2410.21784 | MARCO (Multi-Agent Real-time Chat Orchestration) | 2024 | frontier-scout | CD-02 | **method** |

---

### Category K: Cross-Over & Citation Seeds (7 papers from frontier boundary guard BX-02)

These were surfaced by the BX-02 boundary guard but contain insights relevant to citation quality, LLM citation behaviour, and bibliographic analysis in survey generation.

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 93 | 2008.13020 | A Decade of In-text Citation Analysis | 2020 | frontier-scout | BX-02 | **citation_seed** |
| 94 | 2309.09727 | When LLMs Meet Citation: A Survey | 2023 | frontier-scout | BX-02 | **citation_seed** |
| 95 | 2504.02767 | How Deep Do LLMs Internalize Citation Practices? | 2025 | frontier-scout | BX-02 | **citation_seed** |
| 96 | 2409.02443 | LLMs for Citation Context Analysis | 2024 | frontier-scout | BX-02 | **citation_seed** |
| 97 | 2510.25378 | Hallucinations in Bibliographic Recommendation | 2025 | frontier-scout | BX-02 | **citation_seed** |
| 98 | 2602.01686 | Unmediated AI-Assisted Scholarly Citations | 2026 | frontier-scout | BX-02 | **citation_seed** |
| 99 | 2605.24351 | LLM Bibliometric Cluster Description | 2026 | frontier-scout | BX-02 | **citation_seed** |

---

### Category L: Adjacent Interactive Paradigms (3 papers from frontier F-02)

These are peripheral to literature surveys but relevant to the adaptive/interactive generation paradigm.

| # | ArXiv ID | Title (short) | Year | Source Agent(s) | Source Query(ies) | Likely Role |
|---|----------|---------------|------|-----------------|-------------------|-------------|
| 100 | 2507.17718 | AI Telephone Surveying | 2025 | frontier-scout | F-02 | **citation_seed** |
| 101 | 2501.05985 | LLM Questionnaires Generation/Adaptation | 2025 | frontier-scout | F-02 | **citation_seed** |
| 102 | 2401.12986 | Crowdsourced Adaptive Surveys | 2024 | frontier-scout | F-02 | **citation_seed** |

---

## Cross-Source Validation

Papers appearing in multiple scouts (indicates high-priority candidates):

| ArXiv ID | Short Title | Appears In | Consensus Role |
|----------|------------|------------|----------------|
| 2406.10252 | AutoSurvey | method-scout, survey-scout | method |
| 2510.26012 | AutoSurvey2 | method-scout, survey-scout | method |
| 2509.18661 | Agentic AutoSurvey | method-scout, survey-scout | method |
| 2510.07733 | SurveyG | method-scout, survey-scout | method |
| 2503.04629 | SurveyForge | method-scout, benchmark-scout, survey-scout | method |
| 2502.14776 | SurveyX | method-scout, survey-scout | method |
| 2508.17647 | SurveyGen | method-scout, benchmark-scout, survey-scout, frontier-scout | method |
| 2508.14317 | SurveyGen-I | method-scout, survey-scout, frontier-scout | method |
| 2506.12689 | SciSage / SurveyScope | method-scout, benchmark-scout | method |
| 2510.21900 | IterSurvey / Survey-Arena | method-scout, benchmark-scout, survey-scout | method |
| 2504.08762 | InteractiveSurvey | method-scout, survey-scout, frontier-scout | method |
| 2504.18496 | DimInd | method-scout, survey-scout | method |
| 2402.01788 | LitLLM | method-scout, survey-scout | method |
| 2408.07884 | Instruct LLMs Step by Step | method-scout, survey-scout | method |
| 2412.15249 | LitLLMs (eval) | method-scout, benchmark-scout | method |
| 2510.03120 | SurveyBench | benchmark-scout, survey-scout | benchmark |
| 1805.02262 | Semantic Scholar Graph | method-scout, frontier-scout | theory |
| 2110.06595 | Refcat | method-scout, frontier-scout | theory |
| 2302.07302 | CiteSee | method-scout, frontier-scout | mechanism |
| 2005.11401 | RAG | method-scout, frontier-scout | method |
| 2510.15624 | Build Your Research Group | frontier-scout (F-02, CD-02) | method |

**Top consensus papers** (≥3 scouts): SurveyForge, SurveyGen, SurveyGen-I, IterSurvey, InteractiveSurvey.

---

## Role Distribution

| Role | Count |
|------|-------|
| **method** | 38 |
| **mechanism** | 19 |
| **benchmark** | 17 |
| **theory** | 5 |
| **survey** | 3 |
| **review** | 1 |
| **dataset** | 2 |
| **frontier_signal** | 2 |
| **citation_seed** | 11 |
| **frontier_method** | 4 |
| **Total unique** | 102 |

*Role counts are approximated from primary likely_role assignments. Several papers span multiple categories (e.g., SurveyGen is method + dataset, SciSage is method + benchmark). The 11 citation_seed papers preserve provenance from frontier boundary guards BX-02 and F-02 adjacent paradigms. Method count (38) includes 5 frontier multi-agent papers from CD-02 and 5 survey-scout additions not captured by method-scout queries.

---

## Risks & Notes

1. **No schema/candidate_pool.md exists** — Written from scratch per workflow specification. Schema directory absent; handoff format derived from instruction template.

2. **run_dir recovery** — `run_dir` is `.` (derived from all four scout artifacts, all of which specify `run_dir: .`). This is consistent.

3. **Frontier boundary preservation** — 11 papers from frontier boundary guards (BX-02 cross-over papers, F-02 adjacent paradigms) are included as `citation_seed` to preserve provenance. These should be reviewed during scoring for possible demotion.

4. **Deduplication method** — Deduplication was performed by arXiv ID (stripping version suffixes v1/v2/etc.). No title-based deduplication was needed as no duplicate titles with different IDs were found across scouts.

5. **Older theory papers** — 5 pre-2020 theory/infrastructure papers (2012–2018) from method-scout M-03/M-04 queries remain in the pool. Expected to be demoted during scoring but kept for provenance.

6. **Temporal gap in F-01** — Only 2 papers directly address longitudinal survey maintenance (2502.00881, 1502.01329). This may indicate a genuine research gap rather than a retrieval failure.

7. **File name** — Written as `02_candidate_pool.md` (the pre-existing `candidate_pool.md` at root level was an earlier partial merge containing only benchmark papers; `02_candidate_pool.md` is the authoritative merge of all four scouts).

---

### Supervisor Retrieval (Round 2)

Papers retrieved by Supervisor to fill knowledge gaps identified during Round 2 review.

| # | ArXiv ID | Title (short) | Year | Source | Likely Role |
|---|----------|---------------|------|--------|-------------|
| 103 | 2605.22878 | SciAtlas: Large-Scale KG for Automated Scientific Research | 2026 | supervisor_retrieval_R2 | infrastructure — KG for multi-hop reasoning |
| 104 | 2508.04612 | A Reproducible, Scalable Pipeline for Synthesizing Autoregressive Model Literature | 2025 | supervisor_retrieval_R2 | infrastructure — reproducibility demonstration |
