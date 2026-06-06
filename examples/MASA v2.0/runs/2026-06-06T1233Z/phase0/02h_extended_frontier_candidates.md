# 02h_extended_frontier_candidates.md

**run_dir**: `.`
**source**: `01b_query_plan_extended.md`
**generated**: 2026-06-06T12:47:01+08:00
**scout**: ExtendedFrontierScout

---

## Overview

This file collects candidate papers from the 13 extended queries defined in the Extended Query Plan (`01b_query_plan_extended.md`). Each query targets an underrepresented sub-topic, alternative terminology family, adjacent field, or boundary area identified by gap analysis of the existing candidate pool.

**Relevance signals**: **core** (directly relevant to survey generation), **supporting** (adjacent method or framework that informs survey generation), **boundary** (likely out of scope — kept for conscious exclusion awareness).

---

## EQ-01: Related work generation with large language models
**Query**: "related work generation with large language models"
**Category**: `extended_method`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2205.13339v1 | Target-aware Abstractive Related Work Generation with Contrastive Learning | **core** | Directly generates related work sections using target-aware graph encoder + contrastive learning |
| 2 | 2104.08668v1 | Generating Related Work | **core** | Foundational related work generation; content planning tree + surface realization model |
| 3 | 2412.15249v2 | LitLLMs, LLMs for Literature Review: Are we there yet? | **supporting** | Decomposes literature review into retrieval + planning; zero-shot evaluation |
| 4 | 2306.01779v1 | Conceptual Design Generation Using Large Language Models | **boundary** | Design concept generation, not literature synthesis |
| 5 | 2206.11861v2 | Automatic Generation of Programming Exercises and Code Explanations | **boundary** | Programming education, unrelated to literature survey |
| 6 | 2307.09702v4 | Efficient Guided Generation for Large Language Models | **boundary** | General guided generation (regex/grammar constrained), not survey-specific |
| 7 | 2211.15603v3 | Action-GPT: Leveraging Large-scale Language Models for Action Generation | **boundary** | Text-to-motion generation, unrelated |
| 8 | 1912.02164v4 | Plug and Play Language Models | **boundary** | Controlled text generation framework, pre-dates survey-specific work |
| 9 | 2210.04186v2 | Analogy Generation by Prompting Large Language Models | **boundary** | Analogy generation, unrelated |
| 10 | 2102.07350v1 | Prompt Programming for Large Language Models | **boundary** | Prompt engineering methodology, unrelated |

---

## EQ-02: Multi-document summarization for scientific literature
**Query**: "multi document summarization scientific literature"
**Category**: `extended_method`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2505.16349v1 | Ask, Retrieve, Summarize: A Modular Pipeline for Scientific Literature Summarization | **core** | Modular RAG pipeline for MDS in scientific domain; evaluated on SurveySum |
| 2 | 2402.17311v2 | SKT5SciSumm — Revisiting Extractive-Generative Approach for Multi-Document Scientific Summarization | **core** | SOTA hybrid framework (SPECTER embeddings + T5) on Multi-XScience |
| 3 | 2010.14235v1 | Multi-XScience: A Large-scale Dataset for Extreme Multi-document Summarization of Scientific Articles | **core** | Related-work-as-MDS dataset; directly bridges MDS and survey generation |
| 4 | 2408.16444v1 | SurveySum: A Dataset for Summarizing Multiple Scientific Articles into a Survey Section | **core** | Dedicated dataset for multi-article → survey section summarization |
| 5 | 2508.03962v1 | Accelerating Scientific Discovery with Multi-Document Summarization of Impact-Ranked Papers | **supporting** | MDS integrated with impact-based ranking for literature discovery |
| 6 | 2104.06486v3 | MS2: Multi-Document Summarization of Medical Studies | **supporting** | Large-scale biomedical MDS dataset (470k docs, 20k summaries) |
| 7 | 1706.03449v1 | Scientific document summarization via citation contextualization and scientific discourse | **supporting** | Citation contextualization + discourse facets for scientific summarization |
| 8 | 2011.08072v1 | Topic-Centric Unsupervised Multi-Document Summarization of Scientific and News Articles | **supporting** | Topic-centric unsupervised MDS for scientific articles |
| 9 | 1908.11152v1 | A Summarization System for Scientific Documents | **supporting** | End-to-end system for retrieval + summarization of CS papers |
| 10 | 2312.12915v1 | Survey on Multi-Document Summarization: Systematic Literature Review | **survey** | SLR of MDS methods; contextual reference |

