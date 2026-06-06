# Extended Method Candidates — Broader Net Results

**run_dir**: `.`
**generated**: 2026-06-06T12:47:11+08:00
**source**: `01b_query_plan_extended.md`
**total_candidates**: 88 (before dedup within results)
**total_unique_candidates**: 82 (after dedup)

---

## EQ-01: Related work generation with large language models
**Query**: `related work generation with large language models`
**Category**: `extended_method`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2205.13339v1 | Target-aware Abstractive Related Work Generation with Contrastive Learning | 2022 | method | Direct RW generation method with contrastive learning and target-aware encoder |
| 2104.08668v1 | Generating Related Work | 2021 | method | Content planning + surface realization for RW generation; ACL Anthology dataset |
| 2412.15249v2 | LitLLMs, LLMs for Literature Review: Are we there yet? | 2024 | method+benchmark | Retrieval + planning decomposition for lit review; novel two-step search strategy |

---

## EQ-02: Multi-document summarization for scientific literature
**Query**: `multi document summarization scientific literature`
**Category**: `extended_method`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2505.16349v1 | Ask, Retrieve, Summarize: A Modular Pipeline for Scientific Literature Summarization (XSum) | 2025 | method | Modular RAG pipeline with question-generation + editor for MDS; uses SurveySum |
| 2402.17311v2 | SKT5SciSumm — Revisiting Extractive-Generative Approach for Multi-Document Scientific Summarization | 2024 | method | SPECTER + k-means + T5 hybrid for MDSS; SOTA on Multi-XScience |
| 2508.03962v1 | Accelerating Scientific Discovery with Multi-Document Summarization of Impact-Ranked Papers | 2025 | method+tool | Impact-based paper ranking + lit-review-style summary generation |
| 2104.06486v3 | MS2: Multi-Document Summarization of Medical Studies | 2021 | dataset+method | Large-scale biomedical MDS dataset; BART-based summarization system |
| 2010.14235v1 | Multi-XScience: A Large-scale Dataset for Extreme Multi-document Summarization of Scientific Articles | 2020 | dataset | Extreme summarization dataset; RW generation from abstract + references |
| 1706.03449v1 | Scientific document summarization via citation contextualization and scientific discourse | 2017 | method | Faceted citation summarization; query reformulation + supervised discourse |
| 2011.08072v1 | Topic-Centric Unsupervised Multi-Document Summarization of Scientific and News Articles | 2020 | method | Topic-centric unsupervised abstractive MDSS; MAG-20 dataset |
| 1908.11152v1 | A Summarization System for Scientific Documents | 2019 | tool+system | Full summarization system for CS papers with query-based retrieval |
| 2408.16444v1 | SurveySum: A Dataset for Summarizing Multiple Scientific Articles into a Survey Section | 2024 | dataset+benchmark | Survey-specific MDS dataset; two summarization pipelines evaluated |

---

## EQ-03: Knowledge grounded long form text generation
**Query**: `knowledge grounded long form text generation`
**Category**: `extended_method`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2402.14207v2 | Assisting in Writing Wikipedia-like Articles From Scratch with Large Language Models (STORM) | 2024 | method | Pre-writing stage: perspective discovery + simulated conversations + outline creation |
| 2403.03866v1 | KIWI: A Dataset of Knowledge-Intensive Writing Instructions for Answering Research Questions | 2024 | dataset+benchmark | Scientific domain instruction-following for iterative long-form answer revision |
| 2212.01956v1 | Grounded Keys-to-Text Generation: Towards Factual Open-Ended Generation | 2022 | method | Grounded keys-to-text task; entity description generation with rankers + MAFE metric |

---

## EQ-04: Scientific paper generation and writing with LLMs
**Query**: `scientific paper generation with language models`
**Category**: `extended_method`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2408.06292v3 | The AI Scientist: Towards Fully Automated Open-Ended Scientific Discovery | 2024 | method+system | Full scientific paper generation pipeline; writing, reviewing, and iteration |
| 2508.14273v2 | Let's Use ChatGPT To Write Our Paper! Benchmarking LLMs To Write the Introduction of a Research Paper | 2025 | benchmark+method | SciIG task; multi-dimension evaluation of LLMs for intro generation |
| 2505.11336v2 | XtraGPT: Context-Aware and Controllable Academic Paper Revision via Human-AI Collaboration | 2025 | method+dataset | Section-level revision dataset (7K papers, 140K instructions); open-source revision LLMs |
| 2404.07738v2 | ResearchAgent: Iterative Research Idea Generation over Scientific Literature with LLMs | 2024 | method | Idea generation + multi-agent reviewing; academic graph + knowledge store augmentation |
| 2502.05151v2 | Transforming Science with LLMs: A Survey on AI-assisted Scientific Discovery, Experimentation, Content Generation, and Evaluation | 2025 | survey | Covers full research cycle including literature search, idea gen, content gen, peer review |
| 2305.14259v7 | SciMON: Scientific Inspiration Machines Optimized for Novelty | 2023 | method | Iterative novelty optimization for idea generation; retrieval of inspirations from papers |

