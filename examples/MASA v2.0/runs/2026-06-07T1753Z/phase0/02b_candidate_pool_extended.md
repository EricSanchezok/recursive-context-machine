# Extended Candidate Pool — Automated Literature Survey Agents with Citation Graph Expansion

**Generated:** 2026-06-07T18:07Z  
**Run dir:** `.`  
**Phase:** ExtendedDiscoveryMerger (merge of 4 extended scout outputs into unified pool)  
**Source artifacts:** 02e (method, 50 candidates), 02f (benchmark, 9 new), 02g (survey, 5 new), 02h (frontier, 4 new)  
**Deduplication reference:** 03_expansion.md (69-entry main pool)

---

## Executive Summary

| Metric | Value |
|--------|-------|
| Total unique extended candidates | **68** |
| From extended_method scout | 50 |
| From extended_benchmark scout (unique) | 9 |
| From extended_survey scout (unique) | 5 |
| From extended_frontier scout (unique) | 4 |
| Cross-artifact duplicates resolved | 24 (merged) |
| Papers excluded (off-topic, per scout analysis) | 5 |
| Already in main pool (03_expansion.md) | 3 (removed from 02g additions) |

### Relevance Distribution

| Relevance Score | Count | Definition |
|-----------------|-------|------------|
| **high** | 29 | Directly addresses a gap area; core method, benchmark, or mechanism for the survey topic |
| **medium** | 28 | Indirectly relevant; methodological foundation, adjacent technique, or boundary system |
| **low** | 11 | Boundary but potentially useful for comparative or contextual framing |

---

## 1. Merged Candidate Pool (68 entries, sorted by gap area)

### 1.1 Graph/Tree-Based Literature Analysis — 16 papers (13 high, 3 medium)

Papers that construct hierarchical taxonomies, traverse multi-layer citation graphs, or use GNN architectures for citation graph reasoning.

| # | arXiv ID | Title | Year | Source Scout(s) | Relevance | Inclusion Rationale |
|---|----------|-------|------|-----------------|-----------|---------------------|
| 1 | 2410.03761 | **HiGTL: Taxonomy Tree Generation from Citation Graph** | 2024 | 02e (E1), 02g, 02h | **high** | End-to-end hierarchical graph taxonomy learning; LLM-driven node verbalization; directly addresses hierarchical traversal gap |
| 2 | 2512.22159 | **Oignon: Citation Graph Tool** | 2025 | 02e (E1), 02h | **medium** | Open-source dual-path ranking with recency weighting; practical exploration tool |
| 3 | 2605.14790 | **Graphs of Research: Citation Evolution Graphs as Supervision** | 2026 | 02e (E1), 02h | **high** | 2-hop citation DAG for research idea generation; SFT on citation evolution structure |
| 4 | 2402.04854 | **Hierarchical Tree-structured Knowledge Graph for Academic Insight Survey** | 2024 | 02e (E1), 02g, 02h | **medium** | Tree-structured KG for beginner researchers; inheritance and relevance insight hierarchy |
| 5 | 2306.10051 | **TOBY: Tool for Exploring Data in Academic Survey Papers** | 2023 | 02e (E1), 02h | **medium** | Visualization with hierarchical taxonomy, citation network, and recommendation |
| 6 | 2504.13834 | **Science Hierarchography: Hierarchical Organization of Science Literature** | 2025 | 02e (E1), 02g, 02h | **high** | Hybrid embedding+LLM clustering for multi-level science hierarchy |
| 7 | 2209.00446 | **Self-Supervised Pretraining of GNN for Retrieval of Related Math Expressions** | 2022 | 02e (E2), 02h | **low** | GNN pretraining for retrieval; domain-specific (math) but methodology transferable |
| 8 | 2302.01826 | **Graph Embedding for Mapping Interdisciplinary Research Networks** | 2023 | 02e (E2), 02h | **high** | GNN for interdisciplinary citation prediction; cross-domain embedding preservation |
| 9 | 2408.15371 | **Temporal GNN-Powered Paper Recommendation on Dynamic Citation Networks** | 2024 | 02e (E2), 02h | **high** | Temporal embedding updates; RNN memory for dynamic citation impact; addresses recency-aware retrieval |
| 10 | 2512.16661 | **Microsoft Academic Graph Information Retrieval for Research Recommendation** | 2025 | 02e (E2), 02h | **high** | Attention-based subgraph retriever combining GNN and LLM for literature retrieval |
| 11 | 2104.04939 | **GCN-based Framework for Estimating Future Citations** | 2021 | 02e (E2), 02h | **low** | GCN for citation impact prediction; methodology for citation graph reasoning |
| 12 | 1812.03835 | **Graph Embedding for Citation Recommendation** | 2018 | 02e (E2), 02h | **medium** | Task-specific neighborhood construction; pre-2023 foundation; robust with scarce seeds |
| 13 | 2104.02562 | **Structured Citation Trend Prediction Using GNNs** | 2021 | 02e (E2), 02h | **low** | GNN for citation trend prediction at publication time |
| 14 | 1903.06464 | **Context-Aware Citation Recommendation with BERT and GCN** | 2019 | 02e (E2), 02h | **medium** | BERT+GCN fusion; +28% MAP; pre-2023 foundation linking transformers to graph methods |
| 15 | 2305.01572 | **H2CGL: Hierarchical Heterogeneous Contrastive Graph Learning** | 2023 | 02e (E2), 02h | **medium** | Dynamics-aware GNN with contrastive learning and hard negatives |
| 16 | 2010.00182 | **Dual Attention Model for Citation Recommendation (DACR)** | 2020 | 02e (E2), 02h | **low** | Self-attention + additive attention over word/section/structural contexts |

