# Benchmark Candidates

**run_dir**: `.`
**scout**: BenchmarkScout
**generated**: 2026-06-06T12:35:23+08:00
**source_queries**: B-01 (benchmarks & automatic metrics), B-02 (human evaluation protocols), B-03 (citation precision/recall), evaluation-framework supplement, hallucination-evaluation supplement

---

## Dedicated Survey-Generation Benchmarks

These papers propose a benchmark, dataset, or evaluation framework specifically for LLM-generated academic surveys/literature reviews. Highest relevance.

### 1. SurveyBench
- **arXiv**: 2510.03120
- **Title**: *SurveyBench: Can LLM(-Agents) Write Academic Surveys that Align with Reader Needs?*
- **Authors**: Zhaojun Sun, Xuzhou Zhu, Xuanhe Zhou et al.
- **Date**: Oct 2025
- **Tags**: benchmark, evaluation framework, dataset
- **Key points**:
  - Quiz-driven evaluation framework; 11,343 arXiv topic sources, 4,947 high-quality surveys
  - Multifaceted metric hierarchy: outline quality (coverage breadth, logical coherence), content quality (synthesis granularity, clarity), non-textual richness
  - Dual-mode evaluation: content-based + quiz-based answerability tests aligned with reader needs
  - Shows existing LLM4Survey approaches score ~21% lower than human baselines

### 2. SurGE (Survey Generation Evaluation)
- **arXiv**: 2508.15658
- **Title**: *Benchmarking Computer Science Survey Generation*
- **Authors**: Weihang Su, Anzhe Xie, Qingyao Ai et al.
- **Date**: Aug 2025
- **Tags**: benchmark, dataset, evaluation protocol
- **Key points**:
  - CS-domain benchmark: test instances with topic description, expert-written survey, cited references
  - Large-scale retrieval pool of 1M+ papers
  - 4-dimension automated evaluation: information coverage, referencing accuracy, structural organization, content quality
  - Open-source: https://github.com/oneal2000/SurGE

### 3. SurveyEval
- **arXiv**: 2512.02763
- **Title**: *SurveyEval: Towards Comprehensive Evaluation of LLM-Generated Academic Surveys*
- **Authors**: Jiahao Zhao, Shuaixing Zhang, Nan Xu, Lei Wang
- **Date**: Dec 2025
- **Tags**: benchmark, evaluation framework
- **Key points**:
  - 3-dimension evaluation: overall quality, outline coherence, reference accuracy
  - 7 subjects across disciplines
  - LLM-as-a-Judge + human references for evaluation-human alignment

### 4. DeepSurvey-Bench
- **arXiv**: 2601.15307
- **Title**: *DeepSurvey-Bench: Evaluating Academic Value of Automatically Generated Scientific Survey*
- **Authors**: Guo-Biao Zhang, Ding-Yuan Liu, Da-Yi Wu et al.
- **Date**: Jan 2026
- **Tags**: benchmark, evaluation criteria
- **Key points**:
  - Evaluates "academic value" beyond surface quality
  - 3 value dimensions: informational value, scholarly communication value, research guidance value
  - Human-annotated dataset for academic value
  - Addresses flaws in existing benchmarks (citation counts, structural coherence alone are insufficient)

### 5. SGSimEval
- **arXiv**: 2508.11310
- **Title**: *SGSimEval: A Comprehensive Multifaceted and Similarity-Enhanced Benchmark for Automatic Survey Generation Systems*
- **Authors**: Beichen Guo, Zhiyuan Wen, Yu Yang et al.
- **Date**: Aug 2025
- **Tags**: benchmark, evaluation framework, human preference
- **Key points**:
  - Evaluates outline, content, and references
  - Combines LLM-based scoring with quantitative metrics
  - Introduces human preference metrics emphasizing inherent quality and similarity to humans

### 6. SurveyLens
- **arXiv**: 2602.11238
- **Title**: *SurveyLens: A Research Discipline-Aware Benchmark for Automatic Survey Generation*
- **Authors**: Beichen Guo, Zhiyuan Wen, Jia Gu et al.
- **Date**: Feb 2026
- **Tags**: benchmark, discipline-aware evaluation
- **Key points**:
  - First discipline-aware benchmark; 1,000 human-written surveys across 10 disciplines
  - Dual-lens evaluation: Discipline-Aware Rubric Evaluation + Canonical Alignment Evaluation
  - Evaluates 11 ASG methods including Vanilla LLMs, ASG systems, and Deep Research agents

### 7. Survey-Arena (from IterSurvey)
- **arXiv**: 2510.21900
- **Title**: *Deep Literature Survey Automation with an Iterative Workflow*
- **Authors**: Hongbo Zhang, Han Cui, Yidong Wang et al.
- **Date**: Oct 2025
- **Tags**: pairwise benchmark, comparison framework
- **Key points**:
  - Pairwise benchmark for machine-generated vs. human-written surveys
  - Complements absolute scoring approaches
  - Evaluates content coverage, structural coherence, citation quality

