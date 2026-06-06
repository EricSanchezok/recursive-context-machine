# Extended Frontier Candidates

**run_dir**: `.`
**Date**: 2026-06-06
**Assembled by**: ExtendedFrontierScout (MASA pipeline)
**Source artifact**: `01b_query_plan_extended.md` (15 extended queries)
**Parent pool**: `02_candidate_pool.md` (95 candidates)

---

## Overview

Extended frontier queries executed against arXiv search (topK=10 each, 150 total raw results). Below, results are grouped by extended query type, with key novel candidates highlighted — papers not already in the 95-paper pool.

---

## extended_method (3 queries)

### em-01: "automatic literature survey generation agent"

Widest net on core method category, capturing non-LLM and hybrid approaches.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2510.21900 | 2025 | Deep Literature Survey Automation with an Iterative Workflow (IterSurvey) | **NEW** — recurrent outline generation, Survey-Arena benchmark |
| 2 | 2110.06354v3 | 2021 | Tell Me How to Survey: Literature Review Made Simple with Automatic Reading Path Generation | **NEW** — Reading Path Generation task, SurveyBank dataset, graph-optimization approach |
| 3 | 2402.01788v2 | 2024 | LitLLM: A Toolkit for Scientific Literature Review | **NEW** — RAG-based toolkit with keyword extraction, reranking |
| 4 | 2603.14629 | 2026 | ResearchPilot: A Local-First Multi-Agent System | **NEW** — self-hostable, DSPy-based, open-source |
| 5 | 2201.01880v1 | 2022 | Automatic Related Work Generation: A Meta Study | **NEW** — survey of related work generation field |
| 6 | 2309.01684v1 | 2023 | CRUISE-Screening: Living Literature Reviews Toolbox | **NEW** — living review screening tool |
| 7 | 2306.03535v2 | 2023 | SciLit: Platform for Literature Discovery, Summarization and Citation Generation | Already in pool |
| 8 | 2412.15249v2 | 2024 | LitLLMs, LLMs for Literature Review: Are we there yet? | **NEW** — evaluation protocol for LLM lit review |
| 9 | 1705.05420v6 | 2017 | FAST²: an Intelligent Assistant for Finding Relevant Papers | **NEW** — early active learning for literature screening |
| 10 | 2010.04147v1 | 2020 | Automatic generation of reviews of scientific papers | **NEW** — cocitation graph + BERT extractive summarization on PubMed |

### em-02: "deep research agent paper retrieval summarization"

Captures broader "deep research" paradigm (OpenAI Deep Research, Gemini, Perplexity).

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2507.15245v1 | 2025 | SPAR: Scholar Paper Retrieval with LLM-based Agents | **NEW** — RefChain-based query decomposition, SPARBench |
| 2 | 2403.02574v1 | 2024 | ChatCite: LLM Agent with Human Workflow Guidance for Comparative Literature Summary | **NEW** — human-workflow-mimicking comparative summary agent |
| 3 | 2501.10120v2 | 2025 | PaSa: An LLM Agent for Comprehensive Academic Paper Search | **NEW** — RL-optimized paper search agent, AutoScholarQuery |
| 4 | 2512.06879 | 2025 | WisPaper: Your AI Scholar Search Engine | **NEW** — closed-loop discovery/management platform |
| 5 | 2407.18940v2 | 2024 | LitSearch: A Retrieval Benchmark for Scientific Literature Search | **NEW** — 597 realistic ML/NLP lit search queries |
| 6 | 1905.07870v4 | 2019 | PaperRobot: Incremental Draft Generation of Scientific Ideas | **NEW** — KG-based paper draft generation, Turing tests |
| 7 | 1908.11152v1 | 2019 | A Summarization System for Scientific Documents | Already in pool (or closely related) |
| 8 | 2306.11832v1 | 2023 | QuOTeS: Query-Oriented Technical Summarization | **NEW** — interactive query-focused summarization |
| 9 | 2603.14629 | 2026 | ResearchPilot | Duplicate from em-01 |
| 10 | 2306.03535v2 | 2023 | SciLit | Already in pool |

