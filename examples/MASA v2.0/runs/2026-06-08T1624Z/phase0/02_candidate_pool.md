# Candidate Pool — MASA Pipeline

**Generated**: 2026-06-08
**run_dir**: `.`
**Agent**: DiscoveryMerger
**Status**: merged

---

## Merge Provenance

| Source Scout | Source Candidates | Unique Contributions After Dedup |
|---|---|---|
| MethodScout | 53 | 53 |
| BenchmarkScout | 22 | 22 |
| SurveyScout | ~48 | 48 |
| FrontierScout | 31 | 31 |
| **Total before dedup** | **~154** | **154** |
| **Cross-scout duplicates removed** | | **24** |
| **Final unique pool** | | **130** |

---

## Deduplication Log

Cross-scout duplicates merged (arXiv base ID, keeping earliest or most complete version):

| arXiv Base ID | Title | Occurrences (scout → query) | Resolution |
|---|---|---|---|
| 2506.12689 | SciSage / SurveyScope | MethodScout Q1(#1); BenchmarkScout A8 | Merged — core_method + benchmark provenance |
| 2509.18661 | Agentic AutoSurvey | MethodScout Q1(#2); SurveyScout Q2(#1), Q3(#2) | Merged |
| 2510.07733 | SurveyG | MethodScout Q1(#3); SurveyScout Q1(#2) | Merged |
| 2510.26012 | AutoSurvey2 | MethodScout Q1(#6); BenchmarkScout B2 | Merged |
| 2503.04629 | SurveyForge | MethodScout Q1(#7); SurveyScout Q1(#7); BenchmarkScout A2 | Merged |
| 2508.14317 | SurveyGen-I | MethodScout Q1(#8); SurveyScout Q1(#6) | Merged |
| 2510.21900 | Deep Literature Survey Automation | MethodScout Q1(#9); SurveyScout Q1(#8), Q4(#8) | Merged |
| 2508.11310 | SGSimEval | MethodScout Q1(#5); SurveyScout Q1(#9); BenchmarkScout A5 | Merged |
| 2408.15371 | Temporal GNN Paper Recommendation | MethodScout Q3(#11); FrontierScout Q21(#8) | Merged |
| 2201.01188 | GNN bibliometrics overview | MethodScout Q3(#14); FrontierScout Q21(#1) | Merged |
| 2305.01572 | H2CGL | MethodScout Q3(#17); FrontierScout Q21(#3) | Merged |
| 2104.02562 | Structured Citation Trend Prediction | MethodScout Q3(#15); FrontierScout Q21(#6) | Merged |
| 2510.11409 | LLM for SLR Corpus Filtration | MethodScout Q5(#21); SurveyScout Q4(#5) | Merged |
| 2410.15978 | PROMPTHEUS | MethodScout Q5(#20); SurveyScout Q4(#6) | Merged |
| 2110.06354 | SurveyBank / Reading Path | MethodScout Q8(#44); BenchmarkScout A11 | Merged |
| 2106.01560 | CitationIE | MethodScout Q7(#31); SurveyScout Q5(#1) | Merged |
| 1805.02262 | Literature Graph (Semantic Scholar) | MethodScout Q8(#37); SurveyScout Q5(#2) | Merged |
| 1805.10359 | Graph-based Visualization | MethodScout Q7(#35); SurveyScout Q5(#7) | Merged |
| 2512.22159 | Oignon Citation Graph Tool | MethodScout Q7(#28); SurveyScout Q5(#10) | Merged |
| 2602.11238 | SurveyLens | SurveyScout Q1(#4); BenchmarkScout A6 | Merged |
| 2508.17647 | SurveyGen | SurveyScout Q1(#5); BenchmarkScout B1 | Merged |
| 2502.14776 | SurveyX | SurveyScout Q1(#3); BenchmarkScout B3 | Merged |
| 2504.18765 | Vision for Auto Research | SurveyScout Q3(#4); FrontierScout Q20(#1) | Merged |
| 2510.20844 | AutoResearcher | SurveyScout Q3(#6); FrontierScout Q20(#9) | Merged |

**Total cross-scout duplicates collapsed**: 24

**Within-scout duplicates already removed by scouts**: ~7 (reported in individual scout artifacts)

---

## Candidate Pool (130 unique papers)

### A. Core Method Papers — Survey Generation Systems

Papers describing automated survey generation pipelines, multi-agent frameworks, or systematic review automation tools.

| # | arXiv ID | Title | Year | likely_role | Source Agent(s) | Source Query(ies) |
|---|---|---|---|---|---|---|
| 1 | 2506.12689v2 | SciSage: A Multi-Agent Framework for High-Quality Scientific Survey Generation | 2025 | core_method | MethodScout; BenchmarkScout | Q1; A8 |
| 2 | 2509.18661v1 | Agentic AutoSurvey: Let LLMs Survey LLMs | 2025 | core_method | MethodScout; SurveyScout | Q1; Q2, Q3 |
| 3 | 2510.07733 | SurveyG: A Multi-Agent LLM Framework with Hierarchical Citation Graph | 2025 | core_method | MethodScout; SurveyScout | Q1; Q1 |
| 4 | 2510.10890 | LLM×MapReduce-V3: Enabling Interactive In-Depth Survey Generation | 2025 | core_method | MethodScout | Q1 |
| 5 | 2508.11310v1 | SGSimEval: A Comprehensive Multifaceted Benchmark for ASG Systems | 2025 | benchmark | MethodScout; SurveyScout; BenchmarkScout | Q1; Q1; A5 |
| 6 | 2510.26012 | AutoSurvey2: Empowering Researchers with Next Level Automated Surveys | 2025 | core_method | MethodScout; BenchmarkScout | Q1; B2 |
| 7 | 2503.04629v1 | SurveyForge: On the Outline Heuristics, Memory-Driven Generation | 2025 | core_method | MethodScout; SurveyScout; BenchmarkScout | Q1; Q1; A2 |
| 8 | 2508.14317v1 | SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans | 2025 | core_method | MethodScout; SurveyScout | Q1; Q1 |
| 9 | 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow | 2025 | core_method | MethodScout; SurveyScout | Q1; Q1, Q4 |
| 10 | 2504.08762v1 | InteractiveSurvey: An LLM-based Personalized and Interactive Survey System | 2025 | core_method | MethodScout | Q1 |
| 11 | 2406.10252v2 | AutoSurvey: Large Language Models Can Automatically Write Surveys | 2024 | core_method | SurveyScout | Q1 |
| 12 | 2502.14776v2 | SurveyX: Academic Survey Automation via Large Language Models | 2025 | core_method | SurveyScout; BenchmarkScout | Q1; B3 |
| 13 | 2508.17647v1 | SurveyGen: Quality-Aware Scientific Survey Generation with LLMs | 2025 | core_method | SurveyScout; BenchmarkScout | Q1; B1 |
| 14 | 2510.05138 | LiRA: A Multi-Agent Framework for Reliable and Readable Literature Review Generation | 2025 | core_method | SurveyScout | Q3 |
| 15 | 2508.04306v1 | Multi-Agent Taskforce Collaboration: Self-Correction of Compounding Errors | 2025 | core_method | SurveyScout | Q3 |
| 16 | 2411.06159v3 | Mixture of Knowledge Minigraph Agents for Literature Review Generation | 2024 | core_method | SurveyScout | Q3 |
| 17 | 2504.14822v2 | Completing A Systematic Review in Hours instead of Months with Interactive AI Agents | 2025 | core_method | SurveyScout | Q3 |
| 18 | 2404.17017v1 | AutoGenesisAgent: Self-Generating Multi-Agent Systems for Complex Tasks | 2024 | core_method | SurveyScout | Q3 |
| 19 | 2403.08399v1 | System for systematic literature review using multiple AI agents | 2024 | core_method | SurveyScout | Q4 |
| 20 | 2410.15978v2 | PROMPTHEUS: A Human-Centered Pipeline to Streamline SLRs with LLMs | 2024 | core_method | MethodScout; SurveyScout | Q5; Q4 |
| 21 | 2412.15249v2 | LitLLMs, LLMs for Literature Review: Are we there yet? | 2024 | core_method | MethodScout | Q5 |
| 22 | 2402.01788v2 | LitLLM: A Toolkit for Scientific Literature Review | 2024 | core_method | MethodScout | Q5 |
| 23 | 2408.13450v1 | vitaLITy 2: Reviewing Academic Literature Using LLMs | 2024 | core_method | MethodScout | Q5 |
| 24 | 2407.20906v5 | Automated Review Generation Method Based on LLMs | 2024 | core_method | MethodScout | Q5 |
| 25 | 2510.26750 | ProfOlaf: Semi-Automated Tool for Systematic Literature Reviews | 2025 | core_method | SurveyScout | Q4 |
| 26 | 2412.08578v1 | ML Information Retrieval and Summarisation to Support Systematic Review | 2024 | core_method | SurveyScout | Q4 |
| 27 | 2407.18657v1 | SWARM-SLR: Streamlined Workflow Automation for Machine-actionable SLRs | 2024 | core_method | SurveyScout | Q4 |
| 28 | 2510.20844 | AutoResearcher: Automating Knowledge-Grounded Research Ideation | 2025 | core_method | SurveyScout; FrontierScout | Q3; Q20 |
| 29 | 2504.18765v3 | A Vision for Auto Research with LLM Agents | 2025 | core_method | SurveyScout; FrontierScout | Q3; Q20 |
| 30 | 2110.06354v3 | Tell Me How to Survey: Automatic Reading Path Generation (SurveyBank) | 2021 | core_method | MethodScout; BenchmarkScout | Q8; A11 |
| 31 | 2605.20025 | AutoResearchClaw: Self-Reinforcing Autonomous Research | 2026 | core_method | FrontierScout | Q20 |
| 32 | 2510.15624 | Build Your Personalized Research Group: A Multiagent Framework | 2025 | core_method | FrontierScout | Q20 |
| 33 | 2507.07257v2 | Open Source Planning & Control System for Autonomous Scientific Discovery | 2025 | core_method | FrontierScout | Q20 |
| 34 | 2408.06292v3 | The AI Scientist: Towards Fully Automated Open-Ended Scientific Discovery | 2024 | core_method | FrontierScout | Q20 |
| 35 | 2510.26144 | The FM Agent | 2025 | core_method | FrontierScout | Q20 |
| 36 | 2504.03424v1 | The AI Cosmologist I: An Agentic System for Automated Data Analysis | 2025 | core_method | FrontierScout | Q20 |
| 37 | 2602.07040 | Aster: Autonomous Scientific Discovery over 20x Faster | 2026 | core_method | FrontierScout | Q20 |
| 38 | 2510.26887 | The Denario project: Deep knowledge AI agents for scientific discovery | 2025 | core_method | FrontierScout | Q20 |
| 39 | 2509.16599v2 | CASMA: Computational-Assisted Systematic Review and Meta-Analysis | 2025 | core_method | FrontierScout | Q25 |
| 40 | 1908.06676v1 | Reducing the Effort for Systematic Reviews in SE (EDAM) | 2019 | core_method | FrontierScout | Q25 |
| 41 | 2510.11409 | Leveraging LLMs for Semi-Automatic Corpus Filtration in Systematic Reviews | 2025 | mechanism | MethodScout; SurveyScout | Q5; Q4 |

### B. Mechanism Papers — Citation Graph, Retrieval, and Expansion Methods

Papers describing citation graph traversal, PageRank/influence propagation, forward/backward citation expansion, and graph-based retrieval mechanisms.

| # | arXiv ID | Title | Year | likely_role | Source Agent(s) | Source Query(ies) |
|---|---|---|---|---|---|---|
| 42 | 2408.15371v1 | Temporal GNN-Powered Paper Recommendation on Dynamic Citation Networks | 2024 | mechanism | MethodScout; FrontierScout | Q3; Q21 |
| 43 | 2305.01572v3 | H2CGL: Modeling Dynamics of Citation Network for Impact Prediction | 2023 | mechanism | MethodScout; FrontierScout | Q3; Q21 |
| 44 | 2104.02562v1 | Structured Citation Trend Prediction Using GNNs | 2021 | mechanism | MethodScout; FrontierScout | Q3; Q21 |
| 45 | 2512.22159 | Oignon: Citation Graph Tool | 2025 | mechanism | MethodScout; SurveyScout | Q7; Q5 |
| 46 | 1904.07579v1 | Go Wide, Go Deep: Quantifying Impact through Influence Dispersion Trees | 2019 | mechanism | MethodScout | Q7 |
| 47 | 1806.00089v1 | Cascading Citation Expansion | 2018 | mechanism | MethodScout | Q7 |
| 48 | 2106.01560v1 | CitationIE: Leveraging the Citation Graph for Scientific IE | 2021 | mechanism | MethodScout; SurveyScout | Q7; Q5 |
| 49 | 1310.8224v2 | Transitive Reduction of Citation Networks | 2013 | mechanism | MethodScout | Q7 |
| 50 | 2602.12206 | Making the complete OpenAIRE citation graph easily accessible | 2026 | mechanism | MethodScout | Q7 |
| 51 | 1812.11252v1 | Towards Finding Non-obvious Papers: Analysis of Citation Recommender Systems | 2018 | mechanism | MethodScout | Q7 |
| 52 | 1805.10359v1 | Simplified Graph-based Visualization for Scientific Publication | 2018 | mechanism | MethodScout; SurveyScout | Q7; Q5 |
| 53 | 2101.08577v2 | References of References: How Far is the Knowledge Ancestry | 2021 | mechanism | MethodScout | Q8 |
| 54 | 1805.02262v1 | Construction of the Literature Graph in Semantic Scholar | 2018 | mechanism | MethodScout; SurveyScout | Q8; Q5 |
| 55 | 1711.08913v1 | Paper evolution graph: Multi-view structural retrieval | 2017 | mechanism | MethodScout | Q8 |
| 56 | 1205.1143v1 | Recommendation on Academic Networks using Direction Aware Citation Analysis | 2012 | mechanism | MethodScout | Q8 |
| 57 | 2302.07302v1 | CiteSee: Augmenting Citations with Persistent Historical Context | 2023 | mechanism | MethodScout | Q8 |
| 58 | 2402.08339v1 | Interleaved snowballing: Reducing workload of literature curators | 2024 | mechanism | MethodScout | Q8 |
| 59 | 1507.03650v1 | S-index: Towards Better Metrics for Quantifying Research Impact | 2015 | mechanism | MethodScout | Q9 |
| 60 | 1207.6328v2 | Recent advances in bibliometric indexes and the PaperRank problem | 2012 | mechanism | MethodScout | Q9 |
| 61 | 1505.03008v1 | Do PageRank-based author rankings outperform simple citation counts? | 2015 | mechanism | MethodScout | Q9 |
| 62 | 1803.10713v2 | Biblioranking fundamental physics | 2018 | mechanism | MethodScout | Q9 |
| 63 | 1608.08414v3 | Identification of milestone papers through time-balanced network centrality | 2016 | mechanism | MethodScout | Q9 |
| 64 | 1803.09104v2 | Measuring academic reputation through citation networks via PageRank | 2018 | mechanism | MethodScout | Q9 |
| 65 | 1407.1772v1 | Future Influence Ranking of Scientific Literature | 2014 | mechanism | MethodScout | Q9 |
| 66 | 2104.04939v1 | A GCN-based Framework for Estimating Future Citations Count | 2021 | mechanism | FrontierScout | Q21 |
| 67 | 2210.07343v1 | Scientific Impact of Graph-Based Approaches in Deep Learning | 2022 | mechanism | FrontierScout | Q21 |
| 68 | 2003.12042v1 | A Heterogeneous Dynamical GNN Approach to Quantify Scientific Impact | 2020 | mechanism | FrontierScout | Q21 |
| 69 | 2012.05742v2 | Longitudinal Citation Prediction using Temporal GNNs | 2020 | mechanism | FrontierScout | Q21 |
| 70 | 2106.12340v1 | GraphConfRec: A GNN-Based Conference Recommender System | 2021 | mechanism | FrontierScout | Q21 |
| 71 | 1903.06464v1 | A Context-Aware Citation Recommendation Model with BERT and GCNs | 2019 | mechanism | FrontierScout | Q21 |
| 72 | 2510.26354 | On the Role of Context for Discourse Relation Classification in Scientific Writing | 2025 | mechanism | FrontierScout | Q27 |
| 73 | 1806.03653v1 | SciDTB: Discourse Dependency TreeBank for Scientific Abstracts | 2018 | mechanism | FrontierScout | Q27 |
| 74 | 1909.04758v3 | Scientific Discourse Tagging for Evidence Extraction | 2019 | mechanism | FrontierScout | Q27 |
| 75 | 2505.23252v2 | Automatic Construction of Multiple Classification Dimensions for Scientific Papers | 2025 | mechanism | FrontierScout | Q27 |
| 76 | 2411.18583v1 | Automated Literature Review Using NLP and LLM-Based RAG | 2024 | mechanism | MethodScout | Q5 |

### C. Benchmark & Evaluation Papers

Papers that benchmark, evaluate, or provide evaluation frameworks for automated survey generation.

| # | arXiv ID | Title | Year | likely_role | Source Agent(s) | Source Query(ies) |
|---|---|---|---|---|---|---|
| 77 | 2510.03120 | SurveyBench — quiz-driven evaluation; 11,343 arXiv topics + 4,947 surveys | 2025 | benchmark | BenchmarkScout | A1 |
| 78 | 2512.02763 | SurveyEval — 7-subject evaluation; overall quality, outline coherence, reference accuracy | 2025 | benchmark | BenchmarkScout | A3 |
| 79 | 2508.15658 | SurGE (Survey Generation Eval) — 1M-paper corpus; 4-dimension eval | 2025 | benchmark | BenchmarkScout | A4 |
| 80 | 2602.11238 | SurveyLens — discipline-aware; 1,000 human surveys across 10 disciplines | 2026 | benchmark | SurveyScout; BenchmarkScout | Q1; A6 |
| 81 | 2601.15307 | DeepSurvey-Bench — academic value evaluation; info value, scholarly communication | 2026 | benchmark | BenchmarkScout | A7 |
| 82 | 2406.10291 | ResearchArena — 3-stage eval (discovery, selection, organization); 12M paper offline env | 2024 | benchmark | BenchmarkScout | A9 |
| 83 | 2308.10410 | Wikipedia-style Survey Generation — 99 NLP topics; GPT vs human survey evaluation | 2023 | benchmark | BenchmarkScout | A10 |
| 84 | 2402.16009v1 | PST-Bench: Tracing and Benchmarking the Source of Publications | 2024 | benchmark | MethodScout | Q8 |
| 85 | 2601.14949 | CiteRAG: What Should I Cite? A RAG Benchmark for Citation Prediction | 2026 | benchmark | MethodScout | Q8 |
| 86 | 2409.14913v2 | Towards a Realistic Long-Term Benchmark for Open-Web Research Agents | 2024 | benchmark | SurveyScout | Q2 |

### D. Evaluation Metrics

Foundational and specialised metrics for evaluating survey quality, factuality, and citation accuracy.

| # | arXiv ID | Title | Year | likely_role | Source Agent(s) | Source Query(ies) |
|---|---|---|---|---|---|---|
| 87 | 1904.09675 | BERTScore — BERT-based text generation similarity metric | 2019 | metric | BenchmarkScout | C1 |
| 88 | 2305.14251 | FActScore — atomic fact precision evaluation for long-form generation | 2023 | metric | BenchmarkScout | C2 |
| 89 | 2103.12693 | QuestEval — QA-based factuality evaluation without references | 2021 | metric | BenchmarkScout | C3 |
| 90 | 2309.12455 | LongDocFACTScore — long-document factuality evaluation framework | 2023 | metric | BenchmarkScout | C4 |
| 91 | 2411.16638 | Do Automatic Factuality Metrics Measure Factuality? — critical evaluation | 2024 | critique | BenchmarkScout | C5 |
| 92 | 2010.12495 | Understanding Summarization Evaluation Metrics — ROUGE/BERTScore analysis | 2020 | critique | BenchmarkScout | C6 |
| 93 | 2405.05583v2 | OpenFactCheck: Building, Benchmarking Customized Fact-Checking | 2024 | metric | FrontierScout | Q22 |
| 94 | 2509.21557v1 | Generation-Time vs. Post-hoc Citation: A Holistic Evaluation of LLM Attribution | 2025 | metric | FrontierScout | Q22 |
| 95 | 2510.17853 | CiteGuard: Faithful Citation Attribution for LLMs | 2025 | metric | FrontierScout | Q22 |
| 96 | 2510.12839 | FaStFACT: Faster, Stronger Long-Form Factuality Evaluations | 2025 | metric | FrontierScout | Q22 |
| 97 | 2305.14627v2 | Enabling LLMs to Generate Text with Citations (ALCE benchmark) | 2023 | metric | FrontierScout | Q22 |
| 98 | 2407.12861v2 | CiteME: Can Language Models Accurately Cite Scientific Claims? | 2024 | metric | FrontierScout | Q22 |
| 99 | 2411.02448v3 | Rate, Explain and Cite (REC): Enhanced Explanation and Attribution | 2024 | metric | FrontierScout | Q22 |
| 100 | 2408.11832v2 | OpenFactCheck: A Unified Framework for Factuality Evaluation | 2024 | metric | FrontierScout | Q22 |
| 101 | 2403.18802v4 | Long-form factuality in large language models (SAFE) | 2024 | metric | FrontierScout | Q22 |

### E. Human Evaluation Methodology

| # | arXiv ID | Title | Year | likely_role | Source Agent(s) | Source Query(ies) |
|---|---|---|---|---|---|---|
| 102 | 2108.00308 | Human Evaluation of Creative NLG — survey of human eval practices | 2021 | methodology | BenchmarkScout | D1 |
| 103 | 2109.06835 | Perils of Mechanical Turk for Text Eval — crowdsourcing critique | 2021 | methodology | BenchmarkScout | D2 |

### F. Survey / Review / Taxonomy / Overview Papers (Citation Seeds)

These are surveys, reviews, taxonomies, and vision papers that serve as citation seeds for downstream survey generation. They are not primary method/system papers.

| # | arXiv ID | Title | Year | likely_role | Source Agent(s) | Source Query(ies) |
|---|---|---|---|---|---|---|
| 104 | 2412.19419v1 | Introduction to Graph Neural Networks: A Starting Point | 2024 | survey | MethodScout | Q3 |
| 105 | 2012.08752v4 | Graph Neural Networks: Taxonomy, Advances and Trends | 2020 | survey | MethodScout | Q3 |
| 106 | 2201.01188v1 | Graph Neural Networks: a bibliometrics overview | 2022 | survey | MethodScout; FrontierScout | Q3; Q21 |
| 107 | 1901.00596v4 | A Comprehensive Survey on Graph Neural Networks | 2019 | survey | MethodScout | Q3 |
| 108 | 1812.08434v6 | Graph Neural Networks: A Review of Methods and Applications | 2018 | survey | MethodScout | Q3 |
| 109 | 2409.04600v1 | The emergence of LLMs as a tool in literature reviews | 2024 | survey | MethodScout | Q5 |
| 110 | 2402.08565v2 | Artificial Intelligence for Literature Reviews: Opportunities and Challenges | 2024 | survey | SurveyScout | Q4 |
| 111 | 2401.10917v1 | Artificial intelligence to automate the systematic review of scientific literature | 2024 | survey | SurveyScout | Q4 |
| 112 | 2211.15397v2 | Automating Systematic Literature Reviews with NLP and Text Mining | 2022 | survey | SurveyScout | Q4 |
| 113 | 2503.21460v1 | Large Language Model Agent: A Survey on Methodology, Applications and Challenges | 2025 | survey | SurveyScout | Q2 |
| 114 | 2601.01743 | AI Agent Systems: Architectures, Applications, and Evaluation | 2026 | survey | SurveyScout | Q2 |
| 115 | 2508.05668v3 | A Survey of LLM-based Deep Search Agents: Paradigm, Optimization, Evaluation | 2025 | survey | SurveyScout | Q2 |
| 116 | 2406.05804v6 | A Review of Prominent Paradigms for LLM-Based Agents | 2024 | review | SurveyScout | Q2 |
| 117 | 2308.11432v7 | A Survey on Large Language Model based Autonomous Agents | 2023 | survey | SurveyScout | Q2 |
| 118 | 2508.17692v1 | LLM-based Agentic Reasoning Frameworks: A Survey from Methods to Scenarios | 2025 | survey | SurveyScout | Q2 |
| 119 | 2309.07864v3 | The Rise and Potential of Large Language Model Based Agents: A Survey | 2023 | survey | SurveyScout | Q2 |
| 120 | 2601.12560 | Agentic AI: Architectures, Taxonomies, and Evaluation of LLM Agents | 2026 | taxonomy | SurveyScout | Q2 |
| 121 | 2604.18133 | Multi-Agent Systems: From Classical Paradigms to LFM-Enabled Futures | 2026 | survey | SurveyScout | Q3 |
| 122 | 1710.03094v1 | Characterizing in-text citations in scientific articles: A large-scale analysis | 2017 | analysis | SurveyScout | Q5 |
| 123 | 2509.04190v1 | The changing role of cited papers over time | 2025 | analysis | SurveyScout | Q5 |

### G. Foundational / Theoretical Papers

Classic or foundational works providing theoretical underpinnings for citation analysis, PageRank, or graph neural networks.

| # | arXiv ID | Title | Year | likely_role | Source Agent(s) | Source Query(ies) |
|---|---|---|---|---|---|---|
| 124 | 1810.00826v3 | How Powerful are Graph Neural Networks? | 2018 | foundational | MethodScout | Q3 |
| 125 | 1407.5107v1 | PageRank beyond the Web | 2014 | foundational | MethodScout | Q9 |
| 126 | 1312.3872v1 | Eugene Garfield, Francis Narin, and PageRank: Theoretical Bases | 2013 | foundational | MethodScout | Q9 |

### H. Cross-Domain / Inspirational Papers

Papers from adjacent domains (information foraging, systematic review methodology, discourse structure) that may inform survey generation design.

| # | arXiv ID | Title | Year | likely_role | Source Agent(s) | Source Query(ies) |
|---|---|---|---|---|---|---|
| 127 | 2006.05542v1 | Guidelines for Search Strategy to Update Systematic Literature Reviews in SE | 2020 | citation_seed | FrontierScout | Q25 |
| 128 | 2001.08988v1 | Towards a Framework for Methodology Scoping Reviews | 2020 | citation_seed | FrontierScout | Q25 |
| 129 | 2112.09424v1 | Search Strategy Formulation for Systematic Reviews | 2021 | citation_seed | FrontierScout | Q25 |
| 130 | 2304.13556v1 | The Systematic Review-lution: A Manifesto for HCI | 2023 | citation_seed | FrontierScout | Q25 |
| 131 | 2306.17614v1 | Outcome-based Evaluation of Systematic Review Automation | 2023 | citation_seed | FrontierScout | Q25 |
| 132 | 1709.06758v4 | Shared Latent Space Matrix Factorisation for Review Updates | 2017 | citation_seed | FrontierScout | Q25 |
| 133 | 1606.08157v1 | Risk and Ambiguity in Information Seeking: Eye Gaze Patterns | 2016 | citation_seed | FrontierScout | Q26 |
| 134 | 1907.00488v1 | Topic Modeling the Reading and Writing Behavior of Information Foragers | 2019 | citation_seed | FrontierScout | Q26 |
| 135 | 2508.15043v1 | LitForager: Exploring Multimodal Literature Foraging Strategies | 2025 | citation_seed | FrontierScout | Q26 |
| 136 | 2408.02508v1 | PUREsuggest: Citation-based Literature Search and Visual Exploration | 2024 | citation_seed | FrontierScout | Q26 |
| 137 | 1703.03901v1 | Foraging patterns in online searches | 2017 | citation_seed | FrontierScout | Q26 |
| 138 | 1509.07175v5 | Exploration and Exploitation of Victorian Science in Darwin's Reading Notebooks | 2015 | citation_seed | FrontierScout | Q26 |
| 139 | 2108.13302v1 | A Theoretical Framework for Online Information Search | 2021 | citation_seed | FrontierScout | Q26 |
| 140 | 2304.00121v1 | Decoding the End-to-end Writing Trajectory in Scholarly Manuscripts (ManuScript) | 2023 | citation_seed | FrontierScout | Q27 |
| 141 | 2310.15077v1 | A Discourse Structure-Based Framework for Science Journalism | 2023 | citation_seed | FrontierScout | Q27 |
| 142 | 1903.04427v1 | The rhetorical structure of science? A multidisciplinary analysis | 2019 | citation_seed | FrontierScout | Q27 |
| 143 | 2301.10140v2 | The Semantic Scholar Open Data Platform | 2023 | citation_seed | SurveyScout | Q5 |
| 144 | 1404.5322v1 | CitNetExplorer: A new software tool for analyzing and visualizing citation networks | 2014 | citation_seed | SurveyScout | Q5 |
| 145 | 1902.05170v2 | GrapAL: Connecting the Dots in Scientific Literature | 2019 | citation_seed | SurveyScout | Q5 |
| 146 | 2110.06595v2 | Refcat: The Internet Archive Scholar Citation Graph | 2021 | citation_seed | SurveyScout | Q5 |

### I. Excluded / Boundary Papers (for traceability)

These were retrieved by scouts but marked as out-of-scope. They are listed here for traceability only and should not be considered active candidates.

| # | arXiv ID | Title | Year | likely_role | Source Agent(s) | Exclusion Reason |
|---|---|---|---|---|---|---|
| X1 | 1406.5572v1 | SurveyMan: Programming and Automatically Debugging Surveys | 2014 | excluded | SurveyScout | Human survey design — not automated survey generation |
| X2 | 2312.10256v2 | Multi-agent Reinforcement Learning: A Comprehensive Survey | 2023 | excluded | SurveyScout | Tangential — MARL, not literature review automation |
| X3–X12 | (10 papers) | Generic RAG pipeline papers (PipeRAG, AutoRAG, etc.) | — | excluded | FrontierScout | Q23 — generic RAG, not survey-specific |
| X13–X22 | (10 papers) | Generic summarization papers (pointer-generator, BERTSUM, etc.) | — | excluded | FrontierScout | Q24 — text summarization, not survey synthesis |

---

### J. Expansion-Discovered Papers (Citation Graph + Semantic Neighbors)

Papers discovered through citation graph expansion (`03b_citation_expansion.md`) and semantic neighbor expansion (`03c_semantic_expansion.md`). These supplement the scout-discovered pool above. Format: `arXiv ID — Title — Year — Role — Source — Expansion source`

**Citation Graph Expansion (14 new):**

- `1205.6373v1` — Publication Induced Research Analysis (PIRA) — PageRank on heterogeneous graphs — 2012 — mechanism — expansion_citation — Seed 5 backward ref (citation algorithm)
- `1710.01895v1` — Eugene Garfield's Scholarly Impact: A Scientometric Review — 2017 — foundational — expansion_citation — Seeds 5, 8 backward ref (Garfield legacy)
- `2210.03629v3` — ReAct: Synergizing Reasoning and Acting in Language Models — 2022 — mechanism — expansion_citation — Seed 7 backward ref (agent architecture)
- `2004.09741v1` — On the Performance of Hybrid Search Strategies for Systematic Literature Reviews — 2020 — mechanism — expansion_citation — Seeds 2, 8 backward ref (SLR methodology)
- `2307.02612v1` — Successful Combination of Database Search and Snowballing for Identification of Primary Studies — 2023 — mechanism — expansion_citation — Seeds 2, 8 backward ref (SLR methodology)
- `2010.04665v1` — Scaling Systematic Literature Reviews with Machine Learning Pipelines — 2020 — mechanism — expansion_citation — Seeds 2, 8 backward ref (ML for SLR)
- `2111.07533v4` — Automated scholarly paper review: Concepts, technologies, and challenges — 2021 — survey — expansion_citation — Seeds 1, 2 backward ref (AI-assisted review)
- `2407.14991v1` — Investigating the use of Snowballing on Gray Literature Reviews — 2024 — mechanism — expansion_citation — Seeds 2, 5 backward ref (snowballing extension)
- `2305.15186v1` — SciReviewGen: A Large-scale Dataset for Automatic Literature Review Generation — 2023 — benchmark — expansion_citation — Seeds 1, 3, 4 backward ref (dataset precursor)
- `2605.16475` — Generative Artificial Intelligence for Literature Reviews — 2026 — survey — expansion_citation — Seeds 1, 2 forward ref (frontier survey)
- `2305.08281v2` — FactKB: Generalizable Factuality Evaluation using Language Models — 2023 — metric — expansion_citation — Seeds 3, 6 backward ref (factuality metric)
- `2310.04406v3` — Language Agent Tree Search Unifies Reasoning Acting and Planning in Language Models — 2023 — mechanism — expansion_citation — Seed 7 backward ref (agent reasoning)
- `2504.19678v1` — From LLM Reasoning to Autonomous AI Agents: A Comprehensive Review — 2025 — survey — expansion_citation — Seed 7 forward ref (agent survey follow-up)
- `2409.12177v1` — LitFM: A Retrieval Augmented Structure-aware Foundation Model For Citation Graphs — 2024 — mechanism — expansion_citation — Seeds 5, 7 forward ref (citation graph FM)

**Semantic Neighbor Expansion (54 new):**

**Outline Planning & Structured Document Generation:**
- `1905.10039v1` — Outline Generation: Understanding the Inherent Content Structure (HiStGen) — 2019 — mechanism — expansion_semantic — Q2, Q3 outline generation
- `2509.19370v1` — Meow: End-to-End Outline Writing for Automatic Academic Survey — 2025 — core_method — expansion_semantic — Q2 direct survey outline method
- `2410.06203v1` — Integrating Planning into Single-Turn Long-Form Text Generation — 2024 — mechanism — expansion_semantic — Q2, Q3 planning-based generation
- `2408.07884v1` — Instruct Large Language Models to Generate Scientific Literature Survey Step by Step — 2024 — core_method — expansion_semantic — Q2 step-by-step survey gen
- `2302.04580v1` — BigSurvey: Generating a Structured Summary of Numerous Academic Papers (CAST) — 2023 — core_method — expansion_semantic — Q2 structured multi-doc summarization

**Hierarchical Long-Form Text Generation:**
- `2012.14136v1` — On Generating Extended Summaries of Long Documents (ExtendedSumm) — 2020 — mechanism — expansion_semantic — Q3 hierarchical structure
- `2202.13756v1` — Data-to-text Generation with Variational Sequential Planning — 2022 — mechanism — expansion_semantic — Q3 latent sequential planning
- `2010.07074v2` — Summarize, Outline, and Elaborate (SOE): Long-Text Generation via Hierarchical Supervision — 2020 — mechanism — expansion_semantic — Q3 pipelined generation
- `2410.06802v1` — Seg2Act: Global Context-aware Action Generation for Document Logical Structuring — 2024 — mechanism — expansion_semantic — Q3 document structuring
- `2310.09118v1` — DSG: An End-to-End Document Structure Generator — 2023 — mechanism — expansion_semantic — Q3 document structure parsing
- `2105.09297v1` — Extracting Variable-Depth Logical Document Hierarchy (HELD) — 2021 — mechanism — expansion_semantic — Q3 hierarchy extraction
- `1810.08802v1` — Hierarchical Text Generation using an Outline — 2018 — mechanism — expansion_semantic — Q3 early outline-guided generation
- `2408.05829v1` — HGEN: Supporting Software Maintenance with Dynamically Generated Document Hierarchies — 2024 — citation_seed — expansion_semantic — Q3 software docs, tangential

**Citation Intention Classification:**
- `1904.01608v2` — Structural Scaffolds for Citation Intent Classification (SciCite) — 2019 — mechanism — expansion_semantic — Q4 citation intent dataset
- `2304.12730v2` — CitePrompt: Using Prompts to Identify Citation Intent in Scientific Papers — 2023 — mechanism — expansion_semantic — Q4 prompt-based classification
- `2106.13275v1` — Multitask Learning for Citation Purpose Classification — 2021 — mechanism — expansion_semantic — Q4 multi-task citation purpose
- `2505.21162v1` — Leveraging GANs for Citation Intent Classification and Impact on Network Analysis — 2025 — mechanism — expansion_semantic — Q4 GAN-based classification
- `2005.06611v1` — ImpactCite: An XLNet-based Method for Citation Impact Analysis — 2020 — mechanism — expansion_semantic — Q4 citation sentiment/intent
- `1609.00435v1` — Citation Classification for Behavioral Analysis of a Scientific Field (NLP) — 2016 — mechanism — expansion_semantic — Q4 behavioral study of citations
- `2501.18292v1` — Citation Recommendation based on Argumentative Zoning of User Queries — 2025 — mechanism — expansion_semantic — Q4 citation recommendation + zoning
- `2407.13329v3` — CiteFusion: An Ensemble Framework for Citation Intent Classification — 2024 — mechanism — expansion_semantic — Q4 SOTA ensemble
- `2104.12869v1` — Semantic Analysis for Automated Evaluation of Potential Impact — 2021 — mechanism — expansion_semantic — Q4 citation impact prediction

**Long-Document / Survey Evaluation Metrics:**
- `2305.18201v1` — A Critical Evaluation of Evaluations for Long-form Question Answering — 2023 — critique — expansion_semantic — Q5 LFQA evaluation
- `2210.16732v1` — How Far are We from Robust Long Abstractive Summarization? — 2022 — critique — expansion_semantic — Q5 long summary evaluation
- `2406.19276v1` — VERISCORE: Evaluating Factuality of Verifiable Claims in Long-Form Text — 2024 — metric — expansion_semantic — Q5 factuality eval
- `2208.01030v1` — SMART: Sentences as Basic Units for Text Evaluation — 2022 — metric — expansion_semantic — Q5 sentence-level metric
- `2211.02580v1` — Evaluating and Improving Factuality in Multimodal Abstractive Summarization (CLIPBERTScore) — 2022 — metric — expansion_semantic — Q5 multimodal, tangential
- `2010.12834v2` — GO FIGURE: A Meta Evaluation of Factuality in Summarization — 2020 — critique — expansion_semantic — Q5 factuality meta-evaluation
- `2403.02270v3` — FENICE: Factuality Evaluation based on NLI and Claim Extraction — 2024 — metric — expansion_semantic — Q5 SOTA factuality metric

**Multi-Agent System Evaluation & Architecture:**
- `2506.15451v1` — AgentGroupChat-V2: Divide-and-Conquer for LLM-Based Multi-Agent Systems — 2025 — mechanism — expansion_semantic — Q6 divide-and-conquer MAS
- `2410.02189v2` — Agent-Oriented Planning in Multi-Agent Systems (AOP) — 2024 — mechanism — expansion_semantic — Q6 task decomposition + allocation
- `2510.04311v1` — On the Importance of Task Complexity in Evaluating LLM-Based Multi-Agent Systems — 2025 — critique — expansion_semantic — Q6 task complexity framework
- `2601.11903` — AEMA: Verifiable Evaluation Framework for Trustworthy Agentic LLM Systems — 2026 — benchmark — expansion_semantic — Q6 agent eval framework
- `2404.05569v3` — 360°REA: Reusable Experience Accumulation with 360° Assessment for MAS — 2024 — mechanism — expansion_semantic — Q6 multi-perspective assessment
- `2509.20175` — Federation of Agents: Semantics-Aware Communication Fabric for Large-Scale Agentic AI — 2025 — mechanism — expansion_semantic — Q6 semantic routing, task decomposition
- `2505.18286v1` — Single-agent or Multi-agent Systems? Why Not Both? — 2025 — critique — expansion_semantic — Q6 SAS vs MAS comparison

**Scientific Claim Verification:**
- `2602.07621` — SciClaimEval: Cross-modal Claim Verification in Scientific Papers — 2026 — benchmark — expansion_semantic — Q7 cross-modal claim verification
- `2110.13090v1` — SciClops: Detecting and Contextualizing Scientific Claims for Fact-Checking — 2021 — mechanism — expansion_semantic — Q7 claim extraction pipeline
- `2004.14974v6` — Fact or Fiction: Verifying Scientific Claims (SciFact) — 2020 — benchmark — expansion_semantic — Q7 foundational claim verification
- `2305.16859v1` — Scientific Fact-Checking: A Survey of Resources and Approaches — 2023 — survey — expansion_semantic — Q7 sci fact-checking survey
- `2604.17667` — Peerispect: Claim Verification in Scientific Peer Reviews — 2026 — mechanism — expansion_semantic — Q7 peer review claim verification
- `2204.12263v2` — Science Checker: Extractive-Boolean QA for Scientific Fact Checking — 2022 — mechanism — expansion_semantic — Q7 Boolean QA fact checking
- `2503.21717v1` — CLAIMCHECK: How Grounded are LLM Critiques of Scientific Papers? — 2025 — benchmark — expansion_semantic — Q7 claim grounding benchmark
- `2110.15116v1` — Abstract, Rationale, Stance: Joint Model for Scientific Claim Verification — 2021 — mechanism — expansion_semantic — Q7 joint claim verification
- `2605.27710` — DeepSciVerify: LLM-Driven Evidence Escalation for Claim-Citation Alignment — 2026 — mechanism — expansion_semantic — Q7 citation alignment
- `2508.11122v1` — +VeriRel: Verification Feedback to Enhance Document Retrieval for Scientific Fact Checking — 2025 — mechanism — expansion_semantic — Q7 verification-aware retrieval

**Knowledge Graph Construction from Scientific Literature:**
- `1808.09602v1` — Multi-Task Identification of Entities, Relations, and Coreference for Scientific KG (SciIE/SciERC) — 2018 — mechanism — expansion_semantic — Q8 foundational scientific IE
- `2509.07801v3` — SciNLP: Domain-Specific Benchmark for Full-Text Scientific Entity and Relation Extraction in NLP — 2025 — benchmark — expansion_semantic — Q8 full-text entity/relation benchmark
- `2106.01167v1` — End-to-End NLP Knowledge Graph Construction (SciNLP-KG) — 2021 — mechanism — expansion_semantic — Q8 ACL KG construction
- `2401.09839v1` — MatSciRE: Pointer Networks for Entity and Relation Extraction in Materials Science — 2024 — mechanism — expansion_semantic — Q8 materials science, tangential
- `2011.01103v1` — Generating Knowledge Graphs by Employing NLP/ML within the Scholarly Domain — 2020 — mechanism — expansion_semantic — Q8 general scholarly KG
- `2007.12731v1` — COVID-19 Knowledge Graph: Accelerating Information Retrieval and Discovery — 2020 — mechanism — expansion_semantic — Q8 heterogeneous KG
- `2109.10453v1` — Extracting Fine-Grained Knowledge Graphs of Scientific Claims (SciClaim) — 2021 — mechanism — expansion_semantic — Q8 fine-grained claim graphs
- `2410.21155v1` — SciER: Entity and Relation Extraction Dataset for Datasets, Methods, and Tasks — 2024 — benchmark — expansion_semantic — Q8 entity/relation dataset

---

### K. Canonical Papers from LLM-based Code Editing & Related Fields

Papers identified as canonical by the coverage auditor in the domains of LLM-based code editing, automated program repair, agentic software engineering, and related evaluation benchmarks. Added to ensure completeness of the coverage audit.

| # | arXiv ID | Title | Year | likely_role | Source |
|---|---|---|---|---|---|
| 147 | 2107.03374v2 | Evaluating Large Language Models Trained on Code (Codex) | 2021 | foundational | coverage_auditor [canonical] |
| 148 | 2204.05999v3 | InCoder: A Generative Model for Code Infilling and Synthesis | 2022 | foundational | coverage_auditor [canonical] |
| 149 | 2109.00859v1 | CodeT5: Identifier-aware Unified Pre-trained Encoder-Decoder Models for Code Understanding and Generation | 2021 | foundational | coverage_auditor [canonical] |
| 150 | 2305.07922v2 | CodeT5+: Open Code Large Language Models for Code Understanding and Generation | 2023 | foundational | coverage_auditor [canonical] |
| 151 | 2103.06333v2 | PLBART: Unified Pre-training for Program Understanding and Generation | 2021 | foundational | coverage_auditor [canonical] |
| 152 | 2305.06161v2 | StarCoder: may the source be with you! | 2023 | foundational | coverage_auditor [canonical] |
| 153 | 2308.12950v3 | Code Llama: Open Foundation Models for Code | 2023 | foundational | coverage_auditor [canonical] |
| 154 | 2401.14196v2 | DeepSeek-Coder: When the Large Language Model Meets Programming — The Rise of Code Intelligence | 2024 | foundational | coverage_auditor [canonical] |
| 155 | 2303.08774v6 | GPT-4 Technical Report | 2023 | foundational | coverage_auditor [canonical] |
| 156 | 2401.04088v1 | Mixtral of Experts | 2024 | foundational | coverage_auditor [canonical] |
| 157 | 2208.05446v2 | CoditT5: Pretraining for Source Code and Natural Language Editing | 2022 | mechanism | coverage_auditor [canonical] |
| 158 | 2305.18584v2 | Coeditor: Leveraging Contextual Changes for Multi-round Code Auto-editing | 2023 | mechanism | coverage_auditor [canonical] |
| 159 | 2405.15793v3 | SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering | 2024 | mechanism | coverage_auditor [canonical] |
| 160 | 2404.05427v3 | AutoCodeRover: Autonomous Program Improvement | 2024 | benchmark | coverage_auditor [canonical] |
| 161 | 2407.01489v2 | Agentless: Demystifying LLM-based Software Engineering Agents | 2024 | mechanism | coverage_auditor [canonical] |
| 162 | 2407.16741v3 | OpenHands: An Open Platform for AI Software Developers as Generalist Agents | 2024 | mechanism | coverage_auditor [canonical] |
| 163 | 2310.06770v3 | SWE-bench: Can Language Models Resolve Real-World GitHub Issues? | 2023 | benchmark | coverage_auditor [canonical] |
| 164 | 2007.12626v4 | SummEval: Re-evaluating Summarization Evaluation | 2020 | metric | coverage_auditor [canonical] |

---

## Summary Statistics

| Category | Count | Notes |
|---|---|---|---|
| **Total unique candidates** | **232** | 216 active + 16 boundary/excluded (listed for traceability) |
| **Active candidates** | **216** | 130 from scouts + 68 from expansion + 18 from coverage audit |
| | | |
| **By likely_role** | | |
| core_method | 46 | +5: Meow, Step-by-step survey, BigSurvey/CAST |
| mechanism | 74 | +4 from coverage audit: CoditT5, Coeditor, SWE-agent, Agentless |
| benchmark | 18 | +2 from coverage audit: AutoCodeRover, SWE-bench |
| metric | 21 | +1 from coverage audit: SummEval |
| critique | 6 | +4: LFQA eval, long summ robustness, task complexity, SAS vs MAS |
| survey | 17 | +4: GenAI for LR, LLM→Agent review, automated review, sci fact-checking |
| foundational | 14 | +10 from coverage audit: Codex, InCoder, CodeT5, CodeT5+, PLBART, StarCoder, Code Llama, DeepSeek-Coder, GPT-4 tech report, Mixtral |
| citation_seed | 12 | +1: HGEN (tangential, software docs) |
| methodology | 2 | (unchanged) |
| excluded | 16 | (unchanged) |
| | | |
| **Temporal distribution** | | |
| Pre-2023 | ~50 | ~22% |
| 2023 | ~21 | ~9% |
| 2024 | ~48 | ~21% |
| 2025 | ~58 | ~25% |
| 2026 | ~9 | ~4% |
| (No year meta) | ~24 | ~10% |
| | | |
| **Source distribution** | | |
| Scout contributions (dedup'd) | 130 | MethodScout 53, BenchmarkScout 22, SurveyScout 48, FrontierScout 31 (24 cross-dups merged) |
| Citation expansion added | 14 | From seed paper bibliography resolution |
| Semantic expansion added | 54 | From 8 gap-targeting queries |
| Coverage audit added | 18 | From canonical papers in LLM-based code editing and related fields |
| **Total after expansion** | **216** | **+66.2% expansion factor** |

## Risk Notes

1. **`schema/candidate_pool.md` not found.** The `schema` file at root is a 17-line SurveySpec contract, not a directory containing candidate_pool schema. The pool format is defined inline in this artifact.

2. **`schema/handoff.md` not found.** Handoff schema is provided inline in this response.

3. **Precision of cross-domain papers (Q25–Q27):** PRISMA results (Q25) are software-engineering biased. Information foraging (Q26) and discourse structure (Q27) papers are inspirational rather than directly applicable. They are retained as `citation_seed` papers rather than `core_method` or `mechanism`.

4. **GNN citation papers (Q21):** Many focus on citation-count prediction rather than citation graph traversal for survey expansion. Papers #42 (#2408.15371) and #69 (#2012.05742) are the most directly relevant to dynamic citation expansion for literature review.

5. **Boundary papers (Q23, Q24):** All 20 papers from generic RAG and text summarization queries are excluded per the FrontierScout's classification. Listed only for traceability.

6. **Version normalization:** Dedup was performed on arXiv base IDs (stripping version suffixes like `v1`, `v2`, `v3`). The version string from the most complete source was preserved in the final entry.

---

## Handoff — ExpansionMerger → next agent

| Field | Value |
|---|---|
| **run_dir** | `.` |
| **artifact** | `02_candidate_pool.md` |
| **status** | `extended` |
| **total_candidates** | 232 (216 active + 16 excluded) |
| **core_method** | 46 |
| **mechanism** | 74 |
| **benchmark** | 18 |
| **metric** | 21 |
| **critique** | 6 |
| **methodology** | 2 |
| **survey** | 17 |
| **foundational** | 14 |
| **citation_seed** | 12 |
| **excluded** | 16 |
| **expansion_seeds** | 8 |
| **citation_additions** | 14 |
| **semantic_additions** | 54 |
| **coverage_audit_additions** | 18 |
| **total_expansion_additions** | 86 |
| **expansion_factor** | +66.2% (130 → 216) |

**Provenance preserved**: Scout-discovered entries carry source_agent and source_query provenance. Expansion-discovered entries carry `expansion_citation` or `expansion_semantic` source tag with connected seed information.

**Supervisor Retrieval Round 2 additions:**

- arXiv:2605.07723 — LLM hallucinations in the wild: Large-scale evidence from non-existent citations — Zhao et al. — 2026 — Large-scale audit of 111M references finding ~147K hallucinated citations in 2025 — Source: supervisor_retrieval_R2
- arXiv:2604.22750 — How Do AI Agents Spend Your Money? Analyzing and Predicting Token Consumption in Agentic Coding Tasks — Bai et al. — 2026 — First systematic token consumption analysis for agentic tasks (1000x multiplier) — Source: supervisor_retrieval_R2
- arXiv:2605.14790 — Graphs of Research: Citation Evolution Graphs as Supervision for Research Idea Generation — Gao et al. — 2026 — Citation evolution DAG as supervision for LLM generation — Source: supervisor_retrieval_R2

**Top-10 most valuable candidates** (by downstream utility for survey generation):

1. **2506.12689v2** — SciSage (multi-agent survey framework + SurveyScope benchmark)
2. **2510.07733** — SurveyG (hierarchical citation graph + multi-agent generation)
3. **2406.10252v2** — AutoSurvey (LLMs can automatically write surveys)
4. **2502.14776v2** — SurveyX (academic survey automation via LLMs)
5. **2508.17647v1** — SurveyGen (quality-aware survey generation)
6. **2510.03120** — SurveyBench (quiz-driven benchmark, 11K+ topics)
7. **2509.19370v1** — Meow (end-to-end outline writing for academic surveys)
8. **1904.01608v2** — SciCite (citation intent classification — new capability)
9. **2406.19276v1** — VERISCORE (long-form factuality evaluation — new capability)
10. **2210.03629v3** — ReAct (agent reasoning framework — new capability)

**Risks**: (1) `schema/candidate_pool.md` and `schema/handoff.md` not found on disk; contract definitions are inline. (2) `run_dir` recovered as `.` from scout artifacts. (3) 16 boundary papers (generic RAG and summarization) are excluded but listed for traceability. (4) 68 expansion additions (+52%) is aggressive — downstream PaperFetcher should verify relevance before PDF download. (5) Semantic Q1 (KV cache) entirely drifted — 10 results excluded as LLM inference optimization, not survey generation. (6) **Coverage audit domain mismatch**: 18 canonical papers from LLM-based code editing and automated program repair were added per coverage audit requirements, but these are **outside the survey scope** (automated literature survey generation). Downstream agents should filter or deprioritize these during PaperFetcher and Profiler stages.