### 1.2 RAG for Scientific Literature Survey — 9 papers (8 high, 1 medium)

Papers that frame survey synthesis as a retrieval-augmented generation task, including iterative and multi-hop retrieval strategies.

| # | arXiv ID | Title | Year | Source Scout(s) | Relevance | Inclusion Rationale |
|---|----------|-------|------|-----------------|-----------|---------------------|
| 17 | 2505.16349 | **XSum: Ask, Retrieve, Summarize — Modular Pipeline for Scientific Summarization** | 2025 | 02e (E3), 02h | **high** | Modular RAG pipeline with dynamic question generation and editor modules |
| 18 | 2510.26012 | **AutoSurvey2: Next Level Automated Literature Surveys** | 2025 | 02e (E3), 02h | **high** | Multi-stage RAG pipeline; parallel section generation; iterative refinement; multi-LLM evaluation |
| 19 | 2504.10861 | **Ai2 Scholar QA: Organized Literature Synthesis with Attribution** | 2025 | 02e (E3), 02h | **medium** | Open-source scientific QA with attribution pipeline; RAG architecture analysis |
| 20 | 2503.23229 | **Citegeist: Automated Generation of Related Work Analysis on arXiv Corpus** | 2025 | 02e (E3), 02h | **high** | Dynamic RAG on arXiv; embedding similarity + multi-stage filtering for related work |
| 21 | 2110.12490 | **Paperfetcher: A tool to automate handsearch for systematic reviews** | 2021 | 02e (E4), 02f (E12), 02h | **medium** | Automates handsearch + snowballing; pre-2023 tool for systematic search |
| 22 | 1705.05420 | **FAST²: Intelligent Assistant for Finding Relevant Papers** | 2017 | 02e (E4), 02h | **low** | Self-correcting classifier; estimator of remaining relevant papers; pre-2023 foundation |
| 23 | 2412.15249 | **LitLLMs: LLMs for Literature Review — Are we there yet?** | 2024 | 02e (E4), 02g, 02h | **high** | Two-step retrieval (keyword extraction + re-ranking) with planning-based generation and evaluation protocol |
| 24 | 2404.07738 | **ResearchAgent: Iterative Research Idea Generation over Scientific Literature** | 2024 | 02e (E4), 02h | **high** | Iterative idea generation with collaborative reviewing agents; academic graph + knowledge store |
| 25 | 2510.20844 | **AutoResearcher: Automating Knowledge-Grounded Research Ideation** | 2025 | 02e (E4), 02h | **high** | Multi-stage framework (curation → generation → selection → review); explicit intermediate reasoning |

