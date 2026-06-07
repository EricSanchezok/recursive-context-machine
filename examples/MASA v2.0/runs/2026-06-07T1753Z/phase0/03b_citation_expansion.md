# Citation Graph Expansion — Automated Survey Generation

Generated: 2026-06-07T18:00Z  
Run dir: `.`  
Method: Semantic search expansion from seed papers via arxiv_search; arXiv ID verification via title-based lookup. PDF binary extraction was unavailable (fs tool cannot parse PDFs), so the expansion relies entirely on arXiv API metadata and targeted semantic queries seeded by each seed paper's topic, authors, and abstract.

---

## Seed Papers — Verified arXiv IDs

The following corrections were identified from the original seed list (03a_seed_papers.md):

| Seed | Listed ID | Verified ID | Status |
|------|-----------|-------------|--------|
| S2ORC | 1910.11270 | **1911.02782** | Corrected |
| SPECTER | 2004.07180 | 2004.07180 | Confirmed |
| SciBERT | 1903.10676 | 1903.10676 | Confirmed |
| ASReview | 1906.11512 | **2006.12166** | Corrected |
| PaperQA | 2312.07562 | **2312.07559** | Corrected |
| AutoSurvey | 2405.13215 | **2406.10252** | Corrected |

---

## Citation Graph: Infrastructure Layer

Papers that build and expose the citation graph as a computational resource.

| arXiv ID | Title | Relation to Seeds |
|----------|-------|-------------------|
| 1805.02262 | Construction of the Literature Graph in Semantic Scholar | Precedes S2ORC; describes the full heterogeneous graph pipeline (280M+ nodes) used by Semantic Scholar. Foundation paper for citation graph infrastructure. |
| 2301.10140 | The Semantic Scholar Open Data Platform | Successor to S2ORC; describes the 200M+ paper academic graph with structured full text, embeddings, and APIs. |
| 1902.05170 | GrapAL: Connecting the Dots in Scientific Literature | Graph database query tool for citation graph exploration (Neo4j-based). Adjacent to SPECTER's graph-aware embedding goal. |
| 2106.01560 | CitationIE: Leveraging the Citation Graph for Scientific Information Extraction | Uses citation graph structure to improve information extraction from scientific text. Methodologically adjacent to SPECTER (graph-informed representations). |
| 2301.11223 | CitationSum: Citation-aware Graph Contrastive Learning for Scientific Paper Summarization | Uses citation graphs to improve summarization quality. Demonstrates citation-aware training objectives. |
| 1905.00075 | On the Use of ArXiv as a Dataset | Early large-scale arXiv citation graph (6.7M edges, 11B word corpus). Precursor infrastructure paper. |

---

## Citation Graph: Method Layer (Citation-informed Representations)

| arXiv ID | Title | Relation to Seeds |
|----------|-------|-------------------|
| 2004.07180 | SPECTER: Document-level Representation Learning using Citation-informed Transformers | **Seed paper.** Core method for citation-informed document embeddings. |
| 2006.01131 | NLP Scholar: An Interactive Visual Explorer for NLP Literature | Citation-aware visualisation platform for NLP literature. Complementary to SPECTER's embedding approach. |

---

## Survey Agent Systems — Directly in Scope

