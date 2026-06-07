# Extended Method Candidates — Automated Literature Survey Agents with Citation Graph Expansion

Generated: 2026-06-07T18:05Z  
Run dir: `.`  
Phase: ExtendedMethodScout (supplementary arXiv retrieval for method and mechanism papers)

---

## Summary

| Source | Query | Query ID | New Candidates | Existing (skipped) |
|--------|-------|----------|----------------|---------------------|
| Graph/tree-based literature analysis | hierarchical citation graph survey generation traversal | E1 | 6 | 4 |
| Graph/tree-based literature analysis | graph neural network citation retrieval embedding science | E2 | 10 | 0 |
| RAG for scientific literature survey | retrieval augmented generation scientific paper survey synthesis | E3 | 4 | 6 |
| RAG for scientific literature survey | multi hop retrieval iterative search literature review automation | E4 | 5 | 5 |
| Multi-agent architectures | multi agent collaboration literature survey generation framework | E8 | 8 | 2 |
| Post-training optimization | reinforcement learning feedback optimization survey generation retrieval | E9 | 10 | 0 |
| Human-in-the-loop systems | human in the loop interactive literature survey refinement | E10 | 7 | 3 |
| **Total** | **7 queries** | **E1–E4, E8–E10** | **50** | **20** |

---

## Candidate Papers (50 unique entries, deduplicated by arXiv ID)

### E1 — Hierarchical Citation Graph Survey Generation Traversal

| # | arXiv ID | Title | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------|------|--------------|-------------|------------------|
| 1 | 2410.03761 | HiGTL: Taxonomy Tree Generation from Citation Graph | 2024 | E1 | graph/tree method | End-to-end hierarchical graph taxonomy learning; directly addresses the gap of tree-structured traversal over citation graphs for survey planning |
| 2 | 2512.22159 | Oignon: Citation Graph Tool | 2025 | E1 | graph/tree tool | Free/open-source tool with dual-path ranking and recency weighting for citation graph exploration |
| 3 | 2605.14790 | Graphs of Research: Citation Evolution Graphs as Supervision for Research Idea Generation | 2026 | E1 | graph/tree method | Extracts 2-hop citation graph DAG; supervised fine-tuning on citation evolution for idea generation |
| 4 | 2402.04854 | Hierarchical Tree-structured Knowledge Graph For Academic Insight Survey | 2024 | E1 | graph/tree method | Hierarchical KG reflecting inheritance and relevance insight for beginner researchers |
| 5 | 2306.10051 | TOBY: A Tool for Exploring Data in Academic Survey Papers | 2023 | E1 | graph/tree tool | Visualization tool with hierarchical taxonomy view, document similarity, and citation network |
| 6 | 2504.13834 | Science Hierarchography: Hierarchical Organization of Science Literature | 2025 | E1 | graph/tree method | Hybrid embedding+LLM approach for hierarchical literature organization; directly relevant to hierarchical survey structuring |

### E2 — Graph Neural Network Citation Retrieval & Embeddings

| # | arXiv ID | Title | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------|------|--------------|-------------|------------------|
| 7 | 2209.00446 | Self-Supervised Pretraining of GNN for Retrieval of Related Mathematical Expressions | 2022 | E2 | embedding method | GNN-based embedding for retrieval; relevant methodology even if domain-specific (math expressions) |
| 8 | 2302.01826 | Graph Embedding for Mapping Interdisciplinary Research Networks | 2023 | E2 | embedding method | GNN architecture for interdisciplinary citation prediction; addresses cross-domain retrieval gap |
| 9 | 2408.15371 | Temporal GNN-Powered Paper Recommendation on Dynamic Citation Networks | 2024 | E2 | embedding method | Temporal dimension in citation embeddings; addresses recency-aware retrieval (E6 gap) |
| 10 | 2512.16661 | Microsoft Academic Graph Information Retrieval for Research Recommendation | 2025 | E2 | embedding method | Attention-based subgraph retriever combining GNN and LLM for literature retrieval |
| 11 | 2104.04939 | GCN Framework for Estimating Future Citations Count | 2021 | E2 | embedding method | GCN-based citation prediction; methodology relevant for citation graph reasoning |
| 12 | 1812.03835 | Graph Embedding for Citation Recommendation | 2018 | E2 | embedding method | Task-specific neighborhood construction for citation graph embedding; pre-2023 foundation |
| 13 | 2104.02562 | Structured Citation Trend Prediction Using Graph Neural Networks | 2021 | E2 | embedding method | GNN architecture for citation graph trend prediction |
| 14 | 1903.06464 | Context-Aware Citation Recommendation with BERT and GCN | 2019 | E2 | embedding method | BERT+GCN fusion for context-aware citation recommendation; links foundation models to graph methods |
| 15 | 2305.01572 | H2CGL: Modeling Dynamics of Citation Network for Impact Prediction | 2023 | E2 | embedding method | Hierarchical heterogeneous contrastive graph learning; dynamics-aware citation modelling |
| 16 | 2010.00182 | Dual Attention Model for Citation Recommendation | 2020 | E2 | embedding method | Self-attention + additive attention for citation context embedding |