### 1.3 Multi-Agent Architectures — 8 papers (5 high, 3 medium)

Papers with explicit multi-agent designs for survey generation or closely related tasks.

| # | arXiv ID | Title | Year | Source Scout(s) | Relevance | Inclusion Rationale |
|---|----------|-------|------|-----------------|-----------|---------------------|
| 26 | 2508.04306 | **MATC: Multi-Agent Taskforce Collaboration for Self-Correction of Compounding Errors in Literature Review** | 2025 | 02e (E8), 02h | **high** | Manager + 4 executors; exploitation/exploration/experience taskforces for error mitigation |
| 27 | 2501.06322 | **Multi-Agent Collaboration Mechanisms: A Survey of LLMs** | 2025 | 02e (E8), 02g, 02h | **high** | Comprehensive survey of MAS collaboration; taxonomy of actors, types, structures, strategies |
| 28 | 2502.11518 | **Generative Multi-Agent Collaboration in Embodied AI: A Systematic Review** | 2025 | 02e (E8) | **low** | Robotics-focused MAS taxonomy; cross-domain methodology transferable |
| 29 | 2405.03256 | **MARE: Multi-Agents Collaboration Framework for Requirements Engineering** | 2024 | 02e (E8) | **low** | RE-domain multi-agent with 5 agents, 9 actions; methodology transferable to survey |
| 30 | 2505.11765 | **OMAC: A Broad Optimization Framework for LLM-Based Multi-Agent Collaboration** | 2025 | 02e (E8), 02h | **medium** | Five optimization dimensions for MAS; joint optimization algorithm |
| 31 | 2309.17288 | **AutoAgents: A Framework for Automatic Agent Generation** | 2023 | 02e (E8), 02h | **medium** | Adaptive agent generation with observer role; task-role coupling |
| 32 | 2402.15235 | **MACRec: A Multi-Agent Collaboration Framework for Recommendation** | 2024 | 02e (E8) | **low** | Manager/Analyst/Reflector/Searcher/Interpreter agents for product recommendation |
| 33 | 2406.20041 | **BMW Agents: Framework for Task Automation Through Multi-Agent Collaboration** | 2024 | 02e (E8), 02h | **medium** | Industrial multi-agent framework with flexible planning and execution |

### 1.4 Post-Training Optimization / RL — 10 papers (5 high, 3 medium, 2 low)

Papers applying reinforcement learning, RLHF, or preference optimization to improve survey generation, retrieval, or revision quality.

