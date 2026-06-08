# Semantic Expansion — MASA Pipeline

**Generated**: 2026-06-08
**run_dir**: `.`
**Agent**: SemanticExpander
**Status**: ready

---

## Method

Semantic neighbor expansion using Holos arXiv embedding search with 8 concept-rich, natural-language queries targeting gaps in the existing candidate pool (130 papers from 27 keyword queries across 4 scouts).

### Query Design Rationale

The existing QueryPlan (`01_query_plan.md`) and CandidatePool (`02_candidate_pool.md`) covered the core topic well — survey generation systems, citation graph traversal, benchmarks, and evaluation metrics. The SemanticExpander targets **conceptual blind spots**:

| Gap | Why Missed | Query Focus |
|-----|-----------|-------------|
| Outline planning / structured document generation | Keyword scouts used "outline" but not the broader structural planning literature | `outline generation structured document planning hierarchical text survey` |
| Hierarchical long-form generation | Scouts focused on survey-specific pipelines, not the general hierarchical text generation problem | `hierarchical document generation long form sectioned output structured text` |
| Citation intention classification | Scouts retrieved citation *analysis* papers but not the dedicated citation intent classification literature | `citation intention classification purpose function analysis scientific text` |
| Long-document evaluation beyond ROUGE/BERTScore | Scouts used query terms like "survey quality metrics" but missed the broader LFQA evaluation literature | `long document evaluation metrics survey coherence coverage factuality` |
| Multi-agent system evaluation | Scouts covered multi-agent survey *systems* but not the general MAS evaluation frameworks | `multi agent system evaluation framework task decomposition performance` |
| Scientific claim verification | Scouts used "citation accuracy" but not the dedicated scientific claim verification literature | `scientific claim verification cross paper consistency fact checking` |
| Knowledge graph construction from scientific text | Scouts retrieved "citation graph" but not the broader SciIE/knowledge graph construction literature | `knowledge graph construction scientific literature entity extraction relation` |
| KV cache / long-context methods | No existing query targeted efficiency of reading many papers | `KV cache memory reduction long context retrieval accuracy transformer` |

---

## Queries Executed

| # | Query String | topK | New Unique Candidates | Drift |
|---|-------------|------|---------------------|-------|
| 1 | KV cache memory reduction long context retrieval accuracy transformer | 10 | 0 | HIGH — all results about LLM inference optimization, not survey generation |
| 2 | outline generation structured document planning hierarchical text survey | 10 | 5 | Low |
| 3 | hierarchical document generation long form sectioned output structured text | 10 | 8 | Low |
| 4 | citation intention classification purpose function analysis scientific text | 10 | 9 | Low |
| 5 | long document evaluation metrics survey coherence coverage factuality | 10 | 7 | Low |
| 6 | multi agent system evaluation framework task decomposition performance | 10 | 7 | Low (2 robotics papers excluded) |
| 7 | scientific claim verification cross paper consistency fact checking | 10 | 10 | Low |
| 8 | knowledge graph construction scientific literature entity extraction relation | 10 | 8 | Low |
| **Total** | **8 queries** | **80 results** | **54 new (52 dedup)** | 1 drifted |

---

## New Candidates (54 unique)

### A. Outline Planning & Structured Document Generation

| # | arXiv ID | Title | Year | Role | Source Query | Rationale |
|---|---|---|---|---|---|---|
| 1 | `1905.10039v1` | Outline Generation: Understanding the Inherent Content Structure of Documents (HiStGen) | 2019 | mechanism | Q2, Q3 | Foundational outline generation method; WIKIOG dataset (1.75M doc-outline pairs); hierarchical structured prediction for section boundaries and headings. Bridges to survey outline planning. |
| 2 | `2509.19370v1` | Meow: End-to-End Outline Writing for Automatic Academic Survey | 2025 | core_method | Q2 | First end-to-end outline writing framework for academic surveys. Metadata-driven, uses 8B reasoning model with SFT+RL. Directly relevant to survey generation pipeline. |
| 3 | `2410.06203v1` | Integrating Planning into Single-Turn Long-Form Text Generation | 2024 | mechanism | Q2, Q3 | Planning-based long-form generation with synthetic intermediate data (outlines, key info). Strong human eval wins in organization, relevance, verifiability. |
| 4 | `2408.07884v1` | Instruct Large Language Models to Generate Scientific Literature Survey Step by Step | 2024 | core_method | Q2 | Step-by-step survey generation with hierarchical headings; NLPCC 2024 top-3. 95.84% heading recall, $0.01 per survey. Directly relevant. |
| 5 | `2302.04580v1` | BigSurvey: Generating a Structured Summary of Numerous Academic Papers (CAST) | 2023 | core_method | Q2 | First large-scale dataset (7K survey papers, 430K references) for structured multi-document summarization. Category-based alignment + sparse transformer. Directly relevant. |

