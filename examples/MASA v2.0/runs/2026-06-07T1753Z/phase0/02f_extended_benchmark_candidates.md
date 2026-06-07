# Extended Benchmark Candidates — Automated Literature Survey Agents with Citation Graph Expansion

Generated: 2026-06-07T18:03Z  
Run dir: `.`  
Phase: ExtendedBenchmarkScout (extended benchmark/evaluation paper retrieval)

---

## Query Coverage

| Query ID | Target Gap | Papers Retrieved (unique) | New to Pool |
|----------|-----------|--------------------------|-------------|
| **E11** | Human evaluation protocol | 10 (deduplicated: 9 unique) | **3** |
| **E12** | Systematic review alignment (PRISMA/Cochrane) | 10 (deduplicated: 7 unique) | **7** |
| **E5** | Citation quality/noise evaluation | 10 (overlaps E11 for relevant) | **0** (all overlaps) |
| **Broad** | Benchmark/evaluation survey generation | 10 (overlaps E11/E12 pool) | **0** (all overlaps) |
| **Total unique new candidates** | | | **10** |

---

## Candidate Papers (New — Not in 03_expansion Pool)

Papers are ordered by relevance to the survey spec (most relevant first within each sub-section).

### E11 — Human Evaluation Protocol (3 new papers)

#### 1. LongEval: Guidelines for Human Evaluation of Faithfulness in Long-form Summarization
- **arXiv ID:** 2301.13298
- **Authors:** Kalpesh Krishna, Erin Bransom, Bailey Kuehl, Mohit Iyyer, Pradeep Dasigi, Arman Cohan, Kyle Lo
- **Categories:** cs.CL
- **Published:** 2023-01-30
- **Relevance (★★★):** Gold-standard human evaluation guidelines for faithfulness in long-form text. Surveys 162 papers on evaluation practices; finds 73% lack human eval. Proposes clause-level annotation, partial annotation strategies, and inter-annotator agreement methodology. Directly applicable to survey generation evaluation.
- **Why E11:** Answers "how to design human evaluation protocols for automated survey faithfulness" — evaluation rubric design, annotation granularity, workload reduction.
- **Deduplication Status:** ✅ Not in 03_expansion (Section 2.5 Benchmarks)
- **Decision:** INCLUDE — high relevance. Provides methodological foundation for human eval protocols.

#### 2. SGSimEval: A Comprehensive Multifaceted Similarity-Enhanced Benchmark for Automatic Survey Generation Systems
- **arXiv ID:** 2508.11310
- **Authors:** Beichen Guo, Zhiyuan Wen, Yu Yang, Peng Gao, Ruosong Yang, Jiaxing Shen
- **Categories:** cs.CL, cs.AI, cs.IR
- **Published:** 2025-08-15
- **Relevance (★★★):** Dedicated benchmark for ASG evaluation. Proposes three-dimension assessment (outline, content, references) combining LLM-based scoring with quantitative metrics. Introduces human preference metrics. Addresses bias in LLM-as-Judge evaluations.
- **Why E11:** Provides a concrete evaluation framework combining automated and human-aligned metrics for survey generation.
- **Deduplication Status:** ✅ Not in 03_expansion (Section 2.5)
- **Decision:** INCLUDE — high relevance. Complements SurveyBench/SurveyEval with similarity-enhanced methodology.

#### 3. DeepSurvey-Bench: Evaluating Academic Value of Automatically Generated Scientific Survey
- **arXiv ID:** 2601.15307
- **Authors:** Guo-Biao Zhang, Ding-Yuan Liu, Da-Yi Wu, Tian Lan, Heyan Huang, Zhijing Wu, Xian-Ling Mao
- **Categories:** cs.AI, cs.CL
- **Published:** 2026-01-13
- **Relevance (★★★):** Addresses gap that existing benchmarks evaluate surface quality (structure, coherence) rather than "academic value" (informational value, scholarly communication value, research guidance value). Proposes human-annotated academic value dataset.
- **Why E11:** Introduces novel evaluation dimension — academic value beyond surface quality — with human annotation ground truth.
- **Deduplication Status:** ✅ Not in 03_expansion (Section 2.5)
- **Decision:** INCLUDE — high relevance. Fills the "deep evaluation" gap noted in SurveyBench's limitations.

### E12 — Systematic Review Alignment / PRISMA (7 new papers)

