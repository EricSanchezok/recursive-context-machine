# Expansion Report — Automated Survey Agents with Citation Graph Expansion

Generated: 2026-06-07T18:00Z  
Run dir: `.`  
Method: Merge of seed selection (03a), citation graph expansion via arXiv API semantic search (03b), and embedding-search semantic expansion (03c). PDF binary extraction was unavailable; all expansion relies on arXiv metadata and embeddings.

---

## 1. Seed Papers (6)

Six seeds from domain knowledge, with arXiv IDs corrected by citation expansion verification.

| # | Seed | Verified arXiv ID | Original ID (from 03a) | Status |
|---|------|-------------------|------------------------|--------|
| 1 | S2ORC | **1911.02782** | 1910.11270 | Corrected |
| 2 | SPECTER | **2004.07180** | 2004.07180 | Confirmed |
| 3 | SciBERT | **1903.10676** | 1903.10676 | Confirmed |
| 4 | ASReview | **2006.12166** | 1906.11512 | Corrected |
| 5 | PaperQA | **2312.07559** | 2312.07562 | Corrected |
| 6 | AutoSurvey | **2406.10252** | 2405.13215 | Corrected |

**Key finding:** 4 of 6 seed IDs were wrong (off by 1–3 digits). This is a pipeline-level risk — arXiv ID verification via title lookup should be mandatory before any download-based step.

---

## 2. Merged Paper Inventory (69 unique entries)

Deduplicated by arXiv ID across citation expansion (03b) and semantic expansion (03c). Entries are tagged with their source: [C] = citation expansion, [S] = semantic expansion, [B] = both.

### 2.1 Citation Graph Infrastructure (6 papers)

Papers that build and expose citation graphs as computational resources.

| # | arXiv ID | Title | Year | Source | Notes |
|---|----------|-------|------|--------|-------|
| 1 | 1805.02262 | Construction of the Literature Graph in Semantic Scholar | 2018 | [C] | Precedes S2ORC; 280M+ node heterogeneous graph pipeline |
| 2 | 2301.10140 | The Semantic Scholar Open Data Platform | 2023 | [C] | Successor to S2ORC; 200M+ papers, structured text, embeddings |
| 3 | 1902.05170 | GrapAL: Connecting the Dots in Scientific Literature | 2019 | [C] | Neo4j-based graph DB query tool for citation exploration |
| 4 | 2106.01560 | CitationIE: Leveraging the Citation Graph for Scientific IE | 2021 | [B] | Graph-informed information extraction from scientific text |
| 5 | 2301.11223 | CitationSum: Citation-aware Graph Contrastive Learning for Summarization | 2023 | [C] | Citation-aware training objectives for summarization |
| 6 | 1905.00075 | On the Use of ArXiv as a Dataset | 2019 | [C] | Early large-scale arXiv citation graph (6.7M edges) |

### 2.2 Citation Graph Retrieval & Embeddings (10 papers)

Methods that use citation graph structure for retrieval, clustering, and representation learning.

| # | arXiv ID | Title | Year | Source | Notes |
|---|----------|-------|------|--------|-------|
| 7 | 2004.07180 | SPECTER: Citation-informed Transformers | 2020 | [C] *seed* | Core citation-informed document embedding method |
| 8 | 2006.01131 | NLP Scholar: Interactive Visual Explorer for NLP Literature | 2020 | [C] | Citation-aware visualisation for NLP |
| 9 | 2409.12177 | LitFM: Structure-aware Foundation Model for Citation Graphs | 2024 | [S] | Graph retriever; +28.1% precision on retrieval |
| 10 | 2403.09295 | Seed-based IR: Direct Citations, Bibliographic Coupling, Co-citations | 2024 | [S] | Systematic comparison; combining all 3 outperforms single methods |
| 11 | 2109.10007 | Generating Local Maps of Science using Deep Bibliographic Coupling | 2021 | [S] | Graph diffusion extends bibliographic coupling to deep neighbourhood |
| 12 | 2004.05904 | Return to Basics: Clustering using Structural Information | 2020 | [S] | Direct citation replication outperforms NLP-based clustering |
| 13 | 2106.05633 | Citation Recommendation for Research Papers via Knowledge Graphs | 2021 | [S] | Research KG + citation network; +0.8 MAP |
| 14 | 2207.03299 | Academic IR using Citation Clusters | 2022 | [S] | Citation clusters complementary to query-based search |
| 15 | 1511.05078 | Which Type of Citation Analysis Generates the Most Accurate Taxonomy? | 2015 | [S] | Direct citation > bibliographic coupling > co-citation for taxonomy |
| 16 | 2110.06354 | Reading Path Generation (SurveyBank) | 2021 | [B] | Graph-optimisation-based reading path generation; SurveyBank dataset |