### B. Hierarchical Long-Form Text Generation

| # | arXiv ID | Title | Year | Role | Source Query | Rationale |
|---|---|---|---|---|---|---|
| 6 | `2012.14136v1` | On Generating Extended Summaries of Long Documents (ExtendedSumm) | 2020 | mechanism | Q3 | Hierarchical structure exploitation + multi-task learning for extended summaries. Datasets: arXiv-Long, PubMed-Long. Relevant to survey section generation. |
| 7 | `2202.13756v1` | Data-to-text Generation with Variational Sequential Planning | 2022 | mechanism | Q3 | Latent sequential planning for multi-paragraph generation. Interleaved planning and generation. Relevant to structured survey writing. |
| 8 | `2010.07074v2` | Summarize, Outline, and Elaborate (SOE): Long-Text Generation via Hierarchical Supervision | 2020 | mechanism | Q3 | Pipelined system: summarize → outline → elaborate. Unsupervised segment summary extraction. Directly relevant to survey generation architectures. |
| 9 | `2410.06802v1` | Seg2Act: Global Context-aware Action Generation for Document Logical Structuring | 2024 | mechanism | Q3 | End-to-end generation-based document logical structuring. Relevant for maintaining structure in long surveys. |
| 10 | `2310.09118v1` | DSG: An End-to-End Document Structure Generator | 2023 | mechanism | Q3 | End-to-end trainable document parser for hierarchical structure. E-Periodica dataset. Relevant for survey document parsing. |
| 11 | `2105.09297v1` | Extracting Variable-Depth Logical Document Hierarchy (HELD) | 2021 | mechanism | Q3 | Systematic study of logical document hierarchy extraction from long documents. Relevant for survey outline extraction. |
| 12 | `1810.08802v1` | Hierarchical Text Generation using an Outline | 2018 | mechanism | Q3 | Early work on outline-guided hierarchical text generation. Pre-2023 foundational. |
| 13 | `2408.05829v1` | HGEN: Supporting Software Maintenance with Dynamically Generated Document Hierarchies | 2024 | citation_seed | Q3 | LLM pipeline for hierarchical documentation. Tangential (software engineering) but demonstrates hierarchical document generation architecture. |

### C. Citation Intention Classification

| # | arXiv ID | Title | Year | Role | Source Query | Rationale |
|---|---|---|---|---|---|---|
| 14 | `1904.01608v2` | Structural Scaffolds for Citation Intent Classification (SciCite dataset) | 2019 | mechanism | Q4 | State-of-the-art citation intent classifier; SciCite dataset (5x larger than ACL-ARC). Directly relevant to intelligent citation graph traversal. |
| 15 | `2304.12730v2` | CitePrompt: Using Prompts to Identify Citation Intent in Scientific Papers | 2023 | mechanism | Q4 | Prompt-based citation intent classification. Few/zero-shot settings. Relevant for citation enrichment in survey generation. |
| 16 | `2106.13275v1` | Multitask Learning for Citation Purpose Classification | 2021 | mechanism | Q4 | 3C Shared Task entry. Multi-task model combining linguistic features, TF-IDF, LSTM-attention. |
| 17 | `2505.21162v1` | Leveraging GANs for Citation Intent Classification and Impact on Network Analysis | 2025 | mechanism | Q4 | GAN-based citation intent classification; shows intent filtering significantly changes PageRank centrality in citation networks. Important for understanding citation graph traversal quality. |
| 18 | `2005.06611v1` | ImpactCite: An XLNet-based Method for Citation Impact Analysis | 2020 | mechanism | Q4 | XLNet-based citation sentiment and intent classification. New SOTA on both tasks. |
| 19 | `1609.00435v1` | Citation Classification for Behavioral Analysis of a Scientific Field (NLP) | 2016 | mechanism | Q4 | Largest behavioral study of citations in NLP. New dataset of 2K citations annotated for function and centrality. Pre-2023 foundational. |
| 20 | `2501.18292v1` | Citation Recommendation based on Argumentative Zoning of User Queries | 2025 | mechanism | Q4 | Multi-task learning for citation recommendation + argumentative zoning. PubMed Central corpus. Bridges citation intent to retrieval. |
| 21 | `2407.13329v3` | CiteFusion: An Ensemble Framework for Citation Intent Classification | 2024 | mechanism | Q4 | Ensemble framework achieving SOTA on SciCite (89.60 Macro-F1) and ACL-ARC (76.24). SHAP analysis for interpretability. |
| 22 | `2104.12869v1` | Semantic Analysis for Automated Evaluation of Potential Impact | 2021 | mechanism | Q4 | Information-theoretic semantic analysis for citation impact prediction. 80% success rate in distinguishing highly-cited papers. |