---

## EQ-03: Knowledge grounded long form text generation
**Query**: "knowledge grounded long form text generation"
**Category**: `extended_method`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2402.14207v2 | Assisting in Writing Wikipedia-like Articles From Scratch with Large Language Models (STORM) | **core** | Pre-writing via multi-perspective question asking; directly relevant to survey planning |
| 2 | 2501.03200v1 | The FACTS Grounding Leaderboard: Benchmarking LLMs' Ability to Ground Responses to Long-Form Input | **core** | Long-form factuality grounding benchmark (32k tokens); evaluation methodology transferable |
| 3 | 2403.03866v1 | KIWI: A Dataset of Knowledge-Intensive Writing Instructions for Answering Research Questions | **core** | Iterative revision dataset for scientific writing; expert-annotated instruction-following |
| 4 | 2212.01956v1 | Grounded Keys-to-Text Generation: Towards Factual Open-Ended Generation | **supporting** | Entity description generation with grounding passages and keys |
| 5 | 2204.12681v2 | Building Knowledge-Grounded Dialogue Systems with Graph-Based Semantic Modeling | **boundary** | Dialogue-specific knowledge grounding, not long-form generation |
| 6 | 2005.08365v2 | MixingBoard: a Knowledgeable Stylized Integrated Text Generation Platform | **boundary** | Platform/tooling for constrained generation |
| 7 | 1908.10731v1 | DeepCopy: Grounded Response Generation with Hierarchical Pointer Networks | **boundary** | Dialogue response generation, not long-form |
| 8 | 1702.01932v2 | A Knowledge-Grounded Neural Conversation Model | **boundary** | Dialogue-focused, pre-LLM era |
| 9 | 1903.10245v4 | Knowledge Aware Conversation Generation with Explainable Reasoning | **boundary** | Conversation generation with KG reasoning |
| 10 | 2308.15298v1 | KGConv, a Conversational Corpus grounded in Wikidata | **boundary** | Conversational QA, not generation |

---

## EQ-04: Scientific paper generation and writing with LLMs
**Query**: "scientific paper generation with language models"
**Category**: `extended_method`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2408.06292v3 | The AI Scientist: Towards Fully Automated Open-Ended Scientific Discovery | **core** | Full paper generation pipeline; idea → code → experiment → paper → review |
| 2 | 2508.14273v2 | Let's Use ChatGPT To Write Our Paper! Benchmarking LLMs To Write the Introduction of a Research Paper | **core** | Scientific introduction generation benchmark; 5 models evaluated across multiple dimensions |
| 3 | 2502.05151v2 | Transforming Science with Large Language Models: A Survey on AI-assisted Scientific Discovery | **survey** | Comprehensive survey of AI for scientific research (literature search, idea generation, content generation, peer review) |
| 4 | 2404.07738v2 | ResearchAgent: Iterative Research Idea Generation over Scientific Literature with LLMs | **supporting** | Idea generation with literature grounding; multi-agent review for refinement |
| 5 | 2305.14259v7 | SciMON: Scientific Inspiration Machines Optimized for Novelty | **supporting** | Novelty-optimized idea generation grounded in literature |
| 6 | 2405.17044v3 | Interesting Scientific Idea Generation using Knowledge Graphs and LLMs (SciMuse) | **supporting** | Idea generation with KG + LLM; large-scale human evaluation |
| 7 | 2505.11336v2 | XtraGPT: Context-Aware and Controllable Academic Paper Revision via Human-AI Collaboration | **supporting** | Paper revision framework; section-level revision with instruction following |
| 8 | 2504.17192v3 | Paper2Code: Automating Code Generation from Scientific Papers | **boundary** | Code generation from papers, not paper generation |
| 9 | 1912.01982v1 | Neural Academic Paper Generation | **boundary** | Early LaTeX-level paper generation (character-level LM), pre-LLM era |
| 10 | 2606.05085 | Automatic Generation of Titles for Research Papers Using Language Models | **boundary** | Title generation only, not full paper generation |

---