### em-03: "multi-agent collaboration scientific literature synthesis"

Captures general multi-agent patterns for scientific knowledge work.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2509.18661v1 | 2025 | Agentic AutoSurvey: Let LLMs Survey LLMs | **NEW** — 4-agent framework, 12-dim evaluation, beats AutoSurvey |
| 2 | 2508.04306v1 | 2025 | Multi-Agent Taskforce Collaboration: Self-Correction of Compounding Errors | **NEW** — MATC framework, error-mitigation focus |
| 3 | 2510.05138 | 2025 | LiRA: A Multi-Agent Framework for Reliable and Readable Literature Review Generation | **NEW** — multi-agent for readability + citation quality |
| 4 | 2510.20844 | 2025 | AutoResearcher: Automating Knowledge-Grounded Research Ideation | **NEW** — 4-stage multi-agent ideation pipeline |
| 5 | 2507.10522v1 | 2025 | DeepResearch^Eco: Recursive Agentic Workflow for Scientific QA in Ecology | **NEW** — recursive depth-breadth controlled exploration |
| 6 | 2411.06159v3 | 2024 | Mixture of Knowledge Minigraph Agents for Literature Review | **NEW** — CKMA framework with knowledge minigraphs |
| 7 | 2503.18102v1 | 2025 | AgentRxiv: Towards Collaborative Autonomous Research | **NEW** — preprint-server-based agent collaboration |
| 8 | 2404.07738v2 | 2024 | ResearchAgent: Iterative Research Idea Generation | **NEW** — LLM idea generation with reviewing agents |
| 9 | 2603.14629 | 2026 | ResearchPilot | Duplicate from em-01 |

---

## extended_mechanism (4 queries)

### emc-01: "citation graph traversal paper discovery retrieval"

Captures depth-first, random-walk, and hybrid traversal strategies.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2512.22159 | 2025 | Oignon: Citation Graph Tool | **NEW** — dual-path ranking with recency weighting, open-source |
| 2 | 2106.05633v1 | 2021 | Citation Recommendation via Knowledge Graphs | **NEW** — combines research KGs with citation networks |
| 3 | 1812.11252v1 | 2018 | Towards Finding Non-obvious Papers: Analysis of Citation Recommender Systems | **NEW** — projection graph analysis, non-obvious paper discovery |
| 4 | 2409.12177v1 | 2024 | LitFM: Structure-aware Foundation Model for Citation Graphs | Already in pool |
| 5 | 1711.08913v1 | 2017 | Paper Evolution Graph: Multi-view Structural Retrieval | **NEW** — evolution chains from citation networks |
| 6 | 2106.01560v1 | 2021 | CitationIE: Leveraging Citation Graph for Scientific IE | **NEW** — citation-aware information extraction |
| 7 | 2302.07302v1 | 2023 | CiteSee: Augmenting Citations with Personalized Context | **NEW** — personalized citation augmentation, HCI angle |
| 8 | 1805.02262v1 | 2018 | Construction of the Literature Graph in Semantic Scholar | Already in pool |
| 9 | 1812.03835v1 | 2018 | Graph Embedding for Citation Recommendation | **NEW** — task-specific neighborhood construction for citation embedding |
| 10 | 1205.1143v1 | 2012 | Direction Aware Citation Analysis | **NEW** — early direction-aware citation recommendation |

### emc-02: "knowledge graph survey literature review generation"

Fills the ORKG / knowledge-graph dimension gap.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2012.00456v1 | 2020 | Creating a Scholarly Knowledge Graph from Survey Article Tables | **NEW** — ORKG-based human-in-the-loop KG construction from survey tables |
| 2 | 2006.01747v1 | 2020 | Generate FAIR Literature Surveys with Scholarly Knowledge Graphs | Already in pool |
| 3 | 2208.02334v1 | 2022 | A Knowledge Graph-Based Method for Automating Systematic Literature Reviews | **NEW** — KG-driven semi-automated SLR |
| 4 | 2405.08351v1 | 2024 | KG-EmpiRE: Community-Maintainable KG for Sustainable Literature Review | **NEW** — ORKG-based living review in Requirements Engineering |
| 5 | 2002.00388v4 | 2020 | A Survey on Knowledge Graphs: Representation, Acquisition and Applications | Too broad (general KG survey) |
| 6 | 2210.00105v1 | 2022 | A Decade of KGs in NLP: A Survey | Too broad |
| 7 | 2003.02320v6 | 2020 | Knowledge Graphs (comprehensive textbook-style survey) | Too broad |