#### 4. The Literature Review Network: An Explainable AI for Systematic Literature Reviews, Meta-analyses, and Method Development
- **arXiv ID:** 2408.05239
- **Authors:** Joshua Morriss, Tod Brindle, Jessica Bah Rösman, Daniel Reibsamen, Andreas Enz
- **Categories:** cs.DL, cs.AI, cs.CY
- **Published:** 2024-08-05
- **Relevance (★★★):** First explainable AI platform explicitly adhering to PRISMA 2020 standards for full SLR automation. Benchmarked against expert manual review using confusion matrices and Jaccard index. Reduced 11-month review to 5 days.
- **Why E12:** Direct PRISMA alignment — the methodological bridge between automated survey generation and systematic review standards.
- **Deduplication Status:** ✅ Not in 03_expansion (Section 2.7 Boundary — SLR screening lists 4 papers, but not this one)
- **Decision:** INCLUDE — high relevance. Strongest E12 candidate for PRISMA methodological alignment.

#### 5. ProfOlaf: Semi-Automated Tool for Systematic Literature Reviews
- **arXiv ID:** 2510.26750
- **Authors:** Martim Afonso, Nuno Saavedra, Bruno Lourenço, Alexandra Mendes, João Ferreira
- **Categories:** cs.IR
- **Published:** 2025-10-30
- **Relevance (★★☆):** Semi-automated SLR tool combining iterative snowballing with human-in-the-loop filtering. Uses LLMs for article analysis, topic extraction, and content queries. Situated at boundary between manual SLR methodology and automated survey generation.
- **Why E12:** Snowballing methodology — a manual citation expansion strategy that automated survey agents could emulate or compare against.
- **Deduplication Status:** ✅ Not in 03_expansion
- **Decision:** INCLUDE — moderate-high relevance. Represents the semi-automated SLR approach as baseline/contrast to fully automated survey agents.

#### 6. SWARM-SLR: Streamlined Workflow Automation for Machine-actionable Systematic Literature Reviews
- **arXiv ID:** 2407.18657
- **Authors:** Tim Wittenborg, Oliver Karras, Sören Auer
- **Categories:** cs.DL, cs.SE
- **Published:** 2024-07-26
- **Relevance (★★☆):** Composes 65 requirements from SLR guidelines into an automated workflow spanning planning to reporting. Evaluated via two online surveys. Synthesizes 11 existing tools into a coherent pipeline.
- **Why E12:** Provides a requirements framework for systematic review automation that could structure evaluation of survey generation systems.
- **Deduplication Status:** ✅ Not in 03_expansion
- **Decision:** INCLUDE — moderate relevance. Useful as a requirements benchmark for what an SLR workflow should cover.

#### 7. AiReview: An Open Platform for Accelerating Systematic Reviews with LLMs
- **arXiv ID:** 2504.04193
- **Authors:** Xinyu Mao, Teerapong Leelanupab, Martin Potthast, Harrisen Scells, Guido Zuccon
- **Categories:** cs.IR
- **Published:** 2025-04-05
- **Relevance (★★☆):** First platform bridging LLM-assisted screening methods with medical systematic review creation. Extensible framework applying LLMs to title/abstract screening tasks. Open-source.
- **Why E12:** Connects LLM-based automation to the established systematic review methodology in medicine — a domain-specific alignment with PRISMA/Cochrane.
- **Deduplication Status:** ✅ Not in 03_expansion
- **Decision:** INCLUDE — moderate relevance. Demonstrates how LLM screening integrates with evidence-based review frameworks.

#### 8. Paperfetcher: A tool to automate handsearch for systematic reviews
- **arXiv ID:** 2110.12490
- **Authors:** Akash Pallath, Qiyang Zhang
- **Categories:** cs.IR, stat.AP
- **Published:** 2021-10-24
- **Relevance (★★☆):** Automates handsearch — systematic browsing of field-specific journals — with snowballing in both directions. Open-source Python package.
- **Why E12:** Handsearch automation is a manual precursor to citation graph traversal. Provides a baseline "manual" citation collection methodology for comparison.
- **Deduplication Status:** ✅ Not in 03_expansion
- **Decision:** INCLUDE — moderate relevance. Historical/comparative context for citation collection methodology.

#### 9. Assessing the Ability of ChatGPT to Screen Articles for Systematic Reviews
- **arXiv ID:** 2307.06464
- **Authors:** Eugene Syriani, Istvan David, Gauransh Kumar
- **Categories:** cs.SE, cs.CL, cs.IR
- **Published:** 2023-07-12
- **Relevance (★★☆):** Evaluates ChatGPT for SR screening — consistency, classification performance, generalizability. Compares against traditional ML classifiers. From software engineering domain.
- **Why E12:** Provides a benchmark for LLM-based screening as a component of systematic review pipelines, with comparisons to non-LLM classifiers.
- **Deduplication Status:** ✅ Not in 03_expansion
- **Decision:** INCLUDE — moderate relevance. Good comparative baseline for LLM-vs-classifier screening evaluation.