### E3 — RAG for Scientific Paper Survey Synthesis

| # | arXiv ID | Title | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------|------|--------------|-------------|------------------|
| 17 | 2505.16349 | XSum: Ask, Retrieve, Summarize — A Modular Pipeline for Scientific Literature Summarization | 2025 | E3 | RAG method | Modular RAG pipeline for scientific multi-document summarization; question-generation + editor modules |
| 18 | 2510.26012 | AutoSurvey2: Empowering Researchers with Next Level Automated Literature Surveys | 2025 | E3 | RAG method | Multi-stage RAG pipeline; parallel section generation; iterative refinement; direct RAG-for-survey paper |
| 19 | 2504.10861 | Ai2 Scholar QA: Organized Literature Synthesis with Attribution | 2025 | E3 | RAG method | Free/open-source scientific QA with attribution pipeline; RAG architecture analysis |
| 20 | 2503.23229 | Citegeist: Automated Generation of Related Work Analysis on the arXiv Corpus | 2025 | E3 | RAG method | Dynamic RAG on arXiv corpus; embedding similarity + summarization + multi-stage filtering |

### E4 — Multi-Hop Iterative Retrieval for Literature Review

| # | arXiv ID | Title | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------|------|--------------|-------------|------------------|
| 21 | 2110.12490 | Paperfetcher: A tool to automate handsearch for systematic reviews | 2021 | E4 | iterative retrieval tool | Automates handsearch with snowballing in both directions; pre-2023 tool for systematic review search |
| 22 | 1705.05420 | FAST²: Intelligent Assistant for Finding Relevant Papers | 2017 | E4 | iterative retrieval method | Self-correcting classification algorithm; estimator of remaining relevant papers; pre-2023 foundation |
| 23 | 2412.15249 | LitLLMs, LLMs for Literature Review: Are we there yet? | 2024 | E4 | iterative retrieval method | Two-step search strategy (keyword extraction → retrieval) + re-ranking with attribution; evaluation protocol |
| 24 | 2404.07738 | ResearchAgent: Iterative Research Idea Generation over Scientific Literature | 2024 | E4 | iterative retrieval method | Iterative refinement via collaborative LLM reviewers; academic graph + knowledge store augmentation |
| 25 | 2510.20844 | AutoResearcher: Automating Knowledge-Grounded and Transparent Research Ideation | 2025 | E4 | iterative retrieval method | Multi-stage framework (curation → generation → selection → review); explicit intermediate reasoning states |

### E8 — Multi-Agent Collaboration for Literature Survey

