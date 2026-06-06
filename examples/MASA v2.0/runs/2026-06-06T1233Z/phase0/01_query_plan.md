# QueryPlan

**run_dir**: `.`
**source**: `00_survey_spec.md`
**generated**: 2026-06-06T12:34:39+08:00

---

## Overview

This query plan translates the SurveySpec into typed arXiv retrieval queries. Each query is a short English search string targeting arXiv semantic search. Queries are organised by type to ensure downstream scouts can cover architectural patterns, evaluation methods, open problems, and boundary cases.

**Total queries**: 22

---

## Core Method Queries (4)

Clarify the dominant architectural patterns for LLM-based survey generation.

| ID | Query | Rationale |
|----|-------|-----------|
| `CM-01` | LLM agent architectures for automated survey generation | Anchors the primary question; captures general agent designs for writing tasks |
| `CM-02` | Multi-agent systems for academic literature review writing | Targets coordination patterns in multi-agent survey systems |
| `CM-03` | Retrieval-augmented generation for survey paper composition | Connects RAG to the specific task of composing structured surveys |
| `CM-04` | Hierarchical planning and outline decomposition in LLM writing | Captures planning strategies (hierarchical, iterative) |

---

## Mechanism Queries (5)

Probe specific retrieval, synthesis, and attribution mechanisms.

| ID | Query | Rationale |
|----|-------|-----------|
| `M-01` | Citation-grounded evidence extraction and attribution in RAG | Citation fidelity is a core quality concern |
| `M-02` | Query formulation strategies for scientific literature retrieval | Retrieval quality depends on how queries are formed |
| `M-03` | Iterative refinement and multi-pass drafting for long-form generation | Covers drafting/refinement loops |
| `M-04` | Multi-source synthesis and evidence aggregation in survey systems | How systems combine evidence from many papers |
| `M-05` | Citation graph expansion techniques for literature coverage | Coverage through graph traversal |

---

## Problem Queries (3)

Capture known limitations and open challenges.

| ID | Query | Rationale |
|----|-------|-----------|
| `P-01` | Hallucination and citation fabrication in automated literature surveys | Key limitation; citation hallucination |
| `P-02` | Evaluation standardization challenges for machine-generated surveys | Lack of standardised benchmarks |
| `P-03` | Coverage and completeness gaps in LLM-based survey generation | Gaps in domain coverage, stale knowledge |

---

## Benchmark Queries (3)

Evaluation metrics, benchmarks, and human evaluation protocols.

| ID | Query | Rationale |
|----|-------|-----------|
| `B-01` | Benchmarks and automatic metrics for survey quality evaluation | ROUGE, BLEU, citation precision/recall, FactScore |
| `B-02` | Human evaluation protocols for automated literature reviews | How human judges rate surveys |
| `B-03` | Citation precision and recall evaluation for generated surveys | Specific citation fidelity metrics |

---

## Survey Queries (2)

Catch existing surveys or taxonomies of the field.

| ID | Query | Rationale |
|----|-------|-----------|
| `S-01` | Survey of automated literature review systems using LLMs | Prior surveys of the survey generation field |
| `S-02` | Taxonomy of AI-assisted academic writing and synthesis tools | Broader taxonomy covering relevant tools |

---

## Citation Seed Queries (4)

Named systems mentioned in the spec. Each query targets the system + its methodology.

| ID | Query | Rationale |
|----|-------|-----------|
| `CS-01` | AutoSurvey system for automated literature survey generation | Named anchor system |
| `CS-02` | STORM system for writing structured literature reviews | Named anchor system |
| `CS-03` | PaperQA agent for scientific question answering and synthesis | Named anchor system |
| `CS-04` | SurveyAgent architecture for multi-paper synthesis and survey writing | Named anchor system |

---

## Boundary Queries (2)

Ensure downstream scouts recognise what should *not* dominate retrieval.

| ID | Query | Rationale |
|----|-------|-----------|
| `BX-01` | General-purpose RAG for document retrieval without writing tasks | Catch false positives; exclude if no writing component |
| `BX-02` | Traditional systematic review tools without LLM components | Distinguish pre-LLM methodology |

---

## Frontier Queries (2)

Emerging directions and open challenges that push beyond current work.

| ID | Query | Rationale |
|----|-------|-----------|
| `F-01` | Living and longitudinal literature survey updates with LLMs | Stale knowledge / updating surveys |
| `F-02` | User-steerable and interactive survey generation systems | User steering as open challenge |

---

## Cross-Domain Queries (2)

Adjacent fields that inform survey generation.

| ID | Query | Rationale |
|----|-------|-----------|
| `CD-01` | Multi-agent coordination frameworks for knowledge-intensive tasks | AutoGen and coordination patterns |
| `CD-02` | Self-RAG and RankRAG retrieval mechanisms for factuality | Adjacent retrieval methods that improve factuality |

---

## Query Distribution Summary

| Type | Count | Purpose |
|------|-------|---------|
| Core Method | 4 | Architectural patterns |
| Mechanism | 5 | Specific techniques |
| Problem | 3 | Limitations and challenges |
| Benchmark | 3 | Metrics and evaluation |
| Survey | 2 | Existing surveys |
| Citation Seed | 4 | Named systems |
| Boundary | 2 | Exclusion guardrails |
| Frontier | 2 | Emerging directions |
| Cross-Domain | 2 | Adjacent fields |
| **Total** | **22** | |

---

## Notes

- All queries are short English strings suitable for arXiv semantic search (embedding-indexed).
- Diversity is preserved across all nine types to give downstream scouts maximal coverage.
- The `boundary` queries (BX-01, BX-02) serve as negative-guard signals for the scout classifier.
- If any query yields zero results, downstream scouts may fall back to adjacent queries within the same type.
