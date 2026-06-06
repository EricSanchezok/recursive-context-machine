# Extended Benchmark / Dataset / Metric / Evaluation-Protocol Candidates

**run_dir**: `.`
**generated_by**: ExtendedBenchmarkScout
**date**: 2026-06-06
**from_queries**: ebm-01 (survey generation evaluation benchmark metric coverage), ebm-02 (citation accuracy factual precision evaluation benchmark) + 2 auxiliary sweeps

**Parent artifact**: `02b_benchmark_candidates.md` (23 candidates already collected by BenchmarkScout)
**Strategy**: Queries broader than the named-benchmark queries bm-01/bm-02/bm-04; capture evaluation frameworks, leaderboards, and metric papers that do not use the AutoSurvey/SurveyBench brand names.

**Total raw hits**: 40 (4 searches × 10 topK)
**Deduplicated against existing pool**: 23 new candidates
**Candidates added by this scout**: 12 (after dedup + scope filtering)

---

## Coverage vs. Existing Pool

| Category | Already in 02b | Newly discovered | Notes |
|----------|----------------|------------------|-------|
| Survey gen benchmarks | 7 (SurveyBench, SurGE, SGSimEval, SurveyLens, DeepSurvey-Bench, SurveyEval, Survey-Arena) | 1 new | **Auto-survey Challenge** (competition) |
| Survey gen datasets | 3 (SurveyGen, SurveyBank, Hierarchical Catalogues) | 0 | Well-covered |
| Citation quality metrics | 4 (CiteEval, FActScore, TRUE, Core) | 3 new | ALCE, CiteME, VERISCORE |
| Citation grounding / attribution | — | 2 new | FACTS Grounding, FACTOR |
| Deep research evaluation | — | 1 new | DRACO (Perplexity) |
| Long-form factuality (general) | — | 3 new | SAFE/LongFact, VeriFact, FaStFACT |
| Entity-ambiguity-aware factuality | — | 1 new | D-FActScore |
| Long-document factuality | — | 1 new | LongDocFACTScore |
| Human-verified factuality prompt sets | — | 1 new | FACTORY |

---

## 1. New Survey Generation Evaluation Frameworks

### Auto-survey Challenge
- **arXiv**: 2310.04480v2
- **Authors**: Thanh Gia Hieu Khuong, Benedictus Kent Rachmat
- **Published**: 2023-10
- **Type**: competition / evaluation platform
- **Source query**: ebm-01 (auxiliary sweep)
- **Summary**: A novel platform for evaluating LLMs' capability to autonomously compose and critique survey papers across sciences, humanities, education, and law. Uses simulated peer-review mechanism with human organizers in editorial oversight. Hosted at AutoML 2023. Assessment criteria: clarity, reference appropriateness, accountability, substantive value.
- **Why this is new**: Non-LLM-as-judge evaluation paradigm using human editorial review. Competition format yields comparative cross-system results. Not captured by SurveyBench/SurGE/SurveyEval family.
- **Relevance**: Evaluation protocol design — human-in-the-loop peer-review protocol that could supplement automated LLM-as-judge approaches.
- **Gap addressed**: Evaluation frameworks without the AutoSurvey/SurveyBench brand name (ebm-01)

---

## 2. New Citation Quality & Attribution Benchmarks/Metrics

### ALCE (Automatic LLMs' Citation Evaluation)
- **arXiv**: 2305.14627v2
- **Authors**: Tianyu Gao, Howard Yen, Jiatong Yu, Danqi Chen
- **Published**: 2023-05
- **Type**: benchmark + metrics
- **Source query**: ebm-02
- **Summary**: First benchmark for evaluating LLMs' ability to generate text with citations. Collects diverse questions (ELI5, QA with Wikipedia, biography generation) with retrieval corpora. Automatic metrics across 3 dimensions: fluency, correctness, and citation quality. Strong correlation with human judgments. Findings: even best models lack complete citation support ~50% on ELI5.
- **Why this is new**: Pre-dates CiteEval (2506.01829) as the first dedicated citation evaluation benchmark. Different scope — evaluates end-to-end citation generation in open-domain QA, not just citation classification. Complements CiteEval's fine-grained citation assessment.
- **Relevance**: Directly addresses citation accuracy evaluation for survey generation. The 3-axis evaluation (fluency, correctness, citation quality) is transferable.