| # | arXiv ID | Title | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------|------|--------------|-------------|------------------|
| 26 | 2508.04306 | MATC: Multi-Agent Taskforce Collaboration for Self-Correction of Compounding Errors in Literature Review | 2025 | E8 | multi-agent method | Manager + 4 executor agents; explores exploitation/exploration/experience taskforces; compounding error mitigation |
| 27 | 2501.06322 | Multi-Agent Collaboration Mechanisms: A Survey of LLMs | 2025 | E8 | multi-agent survey | Comprehensive survey of multi-agent collaboration mechanisms; taxonomy of actors, types, structures, strategies |
| 28 | 2502.11518 | Generative Multi-Agent Collaboration in Embodied AI: A Systematic Review | 2025 | E8 | multi-agent survey | Taxonomy of EMAS; perception, planning, communication, feedback analysis; cross-domain methodology |
| 29 | 2405.03256 | MARE: Multi-Agents Collaboration Framework for Requirements Engineering | 2024 | E8 | multi-agent method | Role-based multi-agent collaboration; 5 agents, 9 actions, shared workspace; methodology transferable to survey |
| 30 | 2505.11765 | OMAC: A Broad Optimization Framework for LLM-Based Multi-Agent Collaboration | 2025 | E8 | multi-agent optimization | Five optimization dimensions for MAS; joint optimization across dimensions; applicable to survey agent design |
| 31 | 2309.17288 | AutoAgents: A Framework for Automatic Agent Generation | 2023 | E8 | multi-agent method | Adaptive agent generation; observer role for reflection; task-role coupling |
| 32 | 2402.15235 | MACRec: a Multi-Agent Collaboration Framework for Recommendation | 2024 | E8 | multi-agent method | Manager/Analyst/Reflector/Searcher/Interpreter agents; collaboration patterns transferable to survey agents |
| 33 | 2406.20041 | BMW Agents: Framework For Task Automation Through Multi-Agent Collaboration | 2024 | E8 | multi-agent method | Industrial multi-agent framework with flexible planning and execution; reliability for complex workflows |

### E9 — Post-Training Optimization / RL for Survey Generation

| # | arXiv ID | Title | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------|------|--------------|-------------|------------------|
| 34 | 2403.01304 | Improving Validity of Automatically Generated Feedback via RL | 2024 | E9 | RL optimization | RL (DPO) for correctness and alignment of generated feedback; methodology applicable to survey quality optimization |
| 35 | 2604.02507 | RLHF: A Statistical Perspective | 2026 | E9 | RLHF survey | Statistical perspective on RLHF; reward modelling, DPO, RLAIF; framework-level reference for survey optimization |
| 36 | 2504.12501 | RLHF (Lambert) | 2025 | E9 | RLHF reference | Comprehensive book on RLHF methods; instruction tuning, reward model, rejection sampling, direct alignment |
| 37 | 2305.08844 | RL4F: Generating Natural Language Feedback with RL for Repairing Model Outputs | 2023 | E9 | RL optimization | Multi-agent framework where critique generator trained to maximise end-task performance; relevant to survey revision |
| 38 | 2008.06036 | RL with Trajectory Feedback | 2020 | E9 | RL method | Weak feedback setting (trajectory-level scores); relevant for survey-level quality feedback |
| 39 | 2510.27126 | AURA: RL Framework for AI-Driven Adaptive Conversational Surveys | 2025 | E9 | RL optimization | RL for adaptive conversational surveys; LSDE metric; epsilon-greedy policy for question selection |
| 40 | 2505.22338 | Text2Grad: RL from Natural Language Feedback | 2025 | E9 | RL optimization | Converts textual feedback into span-level gradients; applicable to fine-grained survey revision |
| 41 | 2411.11761 | Mapping out the Space of Human Feedback for RL | 2024 | E9 | RLHF framework | Taxonomy of feedback types for RLHF; nine dimensions, seven quality metrics; design requirements |
| 42 | 2507.04730 | CueLearner: Bootstrapping and Policy Adaptation from Relative Feedback | 2025 | E9 | RL optimization | Relative feedback ("more to the left") for RL; sample efficiency improvement |
| 43 | 1707.07402 | RL for Bandit NMT with Simulated Human Feedback | 2017 | E9 | RL method | Early RLHF method for text generation; advantage actor-critic with attention; foundational for survey RL |

### E10 — Human-in-the-Loop Interactive Survey Systems