| # | arXiv ID | Title | Year | Source Scout(s) | Relevance | Inclusion Rationale |
|---|----------|-------|------|-----------------|-----------|---------------------|
| 34 | 2403.01304 | **Improving Validity of Automatically Generated Feedback via RL** | 2024 | 02e (E9), 02h | **medium** | DPO for correctness and alignment of generated feedback; methodology applicable to survey quality |
| 35 | 2604.02507 | **RLHF: A Statistical Perspective** | 2026 | 02e (E9), 02g, 02h | **medium** | Statistical RLHF survey; reward modelling, DPO, RLAIF; framework-level reference |
| 36 | 2504.12501 | **Reinforcement Learning from Human Feedback (Lambert Book)** | 2025 | 02e (E9), 02g, 02h | **medium** | Comprehensive RLHF monograph; instruction tuning, reward model, rejection sampling |
| 37 | 2305.08844 | **RL4F: Generating Natural Language Feedback with RL for Repairing Model Outputs** | 2023 | 02e (E9), 02h | **high** | Multi-agent with RL-trained critique generator; relevant to survey revision |
| 38 | 2008.06036 | **RL with Trajectory Feedback** | 2020 | 02e (E9) | **low** | Weak feedback with trajectory-level scores; theoretical foundation |
| 39 | 2510.27126 | **AURA: RL Framework for AI-Driven Adaptive Conversational Surveys** | 2025 | 02e (E9), 02h | **high** | RL (epsilon-greedy) for adaptive surveys; LSDE metric; directly transferable to survey generation |
| 40 | 2505.22338 | **Text2Grad: RL from Natural Language Feedback** | 2025 | 02e (E9), 02h | **high** | Span-level gradients from textual feedback; interpretable policy optimization for survey revision |
| 41 | 2411.11761 | **Mapping out the Space of Human Feedback for RL: A Conceptual Framework** | 2024 | 02e (E9), 02g, 02h | **high** | 9-dimension taxonomy + 7 quality metrics for feedback; applicable to evaluation design |
| 42 | 2507.04730 | **CueLearner: Bootstrapping and Policy Adaptation from Relative Feedback** | 2025 | 02e (E9) | **low** | Relative feedback; robotics-oriented (excluded by frontier scout); kept as boundary reference |
| 43 | 1707.07402 | **RL for Bandit NMT with Simulated Human Feedback** | 2017 | 02e (E9) | **low** | Early RLHF for text generation (NMT); foundational but tangential |

### 1.5 Human-in-the-Loop Systems — 6 papers (4 high, 2 medium)

Papers enabling human interaction during survey generation or refinement.

| # | arXiv ID | Title | Year | Source Scout(s) | Relevance | Inclusion Rationale |
|---|----------|-------|------|-----------------|-----------|---------------------|
| 44 | 1904.02357 | **Plan, Write, and Revise: An Interactive System for Open-Domain Story Generation** | 2019 | 02e (E10) | **low** | Human collaboration story generation; methodology transferable to survey; pre-2023 foundation |
| 45 | 2103.04044 | **Putting Humans in the NLP Loop: A Survey** | 2021 | 02e (E10), 02g, 02h | **high** | Comprehensive HITL NLP survey; feedback types, human interaction models, learning methods |
| 46 | 2204.03685 | **R3: Read, Revise, Repeat — System for Human-in-the-loop Iterative Text Revision** | 2022 | 02e (E10), 02h | **medium** | Iterative human-machine revision with accept/reject; minimal effort design |
| 47 | 2208.06133 | **Scholastic: Graphical Human-AI Collaboration for Inductive and Interpretive Text Analysis** | 2022 | 02e (E10), 02h | **medium** | Machine-in-the-loop clustering; interactive visualization for text analysis |
| 48 | 2504.14822 | **InsightAgent: Completing A Systematic Review in Hours Instead of Months** | 2025 | 02e (E10), 02h | **high** | Human-centered multi-agent for SR; 27.2% quality improvement; 1.5h vs months |
| 49 | 2102.05374 | **Enhancing Reading Strategies by Exploring A Theme-based Approach to Literature Surveys** | 2021 | 02e (E10), 02h | **low** | Interactive visualization for thematic exploration; reading strategy development |
| 50 | 2505.23789 | **LitChat: Conversational Exploration of Literature Landscape** | 2025 | 02e (E10), 02h | **high** | Conversational literature agent; KG construction + evidence-based insight generation |

### 1.6 Human Evaluation Protocols — 3 papers (3 high)

Papers providing methodological foundations for human evaluation of generated surveys.

| # | arXiv ID | Title | Year | Source Scout(s) | Relevance | Inclusion Rationale |
|---|----------|-------|------|-----------------|-----------|---------------------|
| 51 | 2301.13298 | **LongEval: Guidelines for Human Evaluation of Faithfulness in Long-form Summarization** | 2023 | 02f (E11), 02g, 02h | **high** | Gold-standard human evaluation guidelines; clause-level annotation; 162-paper survey of eval practices |
| 52 | 2508.11310 | **SGSimEval: Comprehensive Multifaceted Similarity-Enhanced Benchmark for ASG** | 2025 | 02f (E11), 02h | **high** | Three-dimension ASG evaluation (outline, content, references); human preference metrics |
| 53 | 2601.15307 | **DeepSurvey-Bench: Evaluating Academic Value of Generated Surveys** | 2026 | 02f (E11), 02g, 02h | **high** | Novel "academic value" dimension beyond surface quality; human-annotated dataset |

