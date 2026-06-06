# Benchmark / Dataset / Metric Candidate Pool

**run_dir**: `.`
**generated_by**: BenchmarkScout
**date**: 2026-06-06
**from_queries**: bm-01, bm-02, bm-03, bm-04 (query plan) + 2 auxiliary queries

---

## 1. Survey Generation Benchmarks (core)

### SurveyBench
- **arXiv**: 2510.03120
- **Authors**: Zhaojun Sun, Xuzhou Zhu, Xuanhe Zhou et al.
- **Published**: 2025-10
- **Type**: benchmark + evaluation framework
- **Summary**: Quiz-driven evaluation framework for LLM-generated academic surveys. Built from 11,343 arXiv topics and 4,947 high-quality surveys. Multi-faceted metric hierarchy: outline quality (coverage breadth, logical coherence), content quality (synthesis granularity, clarity of insights), non-textual richness. Dual-mode protocol: content-based and quiz-based answerability tests aligned with reader needs.
- **Relevance**: Directly addresses bm-02. The most comprehensive dedicated survey benchmark at time of writing.

### SurGE (Survey Generation Evaluation)
- **arXiv**: 2508.15658
- **Authors**: Weihang Su, Anzhe Xie, Qingyao Ai et al.
- **Published**: 2025-08
- **Type**: benchmark + evaluation framework + corpus
- **Summary**: Benchmark for evaluating scientific survey generation in CS. Collection of test instances (topic + expert-written survey + full references) plus a large-scale corpus of 1M+ papers as retrieval pool. Automated evaluation across 4 dimensions: information coverage, referencing accuracy, structural organization, content quality. Open-sourced.
- **Relevance**: Directly addresses bm-01/bm-04. Good for retrieval+generation evaluation.

### SGSimEval
- **arXiv**: 2508.11310
- **Authors**: Beichen Guo, Zhiyuan Wen, Yu Yang et al.
- **Published**: 2025-08
- **Type**: benchmark + metric
- **Summary**: Multifaceted benchmark for survey generation evaluation combining outline, content, and reference assessments. Integrates LLM-based scoring with quantitative metrics. Introduces human preference metrics emphasizing both inherent quality and similarity to humans. Strong consistency with human assessments reported.
- **Relevance**: Addresses bm-01/bm-03. Good for human-evaluation alignment methods.

### SurveyLens
- **arXiv**: 2602.11238
- **Authors**: Beichen Guo, Zhiyuan Wen, Jia Gu et al.
- **Published**: 2026-02
- **Type**: benchmark + dataset
- **Summary**: First discipline-aware benchmark evaluating ASG across 10 disciplines. Constructs SurveyLens-1k (1,000 high-quality human-written surveys). Dual-lens evaluation: Discipline-Aware Rubric Evaluation (LLM with human-preference-aligned weights) + Canonical Alignment Evaluation (content coverage and synthesis quality). Evaluates 11 ASG methods including Deep Research agents.
- **Relevance**: Extends beyond CS bias — important for cross-domain survey agents.

### DeepSurvey-Bench
- **arXiv**: 2601.15307
- **Authors**: Guo-Biao Zhang, Ding-Yuan Liu, Da-Yi Wu et al.
- **Published**: 2026-01
- **Type**: benchmark
- **Summary**: Evaluates "academic value" (not just surface quality) across 3 dimensions: informational value, scholarly communication value, research guidance value. Constructs dataset with academic value annotations. Critiques existing benchmarks for relying on flawed selection criteria.
- **Relevance**: Addresses metric gaps identified in bm-04.

### SurveyEval
- **arXiv**: 2512.02763
- **Authors**: Jiahao Zhao, Shuaixing Zhang, Nan Xu et al.
- **Published**: 2025-12
- **Type**: benchmark
- **Summary**: Evaluates across 3 dimensions (overall quality, outline coherence, reference accuracy) over 7 subjects. Augments LLM-as-a-Judge with human references to strengthen evaluation-human alignment.
- **Relevance**: Directly addresses bm-04 metric concerns.

### Survey-Arena (from IterSurvey paper)
- **arXiv**: 2510.21900
- **Authors**: Hongbo Zhang, Han Cui, Yidong Wang et al.
- **Published**: 2025-10
- **Type**: benchmark (pairwise)
- **Summary**: Pairwise benchmark that complements absolute scoring. Positions machine-generated surveys relative to human-written ones. Part of the IterSurvey framework paper but the benchmark artifact is separable.
- **Relevance**: Novel pairwise evaluation protocol — addresses bm-03.