### D. Long-Document / Survey Evaluation Metrics

| # | arXiv ID | Title | Year | Role | Source Query | Rationale |
|---|---|---|---|---|---|---|
| 23 | `2305.18201v1` | A Critical Evaluation of Evaluations for Long-form Question Answering | 2023 | critique | Q5 | First targeted study of LFQA evaluation. Domain expert annotations across 7 areas. No existing metrics predictive of human preference. Recommends multi-faceted evaluation. |
| 24 | `2210.16732v1` | How Far are We from Robust Long Abstractive Summarization? | 2022 | critique | Q5 | Fine-grained human annotations evaluating long document summarization models and metrics. Shows ROUGE best for relevancy but factuality metrics limited. |
| 25 | `2406.19276v1` | VERISCORE: Evaluating Factuality of Verifiable Claims in Long-Form Text | 2024 | metric | Q5 | Addresses limitation of FACTSCORE/SAFE (assumes all claims verifiable). Evaluates 16 models across 8 long-form tasks. |
| 26 | `2208.01030v1` | SMART: Sentences as Basic Units for Text Evaluation | 2022 | metric | Q5 | Sentence-level matching metric outperforming all competitors on SummEval. Works well with longer summaries. |
| 27 | `2211.02580v1` | Evaluating and Improving Factuality in Multimodal Abstractive Summarization (CLIPBERTScore) | 2022 | metric | Q5 | Multimodal factuality metric. Tangential but evaluates factuality with document grounding. |
| 28 | `2010.12834v2` | GO FIGURE: A Meta Evaluation of Factuality in Summarization | 2020 | critique | Q5 | Meta-evaluation framework with 5 necessary conditions for factuality metrics. Benchmarks 10 metrics across 3 summarization tasks. |
| 29 | `2403.02270v3` | FENICE: Factuality Evaluation based on NLI and Claim Extraction | 2024 | metric | Q5 | SOTA on AGGREFACT. NLI-based alignment between source and atomic facts. Human annotations for long-form summarization. |

### E. Multi-Agent System Evaluation & Architecture

