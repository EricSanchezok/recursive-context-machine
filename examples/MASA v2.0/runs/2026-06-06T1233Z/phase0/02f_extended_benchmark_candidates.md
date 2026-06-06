# Extended Benchmark Candidates

**run_dir**: `.`
**scout**: ExtendedBenchmarkScout
**generated**: 2026-06-06T12:46:56+08:00
**source_queries**: EQ-13 (evaluation metrics beyond lexical overlap), EQ-10 (hallucination/factuality evaluation), EQ-03 (knowledge grounded generation), EQ-02 (multi-document summarization scientific literature), EQ-01 (related work generation), EQ-07 (living systematic reviews), EQ-09 (systematic review automation ML), EQ-11 (structure aware retrieval)
**deduplication_against**: `02_candidate_pool.md` (82 existing candidates)
**new_candidates_found**: 26

---

## Selection Criteria

Candidates are included if they contribute:
- A **benchmark**, **dataset**, or **metric** directly applicable to evaluating automated survey generation
- An **evaluation protocol** or **leaderboard** for long-form, knowledge-grounded, or scientific text quality
- A **factuality/hallucination evaluation framework** applicable to survey citation fidelity assessment
- A **multi-document summarization evaluation dataset** that could transfer to survey evaluation

Papers already in the main candidate pool (via `02b_benchmark_candidates.md`) are **excluded**.

---

## EQ-13: Evaluation Metrics for Text Quality Beyond Lexical Overlap

### 1. QAEval — QA-based Summary Evaluation
- **arXiv**: 2010.00490
- **Title**: *Towards Question-Answering as an Automatic Metric for Evaluating the Content Quality of a Summary*
- **Authors**: Daniel Deutsch, Tania Bedrax-Weiss, Dan Roth
- **Date**: Oct 2020
- **Tags**: metric, QA-based, content quality, reference-based
- **Relevance to survey evaluation**: Proposes QA-based metric (QAEval) that directly measures information overlap between generated and reference text. Fundamentally different from ROUGE — measures whether a reader can answer questions about the reference using the generated text. Directly applicable to evaluating factual content coverage in survey sections.
- **Notes**: Outperforms ROUGE/BERTScore on summarization meta-evaluation benchmarks. Core idea translates naturally to survey evaluation (quiz-based answerability).

### 2. SMART — Sentence Matching for Text Evaluation
- **arXiv**: 2208.01030
- **Title**: *SMART: Sentences as Basic Units for Text Evaluation*
- **Authors**: Reinald Kim Amplayo, Peter J. Liu, Yao Zhao, Shashi Narayan
- **Date**: Aug 2022
- **Tags**: metric, sentence-level, factuality
- **Relevance to survey evaluation**: Treats sentences (not tokens) as matching units; can evaluate factuality by comparing candidate sentences to source documents. Supports both model-based and string-based matching. Strong performance on SummEval meta-evaluation. Relevant for evaluating survey factuality against cited sources.
- **Notes**: No neural model needed for string-based variant, making it practical for rapid survey evaluation iterations.

### 3. GRUEN — Linguistic Quality Evaluation
- **arXiv**: 2010.02498
- **Title**: *GRUEN for Evaluating Linguistic Quality of Generated Text*
- **Authors**: Wanzheng Zhu, Suma Bhat
- **Date**: Oct 2020
- **Tags**: metric, linguistic quality, reference-less
- **Relevance to survey evaluation**: Reference-less metric evaluating grammaticality, non-redundancy, focus, structure, and coherence. Unsupervised, deterministic, adaptable. Useful for evaluating survey fluency and structural quality without needing human references.
- **Notes**: Complements content-focused metrics (FActScore, QAEval) by covering the linguistic quality dimension that survey benchmarks currently under-evaluate.

### 4. MoverScore — Contextualized Embeddings + Earth Mover Distance
- **arXiv**: 1909.02622
- **Title**: *MoverScore: Text Generation Evaluating with Contextualized Embeddings and Earth Mover Distance*
- **Authors**: Wei Zhao, Maxime Peyrard, Fei Liu, Yang Gao, Christian M. Meyer, Steffen Eger
- **Date**: Sep 2019
- **Tags**: metric, semantic similarity, BERT-based
- **Relevance to survey evaluation**: Combines BERT contextualized embeddings with Earth Mover's Distance to measure semantic similarity beyond lexical overlap. Generalizes across summarization, MT, captioning, data-to-text. Applicable as a semantic similarity baseline for survey-structure-aware evaluation.
- **Notes**: Pre-dates the LLM era but remains a strong semantically-aware metric. Available as web service.