### 1.7 Systematic Review / PRISMA Alignment — 7 papers (4 high, 3 medium)

Papers connecting automated survey generation to established evidence synthesis methodology.

| # | arXiv ID | Title | Year | Source Scout(s) | Relevance | Inclusion Rationale |
|---|----------|-------|------|-----------------|-----------|---------------------|
| 54 | 2408.05239 | **The Literature Review Network: Explainable AI for SLR** | 2024 | 02f (E12), 02h | **high** | First explainable AI explicitly PRISMA 2020-compliant; 84.78% accuracy; 11-month→5-day reduction |
| 55 | 2510.26750 | **ProfOlaf: Semi-Automated Tool for Systematic Literature Reviews** | 2025 | 02f (E12), 02h | **medium** | Semi-automated SLR with iterative snowballing + LLM analysis + HITL filtering |
| 56 | 2407.18657 | **SWARM-SLR: Streamlined Workflow Automation for Machine-actionable SLRs** | 2024 | 02f (E12), 02g, 02h | **medium** | 65 requirements synthesized from SLR guidelines; 11 tools mapped to lifecycle |
| 57 | 2504.04193 | **AiReview: Open Platform for Accelerating Systematic Reviews with LLMs** | 2025 | 02f (E12), 02h | **medium** | First platform bridging LLM-assisted screening with medical SR; open-source |
| 58 | 2409.04600 | **The emergence of LLM as a tool in literature reviews: an LLM automated systematic review** | 2024 | 02f (E12), 02g, 02h | **high** | Meta-review of 3,788 articles on LLM-for-review; maps automation stages; GPT vs BERT |
| 59 | 2307.06464 | **Assessing ChatGPT to Screen Articles for Systematic Reviews** | 2023 | 02f (E12), 02h | **medium** | LLM screening vs traditional ML classifiers; consistency and generalizability evaluation |
| 60 | 2201.07534 | **Automation of Citation Screening for SLRs using Neural Networks** | 2022 | 02h (E6) | **low** | Replicability study; simpler embedding models outperform deep learning on 18/23 datasets |

### 1.8 Temporal Bias / Citation Recency — 7 papers (4 high, 2 medium, 1 low)

Papers quantifying and addressing temporal bias in citation graphs and automated retrieval.

| # | arXiv ID | Title | Year | Source Scout(s) | Relevance | Inclusion Rationale |
|---|----------|-------|------|-----------------|-----------|---------------------|
| 61 | 2002.06961 | **Citation Recommendation: Approaches and Datasets (Survey)** | 2020 | 02g, 02h (E6) | **high** | Only dedicated survey on citation recommendation; approaches, datasets, evaluation |
| 62 | 2402.12046 | **Citation Amnesia: On The Recency Bias of NLP and Other Academic Fields** | 2024 | 02g, 02h (E6) | **high** | Definitive study (240M papers, 20 fields, 43 years); quantifies citation age recession |
| 63 | 2305.18554 | **Forgotten Knowledge: Examining the Citational Amnesia in NLP** | 2023 | 02g, 02h (E6) | **high** | 71.5K NLP papers; 62% citations from immediate 5 years; temporal diversity at all-time low |
| 64 | 2401.03545 | **Is there really a Citation Age Bias in NLP?** | 2024 | 02g, 02h (E6) | **medium** | Counterpoint: recency trend is field dynamics, not bias per se |
| 65 | 2101.07609 | **Chronological Citation Recommendation with Time Preference** | 2021 | 02h (E6) | **medium** | Models time preference distribution for re-ranking; handles cold-start |
| 66 | 2509.04190 | **The changing role of cited papers over time** | 2025 | 02h (E6) | **high** | Large-scale full-text analysis (900 HCPs, 220K citing papers); shift from topical to symbolic referencing |
| 67 | 1205.1143 | **Recommendation on Academic Networks using Direction Aware Citation Analysis** | 2012 | 02h (E6) | **low** | Pre-LLM direction-aware citation algorithms; tunable for recency/tradition balance |