| # | arXiv ID | Title | Year | Source Query | Likely Role | Inclusion Reason |
|---|----------|-------|------|--------------|-------------|------------------|
| 44 | 1904.02357 | Plan, Write, and Revise: An Interactive System for Open-Domain Story Generation | 2019 | E10 | HITL method | Human collaboration at planning and writing stages; 10-50% improvement; methodology transferable to survey |
| 45 | 2103.04044 | Putting Humans in the NLP Loop: A Survey | 2021 | E10 | HITL survey | Comprehensive survey of HITL NLP frameworks; tasks, goals, human interactions, feedback learning methods |
| 46 | 2204.03685 | Read, Revise, Repeat: System for Human-in-the-loop Iterative Text Revision | 2022 | E10 | HITL method | Iterative human-machine text revision; accept/reject suggested edits; minimal human effort design |
| 47 | 2208.06133 | Scholastic: Graphical Human-AI Collaboration for Inductive and Interpretive Text Analysis | 2022 | E10 | HITL method | Machine-in-the-loop clustering for interpretive text analysis; interactive visualisation for sampling |
| 48 | 2504.14822 | InsightAgent: Completing A Systematic Review in Hours instead of Months | 2025 | E10 | HITL method | Human-centered multi-agent for SR; visualisation + real-time feedback; 27.2% quality improvement; 1.5h vs months |
| 49 | 2102.05374 | Enhancing Reading Strategies by Exploring A Theme-based Approach to Literature Surveys | 2021 | E10 | HITL method | Interactive visualisation for thematic exploration; reading strategy development; design recommendations |
| 50 | 2505.23789 | LitChat: Conversational Exploration of Literature Landscape | 2025 | E10 | HITL method | Conversational literature agent; KG construction + data mining; evidence-based insight generation |

---

## Coverage Against Gap Areas

| Gap Area | Query IDs | New Candidates | Notes |
|----------|-----------|----------------|-------|
| Graph/tree-based literature analysis | E1, E2 | 16 (6+10) | Strong coverage: HiGTL, GoR, Science Hierarchography (tree methods) + 10 GNN methods |
| RAG for scientific literature survey | E3, E4 | 9 (4+5) | XSum, AutoSurvey2, Ai2 Scholar QA, Citegeist (RAG) + Paperfetcher, FAST², ResearchAgent etc. (iterative) |
| Multi-agent architectures | E8 | 8 | MATC, OMAC, AutoAgents, MARE + multi-agent surveys |
| Post-training optimization | E9 | 10 | RLHF statistical perspective, RL4F, Text2Grad, AURA adaptive surveys |
| Human-in-the-loop | E10 | 7 | InsightAgent, LitChat, Scholastic, interactive story/survey generation methods |

---

## Deduplication Log

Skipped (already in 03_expansion.md 69-entry pool):

| arXiv ID | Title | Existing Category |
|----------|-------|-------------------|
| 2110.06354 | Tell Me How to Survey / SurveyBank | Retrieval & Embeddings (#16); Benchmarks (#53) |
| 2510.21900 | IterSurvey | Survey Agents (#28) |
| 1805.02262 | Semantic Scholar Literature Graph | Citation Graph Infrastructure (#1) |
| 2301.11223 | CitationSum | Citation Graph Infrastructure (#5) |
| 2402.01788 | LitLLM | Survey Agents (#35) |
| 2504.08762 | InteractiveSurvey | Survey Agents (#26) |
| 2411.14199 | OpenScholar | Survey Agents (#29) |
| 2508.14317 | SurveyGen-I | Survey Agents (#38) |
| 2509.18661 | Agentic AutoSurvey | Survey Agents (#22) |
| 2503.04629 | SurveyForge | Survey Agents (#21); Benchmarks (#52) |
| 2411.06159 | Mixture of Knowledge Minigraph Agents | Survey Agents (#30) |
| 2309.01684 | CRUISE-Screening | Systematic Review / Screening (#42) |

---

## Notes

1. **Temporal distribution:** 3 pre-2020 papers (1705.05420, 1903.06464, 1904.02357), 7 from 2020–2022, 11 from 2023–2024, 29 from 2025–2026. The 2025 skew is expected but less severe than the 54% in the main expansion due to E2 capturing older GNN papers.

2. **No duplicate across queries:** No paper appeared in more than one of the 7 queries (after dedup against existing pool). The queries are well-separated.

3. **Foundation model gap:** SPECTER, SciBERT are seed papers; E2 (GNN) captures methods that build on foundation models. No separate foundation model query needed per extended plan.

4. **Likely roles assigned per schema:** graph/tree method, graph/tree tool, embedding method, RAG method, iterative retrieval method/tool, multi-agent method/survey/optimization, RL method/optimization/survey, HITL method/survey.