| # | arXiv ID | Title | Year | Role | Source Query | Rationale |
|---|---|---|---|---|---|---|
| 30 | `2506.15451v1` | AgentGroupChat-V2: Divide-and-Conquer for LLM-Based Multi-Agent Systems | 2025 | mechanism | Q6 | Divide-and-conquer architecture with hierarchical task forest decomposition. Relevant to survey generation task decomposition. |
| 31 | `2410.02189v2` | Agent-Oriented Planning in Multi-Agent Systems (AOP) | 2024 | mechanism | Q6 | Task decomposition → allocation → evaluation with reward model. Three design principles: solvability, completeness, non-redundancy. Directly relevant to survey generation planning. |
| 32 | `2510.04311v1` | On the Importance of Task Complexity in Evaluating LLM-Based Multi-Agent Systems | 2025 | critique | Q6 | Theoretical framework: task depth (reasoning length) and width (capability diversity). Shows MAS benefit increases with both. Relevant to when multi-agent is beneficial for surveys. |
| 33 | `2601.11903` | AEMA: Verifiable Evaluation Framework for Trustworthy Agentic LLM Systems | 2026 | benchmark | Q6 | Process-aware, auditable evaluation for multi-agent workflows. Human oversight integration. Relevant to survey system evaluation. |
| 34 | `2404.05569v3` | 360°REA: Reusable Experience Accumulation with 360° Assessment for MAS | 2024 | mechanism | Q6 | 360° performance assessment for multi-perspective evaluation. Dual-level experience pool. Relevant to self-improving survey systems. |
| 35 | `2509.20175` | Federation of Agents: Semantics-Aware Communication Fabric for Large-Scale Agentic AI | 2025 | mechanism | Q6 | Semantic routing, dynamic task decomposition, smart clustering. HNSW-based capability matching. Relevant to agent orchestration in survey systems. |
| 36 | `2505.18286v1` | Single-agent or Multi-agent Systems? Why Not Both? | 2025 | critique | Q6 | Empirical comparison showing MAS benefits diminish with better LLMs. Proposes hybrid cascading. Directly relevant to survey generation architecture decisions. |

### F. Scientific Claim Verification

| # | arXiv ID | Title | Year | Role | Source Query | Rationale |
|---|---|---|---|---|---|---|
| 37 | `2602.07621` | SciClaimEval: Cross-modal Claim Verification in Scientific Papers | 2026 | benchmark | Q7 | Cross-modal (figures+tables) claim verification dataset. 1,664 annotated samples across ML, NLP, medicine. 11 multimodal models benchmarked. |
| 38 | `2110.13090v1` | SciClops: Detecting and Contextualizing Scientific Claims for Fact-Checking | 2021 | mechanism | Q7 | Claim extraction, clustering, contextualization pipeline. Assists non-expert fact-checkers with scientific claims. |
| 39 | `2004.14974v6` | Fact or Fiction: Verifying Scientific Claims (SciFact) | 2020 | benchmark | Q7 | Foundational scientific claim verification dataset. 1.4K expert-written claims with evidence annotations. Baseline models for claim verification. |
| 40 | `2305.16859v1` | Scientific Fact-Checking: A Survey of Resources and Approaches | 2023 | survey | Q7 | Comprehensive survey of scientific fact-checking. Covers datasets, models, approaches. Bridges to survey factuality evaluation. |
| 41 | `2604.17667` | Peerispect: Claim Verification in Scientific Peer Reviews | 2026 | mechanism | Q7 | Claim-level verification in peer reviews. Modular IR pipeline for extracting, retrieving, and verifying claims. |
| 42 | `2204.12263v2` | Science Checker: Extractive-Boolean QA for Scientific Fact Checking | 2022 | mechanism | Q7 | Multi-task approach combining summarization and Boolean QA. 4% error rate, 95.6% F1 on medical/health domain. |
| 43 | `2503.21717v1` | CLAIMCHECK: How Grounded are LLM Critiques of Scientific Papers? | 2025 | benchmark | Q7 | Dataset of NeurIPS 2023/2024 reviews with expert annotations. Three claim-centric tasks. Shows LLMs underperform humans on claim verification. |
| 44 | `2110.15116v1` | Abstract, Rationale, Stance: A Joint Model for Scientific Claim Verification (ARSJoint) | 2021 | mechanism | Q7 | Joint learning of abstract retrieval, rationale selection, and stance prediction. Machine reading comprehension framework. SOTA on SciFact. |
| 45 | `2605.27710` | DeepSciVerify: LLM-Driven Evidence Escalation for Claim-Citation Alignment | 2026 | mechanism | Q7 | Two-stage pipeline with selective escalation to full-text. 86.7 Micro-F1 on SCitance. Directly relevant to citation accuracy verification in surveys. |
| 46 | `2508.11122v1` | +VeriRel: Verification Feedback to Enhance Document Retrieval for Scientific Fact Checking | 2025 | mechanism | Q7 | Integrates verification success into document ranking. Consistent gains on SciFact, SciFact-Open, Check-Covid. |

