# ExtendedCandidatePool — Merged Extended Candidate Pool

**run_dir**: `.`
**generated**: 2026-06-06T12:50:39+08:00
**source_scouts**: ExtendedMethodScout, ExtendedBenchmarkScout, ExtendedSurveyScout, ExtendedFrontierScout
**source_scout_artifacts**: 02e_extended_method_candidates.md, 02f_extended_benchmark_candidates.md, 02g_extended_survey_candidates.md, 02h_extended_frontier_candidates.md
**deduplication_against**: `02_candidate_pool.md` (82 existing candidates)
**total_unique_candidates**: 88

---

## Legend

| Column | Description |
|--------|-------------|
| `#` | Sequential unique ID in this pool |
| `arXiv ID` | Base arXiv identifier (version suffix stripped for dedup) |
| `Title` | Short title |
| `Year` | Publication year |
| `relevance_score` | **high** (directly relevant to survey generation), **medium** (indirectly relevant via methodology/evaluation transfer), **low** (boundary probe) |
| `source_agents` | Which extended scout(s) contributed this candidate |
| `source_queries` | Original query ID(s) from the Extended QueryPlan (`01b_query_plan_extended.md`) |
| `notes` | Brief inclusion rationale |

---

## Candidate Pool

| # | arXiv ID | Title | Year | relevance_score | source_agents | source_queries | notes |
|---|----------|-------|------|-----------------|---------------|----------------|-------|
| 1 | 1706.03449 | Scientific document summarization via citation contextualization and scientific discourse | 2017 | high | extended_method, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 2 | 1709.00770 | Understanding the Logical and Semantic Structure of Large Documents | 2017 | high | extended_frontier | EQ-11 | Structure-aware retrieval for multi-section generation; survey outline coherence |
| 3 | 1709.06758 | A shared latent space matrix factorisation method for recommending new trial evidence | 2017 | medium | extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 4 | 1804.05365 | Dimensions: re-discovering the ecosystem of scientific information | 2018 | medium | extended_frontier | EQ-08 | Citation analysis / bibliometrics; informs survey coverage methodology |
| 5 | 1804.09479 | Coverage of highly-cited documents in Google Scholar, Web of Science, and Scopus | 2018 | medium | extended_frontier | EQ-08 | Citation analysis / bibliometrics; informs survey coverage methodology |
| 6 | 1806.06351 | Google Scholar: the 'big data' bibliographic tool | 2018 | medium | extended_frontier | EQ-08 | Citation analysis / bibliometrics; informs survey coverage methodology |
| 7 | 1810.11878 | Unsupervised Evaluation Metrics and Learning Criteria for Non-Parallel Textual Transfer | 2018 | high | extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 8 | 1901.00398 | Judge the Judges: A Large-Scale Evaluation Study of Neural Language Models | 2019 | high | extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 9 | 1901.10133 | Structuring an unordered text document | 2019 | high | extended_frontier | EQ-11 | Structure-aware retrieval for multi-section generation; survey outline coherence |
| 10 | 1905.10039 | Outline Generation: Understanding the Inherent Content Structure of Documents | 2019 | high | extended_method, extended_benchmark, extended_frontier | EQ-02, EQ-11, EQ-13 | Multi-document summarization for scientific literature; methodological precursor |
| 11 | 1908.08610 | Viability of machine learning to reduce workload in systematic review screenings | 2019 | medium | extended_frontier | EQ-09 | Systematic review automation with ML; pre-LLM boundary work for context |
| 12 | 1908.11152 | A Summarization System for Scientific Documents | 2019 | high | extended_method, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 13 | 1909.02622 | MoverScore: Text Generation Evaluating with Contextualized Embeddings and Earth Mover Distance | 2019 | high | extended_method, extended_benchmark, extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 14 | 1910.03678 | Unfolding the Structure of a Document using Deep Learning | 2019 | high | extended_method, extended_frontier | EQ-11 | Structure-aware retrieval for multi-section generation; survey outline coherence |
| 15 | 1911.08836 | Table-Of-Contents generation on contemporary documents | 2019 | high | extended_method, extended_frontier | EQ-11 | Structure-aware retrieval for multi-section generation; survey outline coherence |
| 16 | 2004.06183 | When to Update Systematic Literature Reviews in Software Engineering | 2020 | medium | extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 17 | 2004.14329 | Google Scholar, Microsoft Academic, Scopus, Dimensions, Web of Science, and OpenCitations' COCI | 2020 | medium | extended_method, extended_frontier | EQ-08 | Citation analysis / bibliometrics; informs survey coverage methodology |
| 18 | 2005.10732 | Large-scale comparison of bibliographic data sources | 2020 | medium | extended_method, extended_frontier | EQ-08 | Citation analysis / bibliometrics; informs survey coverage methodology |
| 19 | 2005.11216 | A Generative Approach to Titling and Clustering Wikipedia Sections | 2020 | high | extended_frontier | EQ-11 | Structure-aware retrieval for multi-section generation; survey outline coherence |
| 20 | 2006.05542 | Guidelines for the Search Strategy to Update Systematic Literature Reviews | 2020 | medium | extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 21 | 2006.12166 | ASReview: Open Source Software for Efficient and Transparent Reviews | 2020 | medium | extended_method, extended_benchmark, extended_survey, extended_frontier | EQ-09 | Systematic review automation with ML; pre-LLM boundary work for context |
| 22 | 2010.00490 | QAEval: Towards Question-Answering as an Automatic Metric for Evaluating Summaries | 2020 | high | extended_method, extended_benchmark, extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 23 | 2010.02498 | GRUEN for Evaluating Linguistic Quality of Generated Text | 2020 | high | extended_method, extended_benchmark, extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 24 | 2010.04665 | Scaling Systematic Literature Reviews with Machine Learning Pipelines | 2020 | medium | extended_method, extended_benchmark, extended_frontier | EQ-09 | Systematic review automation with ML; pre-LLM boundary work for context |
| 25 | 2010.12495 | Understanding the Extent to which Summarization Evaluation Metrics Measure Information Quality | 2020 | high | extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 26 | 2010.14235 | Multi-XScience: Large-scale Dataset for Extreme Multi-document Summarization | 2020 | high | extended_method, extended_benchmark, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 27 | 2011.08072 | Topic-Centric Unsupervised Multi-Document Summarization of Scientific and News Articles | 2020 | high | extended_method, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 28 | 2012.14136 | On Generating Extended Summaries of Long Documents | 2020 | high | extended_method, extended_frontier | EQ-11 | Structure-aware retrieval for multi-section generation; survey outline coherence |
| 29 | 2101.11796 | DOC2PPT: Automatic Presentation Slides Generation from Scientific Documents | 2021 | high | extended_method, extended_frontier | EQ-11 | Structure-aware retrieval for multi-section generation; survey outline coherence |
| 30 | 2102.06345 | A Visual Analysis Approach to Update Systematic Reviews | 2021 | medium | extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 31 | 2104.06486 | MS2: Multi-Document Summarization of Medical Studies | 2021 | high | extended_method, extended_benchmark, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 32 | 2104.08668 | Generating Related Work | 2021 | high | extended_method, extended_benchmark, extended_frontier | EQ-01 | Related work generation; direct precursor to survey section generation |
| 33 | 2108.12922 | Continuous Systematic Literature Review: An Approach for Open Science | 2021 | medium | extended_method, extended_survey, extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 34 | 2109.00051 | Towards Sustainability of Systematic Literature Reviews | 2021 | medium | extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 35 | 2109.09784 | Hallucinated but Factual! Inspecting the Factuality of Hallucinations in Abstractive Summarization | 2021 | high | extended_frontier | EQ-10 | Hallucination/factuality evaluation for long-form text; survey quality eval |
| 36 | 2109.12141 | ALL-IN meta-analysis: breathing life into living systematic reviews | 2021 | medium | extended_method, extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 37 | 2110.07850 | End-to-End Segmentation-based News Summarization | 2021 | high | extended_method, extended_frontier | EQ-11 | Structure-aware retrieval for multi-section generation; survey outline coherence |
| 38 | 2110.09147 | BEAMetrics: A Benchmark for Language Generation Evaluation Evaluation | 2021 | high | extended_method, extended_benchmark, extended_survey, extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 39 | 2201.05294 | Multi-Narrative Semantic Overlap Task: Evaluation and Benchmark | 2022 | high | extended_benchmark, extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 40 | 2202.10033 | An open-source integrated framework for the automation of citation collection and screening | 2022 | medium | extended_method, extended_frontier | EQ-09 | Systematic review automation with ML; pre-LLM boundary work for context |
| 41 | 2205.13339 | Target-aware Abstractive Related Work Generation with Contrastive Learning | 2022 | high | extended_method, extended_benchmark, extended_frontier | EQ-01 | Related work generation; direct precursor to survey section generation |
| 42 | 2206.04177 | Towards Continuous Systematic Literature Review in Software Engineering | 2022 | medium | extended_method, extended_survey, extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 43 | 2208.01030 | SMART: Sentences as Basic Units for Text Evaluation | 2022 | high | extended_method, extended_benchmark, extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 44 | 2210.05892 | Perplexity from PLM Is Unreliable for Evaluating Text Quality | 2022 | high | extended_frontier | EQ-13 | Evaluation metrics beyond lexical overlap; survey content quality assessment |
| 45 | 2210.10341 | BioGPT: Generative Pre-trained Transformer for Biomedical Text Generation and Mining | 2022 | medium | extended_method, extended_frontier | EQ-05 | Biomedical lit review automation; boundary method transfer candidate |
| 46 | 2211.15397 | Automating Systematic Literature Reviews with NLP and Text Mining: a SLR | 2022 | medium | extended_method, extended_survey, extended_frontier | EQ-09 | Systematic review automation with ML; pre-LLM boundary work for context |
| 47 | 2212.01956 | Grounded Keys-to-Text Generation: Towards Factual Open-Ended Generation | 2022 | high | extended_method, extended_frontier | EQ-03 | Knowledge-grounded long-form generation; factuality evaluation applicable |
| 48 | 2305.00844 | Automated Paper Screening for Clinical Reviews Using Large Language Models | 2023 | medium | extended_method, extended_frontier | EQ-05 | Biomedical lit review automation; boundary method transfer candidate |
| 49 | 2305.14259 | SciMON: Scientific Inspiration Machines Optimized for Novelty | 2023 | medium | extended_method, extended_frontier | EQ-04 | Scientific paper generation; adjacent domain for survey methodology |
| 50 | 2307.09683 | PubMed and Beyond: Biomedical Literature Search in the Age of Artificial Intelligence | 2023 | medium | extended_survey, extended_frontier | EQ-05 | Biomedical lit review automation; boundary method transfer candidate |
| 51 | 2308.06610 | Bio-SIEVE: Exploring Instruction Tuning LLMs for Systematic Review Automation | 2023 | medium | extended_method, extended_frontier | EQ-05 | Biomedical lit review automation; boundary method transfer candidate |
| 52 | 2308.07517 | Synergi: A Mixed-Initiative System for Scholarly Synthesis and Sensemaking | 2023 | high | extended_method, extended_frontier | EQ-06 | Interactive human-AI literature synthesis; directly applicable to surveys |
| 53 | 2309.01684 | CRUISE-Screening: Living Literature Reviews Toolbox | 2023 | medium | extended_method, extended_benchmark, extended_survey, extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 54 | 2310.16181 | Hidden Citations Obscure True Impact in Science | 2023 | medium | extended_method, extended_survey, extended_frontier | EQ-08 | Citation analysis / bibliometrics; informs survey coverage methodology |
| 55 | 2311.03056 | LitSumm: LLM for literature summarisation of non-coding RNAs | 2023 | medium | extended_method, extended_frontier | EQ-05 | Biomedical lit review automation; boundary method transfer candidate |
| 56 | 2311.06785 | Depth and Breadth of Research Area Coverage and Its Impact on Publication Citation | 2023 | medium | extended_frontier | EQ-08 | Citation analysis / bibliometrics; informs survey coverage methodology |
| 57 | 2312.12915 | Survey on Multi-Document Summarization: Systematic Literature Review | 2023 | high | extended_survey, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 58 | 2402.05317 | Emerging Results on Automated Support for Searching and Selecting Evidence for SLR Updates | 2024 | medium | extended_method, extended_frontier | EQ-07 | Living systematic review paradigm; frontier for continuous survey updating |
| 59 | 2402.12255 | Shallow Synthesis of Knowledge in GPT-Generated Texts: A Case Study | 2024 | high | extended_method, extended_frontier | EQ-06 | Interactive human-AI literature synthesis; directly applicable to surveys |
| 60 | 2402.14207 | STORM: Assisting in Writing Wikipedia-like Articles From Scratch with LLMs | 2024 | high | extended_method, extended_benchmark, extended_frontier | EQ-03 | Knowledge-grounded long-form generation; factuality evaluation applicable |
| 61 | 2402.17311 | SKT5SciSumm: Revisiting Extractive-Generative Approach for Multi-Document Scientific Summarization | 2024 | high | extended_method, extended_benchmark, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 62 | 2403.02574 | ChatCite: LLM Agent with Human Workflow Guidance for Comparative Literature Summary | 2024 | high | extended_method, extended_frontier | EQ-06 | Interactive human-AI literature synthesis; directly applicable to surveys |
| 63 | 2403.03866 | KIWI: Dataset of Knowledge-Intensive Writing Instructions | 2024 | high | extended_method, extended_benchmark, extended_frontier | EQ-03 | Knowledge-grounded long-form generation; factuality evaluation applicable |
| 64 | 2403.18802 | Long-form factuality in large language models (LongFact + SAFE) | 2024 | high | extended_method, extended_benchmark, extended_survey, extended_frontier | EQ-10 | Hallucination/factuality evaluation for long-form text; survey quality eval |
| 65 | 2404.07738 | ResearchAgent: Iterative Research Idea Generation over Scientific Literature | 2024 | high | extended_method, extended_frontier | EQ-06 | Interactive human-AI literature synthesis; directly applicable to surveys |
| 66 | 2405.17044 | Interesting Scientific Idea Generation using Knowledge Graphs and LLMs (SciMuse) | 2024 | medium | extended_frontier | EQ-04 | Scientific paper generation; adjacent domain for survey methodology |
| 67 | 2405.20354 | Efficient Systematic Reviews: Literature Filtering with Transformers and Transfer Learning | 2024 | medium | extended_method, extended_frontier | EQ-05 | Biomedical lit review automation; boundary method transfer candidate |
| 68 | 2406.19276 | VERISCORE: Evaluating the factuality of verifiable claims in long-form text | 2024 | high | extended_method, extended_benchmark, extended_frontier | EQ-10 | Hallucination/factuality evaluation for long-form text; survey quality eval |
| 69 | 2407.09484 | GPTutor: Great Personalized Tutor with Large Language Models | 2024 | low | extended_method | EQ-12 | Educational adaptive survey generation; boundary, limited direct relevance |
| 70 | 2407.17468 | WildHallucinations: Evaluating Long-form Factuality with Real-World Entity Queries | 2024 | high | extended_method, extended_benchmark, extended_survey, extended_frontier | EQ-10 | Hallucination/factuality evaluation for long-form text; survey quality eval |
| 71 | 2408.06292 | The AI Scientist: Towards Fully Automated Open-Ended Scientific Discovery | 2024 | medium | extended_method, extended_frontier | EQ-04 | Scientific paper generation; adjacent domain for survey methodology |
| 72 | 2408.16444 | SurveySum: Dataset for Summarizing Multiple Scientific Articles into a Survey Section | 2024 | high | extended_method, extended_benchmark, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 73 | 2410.01691 | FactAlign: Long-form Factuality Alignment of Large Language Models | 2024 | high | extended_method, extended_benchmark, extended_frontier | EQ-10 | Hallucination/factuality evaluation for long-form text; survey quality eval |
| 74 | 2410.12222 | On A Scale From 1 to 5: Quantifying Hallucination in Faithfulness Evaluation | 2024 | high | extended_frontier | EQ-10 | Hallucination/factuality evaluation for long-form text; survey quality eval |
| 75 | 2411.09255 | DAHL: Domain-specific Automated Hallucination Evaluation in Biomedicine | 2024 | high | extended_method, extended_benchmark, extended_frontier | EQ-10 | Hallucination/factuality evaluation for long-form text; survey quality eval |
| 76 | 2411.14199 | OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs | 2024 | high | extended_method, extended_frontier | EQ-06 | Interactive human-AI literature synthesis; directly applicable to surveys |
| 77 | 2411.15993 | Investigating Factuality in Long-Form Text: Self-Known and Self-Unknown | 2024 | high | extended_method, extended_benchmark, extended_frontier | EQ-10 | Hallucination/factuality evaluation for long-form text; survey quality eval |
| 78 | 2412.08578 | ML Information Retrieval and Summarisation to Support Systematic Review | 2024 | medium | extended_method, extended_frontier | EQ-09 | Systematic review automation with ML; pre-LLM boundary work for context |
| 79 | 2501.03200 | FACTS Grounding Leaderboard: Benchmarking LLMs' Ability to Ground Responses | 2025 | high | extended_benchmark, extended_frontier | EQ-03 | Knowledge-grounded long-form generation; factuality evaluation applicable |
| 80 | 2504.18496 | DimInd: Facets, Taxonomies, and Syntheses for LLM-Assisted Literature Review | 2025 | high | extended_method, extended_survey, extended_frontier | EQ-06 | Interactive human-AI literature synthesis; directly applicable to surveys |
| 81 | 2505.11336 | XtraGPT: Context-Aware and Controllable Academic Paper Revision | 2025 | medium | extended_method, extended_frontier | EQ-04 | Scientific paper generation; adjacent domain for survey methodology |
| 82 | 2505.12265 | Learning Auxiliary Tasks Improves Reference-Free Hallucination Detection | 2025 | high | extended_frontier | EQ-10 | Hallucination/factuality evaluation for long-form text; survey quality eval |
| 83 | 2505.16349 | XSum: Ask, Retrieve, Summarize — Modular Pipeline for Scientific MDS | 2025 | high | extended_method, extended_benchmark, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 84 | 2508.03962 | Accelerating Scientific Discovery with Multi-Document Summarization | 2025 | high | extended_method, extended_frontier | EQ-02 | Multi-document summarization for scientific literature; methodological precursor |
| 85 | 2508.14273 | SciIG Benchmark: Benchmarking LLMs to Write the Introduction of a Research Paper | 2025 | medium | extended_method, extended_frontier | EQ-04 | Scientific paper generation; adjacent domain for survey methodology |
| 86 | 2509.23981 | Automatic selection of primary studies in systematic reviews with evolutionary rule-based classification | 2025 | medium | extended_frontier | EQ-09 | Systematic review automation with ML; pre-LLM boundary work for context |
| 87 | 2510.26750 | ProfOlaf: Semi-Automated Tool for Systematic Literature Reviews | 2025 | medium | extended_method, extended_frontier | EQ-09 | Systematic review automation with ML; pre-LLM boundary work for context |
| 88 | 1606.06424 | A Novel Framework to Expedite Systematic Reviews by Automatically Building IE Training Corpora | 2016 | medium | extended_frontier | EQ-09 | Systematic review automation with ML; pre-LLM boundary work for context |

