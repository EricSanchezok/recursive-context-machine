# Semantic Expansion — Embedding-Search Neighbors

Generated: 2026-06-07T17:57Z  
Run dir: `.`  
Method: 8 concept-rich embedding-search queries against arXiv, topK 10 each. Deduplicated against 6 seed papers and across queries.

---

## Queries

| # | Dimension | Query |
|---|-----------|-------|
| Q1 | Method — citation graph traversal agents | LLM-based survey agents that expand retrieval through forward and backward citation chaining over scientific literature graphs |
| Q2 | Method — graph-enhanced retrieval | Document retrieval methods that use citation graph structure, co-citation, and bibliographic coupling to improve relevance in scientific domains |
| Q3 | Method — plan-then-expand architectures | Multi-stage survey generation systems that plan subtopics then expand via citation traversal before synthesis |
| Q4 | Benchmark — evaluation of survey agents | Benchmarks and evaluation protocols for measuring coverage, faithfulness, and citation accuracy in automated literature surveys |
| Q5 | Benchmark — scientific QA & retrieval | Question answering and factuality evaluation datasets for scientific literature, testing citation-grounded retrieval |
| Q6 | Boundary — systematic review screening | Active learning and machine learning methods for screening and prioritization in systematic literature reviews |
| Q7 | Boundary — citation graph bias | Scientometric approaches to detecting and mitigating citation bias, echo chambers, and Matthew effect in citation analysis |
| Q8 | Synthesis — LLM citation attribution | Methods for faithful citation attribution and grounded synthesis in LLM-generated literature surveys and scientific summaries |

---

## New Candidates (deduplicated, 48 unique)

### Method — Survey Agent Systems (12 candidates)

| # | arXiv ID | Title | Year | Source Query | Notes |
|---|----------|-------|------|-------------|-------|
| 1 | 2510.07733 | SurveyG: Multi-Agent LLM Framework with Hierarchical Citation Graph | 2025 | Q1, Q3 | Hierarchical 3-layer citation graph (Foundation/Development/Frontier); horizontal+vertical traversal |
| 2 | 2507.15245v1 | SPAR: Scholar Paper Retrieval with LLM-based Agents | 2025 | Q1 | RefChain-based query decomposition; SPARBench for evaluation |
| 3 | 2509.18661v1 | Agentic AutoSurvey: Let LLMs Survey LLMs | 2025 | Q1, Q3 | 4 specialized agents; 12-dimension evaluation; 8.18/10 vs AutoSurvey 4.77/10 |
| 4 | 2506.12689v2 | SciSage: Multi-Agent Framework for Scientific Survey Generation | 2025 | Q1 | Reflect-when-you-write paradigm; SurveyScope benchmark; +32% citation F1 |
| 5 | 2510.15682 | SQuAI: Scientific QA with Multi-Agent RAG | 2025 | Q1 | 4-agent RAG over 2.3M arXiv papers; hybrid sparse-dense retrieval |
| 6 | 2408.13450v1 | vitaLITy 2: Reviewing Academic Literature Using LLMs | 2024 | Q1 | RAG architecture with 66k-paper corpus; text embedding search |
| 7 | 2402.01788v2 | LitLLM: Toolkit for Scientific Literature Review | 2024 | Q1 | RAG-based toolkit; re-ranking based on abstract similarity |
| 8 | 2502.14776v2 | SurveyX: Academic Survey Automation via LLMs | 2025 | Q1 | AttributeTree preprocessing; online reference retrieval; re-polishing |
| 9 | 2508.14317v1 | SurveyGen-I: Evolving Plans and Memory-Guided Writing | 2025 | Q3 | Coarse-to-fine retrieval; adaptive planning; memory mechanism for coherence |
| 10 | 2402.14207v2 | STORM: Synthesis of Topic Outlines through Retrieval and Multi-perspective Question Asking | 2024 | Q3 | Wikipedia-style article generation; multi-perspective question asking |
| 11 | 2504.08762v1 | InteractiveSurvey: Personalized and Interactive Survey Generation | 2025 | Q3 | User customization of references, outline, content mid-generation |
| 12 | 2510.10890 | LLM×MapReduce-V3: MCP-Driven Hierarchically Modular Agent System | 2025 | Q3 | Modular MCP server architecture; human-in-the-loop intervention |

### Method — Citation Graph Retrieval & Embeddings (9 candidates)