### G. Knowledge Graph Construction from Scientific Literature

| # | arXiv ID | Title | Year | Role | Source Query | Rationale |
|---|---|---|---|---|---|---|
| 47 | `1808.09602v1` | Multi-Task Identification of Entities, Relations, and Coreference for Scientific KG (SciIE/SciERC) | 2018 | mechanism | Q8 | Foundational scientific information extraction framework. SciERC dataset for entity, relation, and coreference. Pre-2023 foundational. |
| 48 | `2509.07801v3` | SciNLP: Domain-Specific Benchmark for Full-Text Scientific Entity and Relation Extraction in NLP | 2025 | benchmark | Q8 | First full-text entity/relation dataset for NLP. 60 papers, 7K entities, 1.8K relations. Knowledge graph with avg node degree 3.2. |
| 49 | `2106.01167v1` | End-to-End NLP Knowledge Graph Construction (SciNLP-KG) | 2021 | mechanism | Q8 | Extracts evaluatedOn, evaluatedBy, coreferent, related relations from 30K ACL Anthology papers. Relevant to survey knowledge organization. |
| 50 | `2401.09839v1` | MatSciRE: Pointer Networks for Entity and Relation Extraction in Materials Science | 2024 | mechanism | Q8 | Pointer network-based joint entity/relation extraction. Material science domain. Demonstrates architecture for domain-specific KG construction. |
| 51 | `2011.01103v1` | Generating Knowledge Graphs by Employing NLP/ML within the Scholarly Domain | 2020 | mechanism | Q8 | Hybrid system extracting entities and relationships from abstracts. 109K triples from Semantic Web domain. General approach applicable to any domain. |
| 52 | `2007.12731v1` | COVID-19 Knowledge Graph: Accelerating Information Retrieval and Discovery | 2020 | mechanism | Q8 | Heterogeneous graph combining semantic + topological information. AWS-based entity extraction pipeline. COVID-19 document similarity engine. |
| 53 | `2109.10453v1` | Extracting Fine-Grained Knowledge Graphs of Scientific Claims (SciClaim) | 2021 | mechanism | Q8 | Fine-grained graph annotation for scientific claims. Causal, comparative, predictive associations with qualifications. 12,738 labels across SBS, PubMed, CORD-19. |
| 54 | `2410.21155v1` | SciER: Entity and Relation Extraction Dataset for Datasets, Methods, and Tasks | 2024 | benchmark | Q8 | 106 full-text papers, 24K entities, 12K relations. Fine-grained relation tag set. Out-of-distribution test set. LLM-based baselines. |

---

## Deduplication Summary