---

## EQ-05: Biomedical literature review automation with LLMs
**Query**: `biomedical literature review automation language models`
**Category**: `extended_boundary`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2308.06610v1 | Bio-SIEVE: Exploring Instruction Tuning Large Language Models for Systematic Review Automation | 2023 | method | Instruction-tuned LLM for abstract screening in medical systematic reviews |
| 2311.03056v4 | LitSumm: LLM for literature summarisation of non-coding RNAs | 2023 | method+tool | Domain-specific lit summarization with chain-of-prompts + automated checking |
| 2411.18583v1 | Automated Literature Review Using NLP Techniques and LLM-Based RAG | 2024 | method+system | NLP + RAG comparison for automated literature review generation on SciTLDR |
| 2210.10341v3 | BioGPT: Generative Pre-trained Transformer for Biomedical Text Generation and Mining | 2022 | method+model | Domain-specific GPT for biomedical text generation; SOTA on multiple biomedical tasks |
| 2405.20354v2 | Efficient Systematic Reviews: Literature Filtering with Transformers & Transfer Learning | 2024 | method | Transformer-based filtering for systematic reviews; transfer learning from biomedical corpus |
| 2305.00844v1 | Automated Paper Screening for Clinical Reviews Using Large Language Models | 2023 | method | GPT API workflow for title/abstract screening in clinical reviews |
| 2305.15186v1 | SciReviewGen: A Large-scale Dataset for Automatic Literature Review Generation | 2023 | dataset | 10K+ literature reviews, 690K cited papers; Fusion-in-Decoder experiments |

---

## EQ-06: Interactive human AI collaborative literature synthesis
**Query**: `interactive human AI collaborative literature synthesis`
**Category**: `extended_mechanism`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2504.18496v1 | Facets, Taxonomies, and Syntheses: Navigating Structured Representations in LLM-Assisted Literature Review (DimInd) | 2025 | method+tool | Multi-level compression (papers→tables→taxonomies→synthesis); human-in-loop evaluation with 23 researchers |
| 2402.12255v1 | Shallow Synthesis of Knowledge in GPT-Generated Texts: A Case Study in Automatic Related Work Composition | 2024 | analysis+method | ScholaCite tool analysis; citation graph analysis of human vs GPT vs human-AI collaborative writing |
| 2308.07517v1 | Synergi: A Mixed-Initiative System for Scholarly Synthesis and Sensemaking | 2023 | method+tool | Mixed-initiative pipeline; seed threads + citation graphs + LLMs for thread expansion |
| 2504.10861v2 | Ai2 Scholar QA: Organized Literature Synthesis with Attribution | 2025 | method+tool | Open-source QA over scientific literature; full pipeline with attribution |
| 2403.02574v1 | ChatCite: LLM Agent with Human Workflow Guidance for Comparative Literature Summary | 2024 | method | Mimics human workflow: key element extraction + Reflective Incremental Mechanism; G-Score metric |
| 2411.06159v3 | Mixture of Knowledge Minigraph Agents for Literature Review Generation | 2024 | method | KMCA + MPSA agents; knowledge minigraph construction and multi-path summarization |
| 2411.14199v1 | OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs | 2024 | method+system+benchmark | SOTA retrieval-augmented LM; 45M open-access papers; ScholarQABench; outperforms GPT-4o |
| 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow (IterSurvey) | 2025 | method | Recurrent outline generation; paper cards + review-and-refine loop; Survey-Arena benchmark |

---