### 5. BEAMetrics — Benchmark for Language Generation Evaluation Evaluation
- **arXiv**: 2110.09147
- **Title**: *BEAMetrics: A Benchmark for Language Generation Evaluation Evaluation*
- **Authors**: Thomas Scialom, Felix Hill
- **Date**: Oct 2021
- **Tags**: benchmark, meta-evaluation, metric comparison
- **Relevance to survey evaluation**: Provides a unified framework to compare automatic metrics against human judgments across diverse generation tasks and quality dimensions (fluency, coherence, informativeness). Useful for selecting which survey evaluation metrics to trust, and for meta-evaluating any proposed metric.
- **Notes**: Reveals stark task-dependent differences between metrics. Relevant for understanding when ROUGE/BLEU references fail in survey evaluation.

### 6. Multi-Narrative Semantic Overlap / SEM-F1
- **arXiv**: 2201.05294
- **Title**: *Multi-Narrative Semantic Overlap Task: Evaluation and Benchmark*
- **Authors**: Naman Bansal, Mousumi Akter, Shubhra Kanti Karmaker Santu
- **Date**: Jan 2022
- **Tags**: metric, semantic overlap, precision-recall
- **Relevance to survey evaluation**: Proposes SEM-F1 metric for evaluating semantic overlap between multiple narratives. Precision-recall style metric shown to correlate better with human judgment than ROUGE. Directly relevant for evaluating how well a survey covers semantic content from source papers.
- **Notes**: Uses human annotations to create ground-truth semantic overlap labels. The SEM-F1 formulation could transfer to survey coverage evaluation.

---

## EQ-10: Hallucination and Factuality in Long-Form Text Generation

### 7. LongFact + SAFE — Long-Form Factuality Benchmark
- **arXiv**: 2403.18802
- **Title**: *Long-form factuality in large language models*
- **Authors**: Jerry Wei, Chengrun Yang, Xinying Song et al. (Google DeepMind)
- **Date**: Mar 2024
- **Tags**: benchmark, factuality, evaluation framework, leaderboard
- **Relevance to survey evaluation**: Introduces **LongFact** (thousands of questions across 38 topics) and **SAFE** (Search-Augmented Factuality Evaluator) which decomposes long-form responses into atomic facts and verifies each via Google Search. SAFE outperforms crowdsourced human annotators at 1/20th the cost. Extends F1 score as aggregated metric for long-form factuality. **Directly applicable** to evaluating factual precision of survey sections against cited sources.
- **Notes**: Benchmark of 13 models across Gemini, GPT, Claude, PaLM-2. The atomic fact decomposition + search-based verification approach is a template for survey citation fidelity evaluation.

### 8. VERISCORE — Factuality Metric for Verifiable Claims in Long-Form Text
- **arXiv**: 2406.19276
- **Title**: *VERISCORE: Evaluating the factuality of verifiable claims in long-form text generation*
- **Authors**: Yixiao Song, Yekyung Kim, Mohit Iyyer
- **Date**: Jun 2024
- **Tags**: metric, factuality, verifiable claims, long-form
- **Relevance to survey evaluation**: Addresses a key limitation of FActScore and SAFE — they assume all claims are verifiable. VERISCORE distinguishes verifiable from unverifiable claims, more suitable for diverse generation tasks including surveys (which contain both factual claims and opinion/synthesis). Evaluated 16 models across 8 tasks. Shows factuality does not correlate across tasks.
- **Notes**: Critical methodology for survey evaluation where some content is synthesis/analysis (not directly verifiable). Using VERISCORE, one could evaluate factuality of verifiable citations separately from quality of synthesis.

### 9. WildHallucinations — Real-World Entity Query Factuality Benchmark
- **arXiv**: 2407.17468
- **Title**: *WildHallucinations: Evaluating Long-form Factuality in LLMs with Real-World Entity Queries*
- **Authors**: Wenting Zhao, Tanya Goyal, Yu Ying Chiu et al.
- **Date**: Jul 2024
- **Tags**: benchmark, factuality, entity queries, real-world
- **Relevance to survey evaluation**: Evaluates factuality on entities mined from real user-chatbot conversations. 118,785 generations across 15 LLMs on 7,919 entities. Finds LLMs hallucinate more on entities without Wikipedia pages. Relevant for understanding factuality boundaries in survey generation — surveys on niche topics may face similar challenges.
- **Notes**: Shows adding retrieval components only slightly reduces hallucinations. Important caution for survey generation systems relying on RAG.