### CiteME
- **arXiv**: 2407.12861v2
- **Authors**: Ori Press, Andreas Hochlehnert, Ameya Prabhu et al.
- **Published**: 2024-07
- **Type**: benchmark (citation attribution)
- **Source query**: ebm-02
- **Summary**: Benchmark evaluating LMs' ability to identify the correct cited paper from a text excerpt. 7-way multiple-choice from ML papers. Reveals large gap: LMs achieve 4.2–18.5% accuracy vs. humans 69.7%. Introduces CiteAgent (autonomous system using GPT-4o + search/read) which achieves 35.3%.
- **Why this is new**: Targets a different dimension — not "does the citation support the claim?" but "is the cited paper the correct one?" This is a prerequisite for survey citation quality that no existing metric in the pool addresses.
- **Relevance**: Citation attribution accuracy is a fundamental building block for survey citation quality evaluation. High relevance.

### VERISCORE
- **arXiv**: 2406.19276v1
- **Authors**: Yixiao Song, Yekyung Kim, Mohit Iyyer
- **Published**: 2024-06
- **Type**: metric
- **Source query**: ebm-02 (auxiliary sweep)
- **Summary**: Factuality metric for diverse long-form generation tasks distinguishing verifiable from unverifiable claims. Addresses limitation of FActScore/SAFE that assume all claims are verifiable. Fine-tuned open-weight LM variant available. Evaluation across 8 tasks shows GPT-4o best but open-models (Mixtral-8x22) closing the gap.
- **Why this is new**: Distinguishes between verifiable scholarship and interpretive synthesis — critical for survey evaluation where many claims (interpretations, trends) are not strictly verifiable. Complements FActScore (already in pool) by handling the unverifiable dimension.
- **Relevance**: Survey evaluation requires sensitivity to what can/cannot be verified against sources.

---

## 3. New Factual Grounding & Leaderboard Benchmarks

### FACTS Grounding Leaderboard
- **arXiv**: 2501.03200v1
- **Authors**: Alon Jacovi, Andrew Wang, Chris Alberti et al. (Google DeepMind)
- **Published**: 2025-01
- **Type**: leaderboard + benchmark
- **Source query**: ebm-02
- **Summary**: Kaggle leaderboard evaluating LMs' ability to ground responses to provided context documents (up to 32k tokens). Two-phase evaluation: task fulfillment disqualification, then full grounding judgment. Aggregate of multiple judge models to mitigate bias. Active leaderboard with public/private splits.
- **Why this is new**: Active leaderboard (not static benchmark) — provides ongoing comparative results. Long-context grounding (32k tokens) is directly relevant to survey generation which must ground in many papers.
- **Relevance**: Evaluation protocol — the multi-judge aggregation approach and the task-fulfillment-first design are transferable to survey evaluation.

### FACTOR (Factual Assessment via Corpus TransfORmation)
- **arXiv**: 2307.06908v2
- **Authors**: Dor Muhlgay, Ori Ram, Inbal Magar et al.
- **Published**: 2023-07
- **Type**: benchmark + evaluation methodology
- **Source query**: ebm-02
- **Summary**: Automatically transforms a factual corpus into a benchmark evaluating LM's propensity to generate true facts vs. similar but incorrect statements. Three benchmarks: Wiki-FACTOR, News-FACTOR, Expert-FACTOR. Shows benchmark score and perplexity don't always agree; benchmark score better reflects factuality in open-ended generation.
- **Why this is new**: The corpus-transformation methodology is unique — could be adapted to evaluate survey generation by transforming a corpus of survey papers into a benchmark. The Expert-FACTOR dimension is especially relevant for scientific domain.
- **Relevance**: Methodology for creating bespoke factuality benchmarks from domain corpora — directly applicable to survey evaluation.