## EQ-07: Living systematic reviews continuous evidence monitoring
**Query**: `living systematic reviews continuous evidence monitoring`
**Category**: `extended_survey`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2206.04177v1 | Towards Continuous Systematic Literature Review in Software Engineering | 2022 | method+framework | CSLR concept and process in BPMN; case study on feasibility |
| 2108.12922v1 | Continuous Systematic Literature Review: An Approach for Open Science | 2021 | method+framework | CSLR concept + process + tooling proposal for open science |
| 2109.12141v2 | ALL-IN meta-analysis: breathing life into living systematic reviews | 2021 | method+statistical | E-value based anytime-valid meta-analysis for living reviews |
| 2402.05317v1 | Emerging Results on Automated Support for Searching and Selecting Evidence for SLR Updates | 2024 | method | Automated snowballing + ML for selecting studies for SLR updates |
| 2309.01684v1 | CRUISE-Screening: Living Literature Reviews Toolbox | 2023 | tool | Web-based living literature review app with API-connected search + ML screening |

---

## EQ-08: Citation analysis and bibliometric foundations for survey coverage
**Query**: `citation analysis bibliometric coverage literature discovery`
**Category**: `extended_mechanism`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2310.16181v2 | Hidden Citations Obscure True Impact in Science | 2023 | method+analysis | Unsupervised ML on full text to detect hidden citations (obliteration by incorporation) |
| 2004.14329v3 | Google Scholar, Microsoft Academic, Scopus, Dimensions, Web of Science, and OpenCitations' COCI | 2020 | benchmark+analysis | Multidisciplinary coverage comparison of 6 citation sources; citation discovery implications |
| 2005.10732v2 | Large-scale comparison of bibliographic data sources: Scopus, Web of Science, Dimensions, Crossref, and Microsoft Academic | 2020 | benchmark+analysis | Large-scale document/citation coverage comparison across 5 major sources |

---

## EQ-09: Systematic review automation with machine learning
**Query**: `systematic review automation machine learning text mining`
**Category**: `extended_boundary`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2211.15397v2 | Automating Systematic Literature Reviews with NLP and Text Mining: a Systematic Literature Review | 2022 | survey | SLR of automation of SLRs; ML techniques, challenges, and gaps in screening/extraction/synthesis |
| 2010.04665v1 | Scaling Systematic Literature Reviews with Machine Learning Pipelines | 2020 | method+system | End-to-end ML pipeline for search, selection, and extraction; 2-week annotation experiment |
| 2510.26750 | ProfOlaf: Semi-Automated Tool for Systematic Literature Reviews | 2025 | tool | Iterative snowballing + LLM-assisted analysis with human-in-the-loop filtering |
| 2006.12166v3 | ASReview: Open Source Software for Efficient and Transparent Reviews | 2020 | tool+method | Active learning for title/abstract screening; open-source ML-aided pipeline |
| 2202.10033v2 | An open-source integrated framework for the automation of citation collection and screening in systematic reviews | 2022 | tool+method | Bayesian active learning citation screening + multi-source search with unified query syntax |
| 2412.08578v1 | ML Information Retrieval and Summarisation to Support Systematic Review on Outcomes Based Contracting | 2024 | method+system | ML + NLP for automated systematic review stages; social science domain |

---

## EQ-10: Hallucination and factuality in long form text generation
**Query**: `hallucination factuality long form text generation evaluation`
**Category**: `extended_problem`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2411.09255v1 | DAHL: Domain-specific Automated Hallucination Evaluation of Long-Form Text in Biomedicine | 2024 | benchmark+method | DAHL Score; atomic unit decomposition for long-form hallucination evaluation |
| 2410.01691v1 | FactAlign: Long-form Factuality Alignment of Large Language Models | 2024 | method | fKTO alignment algorithm with fine-grained factuality assessments for alignment |
| 2411.15993v1 | Investigating Factuality in Long-Form Text Generation: The Roles of Self-Known and Self-Unknown | 2024 | analysis+method | Factuality decline in later sentences; Self-Known/Self-Unknown analysis across 5 LLM families |
| 2407.17468v1 | WildHallucinations: Evaluating Long-form Factuality in LLMs with Real-World Entity Queries | 2024 | benchmark+method | 7,919 entities mined from real user-chatbot conversations; RAG only slightly reduces hallucination |
| 2403.18802v4 | Long-form factuality in large language models | 2024 | method+benchmark | LongFact + SAFE (Search-Augmented Factuality Evaluator); F1 for factuality; LLM agents beat crowdsourced annotators |
| 2406.19276v1 | VERISCORE: Evaluating the factuality of verifiable claims in long-form text generation | 2024 | metric | Distinguishes verifiable vs unverifiable claims; evaluated across 8 long-form tasks and 16 models |