### 10. DAHL — Domain-Specific Automated Hallucination Evaluation (Biomedicine)
- **arXiv**: 2411.09255
- **Title**: *DAHL: Domain-specific Automated Hallucination Evaluation of Long-Form Text through a Benchmark Dataset in Biomedicine*
- **Authors**: Jean Seo, Jongwon Lim, Dongjun Jang, Hyopil Shin
- **Date**: Nov 2024
- **Tags**: benchmark, hallucination, biomedical, atomic fact decomposition
- **Relevance to survey evaluation**: 8,573 questions across 29 biomedical categories. Decomposes responses into atomic units (similar to FActScore). Introduces DAHL Score as hallucination metric. Demonstrates scaling threshold (7-8B parameters) beyond which size doesn't improve factuality. Methodological template for domain-specific survey evaluation.
- **Notes**: The atomic decomposition approach is directly transferable to survey evaluation. Domain-specific benchmark construction methodology applicable.

### 11. Factuality in Long-Form Text: Self-Known and Self-Unknown
- **arXiv**: 2411.15993
- **Title**: *Investigating Factuality in Long-Form Text Generation: The Roles of Self-Known and Self-Unknown*
- **Authors**: Lifu Tu, Rui Meng, Shafiq Joty, Yingbo Zhou, Semih Yavuz
- **Date**: Nov 2024
- **Tags**: analysis, factuality, self-assessment, long-form
- **Relevance to survey evaluation**: Reveals that factuality declines in later sentences of long-form generation — directly relevant to survey structure where later sections may be less reliable. Introduces Self-Known/Self-Unknown framework for analyzing LLMs' ability to judge their own outputs. Applicable for designing self-assessment evaluation protocols for survey generation.
- **Notes**: Analyzes GPT-4, Gemini-1.5-Pro, Claude-3, Llama-3-70B. Finding about declining factuality over generation length is a critical consideration for multi-section survey evaluation.

### 12. FactAlign — Long-form Factuality Alignment
- **arXiv**: 2410.01691
- **Title**: *FactAlign: Long-form Factuality Alignment of Large Language Models*
- **Authors**: Chao-Wei Huang, Yun-Nung Chen
- **Date**: Oct 2024
- **Tags**: method, alignment, factuality, sentence-level
- **Relevance to survey evaluation**: Proposes fKTO fine-grained sentence-level alignment algorithm for improving long-form factuality. While primarily a method paper, its evaluation methodology (fine-grained factuality assessments for alignment) is relevant for designing survey evaluation protocols.
- **Notes**: The sentence-level fine-grained factuality assessment methodology is transferable.

---

## EQ-03: Knowledge-Grounded Long-Form Text Generation

### 13. FACTS Grounding Leaderboard
- **arXiv**: 2501.03200
- **Title**: *The FACTS Grounding Leaderboard: Benchmarking LLMs' Ability to Ground Responses to Long-Form Input*
- **Authors**: Alon Jacovi, Andrew Wang, Chris Alberti et al. (Google)
- **Date**: Jan 2025
- **Tags**: benchmark, leaderboard, grounding, long-form
- **Relevance to survey evaluation**: **Highly relevant.** Online leaderboard evaluating LMs' ability to generate text factually grounded in provided 32k-token context documents. Two-phase evaluation: (1) disqualify if request unfulfilled, (2) judge if fully grounded. Uses automated judge models with test set validation. The grounding evaluation framework directly translates to evaluating whether survey content is faithful to cited sources.
- **Notes**: Public leaderboard on Kaggle with public/private splits. Actively maintained. The two-phase evaluation design (relevance check + grounding check) could inform survey evaluation protocol design.

### 14. STORM + FreshWiki Dataset
- **arXiv**: 2402.14207
- **Title**: *Assisting in Writing Wikipedia-like Articles From Scratch with Large Language Models*
- **Authors**: Yijia Shao, Yucheng Jiang, Theodore A. Kanell, Peter Xu, Omar Khattab, Monica S. Lam
- **Date**: Feb 2024
- **Tags**: method, dataset, long-form, grounded writing
- **Relevance to survey evaluation**: Introduces **FreshWiki** dataset (recent high-quality Wikipedia articles) and outline assessment methodology for evaluating pre-writing quality. STORM's multi-perspective questioning approach is a template for survey planning evaluation. Outline assessment methodology transferable to survey outline evaluation.
- **Notes**: Expert feedback from Wikipedia editors identifies challenges (source bias transfer, over-association of unrelated facts) that are equally relevant to survey generation.

