# 02b Benchmark & Evaluation Candidates

**run_dir**: `.`
**scout**: BenchmarkScout
**date**: 2026-06-07T10:14:32+08:00

---

## Queries Executed

| # | Query (from QueryPlan) | Source ID | topK | Results |
|---|------------------------|-----------|------|---------|
| 1 | Benchmarks and automatic metrics for survey quality evaluation | B-01 | 10 | 10 |
| 2 | Human evaluation protocols for LLM-generated literature reviews | B-02 | 10 | 10 |
| 3 | Citation precision and recall evaluation in machine-generated surveys | B-03 | 10 | 10 |
| 4 | Dataset for evaluating LLM-generated scientific survey papers | (wider net) | 10 | 10 |
| 5 | Evaluation metrics leaderboard for automated survey generation LLM | (wider net) | 10 | 10 |
| 6 | citation accuracy factual consistency evaluation LLM generated scientific text | (wider net) | 10 | 10 |

**Total unique candidate papers identified**: 24 (deduplicated across all queries)

---

## Candidate Summary

### Category A: Dedicated Survey Generation Benchmarks (10 papers)

These are the most directly relevant — benchmarks purpose-built for evaluating automated survey generation systems.

| ID | arXiv ID | Short Name | Year | Key Features |
|----|----------|-----------|------|-------------|
| B01 | 2510.03120 | SurveyBench | 2025 | Quiz-driven; 11,343 arXiv papers + 4,947 surveys; outline/content/non-textual quality hierarchy; dual-mode eval (content + quiz answerability) |
| B02 | 2508.15658 | SurGE | 2025 | CS focus; 1M+ paper corpus; 4 dimensions (coverage, referencing, structure, content); open-sourced |
| B03 | 2512.02763 | SurveyEval | 2025 | 7 subjects; LLM-as-a-Judge + human references for alignment |
| B04 | 2601.15307 | DeepSurvey-Bench | 2026 | Academic value (informational, scholarly, research guidance); challenges citation-count-based ground truth |
| B05 | 2602.11238 | SurveyLens | 2026 | Discipline-aware; 1,000 surveys across 10 disciplines; dual-lens eval (rubric + canonical alignment) |
| B06 | 2508.11310 | SGSimEval | 2025 | Similarity-enhanced; combines LLM scoring + quantitative metrics; human preference metrics |
| B07 | 2506.12689 | SurveyScope | 2025 | 46 high-impact papers (2020-2025); 11 CS domains; citation F1 |
| B08 | 2510.21900 | Survey-Arena | 2025 | Pairwise comparison benchmark; complements absolute scoring |
| B09 | 2503.08506 | ReviewBench | 2025 | For LLM-generated peer review comments; part of ReviewAgents |
| B10 | 2503.04629 | SurveyBench (SurveyForge) | 2025 | 100 human-written surveys for win-rate; reference/outline/content quality |

### Category B: Citation Attribution & Factuality Benchmarks (5 papers)

These are adjacent but essential for evaluating citation precision/recall and factual consistency in generated surveys.

| ID | arXiv ID | Short Name | Year | Key Features |
|----|----------|-----------|------|-------------|
| B11 | 2407.12861 | CiteME | 2024 | Citation attribution; LM 4-18% vs human 69.7%; CiteAgent system |
| B12 | 2305.14251 | FActScore | 2023 | Atomic factual precision; de facto standard for factuality eval |
| B13 | 2403.18802 | LongFact + SAFE | 2024 | Long-form factuality; 38 topics; search-augmented eval |
| B14 | 2509.25868 | ReFACT | 2025 | Scientific confabulation; 1,001 expert-annotated pairs |
| B15 | 2204.04991 | TRUE | 2022 | Factual consistency meta-evaluation; 11 datasets |

### Category C: Evaluation Datasets (4 papers)

| ID | arXiv ID | Short Name | Year | Key Features |
|----|----------|-----------|------|-------------|
| B16 | 2305.15186 | SciReviewGen | 2023 | 10K+ reviews + 690K cited papers |
| B17 | 2508.17647 | SurveyGen | 2025 | 4,200 surveys across diverse domains; 242K references |
| B18 | 2509.00496 | ResearchQA | 2025 | 21K queries + 160K rubric items from 75 fields |
| B19 | 2503.08506 | Review-CoT | 2025 | 142K review comments for structured reasoning |

### Category D: Evaluation Protocols & Meta-Evaluation (4 papers)

| ID | arXiv ID | Short Name | Year | Key Features |
|----|----------|-----------|------|-------------|
| B20 | 2412.15249 | LitLLMs | 2024 | Evaluation protocol; two-step search + planning decomposition |
| B21 | 2412.13612 | LLMs for Auto Lit Rev | 2024 | Multidimensional; hallucination rates, semantic coverage, factual consistency |
| B22 | 2310.04480 | Auto-survey Challenge | 2023 | Competition platform for autonomous survey writing + peer review |
| B23 | 2308.10410 | Wikipedia-style Survey Eval | 2023 | Early GPT-4 eval; 99 NLP topics; human vs GPT evaluation bias |
| B24 | 2411.16638 | Factuality Metrics Critique | 2024 | Stress-test of automatic factuality metrics; reveals shallow-feature gaming |

---

## Key Findings & Observations

### 1. The benchmark landscape is active and maturing fast
The period 2025-2026 has seen an explosion of dedicated ASG benchmarks: SurveyBench (Oct 2025), SurGE (Aug 2025), SurveyEval (Dec 2025), DeepSurvey-Bench (Jan 2026), SurveyLens (Feb 2026). This suggests the field is rapidly consolidating around standardised evaluation.

### 2. No single dominant benchmark yet
There is no clear "ImageNet for survey generation." Different benchmarks emphasise different dimensions:
- **Reader alignment**: SurveyBench (quiz-based answerability)
- **Discipline coverage**: SurveyLens (10 disciplines)
- **Academic depth**: DeepSurvey-Bench (value dimensions)
- **Scalability**: SurGE (1M+ corpora)

### 3. Citation fidelity is a recognised weak spot
Multiple papers (SurveyForge, SurveyGen, SciSage) explicitly report that citation quality is the hardest dimension. CiteME (B11) provides a dedicated benchmark for this.

### 4. FActScore is the most cited factuality metric in this domain
Every survey-generation paper that measures factuality references FActScore (2305.14251) or its derivatives.

### 5. ResearchQA (B18) is notable as a cross-domain evaluation resource
It distills 75 research fields into 21K queries with rubrics — potentially useful as a multi-field evaluation substrate rather than a dedicated ASG benchmark.

---

## Recommended Seeds for Forward/Backward Citation Chaining

1. **SurveyBench** (2510.03120) — most comprehensive survey-specific benchmark
2. **SurGE** (2508.15658) — largest retrieval corpus (1M+), open-sourced
3. **DeepSurvey-Bench** (2601.15307) — challenges ground-truth assumptions
4. **SurveyLens** (2602.11238) — discipline-aware angle
5. **CiteME** (2407.12861) — citation attribution specifically
6. **FActScore** (2305.14251) — foundational factuality metric
7. **TRUE** (2204.04991) — meta-evaluation framework
