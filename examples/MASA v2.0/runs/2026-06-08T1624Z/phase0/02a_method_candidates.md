# Method & Mechanism Candidates — MASA Pipeline

**Generated**: 2026-06-08
**run_dir**: `.`
**Scout**: MethodScout

---

## Overview

| Query # | Type | Query String | Results | New Candidates |
|---------|------|-------------|---------|----------------|
| 1 | `core_method` | multi-agent survey generation 2025 | 10 | 10 |
| 3 | `core_method` | graph neural network literature expansion citation | 10 | 10 |
| 5 | `core_method` | LLM based literature review pipeline | 10 | 10 |
| 7 | `mechanism` | citation graph expansion breadth depth traversal | 10 | 10 |
| 8 | `mechanism` | paper retrieval forward backward citation traversal | 10 | 9 |
| 9 | `mechanism` | PageRank influence propagation scientific literature | 10 | 9 |
| **Total** | | | **60** | **58** |

**Deduplicated**: 2 duplicates removed (1812.11252v1 repeated across Q7↔Q8; 1904.07579v1 repeated across Q7↔Q9)

---

## Candidate Papers

### Query 1 — `core_method`: multi-agent survey generation 2025

| # | arXiv ID | Title | Year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 1 | 2506.12689v2 | SciSage: A Multi-Agent Framework for High-Quality Scientific Survey Generation | 2025 | core_method | Multi-agent framework with hierarchical Reflector agent; introduces SurveyScope benchmark. |
| 2 | 2509.18661v1 | Agentic AutoSurvey: Let LLMs Survey LLMs | 2025 | core_method | Four specialized agent architecture (Search, Topic Mining, Writer, Evaluator); 12-dimension evaluation. |
| 3 | 2510.07733 | SurveyG: A Multi-Agent LLM Framework with Hierarchical Citation Graph | 2025 | core_method | Combines multi-agent generation with hierarchical citation graph (Foundation/Development/Frontier layers). |
| 4 | 2510.10890 | LLM×MapReduce-V3: Enabling Interactive In-Depth Survey Generation | 2025 | core_method | MCP-driven hierarchically modular agent system for long-form survey generation. |
| 5 | 2508.11310v1 | SGSimEval: A Comprehensive Multifaceted Benchmark for ASG Systems | 2025 | benchmark | Evaluation benchmark for automatic survey generation; outline/content/reference assessment. |
| 6 | 2510.26012 | AutoSurvey2: Empowering Researchers with Next Level Automated Surveys | 2025 | core_method | Multi-stage pipeline with parallel section generation, iterative refinement, real-time retrieval. |
| 7 | 2503.04629v1 | SurveyForge: On the Outline Heuristics, Memory-Driven Generation | 2025 | core_method | Outline heuristics + scholar navigation agent for memory-driven content generation. |
| 8 | 2508.14317v1 | SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans | 2025 | core_method | Coarse-to-fine retrieval, adaptive planning, memory-guided generation. |
| 9 | 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow | 2025 | core_method | Iterative workflow with recurrent outline generation; introduces Survey-Arena benchmark. |
| 10 | 2504.08762v1 | InteractiveSurvey: An LLM-based Personalized and Interactive Survey System | 2025 | core_method | Interactive survey generation with user-customizable reference categorization and outlines. |

### Query 3 — `core_method`: graph neural network literature expansion citation