## EQ-05: Biomedical literature review automation with LLMs
**Query**: "biomedical literature review automation language models"
**Category**: `extended_boundary`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2308.06610v1 | Bio-SIEVE: Exploring Instruction Tuning Large Language Models for Systematic Review Automation | **core** | LLM instruction tuning for medical abstract screening; outperforms ChatGPT |
| 2 | 2311.03056v4 | LitSumm: Large language models for literature summarisation of non-coding RNAs | **core** | Automated literature summarization for RNA biology using LLM + chain of prompts |
| 3 | 2411.18583v1 | Automated Literature Review Using NLP Techniques and LLM-Based Retrieval-Augmented Generation | **core** | Multi-approach comparison (spaCy, T5, GPT-3.5 RAG) for automated lit review |
| 4 | 2405.20354v2 | Efficient Systematic Reviews: Literature Filtering with Transformers & Transfer Learning | **supporting** | Transformer-based filtering for systematic review screening |
| 5 | 2104.06486v3 | MS2: Multi-Document Summarization of Medical Studies | **supporting** | Biomedical MDS dataset; aggregating contradictory evidence across studies |
| 6 | 2307.09683v3 | PubMed and Beyond: Biomedical Literature Search in the Age of Artificial Intelligence | **survey** | Survey of biomedical literature search tools; 36 tools catalogued |
| 7 | 2305.00844v1 | Automated Paper Screening for Clinical Reviews Using Large Language Models | **supporting** | GPT API-based title/abstract screening for clinical reviews |
| 8 | 2408.13450v1 | vitaLITy 2: Reviewing Academic Literature Using Large Language Models | **supporting** | RAG architecture for literature search + summarization in academic domain |
| 9 | 2210.10341v3 | BioGPT: Generative Pre-trained Transformer for Biomedical Text Generation and Mining | **supporting** | Domain-specific generative LM for biomedical text; fluency for biomedical terms |
| 10 | 2002.05637v1 | CBAG: Conditional Biomedical Abstract Generation | **boundary** | Abstract generation from metadata, pre-LLM domain-specific generation |

---

## EQ-06: Interactive human AI collaborative literature synthesis
**Query**: "interactive human AI collaborative literature synthesis"
**Category**: `extended_mechanism`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2504.18496v1 | Facets, Taxonomies, and Syntheses: Navigating Structured Representations in LLM-Assisted Literature Review | **core** | DimInd system; interactive scaffold for literature understanding with structured representations |
| 2 | 2308.07517v1 | Synergi: A Mixed-Initiative System for Scholarly Synthesis and Sensemaking | **core** | Mixed-initiative: user seed threads + citation graph + LLM for synthesis |
| 3 | 2403.02574v1 | ChatCite: LLM Agent with Human Workflow Guidance for Comparative Literature Summary | **core** | Mimics human workflow: key element extraction + Reflective Incremental Mechanism |
| 4 | 2402.12255v1 | Shallow Synthesis of Knowledge in GPT-Generated Texts: A Case Study in Automatic Related Work Composition | **supporting** | Analysis of human-AI collaborative vs pure GPT related work; citation graph analysis |
| 5 | 2504.10861v2 | Ai2 Scholar QA: Organized Literature Synthesis with Attribution | **supporting** | Open-source scientific QA with attribution; full pipeline publicly available |
| 6 | 2411.06159v3 | Mixture of Knowledge Minigraph Agents for Literature Review Generation | **supporting** | CKMAs: collaborative knowledge minigraph agents for automated lit review |
| 7 | 2411.14199v1 | OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs | **supporting** | Specialized RAG from 45M papers; ScholarQABench evaluation; citation accuracy on par with experts |
| 8 | 2504.08762v1 | InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System | **core** | Personalized survey generation; user can customize reference categorization, outline, content |
| 9 | 2404.07738v2 | ResearchAgent: Iterative Research Idea Generation over Scientific Literature | **supporting** | Multi-agent iterative idea generation with reviewing agents |
| 10 | 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow | **core** | IterSurvey: recurrent outline generation + review-and-refine loop with visualization |

---