---

## 2. Survey Generation Datasets

### SurveyGen Dataset
- **arXiv**: 2508.17647
- **Authors**: Tong Bao, Mir Tafseer Nayeem, Davood Rafiei et al.
- **Published**: 2025-08
- **Type**: dataset (4,200+ surveys)
- **Summary**: Large-scale dataset of 4,200+ human-written surveys across diverse scientific domains, with 242,143 cited references and extensive quality-related metadata. Used to build QUAL-SG quality-aware RAG pipeline.
- **Relevance**: Valuable as a ground-truth corpus for evaluation (addresses bm-01 data needs).

### SurveyBank
- **arXiv**: 2110.06354
- **Authors**: Jiayuan Ding, Tong Xiang, Zijing Ou et al.
- **Published**: 2021-10
- **Type**: dataset
- **Summary**: Dataset of survey papers in CS with citation relationships, multi-level reading lists inferred from references. Designed for Reading Path Generation task.
- **Relevance**: Older but provides citation-graph structure for evaluation.

### Hierarchical Catalogues of Literature Reviews Dataset
- **arXiv**: 2304.03512
- **Authors**: Kun Zhu, Xiaocheng Feng, Xiachong Feng et al.
- **Published**: 2023-04
- **Type**: dataset + evaluation metrics
- **Summary**: 7.6k literature review catalogues and 389k reference papers. Designed for hierarchical catalogue generation task (precursor to full survey generation). Proposes informativeness and similarity-to-ground-truth metrics.
- **Relevance**: Useful for evaluating outline/organization aspects of surveys.

---

## 3. Citation Quality and Attribution Metrics

### CiteEval / CiteBench
- **arXiv**: 2506.01829
- **Authors**: Yumo Xu, Peng Qi, Jifan Chen et al.
- **Published**: 2025-06
- **Type**: evaluation framework + benchmark
- **Summary**: Principle-driven citation evaluation focusing on fine-grained citation assessment in context (not just binary NLI). CiteBench: multi-domain benchmark with human annotations. CiteEval-Auto: model-based metrics correlated with human judgments.
- **Relevance**: Directly addresses citation accuracy evaluation (bm-04). Good for assessing citation quality in generated surveys.

### FActScore
- **arXiv**: 2305.14251
- **Authors**: Sewon Min, Kalpesh Krishna, Xinxi Lyu et al.
- **Published**: 2023-05
- **Type**: metric
- **Summary**: Fine-grained atomic evaluation of factual precision. Breaks generations into atomic facts, computes % supported by reliable knowledge source. Automated version using retrieval+LLM with <2% error rate.
- **Relevance**: Factual precision metric (bm-04) applicable to survey content.

### TRUE
- **arXiv**: 2204.04991
- **Authors**: Or Honovich, Roee Aharoni, Jonathan Herzig et al.
- **Published**: 2022-04
- **Type**: evaluation framework + meta-evaluation
- **Summary**: Comprehensive survey and assessment of factual consistency metrics. Standardized collection of texts from diverse tasks, manually annotated. Example-level meta-evaluation protocol.
- **Relevance**: Foundational for understanding factual consistency metrics in survey generation.

### Core
- **arXiv**: 2407.03572
- **Authors**: Zhengping Jiang, Jingyu Zhang, Nathaniel Weir et al.
- **Published**: 2024-07
- **Type**: metric component
- **Summary**: Sub-claim selection component for factual precision metrics that filters obvious/repetitive subclaims. Augments FActScore-like approaches.
- **Relevance**: Could improve factual precision evaluation in surveys.

---

## 4. Citation Graph Retrieval Benchmarks

### OAG-Bench
- **arXiv**: 2402.15810
- **Authors**: Fanjin Zhang, Shijie Shi, Yifan Zhu et al.
- **Published**: 2024-02
- **Type**: benchmark (academic graph mining)
- **Summary**: Multi-aspect, fine-grained benchmark based on Open Academic Graph (OAG). 10 tasks, 20 datasets, 70+ baselines. Includes paper source tracing, scholar profiling, etc. Standardized evaluation protocols.
- **Relevance**: Broader academic graph evaluation — useful for citation graph retrieval component of survey agents.

### Seed-based Citation Retrieval Comparison
- **arXiv**: 2403.09295
- **Authors**: Peter Sjögårde, Per Ahlgren
- **Published**: 2024-03
- **Type**: evaluation (method comparison)
- **Summary**: Compares direct citation, co-citation, bibliographic coupling, and PubMed Related Article score for seed-based retrieval. Uses systematic reviews as baseline. Shows advantage for co-citation, best when combining all three.
- **Relevance**: Directly evaluates citation-graph traversal methods (core mechanism of survey agents).