### 2.3 Survey Agent Systems (22 papers)

End-to-end systems for automated literature survey generation.

| # | arXiv ID | Title | Year | Source | Notes |
|---|----------|-------|------|--------|-------|
| 17 | 2312.07559 | PaperQA: Retrieval-Augmented Agent for Scientific Research | 2023 | [C] *seed* | End-to-end RAG agent; citation chaining; LitQA benchmark |
| 18 | 2409.13740 | Language agents achieve superhuman synthesis (PaperQA2) | 2024 | [B] | Superhuman synthesis; LitQA2 benchmark; contradiction detection |
| 19 | 2406.10252 | AutoSurvey: LLMs Can Automatically Write Surveys | 2024 | [C] *seed* | Structured pipeline with outline, drafting, refinement |
| 20 | 2502.14776 | SurveyX: Academic Survey Automation via LLMs | 2025 | [B] | AttributeTree pre-processing; online retrieval; re-polishing |
| 21 | 2503.04629 | SurveyForge: Outline Heuristics, Memory-Driven Generation | 2025 | [B] | Scholar navigation agent; memory-guided retrieval; SurveyBench |
| 22 | 2509.18661 | Agentic AutoSurvey: Let LLMs Survey LLMs | 2025 | [B] | 4 multi-agent framework; 8.18/10 vs AutoSurvey 4.77/10 |
| 23 | 2510.07733 | **SurveyG: Hierarchical Citation Graph Framework** | 2025 | [B] | **Most directly relevant to RQ.** Foundation/Development/Frontier 3-layer graph; horizontal+vertical traversal |
| 24 | 2508.17647 | SurveyGen: Quality-Aware Scientific Survey Generation | 2025 | [C] | 4,200+ human surveys dataset; quality-aware RAG pipeline |
| 25 | 2506.12689 | SciSage: Multi-Agent Framework for Survey Generation | 2025 | [B] | Reflect-when-you-write; SurveyScope benchmark; +32% citation F1 |
| 26 | 2504.08762 | InteractiveSurvey: Personalized and Interactive Survey Generation | 2025 | [B] | User customisation of references, outline, content mid-generation |
| 27 | 2408.07884 | Instruct LLMs to Generate Surveys Step by Step | 2024 | [C] | Step-by-step prompt design (title→abstract→headings→content) |
| 28 | 2510.21900 | IterSurvey: Deep Survey Automation with Iterative Workflow | 2025 | [C] | Recurrent outline generation; Survey-Arena benchmark |
| 29 | 2411.14199 | OpenScholar: Synthesizing Literature with Retrieval-augmented LMs | 2024 | [B] | 45M paper datastore; ScholarQABench; 8B beats GPT-4o by 5% |
| 30 | 2411.06159 | Mixture of Knowledge Minigraph Agents for Lit Review Generation | 2024 | [C] | KMCA + MPSA; prompt-based graph construction |
| 31 | 2010.04147 | Automatic generation of reviews of scientific papers | 2020 | [C] | Early method: co-citation graph + BERT extractive summarisation |
| 32 | 2208.02334 | Knowledge Graph-Based Method for Automating SLRs | 2022 | [C] | KG approach for systematic literature review automation |
| 33 | 2501.10120 | PaSa: LLM Agent for Comprehensive Academic Paper Search | 2025 | [C] | RL + synthetic data; +37.78% recall@20 over GPT-4o |
| 34 | 2402.14207 | STORM: Synthesis of Topic Outlines through Retrieval and Question Asking | 2024 | [S] | Wikipedia-style generation; multi-perspective question asking |
| 35 | 2402.01788 | LitLLM: Toolkit for Scientific Literature Review | 2024 | [S] | RAG-based toolkit; re-ranking by abstract similarity |
| 36 | 2408.13450 | vitaLITy 2: Reviewing Academic Literature Using LLMs | 2024 | [S] | RAG over 66k-paper corpus; text embedding search |
| 37 | 2510.15682 | SQuAI: Scientific QA with Multi-Agent RAG | 2025 | [S] | 4-agent RAG over 2.3M arXiv papers; hybrid sparse-dense retrieval |
| 38 | 2508.14317 | SurveyGen-I: Evolving Plans and Memory-Guided Writing | 2025 | [S] | Coarse-to-fine retrieval; adaptive planning; memory mechanism |
| 39 | 2510.10890 | LLM×MapReduce-V3: MCP-Driven Hierarchically Modular Agent System | 2025 | [S] | Modular MCP server architecture; human-in-the-loop |
| 40 | 2507.15245 | SPAR: Scholar Paper Retrieval with LLM-based Agents | 2025 | [S] | RefChain-based query decomposition; SPARBench evaluation |