### 15. KIWI — Knowledge-Intensive Writing Instructions Dataset
- **arXiv**: 2403.03866
- **Title**: *KIWI: A Dataset of Knowledge-Intensive Writing Instructions for Answering Research Questions*
- **Authors**: Fangyuan Xu, Kyle Lo, Luca Soldaini, Bailey Kuehl, Eunsol Choi, David Wadden
- **Date**: Mar 2024
- **Tags**: dataset, instruction-following, evaluation, scientific writing
- **Relevance to survey evaluation**: 1,260 interaction turns from 234 sessions where experts iteratively instruct LLMs to revise long-form scientific answers. Includes human evaluation of each response. Reveals models struggle to incorporate new information and perform precise edits. Directly relevant for evaluating iterative survey refinement capabilities.
- **Notes**: From Allen AI. The interaction structure (instruction + model response + human eval) provides a template for evaluating survey revision/refinement pipelines.

---

## EQ-02: Multi-Document Summarization for Scientific Literature

### 16. MS² — Multi-Document Summarization of Medical Studies
- **arXiv**: 2104.06486
- **Title**: *MS²: Multi-Document Summarization of Medical Studies*
- **Authors**: Jay DeYoung, Iz Beltagy, Madeleine van Zuylen, Bailey Kuehl, Lucy Lu Wang
- **Date**: Apr 2021
- **Tags**: dataset, biomedical, multi-document summarization
- **Relevance to survey evaluation**: **470k+ documents, 20k summaries** derived from scientific literature. First large-scale public MDS dataset in the biomedical domain. Designed for systems that assess and aggregate contradictory evidence across multiple studies — directly analogous to survey generation task. Uses BART-based system with structured and free-text formulations.
- **Notes**: From Allen AI. The contradictory-evidence aggregation challenge is a subset of the survey generation problem. Dataset structure (studies → evidence synthesis → summary) mirrors survey pipeline.

### 17. Multi-XScience — Extreme Multi-Document Summarization Dataset
- **arXiv**: 2010.14235
- **Title**: *Multi-XScience: A Large-scale Dataset for Extreme Multi-document Summarization of Scientific Articles*
- **Authors**: Yao Lu, Yue Dong, Laurent Charlin
- **Date**: Oct 2020
- **Tags**: dataset, related-work, multi-document summarization
- **Relevance to survey evaluation**: Constructed from scientific articles — task is writing the related-work section of a paper based on its abstract and referenced articles. Closely related to survey generation. Used for training/evaluating abstractive models.
- **Notes**: The related-work formulation (generate section from abstract + references) directly mirrors one of the core survey generation evaluation tasks.

### 18. SurveySum — Dataset for Summarizing Multiple Scientific Articles into a Survey Section
- **arXiv**: 2408.16444
- **Title**: *SurveySum: A Dataset for Summarizing Multiple Scientific Articles into a Survey Section*
- **Authors**: Leandro Carísio Fernandes et al.
- **Date**: Aug 2024
- **Tags**: dataset, survey section, evaluation
- **Relevance to survey evaluation**: **Directly relevant.** Novel dataset for summarizing multiple scientific articles into a single survey section. Evaluates two specific pipelines across multiple metrics. Addresses the gap in domain-specific summarization tools for survey content.
- **Notes**: Focused on section-level rather than full-survey evaluation. Complements full-survey benchmarks (SurveyBench, SurGE) by enabling fine-grained section quality assessment.

### 19. XSum — Modular RAG Pipeline for Scientific MDS
- **arXiv**: 2505.16349
- **Title**: *Ask, Retrieve, Summarize: A Modular Pipeline for Scientific Literature Summarization*
- **Authors**: Pierre Achkar, Tim Gollub, Martin Potthast
- **Date**: May 2025
- **Tags**: pipeline, evaluation, scientific MDS
- **Relevance to survey evaluation**: Evaluated on SurveySum dataset using CheckEval, G-Eval, and Ref-F1. Question-generation + editor modules for synthesizing scientific summaries. Provides an evaluation benchmark (SurveySum) and metric baselines directly transferable to survey generation evaluation.
- **Notes**: Open-source (github.com/webis-de/scolia25-xsum). The question-generation module is a template for evaluating survey content comprehensiveness.