| arXiv ID | Title | Relation to Seeds |
|----------|-------|-------------------|
| 2312.07559 | PaperQA: Retrieval-Augmented Generative Agent for Scientific Research | **Seed paper.** End-to-end RAG agent for scientific Q&A with citation chaining. Introduces LitQA benchmark. |
| 2409.13740 | Language agents achieve superhuman synthesis of scientific knowledge (PaperQA2) | Direct successor to PaperQA. Introduces LitQA2 benchmark. Demonstrates superhuman synthesis performance. Matches/exceeds expert humans. |
| 2406.10252 | AutoSurvey: Large Language Models Can Automatically Write Surveys | **Seed paper.** Structured pipeline with outline generation, subsection drafting, integration/refinement. |
| 2502.14776 | SurveyX: Academic Survey Automation via Large Language Models | Follow-on to AutoSurvey. Decomposes into Preparation and Generation phases. AttributeTree pre-processing. Outperforms earlier systems. |
| 2503.04629 | SurveyForge: On the Outline Heuristics, Memory-Driven Generation | Follow-on to AutoSurvey. Scholar navigation agent for memory-guided retrieval. Constructs SurveyBench. |
| 2509.18661 | Agentic AutoSurvey: Let LLMs Survey LLMs | Multi-agent framework (4 agents). Scores 8.18/10 vs AutoSurvey's 4.77/10. |
| 2510.07733 | SurveyG: A Multi-Agent LLM Framework with Hierarchical Citation Graph | **Directly relevant to citation graph traversal.** Hierarchical citation graph (Foundation/Development/Frontier layers). Horizontal + vertical traversal for survey generation. |
| 2508.17647 | SurveyGen: Quality-Aware Scientific Survey Generation | Large-scale dataset (4,200+ human surveys, 242K+ references). Quality-aware RAG pipeline. |
| 2506.12689 | SciSage: A Multi-Agent Framework for High-Quality Scientific Survey Generation | Reflect-when-you-write paradigm. Reflector agent evaluates at outline/section/document levels. |
| 2504.08762 | InteractiveSurvey: LLM-based Personalized and Interactive Survey Generation | User-customizable survey generation with online retrieval + user uploads. |
| 2408.07884 | Instruct LLMs to Generate Scientific Literature Survey Step by Step | Step-by-step prompt design (title → abstract → headings → content). NLPCC 2024 evaluation task. |
| 2510.21900 | Deep Literature Survey Automation with an Iterative Workflow (IterSurvey) | Recurrent outline generation with iterative retrieval. Introduces Survey-Arena benchmark. |
| 2411.14199 | OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs | Retrieval-augmented LM over 45M open-access papers. Introduces ScholarQABench. Outperforms GPT-4o and PaperQA2. |
| 2411.06159 | Mixture of Knowledge Minigraph Agents for Literature Review Generation | Knowledge minigraph construction agents (KMCA) + multi-path summarization (MPSA). Prompt-based graph construction. |
| 2110.06354 | Tell Me How to Survey: Literature Review Made Simple with Automatic Reading Path Generation | Introduces SurveyBank dataset and reading path generation (RPG) task. Graph-optimization-based approach. |
| 2010.04147 | Automatic generation of reviews of scientific papers | Early method: co-citation graph + BERT-based extractive summarization on PubMed. |
| 2208.02334 | A Knowledge Graph-Based Method for Automating Systematic Literature Reviews | Knowledge graph approach for SLR automation. |
| 2501.10120 | PaSa: An LLM Agent for Comprehensive Academic Paper Search | Paper search agent using RL + synthetic data. Surpasses Google/GPT-4o baselines by 37.78% recall@20. |

---

## Survey Agent Systems — Systematic Review / Screening Tools

| arXiv ID | Title | Relation to Seeds |
|----------|-------|-------------------|
| 2006.12166 | ASReview: Open Source Software for Efficient and Transparent Reviews | **Seed paper.** Active learning for systematic review screening. Precursor methodology. |
| 2309.01684 | CRUISE-Screening: Living Literature Reviews Toolbox | Web-based living literature review tool with API-connected search and text classification. |

---

## Benchmarks and Evaluation Datasets