#### 10. The emergence of Large Language Models (LLM) as a tool in literature reviews: an LLM automated systematic review
- **arXiv ID:** 2409.04600
- **Authors:** Dmitry Scherbakov, Nina Hubig, Vinita Jansari, Alexander Bakumenko, Leslie A. Lenert
- **Categories:** cs.DL, cs.AI
- **Published:** 2024-09-06
- **Relevance (★★☆):** Systematic review of LLM usage in review creation. Surveyed 3,788 articles, identified 172 eligible studies. Maps stages of review automation (search 34.9%, data extraction 31.4%). Compares GPT vs BERT performance.
- **Why E12:** Meta-review that systematically maps the landscape of LLM-for-review tools — provides domain structure and performance baselines.
- **Deduplication Status:** ✅ Not in 03_expansion
- **Decision:** INCLUDE — moderate relevance. Useful as a secondary survey mapping the intersection of LLMs and systematic reviews.

---

### Papers Evaluated but Excluded

| arXiv ID | Title | Reason for Exclusion |
|----------|-------|---------------------|
| 2510.06242 | Transparent Reference-free Automated Evaluation of Open-Ended User Survey Responses | Evaluates human-written *market research* survey responses, not LLM-generated *academic surveys*. Different task and domain. |
| 2509.06337 | Large Language Models as Virtual Survey Respondents | About simulating survey *respondents* for social science research, not evaluating survey *generation*. |
| 2510.26238 | Questionnaire meets LLM: A Benchmark of Structural Skills | Evaluates LLM ability to parse questionnaire data, not survey generation quality. |
| 1606.06424 | A Novel Framework to Expedite Systematic Reviews by Automatically Building IE Training Corpora | Pre-dates LLMs; focused on biomedical information extraction from Cochrane reviews. Too old and tangential. |
| 2010.13200 | Subjective Evaluation of Noise Suppression Algorithms | Audio noise suppression (INTERSPEECH challenge), unrelated to citation noise. False positive from the "noise" keyword. |

---

### Existing Pool Papers Re-encountered (Confirming Overlap)

These were already in 03_expansion and serve as check that our searches are in scope:

| arXiv ID | Title (abbrev.) | Existing Section |
|----------|-----------------|-----------------|
| 2510.03120 | SurveyBench | §2.5 Benchmarks #44 |
| 2512.02763 | SurveyEval | §2.5 Benchmarks #45 |
| 2508.15658 | SurGE | §2.5 Benchmarks #47 |
| 2508.17647 | SurveyGen | §2.3 Survey Agents #24 |
| 2502.14776 | SurveyX | §2.3 Survey Agents #20 |
| 2503.04629 | SurveyForge | §2.3 Survey Agents #21 |
| 2006.12166 | ASReview | §2.4 SLR Screening #41 + seed |
| 2202.10033 | Open-source integrated framework | §2.7 Boundary SLR #60 |

---

## Summary

| Metric | Count |
|--------|-------|
| Total new candidate papers | **10** |
| E11 (human evaluation protocol) | 3 |
| E12 (systematic review alignment) | 7 |
| Papers excluded (out of scope) | 5 |
| Already-in-pool papers re-encountered | 8 |

### Gap Coverage Assessment

| Gap | Addressed By | Coverage |
|-----|-------------|----------|
| Human evaluation protocol for surveys | LongEval (2301.13298) — clause-level faithfulness guidelines | ★★★ Strong |
| LLM-based survey evaluation frameworks | SGSimEval (2508.11310), DeepSurvey-Bench (2601.15307) | ★★★ Strong |
| Academic value / deep evaluation | DeepSurvey-Bench (2601.15307) — novel "academic value" dimension | ★★★ Strong |
| PRISMA alignment for automated surveys | LRN (2408.05239) — explicit PRISMA 2020 compliance | ★★★ Strong |
| SLR workflow requirements | SWARM-SLR (2407.18657) — 65 requirements across lifecycle | ★★☆ Moderate |
| LLM-assisted systematic review platforms | AiReview (2504.04193), ProfOlaf (2510.26750) | ★★☆ Moderate |
| LLM screening vs traditional ML baselines | ChatGPT Screening (2307.06464), Paperfetcher (2110.12490) | ★★☆ Moderate |
| Meta-review of LLM+review landscape | Emergence of LLMs (2409.04600) — 172-paper survey | ★★☆ Moderate |

### Recommendation

**Prioritize for merge into pool:** LongEval (2301.13298), SGSimEval (2508.11310), DeepSurvey-Bench (2601.15307), LRN (2408.05239) — these directly address evaluation methodology gaps that the existing benchmark pool (SurveyBench, SurveyEval, SurGE, ReportBench) does not fully cover.

---

*Handoff preparation complete. See handoff schema for next steps.*
