# QueryPlan

**run_dir**: `.`
**source**: `00_survey_spec.md`
**generated**: 2026-06-07T10:12:38+08:00

---

## Overview

This query plan translates the SurveySpec into typed arXiv retrieval queries targeting **automated literature survey agents with citation graph expansion**. Each query is a short English search string optimised for arXiv semantic (embedding) search. Diversity is preserved across all types to give downstream scouts maximal coverage of architectures, mechanisms, evaluations, open problems, and adjacent fields.

**Total queries**: 22

---

## Core Method Queries (5)

Dominant architectural patterns for LLM-based survey agents that incorporate citation traversal.

| ID | Query | target_dimension | expected_gain | negative_filter | top_k |
|----|-------|-----------------|---------------|-----------------|-------|
| CM-01 | LLM-based agents for automated literature survey generation | method | General agent architectures for survey writing | Focus on single-paper Q&A, not survey construction | 10 |
| CM-02 | Citation graph traversal for paper discovery in survey systems | method | How citation graphs are used for systematic paper retrieval | Pure bibliometrics without LLM integration | 10 |
| CM-03 | Multi-agent architectures for academic literature review writing | method | Coordination patterns in multi-agent survey pipelines | Single-agent writing without retrieval division | 10 |
| CM-04 | Retrieval-augmented generation with citation chaining | method | RAG systems extended with forward/backward reference linking | General RAG without citation traversal | 10 |
| CM-05 | Iterative paper retrieval and synthesis for survey composition | method | Closed-loop retrieval-writing cycles with citation expansion | One-shot retrieval without iteration | 10 |

---

## Mechanism Queries (4)

Specific retrieval, synthesis, and graph-traversal mechanisms.

| ID | Query | target_dimension | expected_gain | negative_filter | top_k |
|----|-------|-----------------|---------------|-----------------|-------|
| M-01 | Forward and backward reference chaining in automated literature search | mechanism | Direct evidence on chaining strategies | Static citation list without traversal algorithms | 10 |
| M-02 | Citation-aware evidence aggregation for survey writing | mechanism | How systems collate evidence across papers | Aggregation without citation context | 10 |
| M-03 | Query expansion strategies for scientific literature discovery | mechanism | Techniques for broadening or refining search queries | General search relevance without survey focus | 10 |
| M-04 | Citation graph expansion algorithms for coverage maximization | mechanism | Breadth-first vs depth-first expansion, termination criteria | Graph analysis without LLM writing component | 10 |

---

## Problem Queries (3)

Known limitations and open challenges in citation-aware automated survey generation.

| ID | Query | target_dimension | expected_gain | negative_filter | top_k |
|----|-------|-----------------|---------------|-----------------|-------|
| P-01 | Hallucination and fabricated citations in LLM-generated surveys | limitation | Evidence on citation hallucination rates and causes | Generic LLM hallucination without citation focus | 10 |
| P-02 | Evaluation gaps and benchmark scarcity for automated survey systems | limitation | Identifies missing standardised benchmarks | Evaluation of manual review quality | 10 |
| P-03 | Temporal coverage and staleness in citation graph traversals | limitation | How quickly citation-chain surveys become outdated | General recency bias without citation graph context | 10 |

---

## Benchmark Queries (3)

Evaluation metrics, datasets, and human protocols.

| ID | Query | target_dimension | expected_gain | negative_filter | top_k |
|----|-------|-----------------|---------------|-----------------|-------|
| B-01 | Benchmarks and automatic metrics for survey quality evaluation | benchmark | ROUGE, BLEU, FactScore, citation precision/recall | Metrics for general NLG, not survey-specific | 10 |
| B-02 | Human evaluation protocols for LLM-generated literature reviews | benchmark | How human judges rate coverage, accuracy, coherence | Automated metrics without human judgement | 10 |
| B-03 | Citation precision and recall evaluation in machine-generated surveys | metric | Specific citation fidelity metrics for survey outputs | General attribution metrics for all text generation | 10 |