## EQ-07: Living systematic reviews continuous evidence monitoring
**Query**: "living systematic reviews continuous evidence monitoring"
**Category**: `extended_survey`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2206.04177v1 | Towards Continuous Systematic Literature Review in Software Engineering | **core** | CSLR concept and process for continuous SLR updates; formal BPMN process model |
| 2 | 2108.12922v1 | Continuous Systematic Literature Review: An Approach for Open Science | **core** | CSLR proposal with open science practices; process and tooling support |
| 3 | 2309.01684v1 | CRUISE-Screening: Living Literature Reviews Toolbox | **core** | Web-based living lit review application; periodic search + ML screening |
| 4 | 2402.05317v1 | Emerging Results on Automated Support for Searching and Selecting Evidence for SLR Updates | **supporting** | Automated snowballing + ML selection for SLR updates in SE |
| 5 | 2004.06183v1 | When to Update Systematic Literature Reviews in Software Engineering | **supporting** | Decision framework (3PDF) for SLR update necessity |
| 6 | 2006.05542v1 | Guidelines for the Search Strategy to Update Systematic Literature Reviews | **supporting** | Cost-effective search strategy guidelines for SLR updates |
| 7 | 1709.06758v4 | A shared latent space matrix factorisation method for recommending new trial evidence for SR updates | **supporting** | Matrix factorization for ranking trial registrations for SR updates |
| 8 | 2102.06345v1 | A Visual Analysis Approach to Update Systematic Reviews | **supporting** | USR-VTM: Visual Text Mining approach for SR update evidence selection |
| 9 | 2109.00051v1 | Towards Sustainability of Systematic Literature Reviews | **supporting** | Three-dimension sustainability framework for SLRs (social, economic, technical) |
| 10 | 2109.12141v2 | ALL-IN meta-analysis: breathing life into living systematic reviews | **boundary** | Statistical meta-analysis method (e-values), not generation-focused |

---

## EQ-08: Citation analysis and bibliometric foundations for survey coverage
**Query**: "citation analysis bibliometric coverage literature discovery"
**Category**: `extended_mechanism`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2004.14329v3 | Google Scholar, Microsoft Academic, Scopus, Dimensions, Web of Science, and OpenCitations' COCI | **core** | Largest multidisciplinary comparison of citation data sources; coverage by subject category |
| 2 | 2005.10732v2 | Large-scale comparison of bibliographic data sources: Scopus, Web of Science, Dimensions, Crossref, and Microsoft Academic | **core** | Systematic comparison of 5 data sources; document coverage + citation completeness |
| 3 | 2310.16181v2 | Hidden Citations Obscure True Impact in Science | **core** | Identifies hidden citations (textual credit without reference); full-text ML approach |
| 4 | 2311.06785v1 | Depth and Breadth of Research Area Coverage and Its Impact on Publication Citation | **supporting** | Analysis of how coverage breadth/depth affects citation impact |
| 5 | 1804.09479v3 | Coverage of highly-cited documents in Google Scholar, Web of Science, and Scopus | **supporting** | Multidisciplinary coverage comparison for highly-cited documents |
| 6 | 1804.05365v1 | Dimensions: re-discovering the ecosystem of scientific information | **supporting** | Coverage analysis of Dimensions vs Scopus and Google Scholar |
| 7 | 1806.06351v1 | Google Scholar: the 'big data' bibliographic tool | **supporting** | Google Scholar as bibliometric tool; coverage characteristics |
| 8 | 2501.05821v2 | Analysing the coverage of the University of Bologna's bibliographic and citation metadata in OpenCitations | **boundary** | Institutional-level coverage analysis; too specific |
| 9 | 1703.05539v4 | The coverage of Microsoft Academic: Analyzing the publication output of a university | **boundary** | University-level coverage; pre-2020 tool (MA shutdown) |
| 10 | 1711.08769v1 | Microsoft Academic Automatic Document Searches: Accuracy for Journal Articles | **boundary** | Search accuracy analysis; pre-2020 tool (MA shutdown) |

---