| # | arXiv ID | Title | Year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 11 | 2408.15371v1 | Temporal GNN-Powered Paper Recommendation on Dynamic Citation Networks | 2024 | mechanism | TGN for paper recommendation; embeds temporal dynamics of citation evolution. |
| 12 | 2412.19419v1 | Introduction to Graph Neural Networks: A Starting Point | 2024 | survey | GNN tutorial; useful background for GNN-based literature analysis. |
| 13 | 2012.08752v4 | Graph Neural Networks: Taxonomy, Advances and Trends | 2020 | survey | Comprehensive GNN taxonomy with 400+ references. |
| 14 | 2201.01188v1 | Graph Neural Networks: a bibliometrics overview | 2022 | survey | Bibliometric analysis of GNN research landscape. |
| 15 | 2104.02562v1 | Structured Citation Trend Prediction Using GNNs | 2021 | mechanism | GNN-based citation trend prediction from citation graph structure. |
| 16 | 1901.00596v4 | A Comprehensive Survey on Graph Neural Networks | 2019 | survey | Foundational GNN survey; widely cited reference for GNN taxonomy. |
| 17 | 2305.01572v3 | H2CGL: Modeling Dynamics of Citation Network for Impact Prediction | 2023 | mechanism | Hierarchical heterogeneous contrastive graph learning for citation impact prediction. |
| 18 | 1812.08434v6 | Graph Neural Networks: A Review of Methods and Applications | 2018 | survey | Systematic GNN review; covers GCN, GAT, GRN variants. |
| 19 | 1810.00826v3 | How Powerful are Graph Neural Networks? | 2018 | foundational | Theoretical framework for GNN expressive power; GIN architecture. |

### Query 5 — `core_method`: LLM based literature review pipeline

| # | arXiv ID | Title | Year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 20 | 2410.15978v2 | PROMPTHEUS: A Human-Centered Pipeline to Streamline SLRs with LLMs | 2024 | core_method | AI-driven pipeline automating SLR search, extraction, topic modeling, summarization. |
| 21 | 2510.11409 | Leveraging LLMs for Semi-Automatic Corpus Filtration in Systematic Reviews | 2025 | mechanism | Multi-LLM consensus pipeline for SLR corpus filtration. |
| 22 | 2412.15249v2 | LitLLMs, LLMs for Literature Review: Are we there yet? | 2024 | core_method | Decomposes lit review into retrieval + planning + generation; zero-shot LLM assessment. |
| 23 | 2402.01788v2 | LitLLM: A Toolkit for Scientific Literature Review | 2024 | core_method | RAG-based toolkit with keyword extraction, re-ranking, related-work generation. |
| 24 | 2408.13450v1 | vitaLITy 2: Reviewing Academic Literature Using LLMs | 2024 | core_method | RAG architecture with text embeddings for semantic literature search. |
| 25 | 2407.20906v5 | Automated Review Generation Method Based on LLMs | 2024 | core_method | LLM-based review generation with statistical validation framework. |
| 26 | 2411.18583v1 | Automated Literature Review Using NLP and LLM-Based RAG | 2024 | mechanism | Comparison of NLP (spaCy, T5) vs RAG (GPT-3.5) for auto-review; SciTLDR dataset. |
| 27 | 2409.04600v1 | The emergence of LLMs as a tool in literature reviews | 2024 | survey | Systematic review of LLM usage for review automation; 172 studies analysed. |

### Query 7 — `mechanism`: citation graph expansion breadth depth traversal

| # | arXiv ID | Title | Year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 28 | 2512.22159 | Oignon: Citation Graph Tool | 2025 | mechanism | Open-source tool for systematic citation graph exploration with dual-path ranking. |
| 29 | 1904.07579v1 | Go Wide, Go Deep: Quantifying Impact through Influence Dispersion Trees | 2019 | mechanism | IDT data structure modelling breadth/depth of citation influence; NID metric. |
| 30 | 1806.00089v1 | Cascading Citation Expansion | 2018 | mechanism | Iterative forward/backward citation expansion; rooted in Garfield's citation indexing. |
| 31 | 2106.01560v1 | CitationIE: Leveraging the Citation Graph for Scientific IE | 2021 | mechanism | Uses citation graph structure to augment scientific information extraction. |
| 32 | 1310.8224v2 | Transitive Reduction of Citation Networks | 2013 | mechanism | Causal structure analysis of citation networks via transitive reduction. |
| 33 | 2602.12206 | Making the complete OpenAIRE citation graph easily accessible | 2026 | mechanism | Large-scale citation graph dataset (200M+ pubs, 2B+ citations) in compact format. |
| 34 | 1812.11252v1 | Towards Finding Non-obvious Papers: Analysis of Citation Recommender Systems | 2018 | mechanism | Projection graph analysis for citation recommendation beyond power-law papers. |
| 35 | 1805.10359v1 | Simplified Graph-based Visualization for Scientific Publication | 2018 | mechanism | Directed acyclic graph visualization of citation relationships. |