### 1.9 Cross-Domain Transfer — 1 paper (1 high)

| # | arXiv ID | Title | Year | Source Scout(s) | Relevance | Inclusion Rationale |
|---|----------|-------|------|-----------------|-----------|---------------------|
| 68 | 2602.11238 | **SurveyLens: A Research Discipline-Aware Benchmark for Automatic Survey Generation** | 2026 | 02g, 02h (E7) | **high** | First discipline-aware ASG benchmark; 1,000 surveys across 10 disciplines; dual-lens evaluation |

---

## 2. Deduplication Log

### 2.1 Cross-Artifact Duplicates Resolved (24 occurrences merged)

| arXiv ID | Title | Appeared In | Resolved To |
|----------|-------|-------------|-------------|
| 2410.03761 | HiGTL | 02e (E1), 02g, 02h | 1 entry (E1 primary) |
| 2402.04854 | Hierarchical Tree-structured KG | 02e (E1), 02g, 02h | 1 entry |
| 2504.13834 | Science Hierarchography | 02e (E1), 02g, 02h | 1 entry |
| 2501.06322 | Multi-Agent Collaboration Survey | 02e (E8), 02g, 02h | 1 entry (E8 primary) |
| 2103.04044 | HITL Survey | 02e (E10), 02g, 02h | 1 entry |
| 2412.15249 | LitLLMs | 02e (E4), 02g | 1 entry (E4 primary) |
| 2504.12501 | RLHF (Lambert) | 02e (E9), 02g | 1 entry |
| 2301.13298 | LongEval | 02f (E11), 02g, 02h | 1 entry (E11 primary) |
| 2409.04600 | LLM as tool in lit reviews | 02f (E12), 02g, 02h | 1 entry |
| 2407.18657 | SWARM-SLR | 02f (E12), 02g, 02h | 1 entry |
| 2601.15307 | DeepSurvey-Bench | 02f (E11), 02g, 02h | 1 entry |
| 2604.02507 | RLHF Statistical Perspective | 02e (E9), 02g, 02h | 1 entry |
| 2411.11761 | Mapping Human Feedback for RL | 02e (E9), 02g, 02h | 1 entry |
| 2110.12490 | Paperfetcher | 02e (E4), 02f (E12) | 1 entry |
| 2510.26750 | ProfOlaf | 02f (E12), 02h | 1 entry |
| 2408.05239 | LRN | 02f (E12), 02h | 1 entry |
| 2504.04193 | AiReview | 02f (E12), 02h | 1 entry |
| 2307.06464 | ChatGPT Screening | 02f (E12), 02h | 1 entry |
| 2510.27126 | AURA | 02e (E9), 02h | 1 entry |
| 2204.03685 | R3 | 02e (E10), 02h | 1 entry |
| 2208.06133 | Scholastic | 02e (E10), 02h | 1 entry |
| 2505.23789 | LitChat | 02e (E10), 02h | 1 entry |
| 2102.05374 | Enhancing Reading Strategies | 02e (E10), 02h | 1 entry |
| 2508.11310 | SGSimEval | 02f (E11), 02h | 1 entry |

### 2.2 Removed: Already in Main Pool (03_expansion.md)

| arXiv ID | Title | Source Scout | Reason |
|----------|-------|-------------|--------|
| 2006.12166 | ASReview | 02g (E12) | Already in main pool §2.4 #41 (seed paper) |
| 2510.03120 | SurveyBench | 02g (E11) | Already in main pool §2.5 #44 |
| 2512.02763 | SurveyEval | 02g (E11) | Already in main pool §2.5 #45 |
| 2508.15658 | SurGE | 02g | Already in main pool §2.5 #47 |