### emc-03: "snowballing citation expansion backward forward search"

Captures pre-2020 cocitation / snowballing methodology lineage.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2110.12490v3 | 2021 | Paperfetcher: Tool to Automate Handsearch for Systematic Reviews | **NEW** — automates handsearch + snowballing, open-source |
| 2 | 1806.00089v1 | 2018 | Cascading Citation Expansion | **NEW** — iterative citation expansion via Dimensions API |
| 3 | 2101.08577v2 | 2021 | References of References: How Far is the Knowledge Ancestry | **NEW** — backward citation generation analysis |
| 4 | 2402.08339v1 | 2024 | Interleaved Snowballing: Reducing Workload of Literature Curators | **NEW** — formal snowballing method, LitBall app |
| 5 | 2408.02508v1 | 2024 | PUREsuggest: Citation-based Literature Search and Visual Exploration | **NEW** — keyword-steered citation suggestions |
| 6 | 1707.02494v2 | 2017 | Analysis of Footnote Chasing and Citation Searching | **NEW** — user behavior study of Bates' search stratagems |
| 7 | 2207.03299v2 | 2022 | Academic Information Retrieval Using Citation Clusters | **NEW** — citation cluster evaluation for systematic reviews |

### emc-04: "iterative query decomposition refinement academic search"

Captures general query decomposition strategies supporting survey retrieval.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2507.15245v1 | 2025 | SPAR: Scholar Paper Retrieval with LLM-based Agents | Duplicate from em-02 |
| 2 | 2504.05309v1 | 2025 | IterQR: Iterative Framework for LLM-based Query Rewrite | **NEW** — iterative query rewrite with RAG + CoT (e-commerce domain) |
| 3 | 2204.00743v2 | 2022 | Entity-Centric Query Refinement | **NEW** — KG taxonomy-based query refinement |
| 4 | 2305.15053v1 | 2023 | Decomposing Complex Queries for Tip-of-the-tongue Retrieval | **NEW** — query decomposition + specialized sub-retrievers |
| 5 | 2004.02002v3 | 2020 | Talk to Papers: Neural QA for Academic Search | **NEW** — open-domain QA for academic search |
| 6 | 2001.10781v1 | 2020 | Aspect-based Academic Search using Domain-specific KB | **NEW** — aspect-aware retrieval using KB language models |
| 7 | 2311.11226v1 | 2023 | Interactive Query Generation Assistant using LLM-based Prompt Modification | **NEW** — HITL query generation |
| 8 | 2403.15667v1 | 2024 | QueryExplorer: Interactive Query Generation Assistant | **NEW** — interactive query reformulation interface |
| 9 | 2201.09974v1 | 2022 | Generating Clarifying Questions for Query Refinement in Code Search | Related but SE-focused |

---

## extended_problem (2 queries)

### epr-01: "citation hallucination attribution LLM generation"

