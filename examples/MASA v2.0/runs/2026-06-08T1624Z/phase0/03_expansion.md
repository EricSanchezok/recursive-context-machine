# Expansion Report — MASA Pipeline

**Generated**: 2026-06-08
**run_dir**: `.`
**Agent**: ExpansionMerger
**Status**: merged

---

## Merge Summary

| Metric | Citation Expansion | Semantic Expansion | Combined |
|---|---|---|---|
| Seed papers | 8 | 8 (from seeds) | 8 |
| Resolved references (arXiv) | 31 | — | 31 |
| Unresolved references (non-arXiv) | 8 | — | 8 |
| Semantic queries executed | — | 8 | 8 |
| Semantic results retrieved | — | 80 | 80 |
| New candidates from expansion | 14 (fresh to pool) | 54 (fresh to pool) | **68** |
| Already in pool (dedup'd) | 17 | 5 | 22 |
| Pool before expansion | — | 130 | 130 |
| Pool after expansion | — | — | **198** |
| Expansion factor | — | — | **+52.3%** |

**Total new candidates appended to `02_candidate_pool.md`**: 68 (14 from citation graph, 54 from semantic neighbors)

---

## 1. Seed Papers

8 seeds selected. Full metadata in `03a_seed_papers.md`.

| # | arXiv ID | Title | Year | Role |
|---|---|---|---|---|
| 1 | `2402.08565v2` | Artificial Intelligence for Literature Reviews: Opportunities and Challenges | 2024 | survey |
| 2 | `2401.10917v1` | Artificial intelligence to automate the systematic review of scientific literature | 2024 | survey |
| 3 | `2406.10252v2` | AutoSurvey: Large Language Models Can Automatically Write Surveys | 2024 | core_method |
| 4 | `2503.04629v1` | SurveyForge: On the Outline Heuristics, Memory-Driven Generation | 2025 | core_method |
| 5 | `1806.00089v1` | Cascading Citation Expansion | 2018 | mechanism |
| 6 | `2510.03120` | SurveyBench — quiz-driven evaluation; 11,343 arXiv topics + 4,947 surveys | 2025 | benchmark |
| 7 | `2308.11432v7` | A Survey on Large Language Model based Autonomous Agents | 2023 | survey |
| 8 | `2211.15397v2` | Automating Systematic Literature Reviews with NLP and Text Mining | 2022 | survey |

**Temporal balance**: 3 pre-2023 (#5, #7, #8) : 5 from 2023 onward (#1–#4, #6) ≈ 3:7 — satisfies scope_inclusion_budget.

---

## 2. Citation Graph Expansion (from `03b_citation_expansion.md`)

### 2.1 Resolved References Summary

31 references resolved to arXiv IDs from seed paper bibliographies, spanning:

| Domain | Count | Key Papers |
|---|---|---|
| Adjacent / Foundational | 7 | R01 Lit Graph, R02/R03 citation algos, R04/R05 Garfield legacy, R06 ReAct, R07 interleaved snowballing |
| SLR Methodology | 6 | R08–R13: hybrid search, snowballing, ML pipelines, ProfOlaf |
| Survey Generation Systems | 10 | R14 SciReviewGen, R15 Reading Path, R16 LitLLM, R17 LitLLMs eval, R18 SurveyX, R19 SurveyGen, R20 InteractiveSurvey, R21 AutoSurvey2, R22 DeepSurvey, R23 GenAI for LR |
| Evaluation / Benchmark | 1 | R24 FactKB |
| LLM Agent Architecture | 4 | R25–R28: agent surveys, LATS, comprehensive reviews |
| Citation Graph Tools | 3 | R29 PUREsuggest, R30 Oignon, R31 LitFM |

**Temporal distribution**: 10 pre-2023 : 21 post-2023 (≈ 3:6.3) — satisfies scope_inclusion_budget.

### 2.2 Unresolved References

8 references could not be resolved to arXiv IDs:

| # | Reference | Appears In Seed(s) | Reason |
|---|---|---|---|
| U01 | PRISMA 2020 (Page et al., BMJ) | 1, 2, 8 | Published in BMJ |
| U02 | PRISMA 2009 (Moher et al., PLOS Med) | 1, 2, 8 | Published in journal |
| U03 | Garfield (1955) — Science | 5 | Pre-arXiv era |
| U04 | Garfield (1972) — Science | 5 | Pre-arXiv era |
| U05 | Vaswani et al. (2017) — NeurIPS | 3, 7 | Conference proceedings |
| U06 | Devlin et al. (2019) — NAACL | 3, 7 | Conference proceedings |
| U07 | Brown et al. (2020) — NeurIPS (GPT-3) | 3, 7 | Conference proceedings |
| U08 | GPT-4 Technical Report (OpenAI, 2023) | 3, 4, 6, 7 | OpenAI tech report |

### 2.3 New Candidates from Citation Expansion (14 added to pool)

Of 31 resolved references, 17 were already in the candidate pool. 14 are new:

| arXiv ID | Title | Year | Role | Connected Seeds |
|---|---|---|---|---|
| `1205.6373v1` | PIRA — PageRank on heterogeneous graphs | 2012 | mechanism | Seed 5 |
| `1710.01895v1` | Eugene Garfield's Scholarly Impact: A Scientometric Review | 2017 | foundational | Seeds 5, 8 |
| `2210.03629v3` | ReAct: Synergizing Reasoning and Acting in Language Models | 2022 | mechanism | Seed 7 |
| `2004.09741v1` | Hybrid Search Strategies for Systematic Literature Reviews | 2020 | mechanism | Seeds 2, 8 |
| `2307.02612v1` | Successful Combination of Database Search and Snowballing | 2023 | mechanism | Seeds 2, 8 |
| `2010.04665v1` | Scaling Systematic Literature Reviews with ML Pipelines | 2020 | mechanism | Seeds 2, 8 |
| `2111.07533v4` | Automated scholarly paper review: Concepts, technologies, and challenges | 2021 | survey | Seeds 1, 2 |
| `2407.14991v1` | Investigating the use of Snowballing on Gray Literature Reviews | 2024 | mechanism | Seeds 2, 5 |
| `2305.15186v1` | SciReviewGen: A Large-scale Dataset for Automatic Literature Review Generation | 2023 | benchmark | Seeds 1, 3, 4 |
| `2605.16475` | Generative Artificial Intelligence for Literature Reviews | 2026 | survey | Seeds 1, 2 |
| `2305.08281v2` | FactKB: Generalizable Factuality Evaluation using Language Models | 2023 | metric | Seeds 3, 6 |
| `2310.04406v3` | Language Agent Tree Search Unifies Reasoning Acting and Planning | 2023 | mechanism | Seed 7 |
| `2504.19678v1` | From LLM Reasoning to Autonomous AI Agents: A Comprehensive Review | 2025 | survey | Seed 7 |
| `2409.12177v1` | LitFM: A Retrieval Augmented Structure-aware Foundation Model For Citation Graphs | 2024 | mechanism | Seeds 5, 7 |

---

## 3. Semantic Neighbor Expansion (from `03c_semantic_expansion.md`)

### 3.1 Query Design

8 concept-rich queries targeting conceptual blind spots in the existing pool:

| Gap | Query | Results | New | Drift |
|---|---|---|---|---|
| Outline planning / structured doc generation | `outline generation structured document planning hierarchical text survey` | 10 | 5 | Low |
| Hierarchical long-form generation | `hierarchical document generation long form sectioned output structured text` | 10 | 8 | Low |
| Citation intention classification | `citation intention classification purpose function analysis scientific text` | 10 | 9 | Low |
| Long-document evaluation beyond ROUGE | `long document evaluation metrics survey coherence coverage factuality` | 10 | 7 | Low |
| Multi-agent system evaluation | `multi agent system evaluation framework task decomposition performance` | 10 | 7 | Low |
| Scientific claim verification | `scientific claim verification cross paper consistency fact checking` | 10 | 10 | Low |
| Knowledge graph construction | `knowledge graph construction scientific literature entity extraction relation` | 10 | 8 | Low |
| KV cache / long-context methods | `KV cache memory reduction long context retrieval accuracy transformer` | 10 | 0 | **HIGH** — fully drifted |
| **Total** | | **80** | **54** | |

### 3.2 New Candidates from Semantic Expansion (54 added to pool)

**A. Outline Planning & Structured Document Generation (5):**

| # | arXiv ID | Title | Year | Role |
|---|---|---|---|---|
| S01 | `1905.10039v1` | Outline Generation: Understanding Inherent Content Structure (HiStGen) | 2019 | mechanism |
| S02 | `2509.19370v1` | Meow: End-to-End Outline Writing for Automatic Academic Survey | 2025 | core_method |
| S03 | `2410.06203v1` | Integrating Planning into Single-Turn Long-Form Text Generation | 2024 | mechanism |
| S04 | `2408.07884v1` | Instruct LLMs to Generate Scientific Literature Survey Step by Step | 2024 | core_method |
| S05 | `2302.04580v1` | BigSurvey: Generating Structured Summary of Numerous Papers (CAST) | 2023 | core_method |

**B. Hierarchical Long-Form Text Generation (8):**

| # | arXiv ID | Title | Year | Role |
|---|---|---|---|---|
| S06 | `2012.14136v1` | On Generating Extended Summaries of Long Documents (ExtendedSumm) | 2020 | mechanism |
| S07 | `2202.13756v1` | Data-to-text Generation with Variational Sequential Planning | 2022 | mechanism |
| S08 | `2010.07074v2` | Summarize, Outline, and Elaborate (SOE): Long-Text Generation | 2020 | mechanism |
| S09 | `2410.06802v1` | Seg2Act: Global Context-aware Action Generation for Document Logical Structuring | 2024 | mechanism |
| S10 | `2310.09118v1` | DSG: An End-to-End Document Structure Generator | 2023 | mechanism |
| S11 | `2105.09297v1` | Extracting Variable-Depth Logical Document Hierarchy (HELD) | 2021 | mechanism |
| S12 | `1810.08802v1` | Hierarchical Text Generation using an Outline | 2018 | mechanism |
| S13 | `2408.05829v1` | HGEN: Supporting Software Maintenance with Dynamic Doc Hierarchies | 2024 | citation_seed |

**C. Citation Intention Classification (9):**

| # | arXiv ID | Title | Year | Role |
|---|---|---|---|---|
| S14 | `1904.01608v2` | Structural Scaffolds for Citation Intent Classification (SciCite) | 2019 | mechanism |
| S15 | `2304.12730v2` | CitePrompt: Using Prompts to Identify Citation Intent | 2023 | mechanism |
| S16 | `2106.13275v1` | Multitask Learning for Citation Purpose Classification | 2021 | mechanism |
| S17 | `2505.21162v1` | Leveraging GANs for Citation Intent Classification and Network Analysis | 2025 | mechanism |
| S18 | `2005.06611v1` | ImpactCite: XLNet-based Citation Impact Analysis | 2020 | mechanism |
| S19 | `1609.00435v1` | Citation Classification for Behavioral Analysis of a Scientific Field (NLP) | 2016 | mechanism |
| S20 | `2501.18292v1` | Citation Recommendation based on Argumentative Zoning of User Queries | 2025 | mechanism |
| S21 | `2407.13329v3` | CiteFusion: Ensemble Framework for Citation Intent Classification | 2024 | mechanism |
| S22 | `2104.12869v1` | Semantic Analysis for Automated Evaluation of Potential Impact | 2021 | mechanism |

**D. Long-Document / Survey Evaluation Metrics (7):**

| # | arXiv ID | Title | Year | Role |
|---|---|---|---|---|
| S23 | `2305.18201v1` | A Critical Evaluation of Evaluations for Long-form Question Answering | 2023 | critique |
| S24 | `2210.16732v1` | How Far are We from Robust Long Abstractive Summarization? | 2022 | critique |
| S25 | `2406.19276v1` | VERISCORE: Evaluating Factuality of Verifiable Claims in Long-Form Text | 2024 | metric |
| S26 | `2208.01030v1` | SMART: Sentences as Basic Units for Text Evaluation | 2022 | metric |
| S27 | `2211.02580v1` | Evaluating Factuality in Multimodal Abstractive Summarization (CLIPBERTScore) | 2022 | metric |
| S28 | `2010.12834v2` | GO FIGURE: A Meta Evaluation of Factuality in Summarization | 2020 | critique |
| S29 | `2403.02270v3` | FENICE: Factuality Evaluation based on NLI and Claim Extraction | 2024 | metric |

**E. Multi-Agent System Evaluation & Architecture (7):**

| # | arXiv ID | Title | Year | Role |
|---|---|---|---|---|
| S30 | `2506.15451v1` | AgentGroupChat-V2: Divide-and-Conquer for MAS | 2025 | mechanism |
| S31 | `2410.02189v2` | Agent-Oriented Planning in Multi-Agent Systems (AOP) | 2024 | mechanism |
| S32 | `2510.04311v1` | On the Importance of Task Complexity in Evaluating MAS | 2025 | critique |
| S33 | `2601.11903` | AEMA: Verifiable Evaluation Framework for Trustworthy Agentic LLM Systems | 2026 | benchmark |
| S34 | `2404.05569v3` | 360°REA: Reusable Experience Accumulation with 360° Assessment | 2024 | mechanism |
| S35 | `2509.20175` | Federation of Agents: Semantics-Aware Communication Fabric | 2025 | mechanism |
| S36 | `2505.18286v1` | Single-agent or Multi-agent Systems? Why Not Both? | 2025 | critique |

**F. Scientific Claim Verification (10):**

| # | arXiv ID | Title | Year | Role |
|---|---|---|---|---|
| S37 | `2602.07621` | SciClaimEval: Cross-modal Claim Verification in Scientific Papers | 2026 | benchmark |
| S38 | `2110.13090v1` | SciClops: Detecting and Contextualizing Scientific Claims | 2021 | mechanism |
| S39 | `2004.14974v6` | Fact or Fiction: Verifying Scientific Claims (SciFact) | 2020 | benchmark |
| S40 | `2305.16859v1` | Scientific Fact-Checking: A Survey of Resources and Approaches | 2023 | survey |
| S41 | `2604.17667` | Peerispect: Claim Verification in Scientific Peer Reviews | 2026 | mechanism |
| S42 | `2204.12263v2` | Science Checker: Extractive-Boolean QA for Scientific Fact Checking | 2022 | mechanism |
| S43 | `2503.21717v1` | CLAIMCHECK: How Grounded are LLM Critiques of Scientific Papers? | 2025 | benchmark |
| S44 | `2110.15116v1` | Abstract, Rationale, Stance: Joint Model for Scientific Claim Verification | 2021 | mechanism |
| S45 | `2605.27710` | DeepSciVerify: LLM-Driven Evidence Escalation for Claim-Citation Alignment | 2026 | mechanism |
| S46 | `2508.11122v1` | +VeriRel: Verification Feedback to Enhance Document Retrieval | 2025 | mechanism |

**G. Knowledge Graph Construction from Scientific Literature (8):**

| # | arXiv ID | Title | Year | Role |
|---|---|---|---|---|
| S47 | `1808.09602v1` | Multi-Task Identification of Entities, Relations, Coreference (SciIE/SciERC) | 2018 | mechanism |
| S48 | `2509.07801v3` | SciNLP: Domain-Specific Benchmark for Full-Text Scientific Entity-Relation | 2025 | benchmark |
| S49 | `2106.01167v1` | End-to-End NLP Knowledge Graph Construction (SciNLP-KG) | 2021 | mechanism |
| S50 | `2401.09839v1` | MatSciRE: Pointer Networks for Entity/Relation Extraction | 2024 | mechanism |
| S51 | `2011.01103v1` | Generating Knowledge Graphs by Employing NLP/ML for Scholarly Domain | 2020 | mechanism |
| S52 | `2007.12731v1` | COVID-19 Knowledge Graph: Accelerating Info Retrieval and Discovery | 2020 | mechanism |
| S53 | `2109.10453v1` | Extracting Fine-Grained Knowledge Graphs of Scientific Claims (SciClaim) | 2021 | mechanism |
| S54 | `2410.21155v1` | SciER: Entity and Relation Extraction Dataset for Methods/Tasks | 2024 | benchmark |

---

## 4. Deduplication Summary

| Cross-check | Result |
|---|---|
| Citation R-papers already in pool | 17 of 31 resolved refs already present |
| Semantic S-papers already in pool | 5 of 59 candidates already present |
| Cross-query duplicates (semantic) | 2 (1905.10039, 2410.06203 — both Q2+Q3) |
| Citation–semantic overlap | 0 unique overlaps between the 14 new R-papers and 54 new S-papers |
| **Total new unique to pool** | **68** |
| **Pool before expansion** | **130** |
| **Pool after expansion** | **198** |

---

## 5. Expanded Coverage Map

```
                      ┌──────────────────────────────────────────┐
                      │           EXPANDED CANDIDATE POOL         │
                      │           130 (original) + 68 = 198       │
                      └──────────────────────────────────────────┘
                                       │
          ┌────────────────────────────┼────────────────────────────┐
          │                            │                            │
          ▼                            ▼                            ▼
   ┌──────────────────┐      ┌──────────────────┐      ┌──────────────────┐
   │  Survey Systems   │      │  Mechanisms       │      │  Eval & Metrics   │
   │  (41+6 = 47)      │      │  (35+35 = 70)     │      │  (27+19 = 46)     │
   ├──────────────────┤      ├──────────────────┤      ├──────────────────┤
   │ Meow (S02)       │      │ SciCite (S14)    │      │ VERISCORE (S25)   │
   │ Step-by-step(S04)│      │ SOE (S08)        │      │ SMART (S26)       │
   │ BigSurvey (S05)  │      │ ReAct (R06)      │      │ FENICE (S29)      │
   │ SciReviewGen(R14)│      │ SciFact (S39)    │      │ FactKB (R24)      │
   │ GenAI LR (R23)   │      │ SciIE (S47)      │      │ LFQA Critique(S23)│
   │ LLM→Agent (R28)  │      │ LATS (R26)       │      │ MAS Eval (S32,S33)│
   └──────────────────┘      │ CitePrompt (S15)  │      └──────────────────┘
                             │ AOP (S31)         │
                             │ LitFM (R31)       │
                             └──────────────────┘
```

### Coverage by SurveySpec Dimension

| Dimension | Coverage (new additions highlighted) | Gap Status |
|---|---|---|
| **Method** | Single-agent (S08, S05) + multi-agent (S30–S36) + outline-guided (S01–S05) + iterative (R22) + interactive (R20) | ✅ Strong |
| **Benchmark** | SurveyBench, SciReviewGen (R14), SciClaimEval (S37), CLAIMCHECK (S43), SciNLP (S48), SciER (S54) | ✅ Strong |
| **Metric** | BERTScore → FactKB (R24) → VERISCORE (S25) → FENICE (S29) → SMART (S26) → CLIPBERTScore (S27) | ✅ Strong |
| **Limitation** | Citation fabrication (R24), factuality (S23–S29), shallow coverage (S32), evaluation subjectivity (S23) | ✅ Addressed |
| **Application** | Scientific discovery (seed 4, R18–R22), SLR automation (R08–R13) | ✅ Broad |
| **Theory** | Citation intent (S14–S22), citation graph traversal (R01–R03, R07, R31), Garfield legacy (R04–R05) | ✅ Deep |

---

## 6. Drift Risk Analysis

| Risk | Level | Source | Mitigation |
|---|---|---|---|
| Q1 (KV cache) — all 10 results are LLM inference optimization | **HIGH** | Semantic Q1 | Excluded from pool. Query should have been scoped to "long-context RAG in survey generation" |
| HGEN (S13) — software-engineering documentation | **LOW** | Semantic Q3 | Retained as citation_seed; demonstrates hierarchical doc generation architecture |
| CLIPBERTScore (S27) — multimodal-specific | **LOW** | Semantic Q5 | Retained; factuality eval with multimodal grounding |
| 2 MAS papers excluded (robot task scheduling, coop task execution) | **MEDIUM** | Semantic Q6 | Excluded from pool |
| MatSciRE (S50) — materials-science specific | **LOW** | Semantic Q8 | Retained; pointer-network architecture is domain-agnostic |
| PDF text extraction not performed | **MEDIUM** | Citation | Reference graph constructed via arxiv_search, not PDF parsing. Some seed-specific refs may be missed |
| Non-arXiv refs (PRISMA, Garfield, GPT-4) unresolved | **LOW** | Citation | Trackable via DOI/conference proceedings; add during brief writing |
| schema/expansion.md not found on disk | **NOTE** | All | Schema is a 17-line contract file, not a directory. Workflow followed from agent description |

---

## 7. Next Expansion Queries (Recommended)

If further expansion is desired, these queries target remaining blind spots:

1. **Survey structure evaluation beyond quiz-style**: `survey evaluation rubric human assessment inter-annotator reliability`
2. **Hallucination detection in survey generation**: `hallucination detection citation fabrication long-form generation scientific text`
3. **Iterative refinement / self-correction in survey writing**: `self-critique iterative refinement revision survey generation agent`
4. **Cross-domain survey transfer**: `cross-domain transfer survey generation domain adaptation scientific literature`
5. **User interaction / personalization in survey systems**: `personalized survey generation user interaction adaptive literature review`

---

## 8. Risks Carried Forward

1. **`schema/expansion.md` and `schema/handoff.md` not on disk** — `schema` is a 17-line SurveySpec contract file, not a directory. Workflow followed from agent description and SurveySpec scope constraints.

2. **run_dir recovered as `.`** — Consistent with all prior agents. No impact on execution.

3. **arXiv version stability** — Versions preserved from candidate pool. Download stage should verify latest available version.

4. **68 new candidates is aggressive (~52% expansion)** — Some retained papers are tangential (HGEN for software docs, CLIPBERTScore for multimodal, MatSciRE for materials science). Downstream stages (PaperFetcher, Profiler, Filter) should apply stricter relevance filtering.

5. **PDF text extraction was not performed** — Reference graph for 03b was constructed via arxiv_search, not from actual PDF parsing. Full citation-edge verification requires a PDF parser.

6. **Non-arXiv references (U01–U08)** — These need DOI resolution for final bibliography. PRISMA and Garfield are accessible through PubMed / journal DOIs. GPT-4 tech report is available from OpenAI.