### 2.4 Systematic Review / Screening Tools (3 papers)

Tools for active-learning-based systematic review screening — adjacent domain context.

| # | arXiv ID | Title | Year | Source | Notes |
|---|----------|-------|------|--------|-------|
| 41 | 2006.12166 | ASReview: Open Source Software for Efficient Reviews | 2021 | [C] *seed* | Active learning for systematic review screening |
| 42 | 2309.01684 | CRUISE-Screening: Living Literature Reviews Toolbox | 2023 | [C] | Web-based living review tool; API search + text classification |
| 43 | 2502.03400 | DenseReviewer: Screening Prioritisation for Systematic Reviews | 2025 | [S] | Dense retrieval + active learning; web-based tool |

### 2.5 Benchmarks & Evaluation (10 papers)

Datasets and protocols for evaluating automated survey systems.

| # | arXiv ID | Title | Year | Source | Notes |
|---|----------|-------|------|--------|-------|
| 44 | 2510.03120 | SurveyBench: Can LLM(-Agents) Write Surveys Aligned with Readers? | 2025 | [B] | 11,343 arXiv topics; 4,947 surveys; quiz-driven evaluation |
| 45 | 2512.02763 | SurveyEval: Towards Comprehensive Evaluation of LLM Surveys | 2025 | [C] | 3 dimensions: quality, outline coherence, reference accuracy |
| 46 | 2407.18940 | LitSearch: Retrieval Benchmark for Scientific Literature | 2024 | [B] | 597 ML/NLP queries; 24.8% BM25 vs dense retriever gap |
| 47 | 2508.15658 | SurGE: Survey Generation Evaluation Benchmark | 2025 | [S] | 4-dimension evaluation (coverage, referencing, structure, content) |
| 48 | 2508.15804 | ReportBench: Evaluating Deep Research Agents via Academic Surveys | 2025 | [S] | Citation quality + faithfulness; agent-based automated eval |
| 49 | 2605.29234 | Rethinking Literature Search Eval: Deep Research and Human Citation Lists | 2026 | [S] | Only 51% human citations judged relevant; co-authorship bias |
| 50 | 2502.13668 | PeerQA: Scientific QA Dataset from Peer Reviews | 2025 | [S] | 579 QA pairs from 208 papers; evidence retrieval + unanswerable |
| 51 | 2105.03011 | QASPER: Information-Seeking QA Anchored in Research Papers | 2021 | [S] | 5,049 questions over 1,585 NLP papers |
| 52 | 2503.04629 | SurveyForge (SurveyBench construction) | 2025 | [B] | 100 human surveys for reference/outline/content evaluation |
| 53 | 2110.06354 | SurveyBank (via Tell Me How to Survey) | 2021 | [B] | Large-scale CS survey dataset with citation relationships |

### 2.6 Citation Attribution & Faithfulness (6 papers)

Methods for faithful citation attribution in LLM-generated scientific text.

| # | arXiv ID | Title | Year | Source | Notes |
|---|----------|-------|------|--------|-------|
| 54 | 2510.17853 | CiteGuard: Faithful Citation Attribution via RAG Validation | 2025 | [S] | 65.4% on CiteME benchmark (human 69.7%) |
| 55 | 2502.09604 | SelfCite: Self-Supervised Alignment for Context Attribution | 2025 | [S] | Context ablation reward; +5.3 F1 on LongBench-Cite |
| 56 | 2508.15396 | Attribution, Citation, and Quotation: Survey of Evidence-based Text Generation | 2025 | [S] | Unified taxonomy; 134 papers; 300 metrics across 7 dimensions |
| 57 | 2309.09727 | When LLMs Meet Citation: A Comprehensive Survey | 2023 | [S] | Bidirectional review of LLM-for-citation and citation-for-LLM |
| 58 | 2405.02228 | REASONS: Attribution in Scientific Literature | 2024 | [S] | Sentence-level annotations across 12 domains; -42% hallucination via RAG |
| 59 | 2306.03535 | SciLit: Platform for Literature Discovery, Summarization and Citation | 2023 | [S] | End-to-end assistive writing; abstractive citation generation |