---

## EQ-11: Structure aware retrieval for multi section document generation
**Query**: `structure aware retrieval multi section document generation`
**Category**: `extended_mechanism`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 1905.10039v1 | Outline Generation: Understanding the Inherent Content Structure of Documents | 2019 | method | Hierarchical structured prediction for section boundary detection + heading generation; HiStGen model |
| 2508.14317v1 | SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans and Memory-Guided Writing | 2025 | method | Coarse-to-fine retrieval + adaptive planning + memory mechanism for multi-section coherence |
| 1910.03678v1 | Unfolding the Structure of a Document using Deep Learning | 2019 | method | Section identification and classification in large documents; arXiv dataset |
| 2012.14136v1 | On Generating Extended Summaries of Long Documents | 2020 | method | Hierarchical structure-aware extractive summarization; multi-task learning for section-level extraction |
| 2110.07850v1 | End-to-End Segmentation-based News Summarization | 2021 | method+dataset | Joint segmentation + section-level summarization; SegNews dataset |
| 2101.11796v4 | DOC2PPT: Automatic Presentation Slides Generation from Scientific Documents | 2021 | method+dataset | Hierarchical seq2seq for document-to-slide; structure-aware generation with paraphrasing + layout |
| 1911.08836v1 | Table-Of-Contents generation on contemporary documents | 2019 | method+dataset | Neural TOC generation without requiring parsable TOC pages; commercial document domain |

---

## EQ-12: Educational adaptive survey and review generation
**Query**: `educational survey generation adaptive learning materials`
**Category**: `extended_problem`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2502.14776v2 | SurveyX: Academic Survey Automation via Large Language Models | 2025 | method+system | AttributeTree preprocessing + online retrieval + re-polishing; outperforms existing auto-survey systems |
| 2407.09484v1 | GPTutor: Great Personalized Tutor with Large Language Models for Personalized Learning Content Generation | 2024 | tool+method | Personalized content generation with Chain-of-Thoughts prompting; serverless architecture |

---

## EQ-13: Evaluation metrics for automated text quality beyond lexical overlap
**Query**: `evaluation metrics text quality beyond lexical overlap`
**Category**: `extended_benchmark`

| arxiv_id | title | year | likely_role | inclusion_reason |
|----------|-------|------|-------------|-----------------|
| 2010.00490v3 | Towards Question-Answering as an Automatic Metric for Evaluating the Content Quality of a Summary (QAEval) | 2020 | metric | QA-based metric that measures information overlap beyond token matching; outperforms ROUGE |
| 2208.01030v1 | SMART: Sentences as Basic Units for Text Evaluation | 2022 | metric | Sentence-level soft-matching metric with grounding for factuality; SOTA on SummEval |
| 2010.02498v1 | GRUEN for Evaluating Linguistic Quality of Generated Text | 2020 | metric | Reference-less metric for grammaticality, non-redundancy, focus, structure, and coherence |
| 1909.02622v2 | MoverScore: Text Generation Evaluating with Contextualized Embeddings and Earth Mover Distance | 2019 | metric | Contextualized embeddings + distance measure; strong correlation with human judgment across tasks |
| 2110.09147v1 | BEAMetrics: A Benchmark for Language Generation Evaluation Evaluation | 2021 | benchmark | Unified benchmark for comparing metrics across tasks, quality dimensions, and languages |

---

## Deduplication Notes

- **2104.06486v3** (MS2) appeared in both EQ-02 and EQ-05; listed once under EQ-02.
- **2404.07738v2** (ResearchAgent) appeared in both EQ-04 and EQ-06; listed once under EQ-04.
- **2504.08762v1** (InteractiveSurvey) appeared in both EQ-06 and EQ-12; listed once under EQ-06.
- **2509.23981 / 2509.23981v1** identical paper appeared twice in EQ-09 results; listed once.

**Total after deduplication**: 82 unique candidates.

**Source queries with zero/low relevant yield**: EQ-08 (citation analysis) returned mostly bibliometric database comparison papers rather than citation analysis methods; only 3 marginally relevant papers retained. EQ-12 (educational survey) returned mostly question generation papers rather than survey/review synthesis; only 2 retained.