| # | arXiv ID | Title | Year | Source Query | Notes |
|---|----------|-------|------|-------------|-------|
| 13 | 2409.12177v1 | LitFM: Retrieval Augmented Structure-aware Foundation Model for Citation Graphs | 2024 | Q1, Q2 | Graph retriever navigating citation graphs; +28.1% precision on retrieval; 6 downstream tasks |
| 14 | 2403.09295v2 | Seed-based IR in Networks: Direct Citations, Bibliographic Coupling, Co-citations | 2024 | Q2 | Systematic comparison of 3 citation-based approaches; combining all 3 outperforms single methods |
| 15 | 2109.10007v1 | Generating Local Maps of Science using Deep Bibliographic Coupling | 2021 | Q2 | Graph diffusion extends bibliographic coupling to deep neighborhood |
| 16 | 2106.01560v1 | CitationIE: Leveraging the Citation Graph for Scientific Information Extraction | 2021 | Q2 | Citation-aware information extraction; citation graph augments text representations |
| 17 | 2004.05904v1 | Return to Basics: Clustering using Structural Information | 2020 | Q2 | Direct citation replication for clustering; outperforms NLP-based classification |
| 18 | 2106.05633v1 | Citation Recommendation for Research Papers via Knowledge Graphs | 2021 | Q2 | Research KG + citation network combination; +0.8 MAP |
| 19 | 2207.03299v2 | Academic IR using Citation Clusters | 2022 | Q2 | Citation clusters for IR; complementary to query-based search |
| 20 | 1511.05078v2 | Which Type of Citation Analysis Generates the Most Accurate Taxonomy? | 2015 | Q2 | Direct citation > bibliographic coupling > co-citation for taxonomy accuracy |
| 21 | 2110.06354v3 | Reading Path Generation (SurveyBank) | 2021 | Q3 | RPG task; SurveyBank dataset; graph-optimization-based reading path generation |

### Benchmark & Evaluation (10 candidates)

| # | arXiv ID | Title | Year | Source Query | Notes |
|---|----------|-------|------|-------------|-------|
| 22 | 2508.15658v1 | SurGE: Survey Generation Evaluation Benchmark | 2025 | Q4 | 4-dimension evaluation (coverage, referencing, structure, content); 1M-paper corpus |
| 23 | 2510.03120v1 | SurveyBench: How Well Can LLM(-Agents) Write Academic Surveys? | 2025 | Q4 | 11,343 arXiv topics, 4,947 surveys; quiz-driven evaluation; 21% gap vs human |
| 24 | 2508.15804v1 | ReportBench: Evaluating Deep Research Agents via Academic Survey Tasks | 2025 | Q4 | Citation quality + faithfulness dimensions; agent-based automated evaluation |
| 25 | 2605.29234 | Rethinking Literature Search Evaluation: Deep Research and Human Citation Lists | 2026 | Q4 | Deep Research pipeline; only 51% human citations judged relevant; co-authorship bias |
| 26 | 2503.04629v1 | SurveyForge: Outline Heuristics, Memory-Driven Generation, SurveyBench | 2025 | Q3, Q4 | SurveyBench construction (100 human surveys); reference/outline/content evaluation |
| 27 | 2502.13668v1 | PeerQA: Scientific QA Dataset from Peer Reviews | 2025 | Q5 | 579 QA pairs from 208 papers; evidence retrieval + unanswerable classification |
| 28 | 2407.18940v2 | LitSearch: Retrieval Benchmark for Scientific Literature Search | 2024 | Q5 | 597 queries on recent ML/NLP papers; 24.8% gap BM25 vs dense retrievers |
| 29 | 2105.03011v1 | QASPER: Information-Seeking QA Anchored in Research Papers | 2021 | Q5 | 5,049 questions over 1,585 NLP papers; human models underperform by 27 F1 |
| 30 | 2411.14199v1 | OpenScholar: Synthesizing Scientific Literature (ScholarQABench) | 2024 | Q5 | 45M paper datastore; ScholarQABench; 8B model beats GPT-4o by 5% |
| 31 | 2409.13740v2 | Language agents achieve superhuman synthesis (PaperQA2, LitQA2) | 2024 | Q5 | PaperQA2 exceeds human experts; LitQA2 benchmark; contradiction detection |

### Citation Attribution & Faithfulness (6 candidates)

| # | arXiv ID | Title | Year | Source Query | Notes |
|---|----------|-------|------|-------------|-------|
| 32 | 2510.17853 | CiteGuard: Faithful Citation Attribution via Retrieval-Augmented Validation | 2025 | Q8 | Retrieval-aware agent; 65.4% on CiteME benchmark (human 69.7%) |
| 33 | 2502.09604v3 | SelfCite: Self-Supervised Alignment for Context Attribution | 2025 | Q8 | Context ablation reward; +5.3 F1 on LongBench-Cite |
| 34 | 2508.15396v1 | Attribution, Citation, and Quotation: Survey of Evidence-based Text Generation | 2025 | Q8 | Unified taxonomy; 134 papers analyzed; 300 metrics across 7 dimensions |
| 35 | 2309.09727v1 | When LLMs Meet Citation: A Survey | 2023 | Q8 | LLM-for-citation and citation-for-LLM bidirectional review |
| 36 | 2405.02228v3 | REASONS: Attribution in Scientific Literature (new benchmark + methods) | 2024 | Q5, Q8 | Sentence-level annotations across 12 domains; -42% hallucination via RAG |
| 37 | 2306.03535v2 | SciLit: Platform for Literature Discovery, Summarization and Citation | 2023 | Q8 | End-to-end assistive writing; abstractive citation generation |