---

## Survey Queries (2)

Existing surveys or taxonomies of automated literature survey systems.

| ID | Query | target_dimension | expected_gain | negative_filter | top_k |
|----|-------|-----------------|---------------|-----------------|-------|
| S-01 | Survey of AI-powered literature review and synthesis systems | theory | Prior surveys of survey-generation field with taxonomies | Surveys on a different sub-topic | 10 |
| S-02 | Taxonomy of automated academic writing and survey generation tools | theory | Classification schemes for tool capabilities | Writing tools without survey/review purpose | 10 |

---

## Citation Seed Queries (3)

Named systems from the spec and literature.

| ID | Query | target_dimension | expected_gain | negative_filter | top_k |
|----|-------|-----------------|---------------|-----------------|-------|
| CS-01 | STORM system for AI-assisted literature review writing | method | Architecture and evaluation of STORM | Unrelated writing systems | 10 |
| CS-02 | AutoSurvey framework for automated survey generation | method | Pipeline and benchmark results of AutoSurvey | General generation without survey architecture | 10 |
| CS-03 | PaperQA and SurveyAgent for scientific paper synthesis | method | Multi-paper synthesis and citation-aware answering | Single-document QA systems | 10 |

---

## Boundary Queries (2)

Guardrails for what should *not* dominate downstream retrieval.

| ID | Query | target_dimension | expected_gain | negative_filter | top_k |
|----|-------|-----------------|---------------|-----------------|-------|
| BX-01 | General-purpose RAG systems for open-domain question answering | method | Catch false positives; exclude if no survey generation | Must explicitly exclude these via scout classifier | 10 |
| BX-02 | Traditional bibliometric citation analysis without LLM integration | method | Distinguish pre-LLM methodology from current scope | Must explicitly exclude these via scout classifier | 10 |

---

## Frontier Queries (2)

Emerging directions and unsolved challenges that push beyond current work.

| ID | Query | target_dimension | expected_gain | negative_filter | top_k |
|----|-------|-----------------|---------------|-----------------|-------|
| F-01 | Live updating and longitudinal survey maintenance with citation graphs | application | Systems for keeping surveys current as literature grows | One-time survey generation without updating | 10 |
| F-02 | Interactive and user-steerable automated survey generation | application | User-driven steering of survey scope and coverage | Fully autonomous generation without user input | 10 |

---

## Cross-Domain Queries (2)

Adjacent fields that inform survey-generation techniques.

| ID | Query | target_dimension | expected_gain | negative_filter | top_k |
|----|-------|-----------------|---------------|-----------------|-------|
| CD-01 | Knowledge graph traversal and expansion for information retrieval | theory | Graph traversal algorithms applicable to citation graphs | Pure graph theory without retrieval context | 10 |
| CD-02 | Multi-agent orchestration frameworks for complex research tasks | method | Coordination patterns transferable to survey pipelines | Single-agent task decomposition without multi-agent | 10 |

---

## Query Distribution Summary

| Type | Count | Purpose |
|------|-------|---------|
| Core Method | 5 | Architectural patterns |
| Mechanism | 4 | Specific techniques |
| Problem | 3 | Limitations and challenges |
| Benchmark | 3 | Metrics and evaluation |
| Survey | 2 | Existing surveys |
| Citation Seed | 3 | Named systems |
| Boundary | 2 | Exclusion guardrails |
| Frontier | 2 | Emerging directions |
| Cross-Domain | 2 | Adjacent fields |
| **Total** | **22** | |

---

## Notes

- All queries are short English strings suitable for arXiv embedding-based semantic search.
- Diversity is preserved across all nine types to give downstream scouts maximal coverage.
- Boundary queries (BX-01, BX-02) serve as negative-guard signals for the scout classifier — related but out-of-scope content.
- If any query yields zero results, downstream scouts should fall back to adjacent queries within the same type.
- Citation Seed queries (CS-01 through CS-03) can act as entry points for forward/backward reference chaining during discovery expansion.