Broad capture of citation hallucination / attribution landscape.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2403.17104v3 | 2024 | Attribute First, then Generate: Locally-attributable Grounded Text | **NEW** — select-then-generate paradigm for fine-grained attribution |
| 2 | 2408.04568v1 | 2024 | Learning Fine-Grained Grounded Citations (FRONT) | **NEW** — training framework for fine-grained citation |
| 3 | 2410.11217v1 | 2024 | On the Capacity of Citation Generation by LLMs | **NEW** — systematic analysis of LLM citation capability, Generate-then-Refine |
| 4 | 2405.02228v3 | 2024 | Attribution in Scientific Literature: REASONS Dataset | **NEW** — sentence-level citation attribution across 12 domains |
| 5 | 2605.07723 | 2026 | LLM hallucinations in the wild: Large-scale evidence from non-existent citations | **NEW** — 111M references audit, 146K hallucinated citations in 2025 |
| 6 | 2508.00838v1 | 2025 | The Attribution Crisis in LLM Search Results | **NEW** — attribution gap analysis of Gemini/GPT-4o/Perplexity |
| 7 | 2603.03299 | 2026 | How LLMs Cite and Why It Matters: Cross-Model Audit | **NEW** — 69K citation audit, multi-model consensus filter |
| 8 | 2305.14627v2 | 2023 | Enabling LLMs to Generate Text with Citations (ALCE) | Already in pool |
| 9 | 2305.18248v3 | 2023 | Do Language Models Know When They're Hallucinating References? | **NEW** — consistency-check method for detecting hallucinated references |
| 10 | 2404.04631v2 | 2024 | On the Limitations of LLMs: False Attribution (SHI metric) | **NEW** — Simple Hallucination Index metric |

### epr-02: "factual consistency evidence grounding scientific text"

Broad coverage of factual consistency methods transferable to survey evaluation.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2204.04991v3 | 2022 | TRUE: Re-evaluating Factual Consistency Evaluation | **NEW** — standardized meta-evaluation of factual consistency metrics |
| 2 | 2501.03200v1 | 2025 | FACTS Grounding Leaderboard | **NEW** — leaderboard for long-form grounding |
| 3 | 2305.08281v2 | 2023 | FactKB: Generalizable Factuality Evaluation | **NEW** — KB-enhanced factuality evaluation, cross-domain |
| 4 | 1910.12840v1 | 2019 | Evaluating Factual Consistency of Abstractive Summarization | **NEW** — weakly-supervised factual consistency model |
| 5 | 2506.23990v1 | 2025 | Machine Understanding of Scientific Language | **NEW** — thesis covering scientific fact-checking |
| 6 | 2503.08890v2 | 2025 | PlainQAFact: Factual Consistency for Biomedical Plain Language | **NEW** — retrieval-augmented factual consistency for lay summaries |
| 7 | 2509.25868 | 2025 | ReFACT: Benchmark for Scientific Confabulation Detection | **NEW** — 1K expert-annotated confabulation detection |
| 8 | 2203.10133v2 | 2022 | Probing Factually Grounded Content Transfer (Factual Ablation) | **NEW** — factual ablation for measuring consistency |
| 9 | 2108.13134v2 | 2021 | Factual Consistency via Counterfactual Estimation | **NEW** — causal approach to factual consistency |
| 10 | 2305.02104v1 | 2023 | Background Knowledge Grounding for Biomedical Lay Summaries | **NEW** — grounding source evaluation study |

---

## extended_benchmark (2 queries)

### ebm-01: "survey generation evaluation benchmark metric coverage"

Captures evaluation frameworks without named benchmark brands.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2602.11238 | 2026 | SurveyLens: Research Discipline-Aware Benchmark for ASG | **NEW** — discipline-aware evaluation across 10 domains |
| 2 | 2508.15658v1 | 2025 | SurGE: Benchmarking Computer Science Survey Generation | Already in pool (as SurGE) |
| 3 | 2508.11310v1 | 2025 | SGSimEval: Multifaceted Similarity-Enhanced Benchmark for ASG | **NEW** — outline+content+reference evaluation |
| 4 | 2512.02763 | 2025 | SurveyEval: Comprehensive Evaluation of LLM-Generated Surveys | **NEW** — 3-dimension evaluation, 7 subjects |
| 5 | 2510.03120v1 | 2025 | SurveyBench: How Well Can LLM(-Agents) Write Academic Surveys? | Already in pool |
| 6 | 2601.15307 | 2026 | DeepSurvey-Bench: Evaluating Academic Value of Generated Surveys | **NEW** — academic value criteria (informational, scholarly, research guidance) |
| 7 | 2503.04629v1 | 2025 | SurveyForge: Outline Heuristics, Memory-Driven Generation | Already in pool |