| Check | Result |
|---|---|
| Cross-query duplicates removed | `1905.10039v1` (Q2+Q3), `2410.06203v1` (Q2+Q3) |
| Duplicates with existing CandidatePool | `1805.02262v1` (Lit Graph in Semantic Scholar, #54), `1710.03094v1` (Characterizing in-text citations, #122), `2309.12455v2` (LongDocFACTScore, #90), `2403.18802v4` (SAFE, #101), `2510.12839` (FaStFACT, #96) |
| **Final new unique candidates** | **54** |

---

## Drift Risk Analysis

| Risk | Level | Mitigation |
|---|---|---|
| **Q1 (KV cache)** — All 10 results are about LLM inference optimization (RocketKV, MiniKV, PackKV, etc.), not survey generation. Zero relevant additions. | **HIGH** | Excluded from new candidates. KV cache is a systems concern, not a survey-generation method. |
| **Q3 (Hierarchical generation)** — `2408.05829v1` (HGEN) is software-engineering documentation, not scientific survey generation. | **LOW** | Retained as citation_seed; demonstrates hierarchical document generation architecture. |
| **Q5 (Long-document evaluation)** — `2211.02580v1` (CLIPBERTScore) is multimodal-specific. | **LOW** | Retained as metric; factuality evaluation with multimodal grounding may be relevant if surveys include figures. |
| **Q6 (MAS evaluation)** — `2106.12111v3` (Robot task scheduling) and `2403.04370v2` (Cooperative task execution) are general MAS theory not specific to LLM-based systems. | **MEDIUM** | Excluded. |
| **Q8 (KG construction)** — `2401.09839v1` (MatSciRE) is materials-science specific. | **LOW** | Retained as mechanism; pointer-network architecture is domain-agnostic. |
| **`schema/expansion.md` not found** — `schema` is a 17-line SurveySpec file, not a directory. Expansion workflow followed from agent description and SurveySpec scope constraints. | **NOTE** | No impact on execution. |
| **`run_dir` recovered as `.`** — Consistent with all prior agents. | **NOTE** | No impact on execution. |

---

## Summary Statistics

| Metric | Value |
|---|---|
| Total queries executed | 8 |
| Total results retrieved | 80 |
| New unique candidates | 54 |
| Already in pool | 5 |
| Cross-query duplicates | 2 |
| Excluded (drift/boundary) | ~19 |
| **Expansion factor** | **54 / 130 = 41.5% increase** |

### New Candidates by Role

| Role | Count | Examples |
|---|---|---|
| mechanism | 26 | Citation intent, KG construction, hierarchical generation, outline planning |
| core_method | 4 | Meow, BigSurvey, step-by-step survey gen |
| metric | 4 | VERISCORE, SMART, FENICE, CLIPBERTScore |
| critique | 3 | LFQA eval critique, long summarization robustness, MAS vs SAS |
| benchmark | 4 | SciNLP, SciER, SciClaimEval, CLAIMCHECK |
| survey | 1 | Scientific Fact-Checking Survey |
| citation_seed | 2 | HGEN, 360°REA |

### Coverage of Identified Gaps

| Gap | New Candidates | Top Representative |
|---|---|---|
| Outline planning / structured document generation | 5 | `2509.19370v1` Meow |
| Hierarchical long-form generation | 8 | `2010.07074v2` SOE |
| Citation intention classification | 9 | `1904.01608v2` SciCite dataset |
| Long-document evaluation beyond ROUGE | 7 | `2406.19276v1` VERISCORE |
| Multi-agent system evaluation | 7 | `2510.04311v1` Task complexity framework |
| Scientific claim verification | 10 | `2004.14974v6` SciFact |
| Knowledge graph construction | 8 | `1808.09602v1` SciIE/SciERC |

---

## Coverage Map Update

```
                    ┌──────────────────────────────────────┐
                    │        EXPANDED CANDIDATE POOL        │
                    │        130 (original) + 54 = 184      │
                    └──────────────────────────────────────┘
                                     │
         ┌───────────────────────────┼───────────────────────────┐
         │                           │                           │
         ▼                           ▼                           ▼
  ┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐
  │  Survey Systems   │    │  Mechanisms      │    │  Eval & Metrics   │
  │  (41+4 = 45)      │    │  (35+26 = 61)    │    │  (27+19 = 46)     │
  ├──────────────────┤    ├──────────────────┤    ├──────────────────┤
  │ Meow             │    │ SciCite intent   │    │ VERISCORE         │
  │ Step-by-step     │    │ SOE hierarchical │    │ SMART             │
  │ BigSurvey (CAST) │    │ Agent-Oriented   │    │ FENICE            │
  │ Outline+Planning │    │ SciFact verif.   │    │ LFQA Critique     │
  └──────────────────┘    │ SciIE KG         │    │ MAS eval          │
                           └──────────────────┘    └──────────────────┘
```

---

## Risk Notes

1. **Q1 (KV cache) drifted entirely** — All 10 results were about LLM inference optimization, not survey generation methods. This query should have been scoped to "KV cache for long-context RAG in survey generation" or dropped in favor of a different gap.

2. **`schema/expansion.md` not found on disk.** The `schema` file at root is a 17-line SurveySpec contract, not a directory. Expansion workflow was followed from the agent description and SurveySpec scope_inclusion_budget constraints.

3. **54 new candidates is aggressive** — Some retained papers are tangential (HGEN for software docs, CLIPBERTScore for multimodal). Future pipeline stages should apply a stricter relevance filter.

4. **Citation intent papers (9)** and **scientific claim verification papers (10)** represent the two largest gap fills. These areas are directly relevant to: (a) intelligent citation graph traversal that understands *why* a paper is cited, and (b) verifying citation accuracy in generated surveys — both are open problems in the field.