### LitFM Benchmarks
- **arXiv**: 2409.12177
- **Authors**: Jiasheng Zhang, Jialin Chen, Ali Maatouk et al.
- **Published**: 2024-09
- **Type**: benchmark (for citation graph foundation model)
- **Summary**: New benchmark datasets on 3 academic fields with sentence-level citation information and local context. 28.1% improvement on retrieval precision.
- **Relevance**: Citation graph retrieval evaluation for survey agents.

---

## 5. Human Evaluation Protocols

### SGSimEval (see §1) — human preference metrics
Human preference metrics emphasizing both inherent quality and similarity to human-written surveys.

### SurveyBench (see §1) — quiz-driven reader alignment
Dual-mode evaluation (content-based + quiz-based answerability) explicitly aligned with reader informational needs.

### Human Evaluation of Creative NLG Systems
- **arXiv**: 2108.00308
- **Authors**: Mika Hämäläinen, Khalid Alnajjar
- **Published**: 2021-07
- **Type**: survey / guidelines
- **Summary**: Surveys human evaluation practices in creative NLG (INLG 2020, ICCC 2020). Guidelines for future evaluation: define goals clearly, use concrete questions, test setup, use multiple setups, report biases, analyze beyond basic statistics.
- **Relevance**: General guidelines for human evaluation protocol design (bm-03). Not survey-specific but methodology transferable.

---

## 6. Other Relevant Benchmarks (borderline)

### SourceBench
- **arXiv**: 2602.16942
- **Authors**: Hexi Jin, Stephen Liu, Yuheng Li et al.
- **Published**: 2026-02
- **Type**: benchmark (cited web source quality)
- **Summary**: 8-metric framework for evaluating quality of cited web sources across 100 queries. Metrics: content relevance, factual accuracy, objectivity, freshness, authority, clarity.
- **Relevance**: Citation quality metrics could transfer to survey citation evaluation, but focused on web sources not academic papers.

### GraphReview
- **arXiv**: 2605.27204
- **Authors**: Pujun Zheng, Wanying Ren, Jiacheng Yao et al.
- **Published**: 2026-05
- **Type**: method + evaluation framework
- **Summary**: Graph-based LLM framework for paper evaluation using message passing over semantic paper graphs. Evaluates papers in context of contemporaneous and prior work. 29.7% improvement on decision/ranking metrics.
- **Relevance**: Adjacent — paper evaluation rather than survey evaluation, but graph-based evaluation approach may be adaptable.

### Phocus
- **arXiv**: 2201.02915
- **Authors**: Xinrong Zhang, Zihou Ren, Xi Li et al.
- **Published**: 2022-01
- **Type**: metric
- **Summary**: Citation sentiment analysis and ranking model. Classifies citations coarsely, ranks references within a paper.
- **Relevance**: Fine-grained citation quality metric, applicable to survey citation evaluation.

---

## Summary Statistics

| Category | Count | Key Artifacts |
|----------|-------|---------------|
| Survey generation benchmarks | 7 | SurveyBench, SurGE, SGSimEval, SurveyLens, DeepSurvey-Bench, SurveyEval, Survey-Arena |
| Survey generation datasets | 3 | SurveyGen, SurveyBank, Hierarchical Catalogues |
| Citation quality metrics | 4 | CiteEval, FActScore, TRUE, Core |
| Citation graph retrieval benchmarks | 3 | OAG-Bench, Seed-based comparison, LitFM |
| Human evaluation protocols | 3 | SGSimEval (human preference), SurveyBench (quiz-driven), NLG eval survey |
| Other (borderline) | 3 | SourceBench, GraphReview, Phocus |
| **Total** | **23** | |

### Most Frequent Metrics Across Benchmarks
1. **Outline quality** (coverage breadth, logical coherence) — SurveyBench, SGSimEval, SurGE
2. **Content quality** (synthesis granularity, clarity of insights) — SurveyBench, SurveyEval, SurGE
3. **Citation/reference accuracy** — CiteEval, SurveyBench, SurveyEval, SurGE
4. **Factual precision** — FActScore, TRUE, Core
5. **Academic value** (informational, scholarly communication, research guidance) — DeepSurvey-Bench
6. **Human preference / reader alignment** — SGSimEval, SurveyBench (quiz-based)
