# FrontierScout Candidate Pool

**run_dir**: `.`
**artifact**: `02d_frontier_candidates.md`
**scout_query_source**: `01_query_plan.md` — queries fr-01..fr-03 (frontier), cd-01..cd-03 (cross-domain)
**searched_at**: 2026-06-06T14:10+08:00

---

## Queries Executed

| id | query | topK |
|----|-------|------|
| fr-01 | "recent advances automated survey agents 2024 2025" | 10 |
| fr-02 | "graph-based RAG scientific knowledge discovery citation graph traversal" | 10 |
| fr-03 | "self-improving citation traversal survey agent iterative refinement" | 10 |
| cd-01 | "traversal strategies citation graphs knowledge graphs comparison literature survey" | 10 |
| cd-02 | "exploration exploitation tradeoff paper discovery recommender systems citation traversal" | 10 |
| cd-03 | "multi-hop reasoning citation networks question answering scientific literature" | 10 |

**Total raw hits**: 60 (10 per query, including duplicates)
**Unique papers**: ~45 (after cross-query dedup)

---

## Candidate Papers (in-scope, deduplicated)

### 1. Agentic AutoSurvey — 2509.18661v1
**Tags**: multi-agent, 4 specialized agents, 12-dim eval, 8.18/10 vs 4.77/10
**Queries matched**: fr-01, fr-03
**Published**: 2025-09-23
**Why in scope**: Directly addresses automated survey generation via multi-agent orchestration with explicit paper retrieval, topic clustering, writing, and quality evaluation stages. Covers 75–443 papers per topic with citation coverage ≥80%.

### 2. AutoSurvey2 — 2510.26012
**Tags**: multi-stage pipeline, iterative refinement, real-time retrieval, multi-LLM eval
**Queries matched**: fr-01, fr-03
**Published**: 2025-10-29
**Why in scope**: Follow-up to AutoSurvey with parallel section generation, iterative refinement, and real-time retrieval of recent publications. Outperforms retrieval-based baselines.

### 3. SciSage — 2506.12689v2
**Tags**: multi-agent, reflect-when-you-write, hierarchical Reflector agent, SurveyScope benchmark
**Queries matched**: fr-01, fr-03
**Published**: 2025-06-15
**Why in scope**: Multi-agent survey generation framework with hierarchical Reflector agent for evaluating drafts at outline, section, and document levels. +1.73 coherence, +32% citation F1. Releases SurveyScope benchmark (46 papers across 11 CS domains).

### 4. SurveyG — 2510.07733
**Tags**: hierarchical citation graph, 3-layer graph (Foundation/Development/Frontier), multi-agent validation
**Queries matched**: fr-01, fr-03
**Published**: 2025-10-09
**Why in scope**: Directly uses citation graph structure (hierarchical, 3 layers) to guide survey generation. Combines horizontal search within layers and vertical depth traversal across layers. Strong citation-graph-aware design.

### 5. SurveyX — 2502.14776v2
**Tags**: AttributeTree, online reference retrieval, two-phase (Preparation + Generation)
**Queries matched**: fr-01
**Published**: 2025-02-20
**Why in scope**: Decomposes survey writing into preparation and generation phases. Introduces AttributeTree preprocessing and re-polishing. Improves citation quality by 1.76 over baselines.

### 6. SurveyForge — 2503.04629v1
**Tags**: outline heuristics, memory-driven generation, scholar navigation agent, SurveyBench
**Queries matched**: fr-01, fr-03
**Published**: 2025-03-06
**Why in scope**: Learns outline structure from human-written surveys, uses scholar navigation agent for memory-driven paper retrieval. Constructs SurveyBench (100 human surveys for win-rate comparison).

### 7. SurveyBench — 2510.03120v1/v2
**Tags**: evaluation benchmark, quiz-driven, 11,343 arXiv papers, 4,947 surveys, dual-mode eval
**Queries matched**: fr-01
**Published**: 2025-10-03
**Why in scope**: Comprehensive evaluation benchmark for LLM-generated surveys. Metrics: coverage breadth, logical coherence, synthesis granularity, clarity of insights. Average 21% lower than human in content-based evaluation.

### 8. CG-RAG — 2501.15067v1
**Tags**: citation graph retrieval-augmented generation, lexical-semantic graph retrieval (LeSeGR)
**Queries matched**: fr-02
**Published**: 2025-01-25
**Why in scope**: Integrates sparse and dense retrieval signals within citation graph structures for research question answering. Directly relevant as a graph-based RAG method for scientific literature.

### 9. LitFM — 2409.12177v1
**Tags**: literature foundation model, graph retriever, citation graph navigation, 28.1% retrieval precision improvement
**Queries matched**: fr-02, cd-01
**Published**: 2024-09-05
**Why in scope**: First literature foundation model with a novel graph retriever that navigates citation graphs for retrieval. Integrates graph structure during both training and inference. Benchmarks on 3 academic fields.