### 20. SKT5SciSumm — Extractive-Generative for Scientific MDS
- **arXiv**: 2402.17311
- **Title**: *SKT5SciSumm — Revisiting Extractive-Generative Approach for Multi-Document Scientific Summarization*
- **Authors**: Huy Quoc To et al.
- **Date**: Feb 2024
- **Tags**: method, evaluation, scientific MDS
- **Relevance to survey evaluation**: SOTA on Multi-XScience dataset. SPECTER embeddings + k-means extraction + T5 generation. While primarily methodological, its evaluation on Multi-XScience provides a baseline for scientific summarization quality that survey systems can be compared against.
- **Notes**: The SPECTER + clustering approach for scientific text encoding is relevant for retrieval/evaluation of scientific content in surveys.

---

## EQ-01: Related Work Generation

### 21. TAG — Target-aware Abstractive Related Work Generator
- **arXiv**: 2205.13339
- **Title**: *Target-aware Abstractive Related Work Generation with Contrastive Learning*
- **Authors**: Xiuying Chen et al.
- **Date**: May 2022
- **Tags**: method, dataset, related work, evaluation
- **Relevance to survey evaluation**: Proposes target-aware graph encoder + hierarchical decoder + contrastive optimization for related work generation. Evaluates on two public scholar datasets with automatic and human evaluations. The evaluation methodology (mutual information between generated and reference works) is transferable to survey evaluation.
- **Notes**: Pre-LLM approach but evaluation framework (contrastive: maximize MI with references, minimize with non-references) is conceptually relevant.

### 22. Generating Related Work — Content Planning Model
- **arXiv**: 2104.08668
- **Title**: *Generating Related Work*
- **Authors**: Darsh J Shah, Regina Barzilay
- **Date**: Apr 2021
- **Tags**: method, dataset, related work, planning
- **Relevance to survey evaluation**: Models generating related work sections with explicit content planning (tree of cited papers) before surface realization. Contributes an ACL Anthology-based dataset. Outperforms SOTA summarization and MDS models. The content planning approach is a direct precursor to survey outline generation evaluation.
- **Notes**: From MIT. The tree-structured planning evaluation provides a template for evaluating survey outline/planning quality.

---

## EQ-07: Living Systematic Reviews

### 23. CRUISE-Screening — Living Literature Reviews Toolbox
- **arXiv**: 2309.01684
- **Title**: *CRUISE-Screening: Living Literature Reviews Toolbox*
- **Authors**: Wojciech Kusa, Petr Knoth, Allan Hanbury
- **Date**: Sep 2023
- **Tags**: tool, living review, screening, evaluation
- **Relevance to survey evaluation**: Web-based application for conducting living literature reviews (continuously updated). Connected to search engines via API, uses text classification and QA models for citation screening. Open-source. Provides an evaluation framework for maintaining up-to-date literature reviews — directly relevant for evaluating survey update/continuity mechanisms.
- **Notes**: From TU Wien. Relevant for the living/continuous survey dimension of survey generation evaluation.

---

## EQ-09: Systematic Review Automation with ML (Boundary)

### 24. ASReview — Open-Source ML-Aided Systematic Review Pipeline
- **arXiv**: 2006.12166
- **Title**: *Open Source Software for Efficient and Transparent Reviews*
- **Authors**: Rens van de Schoot et al.
- **Date**: Jun 2020
- **Tags**: tool, active learning, screening, open-source
- **Relevance to survey evaluation**: ML-aided pipeline using active learning for title/abstract screening. Widely used in systematic review practice. Provides efficiency benchmarks (simulation studies show far more efficient than manual). The active learning + screening evaluation methodology is relevant for evaluating retrieval/selection components of survey generation systems.
- **Notes**: Boundary candidate — pre-LLM but establishes screening evaluation standards. Could contextualize LLM-era improvements.