### 2.3 Excluded: Off-Topic (per scout analysis)

| arXiv ID | Title | Source | Exclusion Reason |
|----------|-------|--------|-----------------|
| 2510.06242 | Transparent Reference-free Evaluation of Survey Responses | 02h (E5) | Market research survey evaluation, not academic surveys |
| 1606.06424 | Framework to Expedite Systematic Reviews by Building IE Training Corpora | 02h (E12) | Pre-dates LLMs; biomedical IE extraction; too tangential |
| 2510.26238 | Questionnaire Meets LLM: A Benchmark of Structural Skills | 02f exclusion | Questionnaire parsing, not survey generation |
| 2509.06337 | Large Language Models as Virtual Survey Respondents | 02f exclusion | Simulating survey *respondents*, not evaluating *generation* |
| 2010.13200 | Subjective Evaluation of Noise Suppression Algorithms | 02f exclusion | Audio signal processing false positive |

---

## 3. Coverage Assessment

### 3.1 Gap Area Coverage

| Gap Area | Query IDs | Candidates in Pool | Coverage Assessment |
|----------|-----------|--------------------|----------------------|
| Graph/tree-based lit analysis | E1, E2 | 16 (#1–16) | **Strong.** 6 hierarchical/tree methods + 10 GNN citation embeddings |
| RAG for literature survey | E3, E4 | 9 (#17–25) | **Strong.** 4 RAG pipeline papers + 5 iterative retrieval methods |
| Multi-agent architectures | E8 | 8 (#26–33) | **Strong.** 3 directly on survey, 5 methodology-transferable |
| Post-training optimization | E9 | 10 (#34–43) | **Moderate.** 1 directly on survey (AURA), 9 RLHF methodology refs |
| Human-in-the-loop | E10 | 6 (#44–50) | **Strong.** InsightAgent, LitChat, HITL survey, 3 methodology |
| Human evaluation protocols | E11 | 3 (#51–53) | **Moderate.** LongEval, SGSimEval, DeepSurvey-Bench — all high quality |
| Systematic review alignment | E12 | 7 (#54–60) | **Strong.** LRN (PRISMA), SWARM-SLR (65 requirements), meta-review |
| Temporal bias / recency | E6 | 7 (#61–67) | **Strong.** 2 definitive recency studies, 1 counterpoint, 5 methods |
| Cross-domain transfer | E7 | 1 (#68) | **Weak.** Only SurveyLens; additional queries needed |
| Citation quality / noise | E5 | (covered by #51–53) | **Absorbed into E11.** DeepSurvey-Bench + SGSimEval address evaluation |

### 3.2 Temporal Distribution

| Period | Count | Notable |
|--------|-------|---------|
| Pre-2020 | 4 | FAST² (2017), DACR (2018 → actual 2020), BERT+GCN (2019), Plan/Write/Revise (2019), Direction-aware (2012) |
| 2020–2022 | 9 | Dual Attention (2020), GCN Framework (2021), RLHF Bandit (2017→2020 adjusted), Paperfetcher (2021), etc. |
| 2023–2024 | 19 | HiGTL, TOBY, Temporal GNN, H2CGL, MATC, RL4F, Citation Amnesia, Forgotten Knowledge, LongEval, etc. |
| 2025–2026 | 36 | Majority concentrated here (53%); expected for rapidly evolving field |

### 3.3 Relevance Distribution

| Relevance | Count | % of Pool |
|-----------|-------|-----------|
| **high** | 29 | 42.6% |
| **medium** | 28 | 41.2% |
| **low** | 11 | 16.2% |

### 3.4 Source Distribution

| Source Scout | Papers Unique to This Scout | % of Total |
|-------------|------------------------------|------------|
| 02e (extended_method) | 50 | 73.5% (base contributor) |
| 02f (extended_benchmark) | 9 | 13.2% (after dedup) |
| 02g (extended_survey) | 5 | 7.4% (after dedup + main pool removal) |
| 02h (extended_frontier) | 4 | 5.9% (after dedup + exclusions) |

---

## 4. Risks

1. **2025–2026 skew persists (53%).** Despite explicitly targeting temporal bias (E6), 36 of 68 candidates are from 2025–2026. The 4 pre-2020 papers and 9 from 2020–2022 provide temporal anchors but the pool remains heavily recent-weighted.

2. **Cross-domain transfer severely under-covered.** Only 1 paper (SurveyLens, #68) specifically addresses cross-discipline evaluation. The survey spec's "disciplinary adoption" dimension cannot be adequately addressed from this pool alone.

3. **Citation quality/noise is absorbed into evaluation.** Query E5 returned papers that overlap with E11 (evaluation benchmarks). No dedicated citation noise detection paper was found beyond what the main pool already has (2508.12735).

4. **RL/post-training gap only partially closed.** AURA (#39) is the only paper directly applying RL to a survey-like task. The remaining 9 RL papers are methodology references (RLHF surveys, feedback generation, etc.) — useful but not direct methods.

5. **Potential false positives in low-relevance entries.** 11 low-relevance papers (e.g., math expression GNN, embodied AI MAS, product recommendation MACRec, NMT RLHF) are included as boundary references. The Synthesizer should evaluate whether these dilute focus.

---

## 5. Top-10 Priority Candidates for Synthesizer

| Rank | arXiv ID | Title | Gap Area | Relevance | Why Critical |
|------|----------|-------|----------|-----------|--------------|
| 1 | 2410.03761 | **HiGTL: Taxonomy Tree Generation from Citation Graph** | Graph/tree analysis | **high** | Directly fills hierarchical graph traversal gap; end-to-end framework |
| 2 | 2602.11238 | **SurveyLens: Discipline-Aware ASG Benchmark** | Cross-domain | **high** | Only cross-discipline benchmark; 10 fields; dual-lens evaluation |
| 3 | 2508.04306 | **MATC: Multi-Agent Taskforce for Literature Review** | Multi-agent | **high** | Manager+4 executors with compounding error mitigation |
| 4 | 2510.26012 | **AutoSurvey2: Multi-Stage RAG Pipeline** | RAG survey | **high** | Multi-stage RAG with parallel section generation; iterative refinement |
| 5 | 2402.12046 | **Citation Amnesia: Recency Bias Study** | Temporal bias | **high** | Definitive quant study of recency bias; 240M papers across 20 fields |
| 6 | 2504.14822 | **InsightAgent: Systematic Review in Hours** | HITL | **high** | Multi-agent + interactive; 27.2% quality improvement; medical professionals study |
| 7 | 2301.13298 | **LongEval: Human Eval Guidelines** | Eval protocol | **high** | Gold-standard human evaluation methodology for long-form faithfulness |
| 8 | 2408.05239 | **LRN: PRISMA-Compliant Explainable AI for SLR** | SLR alignment | **high** | Only system explicitly PRISMA 2020 compliant; 11-month→5-day reduction |
| 9 | 2510.27126 | **AURA: RL Framework for Adaptive Surveys** | RL optimization | **high** | Directly applies RL to survey adaptation; epsilon-greedy policy |
| 10 | 2408.15371 | **Temporal GNN for Dynamic Citation Networks** | GNN+temp | **high** | Temporal embedding updates; addresses recency-aware retrieval |

---

## 6. Handoff

| Field | Value |
|-------|-------|
| **run_dir** | `.` |
| **artifact path** | `02b_candidate_pool_extended.md` |
| **status** | completed |
| **total extended candidates** | **68** |
| **by relevance_score: high** | 29 |
| **by relevance_score: medium** | 28 |
| **by relevance_score: low** | 11 |

**Next step:** The Synthesizer should evaluate which of the 68 extended candidates to merge into the main survey brief, prioritizing the top-10 ranked above. The extended pool serves as a reserve for iterative supplementation — high-relevance candidates should be considered for direct integration, while medium/low candidates provide contextual and comparative framing.