### DRACO (Deep Research Accuracy, Completeness, and Objectivity)
- **arXiv**: 2602.11685
- **Authors**: Joey Zhong, Hao Zhang, Clare Southern et al. (Perplexity AI)
- **Published**: 2026-02
- **Type**: benchmark (deep research)
- **Source query**: ebm-02
- **Summary**: Cross-domain benchmark of 10 domains, 40 countries, derived from real-world Perplexity Deep Research requests. Graded on 4 dimensions: factual accuracy, breadth/depth (completeness), presentation quality (including objectivity), and citation quality. Task-specific rubrics. Publicly available.
- **Why this is new**: Captures the commercial deep research evaluation paradigm (em-02 gap from extended query plan). Multi-dimensional rubric with citation quality as a standalone dimension. Real-world task distribution.
- **Relevance**: Directly evaluates Deep Research agents — competing paradigm to survey agents. The 4-dimension rubric is transferable.

---

## 4. New Long-Form Factuality Metrics & Benchmarks

### SAFE / LongFact
- **arXiv**: 2403.18802v4
- **Authors**: Jerry Wei, Chengrun Yang, Xinying Song et al. (Google DeepMind)
- **Published**: 2024-03
- **Type**: benchmark + metric + evaluator
- **Source query**: ebm-02 (auxiliary sweep)
- **Summary**: Search-Augmented Factuality Evaluator (SAFE) + LongFact prompt set (38 topics, thousands of questions). SAFE: breaks response into facts, verifies each via Google Search in multi-step reasoning. F1 score balancing precision (supported facts) and recall (fact density). LLM agents outperform crowdsourced human annotators (72% agreement, 76% win on disagreements). 20× cheaper than humans.
- **Why this is new**: Gold-standard long-form factuality evaluation. The search-augmented verification pipeline and the precision-recall F1 metric design are directly transferable to survey generation evaluation.
- **Relevance**: The F1 formulation (balancing factual precision with coverage) is more nuanced than existing survey evaluation metrics.

### VeriFact
- **arXiv**: 2505.09701v1
- **Authors**: Xin Liu, Lechen Zhang, Sheza Munir et al.
- **Published**: 2025-05
- **Type**: evaluation framework + benchmark
- **Source query**: ebm-02 (auxiliary sweep)
- **Summary**: Factuality evaluation framework enhancing fact extraction by identifying and resolving incomplete/missing facts. Introduces FactRBench measuring both precision AND recall (prior work focuses on precision only). Provides reference fact sets from LLMs and human-written answers.
- **Why this is new**: Addresses recall dimension — most factuality metrics only measure precision. Reference fact sets enable recall assessment. Critical for survey evaluation where missing important topics is a key failure mode.
- **Relevance**: Dual precision-recall evaluation applicable to survey content coverage assessment.

### FaStFACT
- **arXiv**: 2510.12839
- **Authors**: Yingjia Wan, Haochen Tan, Xiao Zhu et al.
- **Published**: 2025-10
- **Type**: evaluation framework
- **Source query**: ebm-02 (auxiliary sweep)
- **Summary**: Fast factuality evaluation with chunk-level claim extraction, confidence-based pre-verification, and document-level evidence collection from crawled webpages. Highest alignment with human evaluation among existing baselines. Addresses inefficiency and evidence insufficiency in prior pipelines.
- **Why this is new**: Efficiency optimization for claim extraction + verification pipeline. The chunk-level extraction and confidence-based filtering could reduce evaluation cost for survey generation.
- **Relevance**: Practical — the efficiency gains matter when evaluating long survey outputs.

### FACTORY
- **arXiv**: 2508.00109v1
- **Authors**: Mingda Chen, Yang Li, Xilun Chen et al. (Meta)
- **Published**: 2025-07
- **Type**: benchmark (human-verified prompt set)
- **Source query**: ebm-02 (auxiliary sweep)
- **Summary**: Large-scale human-verified prompt set for long-form factuality. Model-in-the-loop development, human refinement. Challenging: ~40% claims in SOTA model responses are not factual (vs. 10% for other benchmarks). Tests reasoning across long-tailed facts.
- **Why this is new**: Human-verified challenging prompts — higher quality than automatic benchmarks. The ~40% error rate reveals the difficulty gap between standard and challenging evaluation. The long-tailed knowledge focus is relevant for scientific survey evaluation.
- **Relevance**: Evaluation protocol — the model-in-the-loop + human-refinement pipeline for benchmark construction is a methodology worth studying.