---

## Summary Statistics

| Relevance | Count |
|-----------|-------|
| high | 50 |
| medium | 37 |
| low | 1 |
| **Total** | **88** |

| Source Scout | Count |
|--------------|-------|
| ExtendedMethodScout | 48 |
| ExtendedBenchmarkScout | 18 |
| ExtendedSurveyScout | 12 |
| ExtendedFrontierScout | 87 |

*(Note: source scout counts sum to more than 88 because many papers appear in multiple scouts)*

---

## Removed Papers (Already in Main Pool)

The following papers appeared in extended scout results but were removed because they already exist in `02_candidate_pool.md`:

| arXiv ID | Title |
|----------|-------|
| 2411.06159 | Mixture of Knowledge Minigraph Agents for Literature Review Generation |
| 2411.18583 | Automated Literature Review Using NLP Techniques and LLM-Based RAG |
| 2412.15249 | LitLLMs: LLMs for Literature Review — Are we there yet? |
| 2502.14776 | SurveyX: Academic Survey Automation via Large Language Models |
| 2502.05151 | Transforming Science with LLMs: Survey on AI-assisted Scientific Discovery |
| 2504.08762 | InteractiveSurvey: Personalized and Interactive Survey Generation |
| 2504.10861 | Ai2 Scholar QA: Organized Literature Synthesis with Attribution |
| 2508.14317 | SurveyGen-I: Consistent Survey Generation with Evolving Plans |
| 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow (IterSurvey) |
| 2305.15186 | SciReviewGen: Large-scale Dataset for Automatic Literature Review Generation |
| 2408.13450 | vitaLITy 2: Reviewing Academic Literature Using LLMs |

---

## Risks

1. **`schema/candidate_pool.md` does not exist on disk** — the CandidatePool format follows the conventions established by `02_candidate_pool.md`.
2. **Time-range boundary**: Several candidates (MoverScore 2019, QAEval 2020, ASReview 2020, HiStGen 2019, MS2 2021, and others from 2016–2021) fall outside the 2023–2025 focus window. Retained for methodological value; downstream should filter if strict time window is required.
3. **Boundary probes**: EQ-05 (biomedical), EQ-08 (bibliometrics), and EQ-09 (systematic review ML) candidates are pre-LLM or domain-specific boundary work. Method transfer assessment needed downstream.
4. **EQ-12 educational query yielded zero usable core candidates** — all results were educational question generation papers, not survey generation. Included only as a single boundary entry (GPTutor).
5. **Overlapping candidate provenance**: Papers appearing in multiple extended scouts are listed once with merged provenance.
6. **`run_dir` recovery**: The `run_dir` was not explicitly provided in context; defaulted to `.` (current working directory).
