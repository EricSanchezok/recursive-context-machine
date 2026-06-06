# ExtendedQueryPlan — Gap Analysis

**run_dir**: `.`
**source**: `01_query_plan.md`, `02_candidate_pool.md`, `00_survey_spec.md`
**generated**: 2026-06-06T12:44:32+08:00

---

## Gap Analysis Summary

Analysis of the main query plan (22 queries) and candidate pool (82 papers) revealed **8 gap areas** that the extended queries target.

---

### Gap 1: Related Work Generation Framing (Missing Community)

**Evidence**: The candidate pool has only 1 paper (2505.19647, "Select, Read, and Write") explicitly using "related work" framing out of 82 candidates.

**Impact**: The NLP community publishes extensively on "related work generation" (ACL, EMNLP, NAACL venues) but this literature is almost entirely absent. These papers often tackle the same planning-evidence-writing pipeline but use different terminology.

**Targeted by**: EQ-01

---

### Gap 2: Multi-Document Summarization Lineage (Missing Foundation)

**Evidence**: Zero papers in the candidate pool address multi-document summarization (MDS), which is the direct methodological predecessor of survey generation.

**Impact**: Survey generation inherits from MDS (planning, content selection, ordering, attribution). The absence of MDS papers obscures the methodological lineage and misses shared evaluation frameworks (e.g., DUC/TAC tasks).

**Targeted by**: EQ-02

---

### Gap 3: Knowledge-Grounded Generation Methods (Missing Broader Family)

**Evidence**: The pool is dominated by survey-specific systems. No papers frame the problem as "knowledge-grounded long-form generation."

**Impact**: General-purpose architectures for knowledge-grounded generation (e.g., plan-write-edit pipelines, controller-retriever-writer patterns) are directly applicable to surveys. Missing this framing means missing transferable method designs.

**Targeted by**: EQ-03

---

### Gap 4: Scientific Writing Automation (Adjacent Domain)

**Evidence**: The pool contains only survey-generation systems. Full paper generation and scientific writing automation are adjacent with shared components.

**Impact**: Many architectural innovations appear first in the broader scientific writing literature (e.g., hierarchical planning for multi-section documents, citation-aware content planning).

**Targeted by**: EQ-04

---

### Gap 5: Domain-Specific Survey Methods (Biomedical & Medical)

**Evidence**: The pool has only 1 medical paper (2504.14822 — "Completing A Systematic Review in Hours"). No biomedical survey generation papers.

**Impact**: The biomedical community has developed distinct approaches for systematic review automation with LLMs, often emphasizing clinical relevance, PICO (Population-Intervention-Comparison-Outcome) structuring, and risk-of-bias assessment. These methods are transferable and represent a separate literature.

**Targeted by**: EQ-05

---

### Gap 6: Living/Continuous Review Automation (Underdeveloped Frontier)

**Evidence**: The frontier dimension has only 2 papers (Evolving Literature Analysis 2502.18791, vitaLITy 2 2408.13450). The living systematic review literature in evidence-based medicine is much larger.

**Impact**: The spec identifies "longitudinal updates" as an open challenge, but the pool lacks the broader living review automation literature. This literature addresses incremental evidence updating, which directly informs the living survey frontier.

**Targeted by**: EQ-07

---

### Gap 7: Pre-LLM Systematic Review Foundations (Missing Context)

**Evidence**: The scope explicitly excludes "traditional pre-LLM systematic review tools and meta-analysis methodology," but the bibliography/citation graph structure originated there.

**Impact**: Understanding what pre-LLM automation achieved (screening prioritization, automated data extraction, risk-of-bias assessment) provides essential context for evaluating LLM-era advances. The survey risks being abistorical without this boundary understanding.

**Targeted by**: EQ-08, EQ-09

---

### Gap 8: Broad Evaluation Methodology (Missing General Eval Literature)

**Evidence**: The benchmark dimension focuses on survey-specific benchmarks (SurveyBench, SurGE, SurveyEval) and specific metrics (FActScore, ROUGE). The broader evaluation literature for long-form NLG is missing.

**Impact**: General evaluation frameworks for factual consistency (FEVER lineage, TRUE, SummaC, QAFactEval), coherence, and informativeness apply directly to survey evaluation. Missing these means the evaluation dimension is narrower than it needs to be.

**Targeted by**: EQ-10, EQ-13

---

## Supplementary Gaps

| Gap | Description | Targeted By |
|-----|-------------|-------------|
| Interactive collaboration | Human-AI collaborative synthesis tools using different terminology | EQ-06 |
| Structure-aware retrieval | Retrieval methods accounting for document section hierarchy | EQ-11 |
| Educational surveys | Survey generation for learning materials, different use case | EQ-12 |

---

## Query-to-Gap Mapping

| Extended Query | Primary Gap(s) Addressed |
|----------------|-------------------------|
| EQ-01 | Gap 1 (Related Work Framing) |
| EQ-02 | Gap 2 (MDS Lineage) |
| EQ-03 | Gap 3 (Knowledge-Grounded Generation) |
| EQ-04 | Gap 4 (Scientific Writing Automation) |
| EQ-05 | Gap 5 (Domain-Specific Methods) |
| EQ-06 | Supplementary (Interactive Collaboration) |
| EQ-07 | Gap 6 (Living Reviews) |
| EQ-08 | Gap 7 (Bibliometric Foundations) |
| EQ-09 | Gap 7 (Pre-LLM Foundations) |
| EQ-10 | Gap 8 (Broad Evaluation) |
| EQ-11 | Supplementary (Structure-Aware Retrieval) |
| EQ-12 | Supplementary (Educational Surveys) |
| EQ-13 | Gap 8 (Broad Evaluation) |