### ebm-02: "citation accuracy factual precision evaluation benchmark"

Targets citation quality as a standalone evaluation concern.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2506.01829v1 | 2025 | CiteEval: Principle-Driven Citation Evaluation for Source Attribution | **NEW** — multi-domain citation evaluation framework, CiteBench |
| 2 | 2602.16942 | 2026 | SourceBench: Can AI Answers Reference Quality Web Sources? | **NEW** — 8-metric framework for cited web source quality |
| 3 | 2405.05583v2 | 2024 | OpenFactCheck: Building, Benchmarking Fact-Checking Systems | **NEW** — unified fact-checking framework |
| 4 | 2407.12861v2 | 2024 | CiteME: Can Language Models Accurately Cite Scientific Claims? | **NEW** — claim-to-paper attribution benchmark |
| 5 | 2305.14251v2 | 2023 | FActScore: Fine-grained Atomic Evaluation of Factual Precision | **NEW** — atomic fact decomposition for factual precision |
| 6 | 2407.03572v2 | 2024 | Core: Robust Factual Precision with Informative Sub-Claim ID | **NEW** — sub-claim selection to prevent score manipulation |
| 7 | 2602.11685 | 2026 | DRACO: Cross-Domain Benchmark for Deep Research Accuracy | **NEW** — 4-dimension evaluation of deep research outputs |
| 8 | 2501.03200v1 | 2025 | FACTS Grounding Leaderboard | Duplicate from epr-02 |

---

## extended_survey (1 query)

### esv-01: "LLM agent literature survey review 2024 2025"

Recent surveys cataloging the LLM-agent-meets-scientific-literature space.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2503.21460v1 | 2025 | Large Language Model Agent: A Survey on Methodology, Applications and Challenges | **NEW** — comprehensive LLM agent survey |
| 2 | 2508.17281v1 | 2025 | From Language to Action: A Review of LLMs as Autonomous Agents | **NEW** — structured review of 68 datasets for LLM agents |
| 3 | 2504.19678v1 | 2025 | From LLM Reasoning to Autonomous AI Agents: Comprehensive Review | **NEW** — ~60 benchmarks taxonomy, agent protocols (ACP, MCP, A2A) |
| 4 | 2308.11432v7 | 2023 | A Survey on LLM based Autonomous Agents | Already in pool or too broad |
| 5 | 2309.07864v3 | 2023 | The Rise and Potential of LLM Based Agents: A Survey | Already in pool or too broad |
| 6 | 2409.04600v1 | 2024 | The emergence of LLMs as a tool in literature reviews: LLM automated systematic review | **NEW** — LLM-assisted systematic review methodology |
| 7 | 2508.05668v3 | 2025 | A Survey of LLM-based Deep Search Agents | **NEW** — systematic analysis of search agent paradigm |
| 8 | 2503.23037v2 | 2025 | Agentic Large Language Models, a survey | **NEW** — reasoning/act/interact taxonomy for agentic LLMs |

---

## extended_boundary (3 queries)

### eby-01: "reinforcement learning paper retrieval citation traversal"

Probes RL-for-citation-traversal gap (high drift risk ~80%).

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2601.18207 | 2026 | PaperSearchQA: Learning to Search and Reason over Scientific Papers with RLVR | **NEW** — RLVR for biomedical paper search, 60K QA dataset |
| 2 | 2509.05874v1 | 2025 | Learning to Construct Knowledge through Sparse Reference Selection with RL | **NEW** — DRL for sparse reference selection in drug-gene discovery |
| 3 | 2205.15281v1 | 2022 | Learning Open Domain Multi-hop Search Using RL | **NEW** — actor-critic for multi-hop search over Wikipedia |
| 4 | 2501.10120v2 | 2025 | PaSa: LLM Agent for Comprehensive Academic Paper Search | Duplicate — already noted (uses RL optimization) |
| 5 | 2403.08737v1 | 2024 | ILCiteR: Evidence-grounded Interpretable Local Citation Recommendation | **NEW** — evidence-grounded citation recommendation |