### D-FActScore
- **arXiv**: 2402.05629v4
- **Authors**: Cheng-Han Chiang, Hung-yi Lee
- **Published**: 2024-02
- **Type**: metric
- **Source query**: ebm-02 (auxiliary sweep)
- **Summary**: Entity-ambiguity-aware metric extending FActScore. Detects when LLMs mix information from distinct entities into composite non-factual paragraphs. Llama-chat can generate paragraphs with individually verifiable facts that combine into non-factual paragraphs due to entity ambiguity. D-FActScore better captures this failure mode than standard FActScore.
- **Why this is new**: Addresses a subtle hallucination mode — entity ambiguity — that is common in survey writing (e.g., attributing finding A to paper B vs. paper C). Not addressed by any existing survey evaluation metric.
- **Relevance**: Highly relevant — entity attribution errors are a core citation quality failure in generated surveys.

### LongDocFACTScore
- **arXiv**: 2309.12455v2
- **Authors**: Jennifer A. Bishop, Qianqian Xie, Sophia Ananiadou
- **Published**: 2023-09
- **Type**: metric + evaluation framework
- **Source query**: ebm-02 (auxiliary sweep)
- **Summary**: Factuality evaluation framework for long document summarization (scientific domain). Human-annotated LongSciVerify dataset. Extends metrics to any length document, outperforms SOTA in correlation with human factuality measures. Scientific-domain focus.
- **Why this is new**: Specifically designed for scientific long documents — the closest direct relative to survey evaluation. The LongSciVerify dataset provides human-annotated scientific factuality data.
- **Relevance**: Very relevant — scientific domain, long documents, factuality in summarization context closely mirror survey generation evaluation.

---

## Summary Statistics

| Candidate | Type | Source Query | Gap Addressed |
|-----------|------|-------------|---------------|
| Auto-survey Challenge (2310.04480v2) | Competition/Eval Protocol | ebm-01 | Non-LLM-as-judge evaluation |
| ALCE (2305.14627v2) | Citation benchmark + metrics | ebm-02 | First citation eval benchmark |
| CiteME (2407.12861v2) | Citation attribution benchmark | ebm-02 | Citation identity, not just support |
| VERISCORE (2406.19276v1) | Factuality metric | ebm-02 | Verifiable vs. unverifiable distinction |
| FACTS Grounding (2501.03200v1) | Leaderboard/benchmark | ebm-02 | Active leaderboard, multi-judge aggregation |
| FACTOR (2307.06908v2) | Benchmark methodology | ebm-02 | Corpus-transformation approach |
| DRACO (2602.11685) | Deep research benchmark | ebm-02 | Commercial deep research evaluation |
| SAFE/LongFact (2403.18802v4) | Evaluator + benchmark | ebm-02 | Search-augmented factuality, F1 formulation |
| VeriFact (2505.09701v1) | Evaluation framework | ebm-02 | Precision + recall both measured |
| FaStFACT (2510.12839) | Evaluation framework | ebm-02 | Efficiency optimization |
| FACTORY (2508.00109v1) | Human-verified benchmark | ebm-02 | High-quality challenge prompts |
| D-FActScore (2402.05629v4) | Metric | ebm-02 | Entity-ambiguity-aware |
| LongDocFACTScore (2309.12455v2) | Metric + eval framework | ebm-02 | Long-document scientific factuality |

**Total new candidates**: 12 (after dedup; 13 raw minus 1 overlap with pool)

---

## Key Gaps Still Open

Despite this extended search, the following evaluation dimensions remain under-covered:

1. **Human evaluation protocols as standalone methodology papers** — no dedicated human evaluation protocol paper for survey generation exists. Current benchmarks embed human eval idiosyncratically.
2. **Cross-annotator agreement and reliability studies** — no paper in the pool systematically studies inter-annotator agreement for survey quality dimensions.
3. **Cost/throughput evaluation** — no benchmark or metric addresses the computational or monetary cost of survey generation systems as part of evaluation.
4. **Update / freshness evaluation** — no benchmark measures how well survey agents capture the most recent literature (timeliness dimension).