### 10. Deep Literature Survey Automation with Iterative Workflow — 2510.21900
**Tags**: iterative/recurrent outline generation, paper cards, review-and-refine loop, Survey-Arena
**Queries matched**: fr-03
**Published**: 2025-10-24
**Why in scope**: Framework based on recurrent outline generation where a planning agent incrementally retrieves, reads, and updates the outline. Introduces Survey-Arena pairwise benchmark.

### 11. ResearchAgent — 2404.07738v2
**Tags**: iterative research idea generation, academic graph, LLM reviewing agents, human preference alignment
**Queries matched**: fr-03
**Published**: 2024-04-11
**Why in scope**: Iterative research idea generation over scientific literature with LLM-based reviewing agents. Uses academic graph for paper discovery and multi-agent feedback loops.

### 12. SurveyGen-I — 2508.14317v1
**Tags**: coarse-to-fine retrieval, adaptive planning, memory-guided generation, evolving plans
**Queries matched**: fr-03
**Published**: 2025-08-20
**Why in scope**: Combines coarse-to-fine retrieval with adaptive planning and memory mechanism for coherent multi-section survey generation. Dynamic refinement across subsections.

### 13. InteractiveSurvey — 2504.08762v1
**Tags**: interactive, personalized, user customization, online retrieval + user uploads
**Queries matched**: fr-03
**Published**: 2025-03-31
**Why in scope**: LLM-based personalized and interactive survey generation. Users customize reference categorization, outline, and content through an intuitive interface.

### 14. FAIR Literature Surveys with Scholarly KGs — 2006.01747v1 (ORKG)
**Tags**: Open Research Knowledge Graph, FAIR data principles, contribution comparison
**Queries matched**: cd-01
**Published**: 2020-06-02
**Why in scope**: Uses scholarly knowledge graph (ORKG) to generate FAIR literature surveys by comparing research contributions. Demonstrates methodology for contribution-based survey synthesis.

---

## Boundary Papers (out-of-scope but adjacent)

These were retrieved by frontier/cross-domain queries but fall outside scope.

| arXiv ID | Title | Reason for Boundary | Queries |
|----------|-------|---------------------|---------|
| 2503.23350v4 | Survey of WebAgents | Web automation agents, not literature survey agents | fr-01 |
| 2504.02891v1 | Automated Survey Collection with LLM-based Conversational Agents | Phone surveys for healthcare data, not academic literature surveys | fr-01 |
| 2408.02508v1 | PUREsuggest | Citation-based search and visualization without synthesis | fr-03 |
| 2104.03057v1 | Enhancing Summarization with Citation Graph | Paper summarization, not survey generation (adjacent technique) | fr-02 |
| 2512.22159 | Oignon | Citation graph visualization tool, no survey generation | fr-02, cd-01 |
| 1902.05170v2 | GrapAL | Graph DB infrastructure, not survey-agent framing | fr-02 |
| 2106.05633v1 | Citation Recommendation via KGs | Recommendation without synthesis | fr-02, cd-01 |
| 2106.01560v1 | CitationIE | Information extraction, not survey generation | fr-02, cd-01 |
| 2002.06961v2 | Citation Recommendation: Approaches and Datasets | Pure recommendation survey, no synthesis | cd-01 |
| 2003.02320v6 | Knowledge Graphs (survey) | General KG survey, no literature survey agent focus | cd-01 |
| 1805.02262v1 | Semantic Scholar Literature Graph | Infrastructure/platform paper | fr-02, cd-01 |
| 2301.10140v2 | Semantic Scholar Open Data Platform | Infrastructure/platform paper | fr-02, cd-01 |
| 2302.02231v2 | PubGraph | Knowledge graph construction, not survey | fr-02 |
| 2409.12177v1 | LitFM | (already in-scope) but also adjacent to citation graph infra | fr-02, cd-01 |
| All cd-02 results | Exploration/exploitation papers | General recommender systems (music, video, etc.), not literature survey | cd-02 |
| All cd-03 results | Multi-hop QA papers | General QA on Wikipedia/documents, not citation-graph survey | cd-03 |

---

## Summary

- **In-scope candidates**: 14 unique papers
- **Boundary papers encountered**: 18+ (most from cd-02 and cd-03 were uniformly out-of-scope)
- **Date range**: 2020-06 to 2025-10
- **Diversity**: covers multi-agent architectures (AutoSurvey, SciSage, Agentic AutoSurvey), citation-graph-aware methods (SurveyG, CG-RAG, LitFM), iterative/refinement approaches (IterSurvey, ResearchAgent, SurveyGen-I), evaluation (SurveyBench), knowledge-graph-based (ORKG), and interactive systems (InteractiveSurvey)

**Notable signal**: cd-02 (exploration/exploitation recommender systems) and cd-03 (multi-hop QA) both returned almost entirely out-of-scope material — this confirms the boundary is well-calibrated and those queries serve as effective discriminators.