### eby-02: "scientific claim verification fact checking attribution retrieval"

Adjacent area with transferable evidence-gathering techniques.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2004.14974v6 | 2020 | Fact or Fiction: Verifying Scientific Claims (SciFact) | **NEW** — foundational scientific claim verification dataset |
| 2 | 2202.02646v2 | 2022 | RerrFact: Reduced Evidence Retrieval for Scientific Claim Verification | **NEW** — modular claim verification approach |
| 3 | 2407.12861v2 | 2024 | CiteME: Language Models Accurately Cite Scientific Claims | Duplicate from ebm-02 |
| 4 | 2203.12990v1 | 2022 | Generating Scientific Claims for Zero-Shot Fact Checking | **NEW** — claim generation for zero-shot verification |
| 5 | 2305.18265v1 | 2023 | Check-COVID: Fact-Checking COVID-19 News with Scientific Evidence | **NEW** — cross-genre fact-checking (news → scientific evidence) |
| 6 | 2107.08188v1 | 2021 | SciVer Shared Task on Scientific Claim Verification | **NEW** — shared task overview |
| 7 | 2010.03001v5 | 2020 | A Review on Fact Extraction and Verification (FEVER) | **NEW** — comprehensive review of FEVER task |
| 8 | 2510.22055 | 2025 | QuanTemp++: Benchmark for Numerical Fact-Checking | **NEW** — numerical claim verification with claim decomposition |

### eby-03: "human-in-the-loop systematic review screening LLM assisted"

Boundary between automated survey agents and human-assisted SLR tools.

| # | arXiv ID | Year | Title | Novelty vs pool |
|---|----------|------|-------|-----------------|
| 1 | 2501.11840v1 | 2025 | LLMs with Human-In-The-Loop Validation for SR Data Extraction (AIDE) | **NEW** — open-source HITL extraction tool |
| 2 | 2510.06708 | 2025 | AISysRev: LLM-based Tool for Title-abstract Screening | **NEW** — web-app with 4-category classification |
| 3 | 2412.15247v1 | 2024 | Streamlining Systematic Reviews: Novel Application of LLMs | **NEW** — RAG for full-text screening, 95.5% time reduction |
| 4 | 2510.11409 | 2025 | LLMs for Semi-Automatic Corpus Filtration in SLRs (LLMSurver) | **NEW** — consensus-based multi-LLM filtering, visual interface |
| 5 | 2404.15667v4 | 2024 | Promise and Challenges of Using LLMs to Accelerate SR Screening | **NEW** — empirical study with GPT-3.5/4, few-shot analysis |
| 6 | 2311.07918v1 | 2023 | Automated Title and Abstract Screening (GPTscreenR) | **NEW** — R package for GPT-4 screening |
| 7 | 2407.10652v2 | 2024 | Cutting Through the Clutter: LLMs for Efficient Filtration in SLRs | **NEW** — 8.3K paper filtration study, consensus recall >98.8% |
| 8 | 2504.04193v1 | 2025 | AiReview: Open Platform for Accelerating SRs with LLMs | **NEW** — first platform bridging LLM screening and medical SRs |
| 9 | 2411.02451v2 | 2024 | High-performance Automated Abstract Screening with LLM Ensembles | **NEW** — Cochrane evaluation, LLM ensemble achieves perfect sensitivity |
| 10 | 2310.17526v2 | 2023 | Can LLMs Replace Humans in Systematic Review? (GPT-4 evaluation) | **NEW** — comprehensive GPT-4 evaluation across languages |

---

## Counts Summary

| Query Type | Total Queries | Novel Candidates (est.) |
|------------|--------------|------------------------|
| extended_method | 3 | ~22 |
| extended_mechanism | 4 | ~23 |
| extended_problem | 2 | ~18 |
| extended_benchmark | 2 | ~12 |
| extended_survey | 1 | ~6 |
| extended_boundary | 3 | ~22 |
| **Total** | **15** | **~103** |

*Note: Some candidates appear across multiple queries (e.g., SPAR, PaSa, ResearchPilot). Deduplication will be required before merging.*