### 8. SurveyForge's SurveyBench
- **arXiv**: 2503.04629
- **Title**: *SurveyForge: On the Outline Heuristics, Memory-Driven Generation, and Multi-dimensional Evaluation*
- **Authors**: Xiangchao Yan, Shiyang Feng, Jiakang Yuan et al.
- **Date**: Mar 2025
- **Tags**: benchmark, win-rate comparison
- **Key points**:
  - 100 human-written surveys for win-rate comparison
  - 3-dimension assessment: reference, outline, content quality

---

## Datasets for Survey Generation

These papers contribute large-scale datasets used to train or evaluate survey generation systems.

### 9. SciReviewGen
- **arXiv**: 2305.15186
- **Title**: *SciReviewGen: A Large-scale Dataset for Automatic Literature Review Generation*
- **Authors**: Tetsu Kasanishi, Masaru Isonuma, Junichiro Mori, Ichiro Sakata
- **Date**: May 2023
- **Tags**: dataset, literature reviews
- **Key points**:
  - 10,000+ literature reviews, 690,000 cited papers
  - Human evaluation benchmarks for transformer-based summarization models
  - Reveals hallucination challenges

### 10. SurveyGen
- **arXiv**: 2508.17647
- **Title**: *SurveyGen: Quality-Aware Scientific Survey Generation with Large Language Models*
- **Authors**: Tong Bao, Mir Tafseer Nayeem, Davood Rafiei, Chengzhi Zhang
- **Date**: Aug 2025
- **Tags**: dataset, quality metadata
- **Key points**:
  - 4,200+ human-written surveys across diverse domains
  - 242,143 cited references with quality-related metadata
  - Supports quality-aware RAG pipeline evaluation

### 11. Hierarchical Catalogue Generation Dataset
- **arXiv**: 2304.03512
- **Title**: *Hierarchical Catalogue Generation for Literature Review: A Benchmark*
- **Authors**: Kun Zhu, Xiaocheng Feng, Xiachong Feng et al.
- **Date**: Apr 2023
- **Tags**: dataset, catalogue generation
- **Key points**:
  - 7.6K literature review catalogues, 389K reference papers
  - Evaluation metrics for informativeness and structural similarity

### 12. Gen-Review
- **arXiv**: 2510.21192
- **Title**: *Gen-Review: A Large-scale Dataset of AI-Generated (and Human-written) Peer Reviews*
- **Authors**: Luca Demetrio, Giovanni Apruzzese, Kathrin Grosse et al.
- **Date**: Oct 2025
- **Tags**: dataset, AI-generated peer reviews
- **Key points**:
  - 81K LLM-generated reviews for ICLR 2018–2025
  - Linked to papers and original human reviews
  - Enables detection of AI-written reviews and bias analysis

---

## General Evaluation Frameworks & Metrics

These papers propose evaluation metrics or frameworks applicable (though not exclusively) to survey-quality evaluation.

### 13. FActScore
- **arXiv**: 2305.14251
- **Title**: *FActScore: Fine-grained Atomic Evaluation of Factual Precision in Long Form Text Generation*
- **Authors**: Sewon Min, Kalpesh Krishna, Xinxi Lyu et al.
- **Date**: May 2023
- **Tags**: metric, factuality, atomic evaluation
- **Key points**:
  - Decomposes generation into atomic facts; computes % supported by knowledge source
  - Widely used as a factual consistency metric for long-form generation
  - Automated version uses retrieval + LLM with <2% error rate vs. human

### 14. PROXYQA
- **arXiv**: 2401.15042
- **Title**: *PROXYQA: An Alternative Framework for Evaluating Long-Form Text Generation with LLMs*
- **Authors**: Haochen Tan, Zhijiang Guo, Zhan Shi et al.
- **Date**: Jan 2024
- **Tags**: evaluation framework, long-form generation
- **Key points**:
  - Meta-questions with proxy-questions and pre-annotated answers
  - Evaluates generated content quality through QA accuracy
  - Self-consistent and aligns with human evaluation

### 15. TRUE
- **arXiv**: 2204.04991
- **Title**: *TRUE: Re-evaluating Factual Consistency Evaluation*
- **Authors**: Or Honovich, Roee Aharoni, Jonathan Herzig et al.
- **Date**: Apr 2022
- **Tags**: survey, meta-evaluation, factual consistency
- **Key points**:
  - Comprehensive survey and assessment of factual consistency metrics
  - Standardized collection across 11 datasets
  - Large-scale NLI and question-generation approaches achieve strong results

### 16. Automatic Evaluation Metrics for Scientific Research
- **arXiv**: 2503.05712
- **Title**: *Automatic Evaluation Metrics for Artificially Generated Scientific Research*
- **Authors**: Niklas Höpner, Leon Eshuijs, Dimitrios Alivanistos et al.
- **Date**: Feb 2025
- **Tags**: metric, citation count prediction, review score prediction
- **Key points**:
  - Investigates citation count prediction and review score prediction as automatic evaluation
  - Simple models based on title+abstract outperform LLM reviewers