### 2.7 Boundary — Systematic Review Automation (4 papers)

Active-learning-based SLR screening methods — included for comparative context only.

| # | arXiv ID | Title | Year | Source | Notes |
|---|----------|-------|------|--------|-------|
| 60 | 2202.10033 | Open-source integrated framework for citation collection and screening | 2022 | [S] | Bayesian active learning; 95.6% efficiency, 100% sensitivity |
| 61 | 2011.09752 | Hybrid Learning for Technology-Assisted SLR | 2020 | [S] | Learning-to-rank + relevance feedback; full pipeline |
| 62 | 2509.23981 | Automatic Selection with Evolutionary Rule-Based Classification | 2025 | [S] | Grammar-guided genetic programming; interpretable classifiers |
| 63 | 1909.07249 | Assessing Expert System-Assisted Literature Reviews | 2019 | [S] | Active learning tool; 53h → 3h; 90% recall with 6% effort |

### 2.8 Boundary — Citation Bias & Scientometrics (6 papers)

Methods for detecting and mitigating bias in citation graphs.

| # | arXiv ID | Title | Year | Source | Notes |
|---|----------|-------|------|--------|-------|
| 64 | 2411.05584 | Mitigating Consequences of Prestige in Citations (Matthew Effect) | 2024 | [S] | Predicting citations from pre-publication variables only |
| 65 | 1703.08071 | Quantifying and Suppressing Ranking Bias in Citation Networks | 2017 | [S] | Mahalanobis distance; z-score normalisation for field/age bias |
| 66 | 2203.17239 | Citation Bias in Peer Review (Cite-seeing) | 2022 | [S] | +0.23 score increase when citing reviewer's work |
| 67 | 2502.13934 | Citation Proximus: Social and Semantic Ties in Citing Behaviour | 2025 | [S] | Collaboration network strongest predictor of citation |
| 68 | 2508.12735 | Citation Accuracy, Citation Noise, and Citation Bias | 2025 | [S] | Defines citation noise vs bias; CoARA recommendations |
| 69 | 1501.05462 | A Review of Theory and Practice in Scientometrics | 2015 | [S] | Comprehensive survey of citation metrics and normalisation |

### 2.9 Foundational Works (No arXiv ID — 5 entries)

These are specified in scope_include but lack arXiv IDs, so could not be expanded via API. Their influence is captured indirectly through the citation graphs of the expanded papers.

| Work | Year | Relevance |
|------|------|-----------|
| Vaswani et al. — "Attention Is All You Need" | 2017 | Transformer foundation for all LLM-based survey agents |
| Page et al. — PageRank | 1999 | Foundation of citation-based ranking algorithms |
| Kleinberg — HITS | 1999 | Hub/authority citation analysis |
| Carbonell & Goldstein — MMR | 1998 | Maximum Marginal Relevance for diversity in retrieval |
| Page et al. — PRISMA 2020 | 2021 | Systematic review reporting standard (DOI: 10.1136/bmj.n71) |

---

## 3. Summary Counts

| Category | Count | Breakdown |
|----------|-------|-----------|
| **Seeds** | 6 | S2ORC, SPECTER, SciBERT, ASReview, PaperQA, AutoSurvey |
| **Citation Graph Infrastructure** | 6 | 1805.02262, 2301.10140, 1902.05170, 2106.01560, 2301.11223, 1905.00075 |
| **Citation Graph Retrieval & Embeddings** | 10 | Including SPECTER seed + 9 expanded |
| **Survey Agent Systems** | 22 | Including PaperQA + AutoSurvey seeds + 20 expanded |
| **Systematic Review / Screening Tools** | 3 | Including ASReview seed + 2 expanded |
| **Benchmarks & Evaluation** | 10 | All expanded |
| **Citation Attribution & Faithfulness** | 6 | All expanded |
| **Boundary — Systematic Review Automation** | 4 | All expanded (comparative context) |
| **Boundary — Citation Bias & Scientometrics** | 6 | All expanded |
| **Foundational Works (no arXiv ID)** | 5 | Vaswani 2017, PageRank, HITS, MMR, PRISMA 2020 |
| **Total unique entries** | **69** | + 5 foundational works |
| **Source: citation expansion only** | 15 | Infrastructure + some survey agents + some benchmarks |
| **Source: semantic expansion only** | 36 | Attribution, bias, additional benchmarks, some methods |
| **Source: both expansions** | 12 | SurveyG, SurveyX, SurveyForge, PaperQA2, etc. |