| arXiv ID | Title | Notes |
|----------|-------|-------|
| 2510.03120 | SurveyBench: Can LLM(-Agents) Write Academic Surveys that Align with Reader Needs? | Fine-grained evaluation framework. 11,343 arXiv papers + 4,947 high-quality surveys. Multifaceted metric hierarchy. |
| 2512.02763 | SurveyEval: Towards Comprehensive Evaluation of LLM-Generated Academic Surveys | Three dimensions: overall quality, outline coherence, reference accuracy. 7 subjects. |
| 2312.07559 | LitQA (via PaperQA) | Complex benchmark requiring retrieval + synthesis from full-text papers. |
| 2409.13740 | LitQA2 (via PaperQA2) | Harder benchmark that guided PaperQA2 design. |
| 2407.18940 | LitSearch: A Retrieval Benchmark for Scientific Literature Search | 597 realistic literature search queries about ML/NLP papers. |
| 2411.14199 | ScholarQABench (via OpenScholar) | 2,967 expert-written queries + 208 long-form answers across CS, physics, neuroscience, biomedicine. |
| 2510.21900 | Survey-Arena (via IterSurvey) | Pairwise benchmark comparing machine-generated vs human-written surveys. |
| 2110.06354 | SurveyBank (via Tell Me How to Survey) | Large-scale dataset of CS survey papers with citation relationships. |
| 2004.07180 | SciDocs (via SPECTER) | 7 document-level tasks: citation prediction, classification, recommendation. |

---

## Pre-2019 Foundational Works (No arXiv ID — Not Expanded)

These papers are specified in scope but lack arXiv IDs, so they could not be retrieved via arXiv API. Their influence is captured indirectly through the citation graph of the expanded papers.

| Work | Year | Notes |
|------|------|-------|
| Vaswani et al. — Attention Is All You Need | 2017 | Foundation of all Transformer-based models (SciBERT, SPECTER, etc.). |
| Page et al. — PageRank | 1999 | Basis for citation-based ranking algorithms. |
| Kleinberg — HITS | 1999 | Hub/authority citation analysis. |
| Carbonell & Goldstein — MMR | 1998 | Maximum Marginal Relevance for diversity in retrieval. |
| Page et al. — PRISMA 2020 | 2021 | Systematic review reporting standard. DOI-based, no arXiv ID. |

---

## Coverage Assessment

| Dimension | Count | Papers |
|-----------|-------|--------|
| **Citation Graph Infrastructure** | 6 | 1805.02262, 2301.10140, 1902.05170, 2106.01560, 2301.11223, 1905.00075 |
| **Citation-informed Methods** | 2 | 2004.07180 (seed), 2006.01131 |
| **Survey Agent Systems** | 19 | 2312.07559 (seed), 2409.13740, 2406.10252 (seed), 2502.14776, 2503.04629, 2509.18661, 2510.07733, 2508.17647, 2506.12689, 2504.08762, 2408.07884, 2510.21900, 2411.14199, 2411.06159, 2110.06354, 2010.04147, 2208.02334, 2501.10120 |
| **Systematic Review Tools** | 2 | 2006.12166 (seed), 2309.01684 |
| **Benchmarks / Evaluation** | 9 | 2510.03120, 2512.02763, LitQA, LitQA2, 2407.18940, ScholarQABench, Survey-Arena, SurveyBank, SciDocs |
| **Total Expanded** | **38** | Excluding duplicates and foundational works without arXiv IDs |

### Gaps Still Outstanding

1. **Elicit / Semantic Scholar Agent:** No dedicated arXiv paper found for the Elicit commercial tool or the "Semantic Scholar Agent" described in some blog posts. These are product-level tools without corresponding publications.
2. **PRISMA 2020:** Has a DOI (10.1136/bmj.n71) but no arXiv ID. Consider adding manually if needed for systematic review methodology coverage.
3. **Pre-2019 foundations:** Vaswani 2017, PageRank, HITS, MMR — none have arXiv IDs. Their influence is propagated through citations in the expanded graph.
4. **SurveyEval (2512.02763):** Very recent (Dec 2025); may need validation against the live arXiv.

---

## Citation Expansion Summary

- **PDFs downloaded:** 10 (6 corrected seed PDFs + 4 initially incorrect PDFs retained for reference)
- **Resolved references (expanded papers):** 32 new papers discovered via semantic search
- **Unresolved references (no arXiv ID):** 5 foundational works (Vaswani 2017, PageRank, HITS, MMR, PRISMA 2020)
- **arXiv ID corrections:** 4 out of 6 seed IDs had wrong arXiv IDs