### 25. Scaling Systematic Literature Reviews with ML Pipelines
- **arXiv**: 2010.04665
- **Title**: *Scaling Systematic Literature Reviews with Machine Learning Pipelines*
- **Authors**: Seraphina Goldfarb-Tarrant et al.
- **Date**: Oct 2020
- **Tags**: pipeline, evaluation, screening, extraction
- **Relevance to survey evaluation**: Constructs full pipeline (search → selection → extraction) with experiments on human-time vs. system-quality tradeoffs. Tests classifier generalization across countries. Shows surprising accuracy with only 2 weeks of annotation (15% of manual time). The pipeline evaluation methodology informs survey system component-wise evaluation.
- **Notes**: Boundary candidate. The quality-vs-cost tradeoff analysis is relevant for evaluating survey generation efficiency.

---

## EQ-11: Structure-Aware Retrieval for Multi-Section Document Generation

### 26. HiStGen + WIKIOG — Outline Generation Dataset and Model
- **arXiv**: 1905.10039
- **Title**: *Outline Generation: Understanding the Inherent Content Structure of Documents*
- **Authors**: Ruqing Zhang, Jiafeng Guo, Yixing Fan, Yanyan Lan, Xueqi Cheng
- **Date**: May 2019
- **Tags**: dataset, outline generation, structure, evaluation
- **Relevance to survey evaluation**: Introduces **WIKIOG dataset** (1.75M document-outline pairs) for outline generation task. Proposes HiStGen hierarchical structured generation model. Three-level coherence modeling (context paragraphs, section-heading, inter-heading coherence). Directly relevant for evaluating survey outline quality — a dimension currently under-evaluated in existing survey benchmarks.
- **Notes**: The WIKIOG dataset and outline evaluation methodology could supplement existing survey benchmarks that evaluate outline quality (e.g., SurveyForge, SurveyBench).

---

## Summary Statistics

| Source Query | New Candidates | Key Papers |
|---|---|---|
| EQ-13 (beyond lexical overlap metrics) | 6 | QAEval, SMART, GRUEN, MoverScore, BEAMetrics, SEM-F1 |
| EQ-10 (hallucination/factuality evaluation) | 6 | LongFact+SAFE, VERISCORE, WildHallucinations, DAHL, Self-Known/Self-Unknown, FactAlign |
| EQ-03 (knowledge grounded generation) | 3 | FACTS Grounding, STORM+FreshWiki, KIWI |
| EQ-02 (multi-document summarization) | 5 | MS², Multi-XScience, SurveySum, XSum, SKT5SciSumm |
| EQ-01 (related work generation) | 2 | TAG, Generating Related Work |
| EQ-07 (living systematic reviews) | 1 | CRUISE-Screening |
| EQ-09 (systematic review automation ML) | 2 | ASReview, Scaling SLRs with ML |
| EQ-11 (structure aware retrieval) | 1 | HiStGen + WIKIOG |
| **Total** | **26** | |

---

## Top Candidates for Inclusion in Survey (by relevance to automated survey evaluation)

1. **LongFact + SAFE** (2403.18802) — Most comprehensive long-form factuality benchmark; atomic fact decomposition + search-based verification directly transferable to survey citation fidelity evaluation.
2. **FACTS Grounding Leaderboard** (2501.03200) — Active leaderboard for evaluating grounding in long-form context; two-phase evaluation design (request fulfillment + grounding) applicable to survey evaluation protocol design.
3. **VERISCORE** (2406.19276) — Addresses verifiable vs. unverifiable claim distinction essential for survey evaluation where synthesis/analysis content is not directly verifiable.
4. **SurveySum** (2408.16444) — Only dedicated dataset for summarizing articles into survey sections; fills the section-level evaluation gap.
5. **MS²** (2104.06486) — Largest public MDS dataset in biomedical domain; models contradictory evidence aggregation — a core survey task.

## Risks

1. **Year range**: Several candidates (QAEval 2020, MoverScore 2019, HiStGen 2019, MS² 2021, ASReview 2020) fall outside the spec's 2023–2025 focus window. Retained for methodological value; downstream should filter if strict time window applies.
2. **Boundary candidates**: EQ-09 candidates (ASReview, Scaling SLRs) are pre-LLM systematic review automation. Include only to contextualize LLM-era advances, not as primary survey generation evaluation tools.
3. **Multi-disciplinary coverage gap**: MS², DAHL, and CRUISE-Screening are biomedical-focused; their domain-specific evaluation methodology may not directly transfer to general scientific survey evaluation.
4. **Equivalence to existing pool**: Several results (SurveyGen-I, FActScore, HALoGEN) were already in the main candidate pool and excluded via deduplication.