### Boundary — Systematic Review Automation (5 candidates)

| # | arXiv ID | Title | Year | Source Query | Notes |
|---|----------|-------|------|-------------|-------|
| 38 | 2502.03400v1 | DenseReviewer: Screening Prioritisation for Systematic Reviews | 2025 | Q6 | Dense retrieval + active learning; web-based screening tool |
| 39 | 2202.10033v2 | Open-source integrated framework for citation collection and screening | 2022 | Q6 | Bayesian active learning; 95.6% efficiency with 100% sensitivity |
| 40 | 2011.09752v1 | Hybrid Learning for Technology-Assisted SLR | 2020 | Q6 | Learning-to-rank + relevance feedback; full pipeline from protocol to screening |
| 41 | 2509.23981v1 | Automatic Selection of Primary Studies with Evolutionary Rule-Based Classification | 2025 | Q6 | Grammar-guided genetic programming; interpretable classifiers |
| 42 | 1909.07249v4 | Assessing Expert System-Assisted Literature Reviews | 2019 | Q6 | Active learning tool reduces 53h → 3h; 90% recall with 6% human effort |

### Boundary — Citation Bias & Scientometrics (6 candidates)

| # | arXiv ID | Title | Year | Source Query | Notes |
|---|----------|-------|------|-------------|-------|
| 43 | 2411.05584v2 | Mitigating Consequences of Prestige in Citations (Matthew Effect) | 2024 | Q7 | Predicting citations from pre-publication variables only; mitigates prestige bias |
| 44 | 1703.08071v1 | Quantifying and Suppressing Ranking Bias in Citation Networks | 2017 | Q7 | Mahalanobis distance framework; z-score normalization reduces field/age bias |
| 45 | 2203.17239v1 | Citation Bias in Peer Review (Cite-seeing) | 2022 | Q7 | +0.23 score increase on 5-pt Likert when citing reviewer's work |
| 46 | 2502.13934v1 | Citation Proximus: Social and Semantic Ties in Citing Behaviour | 2025 | Q7 | Collaboration network strongest predictor; prestige matters only for highly-cited |
| 47 | 2508.12735v1 | Citation Accuracy, Citation Noise, and Citation Bias (foundational) | 2025 | Q7 | Defines citation noise vs bias; CoARA recommendations |
| 48 | 1501.05462v3 | A Review of Theory and Practice in Scientometrics | 2015 | Q7 | Comprehensive survey of citation metrics, normalization, JIF, mapping |

---

## Drift Risk Assessment

### Risk 1: Strong 2025 recency skew
26 of 48 new candidates (54%) are from 2025. The embedding search naturally favours recent work because query concepts (LLM agents, survey automation) are most densely populated in the last 1–2 years. Mitigation: the seed set (2019–2024) and the pre-2023 citation analysis papers (e.g., 1511.05078v2, 1203.6742v1, 1501.05462v3) provide temporal anchors. However, the survey final brief must deliberately weight pre-2023 foundational works even if they are underrepresented in embedding neighbourhoods.

### Risk 2: Topic boundary creep — systematic review automation
Queries Q6 and Q7 intentionally probed boundary domains. Q6 returned strong candidates on active-learning-based systematic review screening (DenseReviewer, FASTREAD, evolutionary classification). These are adjacent but not core — they automate *screening*, not *citation-graph-driven survey generation*. Inclusion risk: the survey scope could drift into generic SLR automation. Mitigation: these candidates are tagged as boundary terms; include only as comparative context in the final brief, not as primary method taxonomy entries.

### Risk 3: Overlap with seed papers incomplete
The ASReview paper (seed #4, 1906.11512) was not directly returned by any query (only a later journal version 2006.12166v3 appeared). PaperQA (seed #5, 2312.07562) appeared as 2312.07559v2 — likely the same paper but with a one-digit arXiv ID offset. AutoSurvey (seed #6, 2405.13215) was not returned by any query. S2ORC, SPECTER, and SciBERT were also absent from results. This suggests the embedding search space of arXiv does not strongly overlap with the seed IDs, which may indicate that the seeds cover infrastructure/embedding papers rather than the agent systems that are more topically visible.

---

## Summary

| Metric | Value |
|--------|-------|
| Queries executed | 8 |
| Total raw results | 80 |
| Unique new candidates | 48 |
| Candidates overlapping with seeds | ~2 (PaperQA, ASReview) |
| Temporal range of candidates | 2015–2026 |
| Method candidates | 21 |
| Benchmark candidates | 10 |
| Boundary candidates | 11 |
| Attribution/synthesis candidates | 6 |