---

## Hallucination Detection & Factuality Benchmarks

These evaluate factuality/hallucination in LLM outputs, relevant to citation fidelity assessment.

### 17. HaluEval
- **arXiv**: 2305.11747
- **Title**: *HaluEval: A Large-Scale Hallucination Evaluation Benchmark for Large Language Models*
- **Authors**: Junyi Li, Xiaoxue Cheng, Wayne Xin Zhao et al.
- **Date**: May 2023
- **Tags**: benchmark, hallucination detection
- **Key points**:
  - Large collection of generated and human-annotated hallucinated samples
  - ChatGPT-based two-step sampling-then-filtering framework
  - ~19.5% of ChatGPT responses hallucinate

### 18. HALoGEN
- **arXiv**: 2501.08292
- **Title**: *HALoGEN: Fantastic LLM Hallucinations and Where to Find Them*
- **Authors**: Abhilasha Ravichander, Shrusti Ghela, David Wadden, Yejin Choi
- **Date**: Jan 2025
- **Tags**: benchmark, hallucination, verifier
- **Key points**:
  - 10,923 prompts across 9 domains including scientific attribution
  - Automatic high-precision verifiers for each domain
  - Error classification: Type A (incorrect recollection), Type B (incorrect knowledge), Type C (fabrication)

### 19. SelfCheckGPT
- **arXiv**: 2303.08896
- **Title**: *SelfCheckGPT: Zero-Resource Black-Box Hallucination Detection for Generative LLMs*
- **Authors**: Potsawee Manakul, Adian Liusie, Mark J. F. Gales
- **Date**: Mar 2023
- **Tags**: metric, hallucination detection, sampling-based
- **Key points**:
  - Zero-resource hallucination detection using response consistency
  - No external database required; applicable to black-box models
  - Used WikiBio dataset for evaluation

---

## Human Evaluation Protocols

Papers that study or propose human evaluation methodologies for reviews and surveys.

### 20. LitLLMs
- **arXiv**: 2412.15249
- **Title**: *LitLLMs, LLMs for Literature Review: Are we there yet?*
- **Authors**: Shubham Agarwal, Gaurav Sahu, Abhay Puri et al.
- **Date**: Dec 2024
- **Tags**: evaluation protocol, literature review
- **Key points**:
  - Zero-shot evaluation of LLMs for literature review writing
  - Test sets from arXiv papers with rolling protocol to avoid test set contamination
  - Two-step search strategy for retrieval + plan-then-execute for generation

### 21. Auto-survey Challenge
- **arXiv**: 2310.04480
- **Title**: *Auto-survey Challenge*
- **Authors**: Thanh Gia Hieu Khuong, Benedictus Kent Rachmat
- **Date**: Oct 2023
- **Tags**: competition, evaluation framework
- **Key points**:
  - Competition framework for LLM survey generation and critique
  - Simulated peer-review with human editorial oversight
  - Criteria: clarity, reference appropriateness, accountability, substantive value

### 22. Outcome-based Evaluation of Systematic Review Automation
- **arXiv**: 2306.17614
- **Title**: *Outcome-based Evaluation of Systematic Review Automation*
- **Authors**: Wojciech Kusa, Guido Zuccon, Petr Knoth, Allan Hanbury
- **Date**: Jun 2023
- **Tags**: evaluation framework, outcome-based
- **Key points**:
  - Evaluation framework accounting for impact of included/excluded studies on review outcomes
  - Demonstrates that not all inclusion decisions are equally important
  - Relevant for assessing retrieval quality in survey generation pipelines

---

## Summary Statistics

| Category | Count | Key Papers |
|---|---|---|
| Dedicated Survey Benchmarks | 8 | SurveyBench, SurGE, SurveyEval, DeepSurvey-Bench, SGSimEval, SurveyLens, Survey-Arena, SurveyForge's SurveyBench |
| Survey Datasets | 4 | SciReviewGen, SurveyGen, Hierarchical Catalogue, Gen-Review |
| General Evaluation Frameworks | 4 | FActScore, PROXYQA, TRUE, AutoEvalMetrics |
| Hallucination Benchmarks | 3 | HaluEval, HALoGEN, SelfCheckGPT |
| Human Evaluation Protocols | 3 | LitLLMs, Auto-survey Challenge, Outcome-based Eval |
| **Total** | **22** | |

Top candidates for inclusion in survey (by relevance to automated survey evaluation):
1. **SurveyBench** (2510.03120) — Most comprehensive dedicated survey benchmark
2. **SurGE** (2508.15658) — Standardized benchmark with retrieval pool
3. **SurveyEval** (2512.02763) — Multi-subject evaluation framework
4. **DeepSurvey-Bench** (2601.15307) — Academic value dimension
5. **FActScore** (2305.14251) — Widely used factual precision metric