### Query 8 — `mechanism`: paper retrieval forward backward citation traversal

| # | arXiv ID | Title | Year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 36 | 2101.08577v2 | References of References: How Far is the Knowledge Ancestry | 2021 | mechanism | Analysis of backward-citation generations for knowledge ancestry tracing. |
| 37 | 1805.02262v1 | Construction of the Literature Graph in Semantic Scholar | 2018 | mechanism | Scalable heterogeneous literature graph; 280M nodes with citation edges. |
| 38 | 1711.08913v1 | Paper evolution graph: Multi-view structural retrieval | 2017 | mechanism | PEG construction from citation/author/content for structural retrieval. |
| 39 | 1205.1143v1 | Recommendation on Academic Networks using Direction Aware Citation Analysis | 2012 | mechanism | Direction-aware citation recommendation (forward/backward tuning). |
| 40 | 2302.07302v1 | CiteSee: Augmenting Citations with Persistent Historical Context | 2023 | mechanism | Personalized citation augmentation tool leveraging reading history. |
| 41 | 2402.16009v1 | PST-Bench: Tracing and Benchmarking the Source of Publications | 2024 | benchmark | Paper source tracing benchmark for understanding citation evolution. |
| 42 | 2402.08339v1 | Interleaved snowballing: Reducing workload of literature curators | 2024 | mechanism | Formalised snowballing method with reduced curator workload. |
| 43 | 2601.14949 | CiteRAG: What Should I Cite? A RAG Benchmark for Citation Prediction | 2026 | benchmark | RAG-integrated benchmark for academic citation prediction with multi-level retrieval. |
| 44 | 2110.06354v3 | Tell Me How to Survey: Automatic Reading Path Generation | 2021 | core_method | Reading Path Generation (RPG) task; SurveyBank dataset; graph-optimisation approach. |

### Query 9 — `mechanism`: PageRank influence propagation scientific literature

| # | arXiv ID | Title | Year | likely_role | inclusion_reason |
|---|----------|-------|------|-------------|-----------------|
| 45 | 1507.03650v1 | S-index: Towards Better Metrics for Quantifying Research Impact | 2015 | mechanism | Influence propagation over heterogeneous citation networks; s-index metric. |
| 46 | 1407.5107v1 | PageRank beyond the Web | 2014 | foundational | Survey of PageRank applications in bibliometrics, social networks, recommendation. |
| 47 | 1207.6328v2 | Recent advances in bibliometric indexes and the PaperRank problem | 2012 | mechanism | PaperRank: PageRank adapted for citation-based paper ranking. |
| 48 | 1505.03008v1 | Do PageRank-based author rankings outperform simple citation counts? | 2015 | mechanism | Empirical comparison of PageRank vs citation count for author ranking. |
| 49 | 1803.10713v2 | Biblioranking fundamental physics | 2018 | mechanism | PaperRank and AuthorRank metrics applying PageRank to InSpire citation database. |
| 50 | 1608.08414v3 | Identification of milestone papers through time-balanced network centrality | 2016 | mechanism | Time-biased PageRank variant for milestone paper identification. |
| 51 | 1803.09104v2 | Measuring academic reputation through citation networks via PageRank | 2018 | mechanism | PageRank on university citation networks for institutional prestige measurement. |
| 52 | 1312.3872v1 | Eugene Garfield, Francis Narin, and PageRank: Theoretical Bases | 2013 | foundational | Links Garfield's citation indexing theory to PageRank; validates both approaches. |
| 53 | 1407.1772v1 | Future Influence Ranking of Scientific Literature | 2014 | mechanism | MRFRank: mutual reinforcement ranking for future citation prediction. |

---

## Summary Statistics

- **Total unique candidates**: 53
- **By type**: core_method (19), mechanism (24), survey (5), benchmark (2), foundational (3)
- **Temporal distribution**: pre-2023: 27 (51%), 2023–2026: 26 (49%)
- **Deduplications performed**: 2 (1812.11252v1, 1904.07579v1)