## EQ-09: Systematic review automation with machine learning
**Query**: "systematic review automation machine learning text mining"
**Category**: `extended_boundary`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2006.12166v3 | Open Source Software for Efficient and Transparent Reviews (ASReview) | **core** | Active learning pipeline for systematic review screening; widely used open-source tool |
| 2 | 2211.15397v2 | Automating Systematic Literature Reviews with Natural Language Processing and Text Mining: a SLR | **survey** | SLR of text mining automation of SLR creation; identifies objectives, techniques, gaps |
| 3 | 2010.04665v1 | Scaling Systematic Literature Reviews with Machine Learning Pipelines | **supporting** | End-to-end ML pipeline for SR automation; search + selection + extraction |
| 4 | 2510.26750 | ProfOlaf: Semi-Automated Tool for Systematic Literature Reviews | **supporting** | Semi-automated SR tool: iterative snowballing + LLM analysis |
| 5 | 2202.10033v2 | An open-source integrated framework for the automation of citation collection and screening | **supporting** | Bayesian active ML screening + automatic citation collection |
| 6 | 2509.23981v1 | Automatic selection of primary studies in systematic reviews with evolutionary rule-based classification | **supporting** | Evolutionary ML (grammar-guided GP) for interpretable study selection |
| 7 | 2412.08578v1 | Machine Learning Information Retrieval and Summarisation to Support Systematic Review | **supporting** | ML + NLP pipeline for social science systematic reviews |
| 8 | 1606.06424v1 | A Novel Framework to Expedite Systematic Reviews by Automatically Building Information Extraction Training Corpora | **supporting** | Automatic IE training corpus construction for SR data extraction |
| 9 | 1908.08610v1 | Viability of machine learning to reduce workload in systematic review screenings | **supporting** | SVM-based abstract screening for health sciences; 70% workload reduction |
| 10 | 2004.14329v3 | (duplicate of EQ-08 #1) | | |

---

## EQ-10: Hallucination and factuality in long form text generation
**Query**: "hallucination factuality long form text generation evaluation"
**Category**: `extended_problem`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2403.18802v4 | Long-form factuality in large language models (LongFact + SAFE) | **core** | Foundational: LongFact prompt set; SAFE evaluator with Google Search; F1-based metric |
| 2 | 2406.19276v1 | VERISCORE: Evaluating the factuality of verifiable claims in long-form text generation | **core** | Distinguishes verifiable vs unverifiable claims; applicable across diverse long-form tasks |
| 3 | 2411.15993v1 | Investigating Factuality in Long-Form Text Generation: The Roles of Self-Known and Self-Unknown | **core** | Analysis of factuality degradation across sentence positions; self-assessment capabilities |
| 4 | 2407.17468v1 | WildHallucinations: Evaluating Long-form Factuality in LLMs with Real-World Entity Queries | **core** | Factuality evaluation on entities mined from real-world user-chatbot conversations |
| 5 | 2411.09255v1 | DAHL: Domain-specific Automated Hallucination Evaluation of Long-Form Text (Biomedicine) | **supporting** | Atomic claim decomposition + DAHL Score for biomedical domain |
| 6 | 2410.01691v1 | FactAlign: Long-form Factuality Alignment of Large Language Models | **supporting** | Fine-grained sentence-level alignment (fKTO) for improving factuality |
| 7 | 2505.12265v1 | Learning Auxiliary Tasks Improves Reference-Free Hallucination Detection | **supporting** | RATE-FT: auxiliary task fine-tuning for hallucination detection; tested on LongFact |
| 8 | 2109.09784v2 | Hallucinated but Factual! Inspecting the Factuality of Hallucinations in Abstractive Summarization | **supporting** | Detects factual vs non-factual hallucinations using entity prior/posterior probabilities |
| 9 | 2410.12222v3 | On A Scale From 1 to 5: Quantifying Hallucination in Faithfulness Evaluation | **supporting** | Rubric-based LLM scoring for faithfulness; synthetic unfaithful data generation |
| 10 | 2410.09962v2 | LongHalQA: Long-Context Hallucination Evaluation for MultiModal LLMs | **boundary** | Multimodal (vision + language) hallucination; not text-only generation |

---

## EQ-11: Structure aware retrieval for multi section document generation
**Query**: "structure aware retrieval multi section document generation"
**Category**: `extended_mechanism`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2508.14317v1 | SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans and Memory-Guided Writing | **core** | Coarse-to-fine retrieval; adaptive planning; memory mechanism for cross-section coherence |
| 2 | 1905.10039v1 | Outline Generation: Understanding the Inherent Content Structure of Documents | **core** | HiStGen: hierarchical structured neural model for outline generation; three-level coherence |
| 3 | 2012.14136v1 | On Generating Extended Summaries of Long Documents | **supporting** | Multi-task learning exploiting hierarchical document structure for extended summaries |
| 4 | 2110.07850v1 | End-to-End Segmentation-based News Summarization | **supporting** | Joint document segmentation + section-level summarization |
| 5 | 2101.11796v4 | DOC2PPT: Automatic Presentation Slides Generation from Scientific Documents | **supporting** | Hierarchical seq2seq exploiting document structure for slide generation |
| 6 | 1910.03678v1 | Unfolding the Structure of a Document using Deep Learning | **supporting** | Section classification and structure understanding for large documents |
| 7 | 1709.00770v1 | Understanding the Logical and Semantic Structure of Large Documents | **supporting** | Semantic section identification + labeling for scholarly articles |
| 8 | 1911.08836v1 | Table-Of-Contents generation on contemporary documents | **supporting** | Neural TOC generation pipeline for non-standardized documents |
| 9 | 1901.10133v1 | Structuring an unordered text document | **supporting** | Section segmentation based on keywords/embeddings |
| 10 | 2005.11216v1 | A Generative Approach to Titling and Clustering Wikipedia Sections | **supporting** | Section heading generation; analysis of decoder architectures for semantic encoding |

---

## EQ-12: Educational adaptive survey and review generation
**Query**: "educational survey generation adaptive learning materials"
**Category**: `extended_problem`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2306.02457v1 | Adaptive and Personalized Exercise Generation for Online Language Learning | **boundary** | Exercise generation, not survey generation; method transfer potential limited |
| 2 | 2106.04262v1 | Question Generation for Adaptive Education | **boundary** | Question generation conditioned on student state; different task |
| 3 | 2212.03869v1 | Pre-Training With Scientific Text Improves Educational Question Generation | **boundary** | EduQG: educational question generation from scientific text |
| 4 | 2305.07871v1 | Scalable Educational Question Generation with Pre-trained Language Models | **boundary** | Scalable question generation, not survey generation |
| 5 | 1407.6056v1 | Generation of pedagogical content based on the learning style | **boundary** | Pre-LLM adaptive hypermedia content generation |
| 6 | 2309.15004v1 | Automating question generation from educational text | **boundary** | MCQ generation from textbooks; different paradigm |
| 7 | 2407.09484v1 | GPTutor: Great Personalized Tutor with Large Language Models | **boundary** | Personalized tutoring content, not survey generation |
| 8 | 2509.03535v1 | QuesGenie: Intelligent Multimodal Question Generation | **boundary** | Multimodal question generation |
| 9 | 2502.14776v2 | SurveyX: Academic Survey Automation via LLMs | **boundary** | (Already in main pool as core method; caught here due to query overlap) |
| 10 | 2504.08762v1 | InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System | **boundary** | (Already in main pool; query overlap) |

**Note**: EQ-12 returned mostly educational question generation papers, not survey/review generation for educational purposes. Only minimal method transfer potential. Candidates 9-10 are duplicates from the main pool.

---

## EQ-13: Evaluation metrics for text quality beyond lexical overlap
**Query**: "evaluation metrics text quality beyond lexical overlap"
**Category**: `extended_benchmark`

| # | arXiv ID | Title | Relevance | Cue |
|---|----------|-------|-----------|-----|
| 1 | 2010.00490v3 | Towards Question-Answering as an Automatic Metric for Evaluating Summaries (QAEval) | **core** | QA-based content quality metric; directly measures information overlap beyond ROUGE |
| 2 | 2208.01030v1 | SMART: Sentences as Basic Units for Text Evaluation | **core** | Sentence-level matching with grounding to source for factuality evaluation |
| 3 | 2010.12495v1 | Understanding the Extent to which Summarization Evaluation Metrics Measure Information Quality | **core** | Critical analysis: ROUGE/BERTScore measure topic overlap, not information quality |
| 4 | 1909.02622v2 | MoverScore: Text Generation Evaluating with Contextualized Embeddings and Earth Mover Distance | **core** | Semantic distance metric using contextualized embeddings; cross-task validation |
| 5 | 2110.09147v1 | BEAMetrics: A Benchmark for Language Generation Evaluation Evaluation | **core** | Benchmark for evaluating evaluation metrics across diverse tasks and dimensions |
| 6 | 2010.02498v1 | GRUEN for Evaluating Linguistic Quality of Generated Text | **supporting** | Reference-less metric for grammaticality, non-redundancy, focus, structure, coherence |
| 7 | 2201.05294v1 | Multi-Narrative Semantic Overlap Task: Evaluation and Benchmark | **supporting** | SEM-F1: precision-recall metric for semantic overlap; higher human correlation than ROUGE |
| 8 | 1901.00398v2 | Judge the Judges: A Large-Scale Evaluation Study of Neural Language Models | **supporting** | Meta-evaluation: human vs automated evaluators for review generation |
| 9 | 1810.11878v2 | Unsupervised Evaluation Metrics and Learning Criteria for Non-Parallel Textual Transfer | **supporting** | Multi-aspect evaluation (classification accuracy + semantic preservation + fluency) |
| 10 | 2210.05892v2 | Perplexity from PLM Is Unreliable for Evaluating Text Quality | **supporting** | Critical analysis of PPL limitations (length bias, repetition, punctuation effects) |

---

## Summary

| Query | Core | Supporting | Boundary/Survey | Duplicate | Total |
|-------|------|------------|-----------------|-----------|-------|
| EQ-01 | 2    | 1          | 7               | 0         | 10    |
| EQ-02 | 4    | 5          | 1               | 0         | 10    |
| EQ-03 | 3    | 1          | 6               | 0         | 10    |
| EQ-04 | 2    | 5          | 3               | 0         | 10    |
| EQ-05 | 3    | 5          | 2               | 0         | 10    |
| EQ-06 | 5    | 5          | 0               | 0         | 10    |
| EQ-07 | 3    | 6          | 1               | 0         | 10    |
| EQ-08 | 3    | 4          | 3               | 0         | 10    |
| EQ-09 | 1    | 7          | 1               | 1         | 10    |
| EQ-10 | 4    | 5          | 1               | 0         | 10    |
| EQ-11 | 2    | 8          | 0               | 0         | 10    |
| EQ-12 | 0    | 0          | 8               | 2         | 10    |
| EQ-13 | 5    | 5          | 0               | 0         | 10    |
| **Total** | **37** | **57** | **33** | **3** | **130** |

**Unique candidates recommended for downstream scouts** (core + supporting, deduplicated): ~84

---

## Candidates for downstream scouts

### Core candidates (37) — directly relevant to survey generation:

**EQ-01 (Related work generation)**: Target-aware Related Work (2205.13339), Generating Related Work (2104.08668)

**EQ-02 (MDS for scientific literature)**: XSum (2505.16349), SKT5SciSumm (2402.17311), Multi-XScience (2010.14235), SurveySum (2408.16444)

**EQ-03 (Knowledge grounded long-form)**: STORM (2402.14207), FACTS Grounding (2501.03200), KIWI (2403.03866)

**EQ-04 (Scientific paper generation)**: AI Scientist (2408.06292), SciIG Benchmark (2508.14273)

**EQ-05 (Biomedical lit review)**: Bio-SIEVE (2308.06610), LitSumm (2311.03056), Auto Lit Review (2411.18583)

**EQ-06 (Interactive/Human-AI)**: DimInd (2504.18496), Synergi (2308.07517), ChatCite (2403.02574), InteractiveSurvey (2504.08762), IterSurvey (2510.21900)

**EQ-07 (Living systematic reviews)**: CSLR (2206.04177), CSLR Open Science (2108.12922), CRUISE-Screening (2309.01684)

**EQ-08 (Bibliometrics)**: Citation source comparison (2004.14329), Data source comparison (2005.10732), Hidden Citations (2310.16181)

**EQ-09 (Systematic review ML)**: ASReview (2006.12166)

**EQ-10 (Hallucination/factuality)**: LongFact+SAFE (2403.18802), VERISCORE (2406.19276), Factuality in Long-Form (2411.15993), WildHallucinations (2407.17468)

**EQ-11 (Structure-aware retrieval)**: SurveyGen-I (2508.14317), Outline Generation (1905.10039)

**EQ-13 (Evaluation metrics)**: QAEval (2010.00490), SMART (2208.01030), Information Quality analysis (2010.12495), MoverScore (1909.02622), BEAMetrics (2110.09147)

### Noteworthy boundary papers for conscious exclusion:

- **FACTS Grounding (2501.03200)** — could be moved to supporting if evaluation methodology is directly applicable
- **AI Scientist (2408.06292)** — bridges survey generation and full scientific discovery automation; likely relevant for Section 7 (Future Directions)
- **Hidden Citations (2310.16181)** — relevant to Section 5 (Coverage and Completeness Challenges)
- **ASReview (2006.12166)** — pre-LLM but methodologically influential for screening automation