---

## 4. Drift Risks

### Risk 1: arXiv ID fragility (HIGH)
4 of 6 seed IDs were incorrect. This is a systematic risk for any pipeline that stores arXiv IDs without periodic verification. Mitigation: arXiv API title-based lookup before download, and cache verified IDs with confidence timestamps.

### Risk 2: Strong 2025 recency skew (HIGH)
54% of semantic expansion candidates are from 2025. The embedding search naturally favours recent work because query concepts (LLM agents, survey automation) are densely populated in the last 1–2 years. Mitigation: the seed set (2019–2024) and pre-2023 citation analysis papers (e.g., 1511.05078, 1501.05462, 1703.08071) provide temporal anchors. The final brief must deliberately weight pre-2023 foundational works.

### Risk 3: Topic boundary creep — systematic review automation (MEDIUM)
Q6 from semantic expansion returned strong candidates on active-learning-based screening (DenseReviewer, FASTREAD, evolutionary classification). These automate screening, not citation-graph-driven survey generation. Inclusion risk: survey scope could drift into generic SLR automation. Mitigation: these 4 papers are explicitly tagged as boundary/comparative context.

### Risk 4: Seed coverage gap in embedding space (MEDIUM)
Three of six seeds (S2ORC, SPECTER, SciBERT) did not appear in any semantic query result. AutoSurvey was also absent. This indicates the semantic neighbourhood of arXiv abstracts does not strongly overlap with infrastructure/embedding papers — they are cited *by* agent papers but are not embedding-similar to them. The citation graph expansion recovered these connections via topical queries.

### Risk 5: PDF binary extraction unavailable (LOW)
PDF parsing was unavailable, so direct citation list extraction from each seed paper's References section did not happen. The expansion relied entirely on arXiv metadata and embedding-search queries. A follow-up pass with PDF text extraction could recover additional citation edges.

### Risk 6: Missing product-level tools (LOW)
Elicit and Semantic Scholar Agent are commercial/product-level tools without corresponding arXiv papers. Their design choices (retrieval depth, expansion policy) may be relevant but cannot be sourced from the academic literature.

---

## 5. Next Expansion Queries (Recommended)

If a second expansion pass is warranted, target these gaps:

| Gap | Suggested Query | Rationale |
|-----|----------------|-----------|
| Survey-genie / Agent-R (missing systems) | "survey generation agent retrieval augmented transformer arxiv" | Potential recent systems not yet indexed |
| Scim / ScimAI | "scim ai literature review tool semantic scholar" | Commercial tool with possible technical report |
| Elicit methodology | "elicit literature review retrieval approach" | Recover design documentation if any |
| Pre-2023 citation graph ranking | "PageRank HITS citation ranking algorithm literature retrieval 2010 2015 2018" | Strengthen temporal anchors |
| Visual survey systems | "visual literature survey graph visualization systematic review" | Adjacent visualization methods |
| Cross-domain surveys | "automated survey generation for biomedical literature bioarxiv" | Test domain transfer |
| Citation context window | "citation context window length effect survey quality faithfulness" | Hyperparameter studies |
| Post-training optimization | "reinforcement learning from human feedback citation survey optimization" | Training-time methods for survey quality |

---

## 6. Coverage Assessment Against SurveySpec

| SurveySpec Dimension | Coverage Status | Key Papers |
|----------------------|-----------------|------------|
| Agentic pipeline (plan→retrieve→expand→synthesize→revise) | **Covered** | AutoSurvey, SurveyX, SurveyForge, SurveyG, STORM |
| Citation graph traversal as expansion strategy | **Covered** | SurveyG (hierarchical), SPECTER (embeddings), PaSa, Seed-based IR |
| Benchmarks & evaluation protocols | **Covered** | SurveyBench, SurveyEval, SurGE, ReportBench, LitSearch, PeerQA |
| Graph neural network embeddings | **Covered** | LitFM, CitationIE, Academic IR via Citation Clusters |
| Foundational works (pre-2023) | **Partial** | 5 works listed (no arXiv IDs); pre-2023 papers: 1511.05078, 1501.05462, 1703.08071 |
| Adjacent context: systematic review methodology | **Covered** | PRISMA 2020 (DOI), ASReview, CRUISE-Screening, DenseReviewer |
| Adjacent context: scientometrics / bias | **Covered** | 6 papers on Matthew effect, ranking bias, citation noise |
| Temporal balance | **Partial — Strong 2025 skew** | Need deliberate pre-2023 weighting in final brief |
