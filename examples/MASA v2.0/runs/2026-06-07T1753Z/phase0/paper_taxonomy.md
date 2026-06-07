# Paper Taxonomy — Automated Literature Survey Agents with Citation Graph Expansion

## Overview

This taxonomy classifies 139 papers from the candidate pool into 6 primary categories based on their architectural type, with secondary dimensions capturing citation graph awareness, retrieval method, iteration strategy, and relationship to the core survey topic.

---

## Classification Dimensions

| Dimension | Possible Values | Criteria |
|-----------|---------------|---------|
| **Architecture Type** (primary) | `graph_enhanced_retrieval`, `single_agent_pipeline`, `multi_agent_pipeline`, `hybrid_interactive`, `iterative_refinement`, `benchmark_evaluation` | The fundamental architectural pattern: whether citation graph structure is the primary mechanism (graph_enhanced), whether a single LLM orchestrates all stages (single_agent), whether multiple specialized agents coordinate (multi_agent), whether the system requires human interaction (hybrid_interactive), whether the system uses RL/iterative feedback loops (iterative_refinement), or whether the paper contributes an evaluation benchmark/dataset (benchmark_evaluation). |
| **Citation Graph Awareness** | `none`, `bfs` (traversal), `embedding` (graph-aware embeddings), `hierarchical`, `gnn` (graph neural network) | How deeply the system models or traverses the citation graph structure. |
| **Retrieval Method** | `embedding`, `keyword`, `hybrid`, `graph_traversal`, `active_learning` | The primary mechanism for retrieving relevant papers. |
| **Iteration Strategy** | `single_pass`, `multi_round`, `interactive` | Whether the system generates output in one pass, iteratively refines, or requires user interaction between steps. |
| **Relation to Survey Topic** | `core` (direct survey generation system), `supporting` (citation graph method, embedding, or tool), `analysis` (bias study, scientometric analysis), `benchmark` (evaluation dataset/protocol), `foundation` (pre-LLM algorithmic foundation), `boundary` (adjacent domain — systematic review automation) | The paper's role relative to the core research question. |

---

## Category 1: Graph-Enhanced Retrieval

### Definition
Papers where citation graph structure is the primary computational mechanism. This category includes citation graph infrastructure construction, graph neural network (GNN) methods for citation embedding/retrieval/recommendation, citation graph traversal algorithms, and graph-based clustering/taxonomy generation. The common thread is that the graph — not an LLM pipeline — does the heavy lifting for literature understanding and retrieval.

### Papers

| arXiv ID | Title | Category-Specific Notes |
|----------|-------|------------------------|
| 1205.1143 | Recommendation on Academic Networks using Direction Aware Citation Analysis | Pre-LLM direction-aware citation analysis; forward/backward influence distinction |
| 1511.05078 | Which Type of Citation Analysis Generates the Most Accurate Taxonomy? | Direct citation vs bibliographic coupling vs co-citation comparison |
| 1805.02262 | Construction of the Literature Graph in Semantic Scholar | 280M+ node heterogeneous graph infrastructure pipeline |
| 1812.03835 | Graph Embedding for Citation Recommendation | Task-specific neighborhood construction for graph embedding |
| 1902.05170 | GrapAL: Connecting the Dots in Scientific Literature | Neo4j-based graph DB query tool for citation exploration |
| 1903.06464 | Context-Aware Citation Recommendation with BERT and GCN | BERT+GCN fusion; +28% MAP |
| 1905.00075 | On the Use of ArXiv as a Dataset | Early large-scale arXiv citation graph (6.7M edges) |
| 2004.05904 | Return to Basics: Clustering using Structural Information | Direct citation replication outperforms NLP-based clustering |
| 2004.07180 | SPECTER: Citation-informed Transformers | Core citation-informed document embedding method |
| 2006.01131 | NLP Scholar: Interactive Visual Explorer for NLP Literature | Citation-aware visualization for NLP |
| 2010.00182 | DACR: Dual Attention Model for Citation Recommendation | Self-attention + additive attention over structural contexts |
| 2101.07609 | Chronological Citation Recommendation with Time Preference | Models time preference distribution for citation re-ranking |
| 2104.02562 | Structured Citation Trend Prediction Using GNNs | GNN for citation trend prediction at publication time |
| 2104.04939 | GCN-based Framework for Estimating Future Citations | GCN for citation impact prediction |
| 2106.01560 | CitationIE: Leveraging the Citation Graph for Scientific IE | Graph-informed information extraction from scientific text |
| 2106.05633 | Citation Recommendation for Research Papers via Knowledge Graphs | Research KG + citation network; +0.8 MAP |
| 2109.10007 | Generating Local Maps of Science using Deep Bibliographic Coupling | Graph diffusion extends bibliographic coupling to deep neighbourhood |
| 2110.06354 | Reading Path Generation (SurveyBank) | Graph-optimisation-based reading path generation; SurveyBank dataset |
| 2207.03299 | Academic IR using Citation Clusters | Citation clusters complementary to query-based search |
| 2209.00446 | Self-Supervised Pretraining of GNN for Retrieval of Related Math Expressions | GNN pretraining for retrieval; domain-specific math |
| 2301.10140 | The Semantic Scholar Open Data Platform | 200M+ papers, structured text, embeddings |
| 2301.11223 | CitationSum: Citation-aware Graph Contrastive Learning for Summarization | Citation-aware training objectives for summarization |
| 2302.01826 | Graph Embedding for Mapping Interdisciplinary Research Networks | GNN for interdisciplinary citation prediction; cross-domain embedding |
| 2305.01572 | H2CGL: Hierarchical Heterogeneous Contrastive Graph Learning | Dynamics-aware GNN with contrastive learning and hard negatives |
| 2402.04854 | Hierarchical Tree-structured Knowledge Graph for Academic Insight Survey | Tree-structured KG for beginner researchers |
| 2403.09295 | Seed-based IR: Direct Citations, Bibliographic Coupling, Co-citations | Systematic comparison combining all 3 methods |
| 2408.15371 | Temporal GNN-Powered Paper Recommendation on Dynamic Citation Networks | Temporal embedding updates; RNN memory for dynamic citation impact |
| 2409.12177 | LitFM: Structure-aware Foundation Model for Citation Graphs | Graph transformer; +28.1% precision on retrieval |
| 2410.03761 | HiGTL: Taxonomy Tree Generation from Citation Graph | End-to-end hierarchical graph taxonomy learning; LLM-driven node verbalization |
| 2502.13934 | Citation Proximus: Social and Semantic Ties in Citing Behaviour | Collaboration network strongest predictor of citation |
| 2504.13834 | Science Hierarchography: Hierarchical Organization of Science Literature | Hybrid embedding+LLM clustering for multi-level science hierarchy |
| 2509.04190 | The Changing Role of Cited Papers over Time | Large-scale full-text analysis; shift from topical to symbolic referencing |
| 2510.07733 | SurveyG: Hierarchical Citation Graph Framework | Foundation/Development/Frontier 3-layer graph; horizontal+vertical traversal |
| 2512.16661 | Microsoft Academic Graph Information Retrieval for Research Recommendation | Attention-based subgraph retriever combining GNN and LLM |
| 2512.22159 | Oignon: Citation Graph Tool | Open-source dual-path ranking with recency weighting |
| 2605.14790 | Graphs of Research: Citation Evolution Graphs as Supervision | 2-hop citation DAG for research idea generation; SFT on citation evolution |

### Shared Characteristics
- **Citation graph is the primary data structure** — All papers model, construct, traverse, or embed citation graphs as their central mechanism
- **Pre-LLM or LLM-agnostic heritage** — Many papers predate LLM-based survey agents, providing algorithmic foundations
- **Retrieval-focused** — The goal is effective literature retrieval or representation, not survey synthesis
- **Structural signals over semantic** — Graph structure (edges, paths, neighborhoods) is weighted at least as heavily as text content

### Distinguishing Features (intra-category variation)
- **Infrastructure vs GNN vs traversal**: Infrastructure papers (1805.02262, 2301.10140, 1902.05170) build graph databases; GNN papers (2409.12177, 2305.01572, 2408.15371) learn graph-aware embeddings; traversal papers (2510.07733, 2110.06354, 1205.1143) develop graph algorithms
- **Scale**: From 6.7M edges (1905.00075) to 280M+ nodes (1805.02262) to full Semantic Scholar scale
- **Time-awareness**: Some papers model temporal dynamics (2408.15371, 2101.07609, 1205.1143); others are static
- **Taxonomy focus**: Papers like 2410.03761 and 1511.05078 explicitly generate taxonomies from citation structure, while others focus on retrieval quality

---

## Category 2: Single-Agent Pipeline

### Definition
Papers where a single LLM (or a single pipeline controlled by one LLM) orchestrates all stages of survey generation or literature analysis. The pipeline may have multiple stages (outline → retrieve → draft → refine), but all stages are handled by a single agent without coordination with other specialized agents. This category also includes analysis papers (bias studies, scientometric surveys) and foundational scientific NLP models (SciBERT, S2ORC) — any paper where the contribution is a pipeline, method, or analysis rather than an evaluation benchmark or dataset. Note: evaluation benchmarks/datasets are classified under category 6 (Benchmark Evaluation), not here.

### Papers

| arXiv ID | Title | Category-Specific Notes |
|----------|-------|------------------------|
| 1501.05462 | A Review of Theory and Practice in Scientometrics | Survey of citation metrics and normalisation |
| 1703.08071 | Quantifying and Suppressing Ranking Bias in Citation Networks | Mahalanobis distance; z-score normalisation |
| 1911.02782 | S2ORC: The Semantic Scholar Open Research Corpus | 8.1M paper open corpus |
| 1903.10676 | SciBERT: A Pretrained Language Model for Scientific Text | Scientific domain BERT |
| 2002.06961 | Citation Recommendation: Approaches and Datasets (Survey) | Survey paper on citation recommendation |
| 2006.12166 | ASReview: Open Source Software for Efficient Reviews | Active learning SLR screening tool (boundary context) |
| 2010.04147 | Automatic generation of reviews of scientific papers | Co-citation graph + BERT extractive summarisation |
| 2011.09752 | Hybrid Learning for Technology-Assisted SLR | Learning-to-rank + relevance feedback pipeline |
| 2102.05374 | Enhancing Reading Strategies by Exploring A Theme-based Approach | Interactive visualization for thematic exploration |
| 2105.03011 | QASPER: Information-Seeking QA Anchored in Research Papers | 5,049 questions over 1,585 NLP papers (benchmark) |
| 2201.07534 | Automation of Citation Screening for SLRs using Neural Networks | Simpler embedding models outperform deep learning |
| 2202.10033 | Open-source integrated framework for citation collection and screening | Bayesian active learning tool |
| 2203.17239 | Citation Bias in Peer Review (Cite-seeing) | Citation bias analysis in peer review |
| 2208.02334 | Knowledge Graph-Based Method for Automating SLRs | KG approach for SLR automation |
| 2301.13298 | LongEval: Guidelines for Human Evaluation of Faithfulness | Human evaluation guidelines for long-form summarization |
| 2305.18554 | Forgotten Knowledge: Examining the Citational Amnesia in NLP | 71.5K NLP papers; 62% citations from last 5 years |
| 2306.03535 | SciLit: Platform for Literature Discovery, Summarization and Citation | End-to-end assistive writing tool |
| 2306.10051 | TOBY: Tool for Exploring Data in Academic Survey Papers | Visualization with hierarchical taxonomy and citation network |
| 2307.06464 | Assessing ChatGPT to Screen Articles for Systematic Reviews | LLM screening vs traditional ML classifiers |
| 2309.01684 | CRUISE-Screening: Living Literature Reviews Toolbox | Web-based living review tool |
| 2309.09727 | When LLMs Meet Citation: A Comprehensive Survey | Bidirectional review of LLM-for-citation |
| 2312.07559 | PaperQA: Retrieval-Augmented Agent for Scientific Research | Single-agent with citation chaining; LitQA benchmark |
| 2401.03545 | Is there really a Citation Age Bias in NLP? | Counterpoint: recency trend is field dynamics, not bias |
| 2402.01788 | LitLLM: Toolkit for Scientific Literature Review | RAG-based toolkit; re-ranking by abstract similarity |
| 2402.12046 | Citation Amnesia: On The Recency Bias of NLP | 240M papers, 20 fields, 43 years; quantifies citation age recession |
| 2402.14207 | STORM: Synthesis of Topic Outlines through Retrieval and Question Asking | Wikipedia-style generation; multi-perspective question asking |
| 2405.02228 | REASONS: Attribution in Scientific Literature | Sentence-level citation annotations across 12 domains |
| 2406.10252 | AutoSurvey: LLMs Can Automatically Write Surveys | Seed paper; structured outline→retrieve→draft→refine pipeline |
| 2407.18657 | SWARM-SLR: Streamlined Workflow Automation for SLRs | 65 requirements synthesized from SLR guidelines |
| 2408.05239 | LRN: The Literature Review Network — Explainable AI for SLR | First PRISMA 2020-compliant explainable AI system |
| 2408.07884 | Instruct LLMs to Generate Surveys Step by Step | Step-by-step prompt design for survey generation |
| 2408.13450 | vitaLITy 2: Reviewing Academic Literature Using LLMs | RAG over 66k-paper corpus |
| 2409.04600 | The Emergence of LLM as a Tool in Literature Reviews | Meta-review of 3,788 articles on LLM-for-review |
| 2409.13740 | PaperQA2: Language Agents Achieve Superhuman Synthesis | Superhuman synthesis; LitQA2; contradiction detection |
| 2411.05584 | Mitigating Consequences of Prestige in Citations (Matthew Effect) | Predicting citations from pre-publication variables |
| 2411.06159 | Mixture of Knowledge Minigraph Agents for Lit Review Generation | KMCA + MPSA; prompt-based graph construction (single-agent framing) |
| 2411.14199 | OpenScholar: Synthesizing Literature with Retrieval-augmented LMs | 45M paper datastore; ScholarQABench; 8B beats GPT-4o |
| 2412.15249 | LitLLMs: LLMs for Literature Review — Are we there yet? | Two-step retrieval with planning-based generation |
| 2502.03400 | DenseReviewer: Screening Prioritisation for Systematic Reviews | Dense retrieval + active learning screening tool |
| 2502.09604 | SelfCite: Self-Supervised Alignment for Context Attribution | Context ablation reward; +5.3 F1 on LongBench-Cite |
| 2502.13668 | PeerQA: Scientific QA Dataset from Peer Reviews | 579 QA pairs from 208 papers |
| 2502.14776 | SurveyX: Academic Survey Automation via LLMs | AttributeTree pre-processing; online retrieval; re-polishing |
| 2503.04629 | SurveyForge: Outline Heuristics, Memory-Driven Generation | Scholar navigation agent; memory-guided retrieval |
| 2503.23229 | Citegeist: Automated Generation of Related Work Analysis | Dynamic RAG on arXiv; embedding similarity + multi-stage filtering |
| 2504.10861 | Ai2 Scholar QA: Organized Literature Synthesis with Attribution | Open-source scientific QA with attribution pipeline |
| 2505.16349 | XSum: Ask, Retrieve, Summarize — Modular Pipeline | Modular RAG pipeline with dynamic question generation |
| 2507.15245 | SPAR: Scholar Paper Retrieval with LLM-based Agents | RefChain-based query decomposition; SPARBench |
| 2508.11310 | SGSimEval: Comprehensive Multifaceted Similarity-Enhanced Benchmark for ASG | Three-dimension ASG evaluation benchmark |
| 2508.12735 | Citation Accuracy, Citation Noise, and Citation Bias | Defines citation noise vs bias; CoARA recommendations |
| 2508.14317 | SurveyGen-I: Evolving Plans and Memory-Guided Writing | Coarse-to-fine retrieval; adaptive planning; memory mechanism |
| 2508.15396 | Attribution, Citation, and Quotation: Survey of Evidence-based Text Generation | Unified taxonomy; 134 papers; 300 metrics |
| 2508.15658 | SurGE: Survey Generation Evaluation Benchmark | 4-dimension evaluation benchmark |
| 2508.15804 | ReportBench: Evaluating Deep Research Agents via Academic Surveys | Citation quality + faithfulness benchmark |
| 2508.17647 | SurveyGen: Quality-Aware Scientific Survey Generation | 4,200+ human surveys dataset; quality-aware RAG pipeline |
| 2509.23981 | Automatic Selection with Evolutionary Rule-Based Classification | Grammar-guided genetic programming for screening |
| 2510.03120 | SurveyBench: Can LLM(-Agents) Write Surveys Aligned with Readers? | 11,343 arXiv topics; quiz-driven evaluation |
| 2510.10890 | LLM×MapReduce-V3: MCP-Driven Hierarchically Modular Agent System | Modular MCP server architecture |
| 2510.15682 | SQuAI: Scientific QA with Multi-Agent RAG | 4-agent RAG over 2.3M arXiv papers |
| 2510.17853 | CiteGuard: Faithful Citation Attribution via RAG Validation | 65.4% on CiteME benchmark (human 69.7%) |
| 2510.20844 | AutoResearcher: Automating Knowledge-Grounded Research Ideation | Multi-stage framework for research ideation |
| 2510.26012 | AutoSurvey2: Next Level Automated Literature Surveys | Multi-stage RAG pipeline; parallel section generation |
| 2510.26750 | ProfOlaf: Semi-Automated Tool for Systematic Literature Reviews | Semi-automated SLR with iterative snowballing |
| 2512.02763 | SurveyEval: Towards Comprehensive Evaluation of LLM Surveys | 3 dimensions: quality, outline coherence, reference accuracy |
| 2601.15307 | DeepSurvey-Bench: Evaluating Academic Value of Generated Surveys | Novel "academic value" dimension |
| 2602.11238 | SurveyLens: A Research Discipline-Aware Benchmark for ASG | First discipline-aware ASG benchmark; 10 disciplines |
| 2605.29234 | Rethinking Literature Search Eval: Deep Research and Human Citation Lists | Only 51% human citations judged relevant |

### Shared Characteristics
- **Single locus of control** — One LLM or pipeline manages all stages of generation or analysis
- **Broad scope** — Covers everything from core survey systems (AutoSurvey, PaperQA) to benchmarks (SurveyBench, SurGE) to analysis papers (Citation Amnesia)
- **Retrieval via search/embedding** — Most papers use embedding similarity or keyword search, not graph traversal
- **Prevalent in evaluation work** — All benchmark/dataset papers that merely provide evaluation infrastructure (not an operational system) are placed here

### Distinguishing Features (intra-category variation)
- **Core survey systems vs benchmarks vs analysis**: Core systems (AutoSurvey, SurveyX, OpeanScholar) generate surveys; benchmarks (SurveyBench, SurGE, ReportBench) provide evaluation; analysis papers (Citation Amnesia, Forgotten Knowledge) study phenomena
- **Citation graph awareness**: Ranges from `none` (most RAG survey systems) to `bfs` (PaperQA, SciLit) to `embedding` (SPECTER)
- **Iteration**: Most are `single_pass`, but some use `multi_round` (PaperQA, IterSurvey)
- **Boundary vs core**: Some papers are adjacent SLR screening tools (DenseReviewer, CRUISE-Screening) included for comparative context

---

## Category 3: Multi-Agent Pipeline

### Definition
Papers with explicit multi-agent architectures where specialized agents with distinct roles (planner, searcher, writer, reviewer, reflector) coordinate to produce surveys or conduct literature analysis. The coordination pattern distinguishes these from single-agent pipelines: agents communicate via shared task boards, message passing, or sequential handoffs.

### Papers

| arXiv ID | Title | Category-Specific Notes |
|----------|-------|------------------------|
| 2305.08844 | RL4F: Generating Natural Language Feedback with RL for Repairing Model Outputs | Generator + RL-trained Critique with iterative refinement |
| 2309.17288 | AutoAgents: A Framework for Automatic Agent Generation | Adaptive agent generation with observer role |
| 2404.07738 | ResearchAgent: Iterative Research Idea Generation over Scientific Literature | Collaborative reviewing agents; academic graph + knowledge store |
| 2405.03256 | MARE: Multi-Agents Collaboration Framework for Requirements Engineering | 5 agents (Manager/Analyst/Reflector/Searcher/Interpreter) for RE |
| 2406.20041 | BMW Agents: Framework for Task Automation Through Multi-Agent Collaboration | Industrial multi-agent framework with flexible planning and execution |
| 2402.15235 | MACRec: A Multi-Agent Collaboration Framework for Recommendation | Product recommendation MAS (boundary context) |
| 2501.06322 | Multi-Agent Collaboration Mechanisms: A Survey of LLMs | Comprehensive survey of MAS collaboration |
| 2502.11518 | Generative Multi-Agent Collaboration in Embodied AI: A Systematic Review | Robotics-focused MAS taxonomy (boundary context) |
| 2504.08762 | InteractiveSurvey: Personalized and Interactive Survey Generation | User customisation of references mid-generation (multi-agent with user) |
| 2505.11765 | OMAC: A Broad Optimization Framework for LLM-Based Multi-Agent Collaboration | Five optimization dimensions for MAS |
| 2506.12689 | SciSage: Multi-Agent Framework for Survey Generation | 4 agents: Searcher, Writer, Reflector, Refiner; +32% citation F1 |
| 2508.04306 | MATC: Multi-Agent Taskforce Collaboration for Self-Correction of Compounding Errors | Manager + 4 executors; exploitation/exploration/experience taskforces |
| 2509.18661 | Agentic AutoSurvey: Let LLMs Survey LLMs | 4 agents: Planner, Researcher, Writer, Reviewer; 8.18/10 quality |
| 2605.29522 | DeepSurvey: Enhancing Analytical Depth and Citation Reliability | Agentic system with citation-graph expansion + multi-granularity refinement; 8.644/10 content score; 83.3% expert preference over human

### Shared Characteristics
- **Agent specialization** — Each agent has a distinct role (planning, retrieval, writing, review, reflection)
- **Coordination overhead** — Agents communicate through shared task boards, message passing, or sequential handoffs
- **Quality improvement** — Multi-agent architectures consistently outperform single-agent baselines (SciSage: +32% citation F1; Agentic AutoSurvey: 8.18 vs 4.77/10)
- **Mostly recent** — All are from 2024–2025, reflecting the rapid adoption of multi-agent designs

### Distinguishing Features (intra-category variation)
- **Survey-specific vs general MAS**: Some papers are specifically for survey generation (SciSage, Agentic AutoSurvey, MATC); others are general MAS frameworks (AutoAgents, BMW Agents, OMAC)
- **Agent count**: Ranges from 2 (RL4F) to 4–5 (SciSage, Agentic AutoSurvey, MATC) to 6+ (BMW Agents)
- **Training approach**: Most use prompting-only; RL4F uses RL training for the critique agent
- **Citation awareness**: SciSage uses BFS citation chaining; Agentic AutoSurvey uses none; MATC uses none
- **Error mitigation**: MATC specifically targets cascading errors; RL4F targets output quality

---

## Category 4: Hybrid Interactive

### Definition
Systems that embed human interaction as a core part of the survey generation or literature analysis workflow. The human provides oversight, annotations, relevance judgments, or mid-generation feedback that directly shapes the output. This category includes human-in-the-loop (HITL) systems, interactive visualization tools, and conversational agents where the human guides the process.

### Papers

| arXiv ID | Title | Category-Specific Notes |
|----------|-------|------------------------|
| 1705.05420 | FAST²: Intelligent Assistant for Finding Relevant Papers | Self-correcting classifier with human feedback |
| 1904.02357 | Plan, Write, and Revise: An Interactive System for Open-Domain Story Generation | Human collaboration for story generation (transferable methodology) |
| 1909.07249 | Assessing Expert System-Assisted Literature Reviews | Active learning tool; 90% recall with 6% effort |
| 2006.12166 | ASReview: Open Source Software for Efficient Reviews | Active learning screening with user relevance labeling |
| 2103.04044 | Putting Humans in the NLP Loop: A Survey | Comprehensive HITL NLP survey |
| 2110.12490 | Paperfetcher: A tool to automate handsearch for systematic reviews | Automates handsearch + snowballing with human oversight |
| 2204.03685 | R3: Read, Revise, Repeat — System for Human-in-the-loop Iterative Text Revision | Iterative human-machine revision with accept/reject |
| 2208.06133 | Scholastic: Graphical Human-AI Collaboration for Inductive and Interpretive Text Analysis | Machine-in-the-loop clustering with interactive visualization |
| 2407.18940 | LitSearch: Retrieval Benchmark for Scientific Literature | 597 ML/NLP queries; 24.8% gap between BM25 and dense retriever |
| 2504.04193 | AiReview: Open Platform for Accelerating Systematic Reviews with LLMs | LLM-assisted screening platform (human-in-the-loop design) |
| 2504.14822 | InsightAgent: Completing A Systematic Review in Hours Instead of Months | 6-agent system with human orchestrator; 27.2% quality improvement |
| 2505.23789 | LitChat: Conversational Exploration of Literature Landscape | Conversational literature agent with KG construction |

### Shared Characteristics
- **Human in the loop** — Human provides relevance judgments, feedback, or oversight at key stages
- **Active learning** — Many use active learning to minimize human labeling effort while maximizing recall
- **Efficiency goal** — Dramatic time reduction from months to hours (InsightAgent: 1.5h vs months; FAST²: 53h → 3h)
- **Screening/retrieval focus** — Most focus on efficient screening and retrieval rather than survey synthesis

### Distinguishing Features (intra-category variation)
- **Human role**: From orchestrator (InsightAgent) to labeler (ASReview) to conversational partner (LitChat)
- **Automation level**: Some require active human participation throughout (InsightAgent); others are one-time labeling (ASReview)
- **Survey generation vs screening**: Only InteractiveSurvey directly generates surveys; others are screening/retrieval tools
- **Multi-agent vs single-agent**: InsightAgent and MATC are multi-agent; ASReview and R3 are single-agent with human loop

---

## Category 5: Iterative Refinement

### Definition
Papers where iterative improvement — through reinforcement learning, self-evaluation, or procedural refinement — is the core mechanism. The system generates an initial output, evaluates it, and revises based on feedback signals. This category includes RL-based optimization (RLHF, DPO, RL from feedback), iterative survey generation, and adaptive questioning strategies.

### Papers

| arXiv ID | Title | Category-Specific Notes |
|----------|-------|------------------------|
| 1707.07402 | RL for Bandit NMT with Simulated Human Feedback | Early RLHF for text generation; foundational but tangential |
| 2008.06036 | RL with Trajectory Feedback | Weak feedback with trajectory-level scores |
| 1707.07402 | RL for Bandit NMT with Simulated Human Feedback | Early RLHF for text generation; foundational but tangential |
| 2008.06036 | RL with Trajectory Feedback | Weak feedback with trajectory-level scores |
| 2403.01304 | Improving Validity of Automatically Generated Feedback via RL | DPO for correctness and alignment of generated feedback |
| 2411.11761 | Mapping out the Space of Human Feedback for RL: A Conceptual Framework | 9-dimension taxonomy + 7 quality metrics for feedback |
| 2501.10120 | PaSa: LLM Agent for Comprehensive Academic Paper Search | RL + synthetic data; +37.78% recall@20 over GPT-4o |
| 2504.12501 | Reinforcement Learning from Human Feedback (Lambert Book) | Comprehensive RLHF monograph |
| 2505.22338 | Text2Grad: RL from Natural Language Feedback | Span-level gradients from textual feedback |
| 2507.04730 | CueLearner: Bootstrapping and Policy Adaptation from Relative Feedback | Relative feedback learning (boundary context) |
| 2510.21900 | IterSurvey: Deep Survey Automation with Iterative Workflow | Recurrent outline generation; iterative refinement with self-evaluation |
| 2510.27126 | AURA: RL Framework for AI-Driven Adaptive Conversational Surveys | RL (epsilon-greedy) for adaptive surveys; LSDE metric |
| 2604.02507 | RLHF: A Statistical Perspective | Statistical RLHF survey; reward modelling, DPO, RLAIF |

### Shared Characteristics
- **Feedback-driven improvement** — All systems use some form of feedback (RL reward, self-evaluation, human critique) to iteratively improve
- **Not primarily architectural** — The focus is on the learning/refinement mechanism rather than the pipeline architecture
- **Cross-cutting** — Many papers classified here could also fit multi_agent (RL4F) or single_agent (IterSurvey) categories; the refinement strategy is the defining characteristic

### Distinguishing Features (intra-category variation)
- **RL vs procedural**: Some use formal RL (PaSa, AURA); others use procedural self-evaluation (IterSurvey)
- **Survey-specific vs general**: AURA is survey-specific; Text2Grad and RL4F are general NLG methods
- **Training vs inference**: Some require training (PaSa, Text2Grad); others work at inference time (IterSurvey)
- **Tangential boundary**: RLHF references (Lambert book, RLHF statistical perspective) are context papers

---

## Category 6: Benchmark Evaluation

### Definition
Papers that contribute evaluation benchmarks, datasets, protocols, or rubrics for automated survey generation or literature QA. These papers do not propose novel survey-generation architectures; their primary contribution is the evaluation infrastructure — benchmarks, metrics, human evaluation protocols, or annotated datasets. This category includes survey evaluation benchmarks (SurveyBench, SurGE, ReportBench, DeepSurvey-Bench, SurveyLens, SGSimEval), literature QA datasets (LitQA, ScholarQABench), and annotation resources (QASPER, REASONS, PeerQA).

### Papers

| arXiv ID | Title | Category-Specific Notes |
|----------|-------|------------------------|
| 2105.03011 | QASPER: Information-Seeking QA Anchored in Research Papers | 5,049 questions over 1,585 NLP papers (benchmark) |
| 2405.02228 | REASONS: Attribution in Scientific Literature | Sentence-level citation annotations across 12 domains |
| 2411.14199 | OpenScholar: Synthesizing Literature with Retrieval-augmented LMs | 45M paper datastore; ScholarQABench (core system + benchmark) |
| 2502.13668 | PeerQA: Scientific QA Dataset from Peer Reviews | 579 QA pairs from 208 papers |
| 2508.11310 | SGSimEval: Comprehensive Multifaceted Similarity-Enhanced Benchmark for ASG | Three-dimension ASG evaluation benchmark |
| 2508.15658 | SurGE: Survey Generation Evaluation Benchmark | 4-dimension evaluation benchmark |
| 2508.15804 | ReportBench: Evaluating Deep Research Agents via Academic Surveys | Citation quality + faithfulness benchmark |
| 2510.03120 | SurveyBench: Can LLM(-Agents) Write Surveys Aligned with Readers? | 11,343 arXiv topics; quiz-driven evaluation |
| 2512.02763 | SurveyEval: Towards Comprehensive Evaluation of LLM Surveys | 3 dimensions: quality, outline coherence, reference accuracy |
| 2601.15307 | DeepSurvey-Bench: Evaluating Academic Value of Generated Surveys | Novel "academic value" dimension |
| 2602.11238 | SurveyLens: A Research Discipline-Aware Benchmark for ASG | First discipline-aware ASG benchmark; 10 disciplines |
| 2605.29234 | Rethinking Literature Search Eval: Deep Research and Human Citation Lists | Only 51% human citations judged relevant |
| 2512.20854 | How important is Recall for Measuring Retrieval Quality? | Correlation analysis between retrieval metrics and LLM-judged generation quality; recall-free metric proposal |

### Shared Characteristics

- **Evaluation infrastructure is the primary contribution** — Not a survey-generation system but a way to measure one
- **Metric design** — Each benchmark proposes novel metrics or evaluation dimensions
- **Varied task definitions** — Some measure factual QA (LitQA), others survey quality (SurGE, SurveyBench), others citation accuracy (ReportBench)
- **Annotated datasets** — Many include human-curated question-answer pairs or quality ratings

### Distinguishing Features (intra-category variation)
- **QA-focused vs survey-focused**: LitQA/ScholarQABench test factual retrieval and synthesis; SurveyBench/SurGE test holistic survey quality
- **Automatic vs human evaluation**: Some are fully automatic (quiz-driven SurveyBench); others require human annotation (SurGE)
- **Discipline scope**: Most are CS/AI-only; SurveyLens spans 10 disciplines; DeepSurvey-Bench adds "academic value"
- **Metric count**: Range from single-metric (LitQA: accuracy) to multi-dimensional (SurGE: 4 dimensions)

---

No papers are placed in this category. All 137 profiles have been assigned to one of the six primary categories above based on their Method Category field and architectural analysis.

---

## Cross-Category Comparative Matrix

### Secondary Dimension 1: Citation Graph Awareness

| Awareness Level | Graph-Enhanced Retrieval | Single-Agent Pipeline | Multi-Agent Pipeline | Hybrid Interactive | Iterative Refinement | Benchmark Evaluation |
|----------------|------------------------|----------------------|--------------------|--------------------|--------------------|--------------------|
| **gnn** | 2409.12177, 2305.01572, 2408.15371 | — | — | — | — | — |
| **hierarchical** | 2510.07733, 2410.03761, 1805.02262 | — | — | — | — | — |
| **bfs** | 1205.1143, 2101.07609, 2403.09295 | 2312.07559, 2409.13740, 2506.12689 | 2506.12689, 2504.14822 | 2504.14822 | — | — |
| **embedding** | 2004.07180, 2409.12177, 1812.03835 | 2408.13450, 2411.14199 | — | — | — | — |
| **none** | — | Most RAG survey systems (AutoSurvey, SurveyX, STORM) | Agentic AutoSurvey, MATC | ASReview, R3 | AURA, IterSurvey | All benchmarks (evaluation-only infrastructure)

### Secondary Dimension 2: Iteration Strategy

| Strategy | Graph-Enhanced Retrieval | Single-Agent Pipeline | Multi-Agent Pipeline | Hybrid Interactive | Iterative Refinement | Benchmark Evaluation |
|----------|------------------------|----------------------|--------------------|--------------------|--------------------|--------------------|
| **single_pass** | Most (static methods) | Most core systems, analysis papers | — | — | — | All benchmarks |
| **multi_round** | — | PaperQA, OpenScholar, SurveyGen | SciSage, MATC, Agentic AutoSurvey | — | IterSurvey | — |
| **interactive** | — | — | InsightAgent | All HITL systems | AURA | — |

---

## Rationale for Category Boundaries

### graph_enhanced_retrieval vs single_agent_pipeline
Papers where **citation graph structure** is the primary computational mechanism belong in graph_enhanced_retrieval even if they use a single-agent pipeline for generation. Example: SurveyG (2510.07733) has a single-agent pipeline but its core innovation is the hierarchical citation graph traversal — the graph structure drives everything.

### single_agent_pipeline vs multi_agent_pipeline
The boundary is **agent count and specialization**. If a single LLM handles all stages (even if there are multiple stages), it's single_agent. If multiple specialized agents with distinct roles coordinate, it's multi_agent. Example: Agentic AutoSurvey (2509.18661) has 4 specialized agents → multi_agent; AutoSurvey (2406.10252) has one LLM for the whole pipeline → single_agent.

### single_agent_pipeline vs benchmark_evaluation
The boundary is **whether the paper contributes an operational system or an evaluation resource**. Papers that describe a survey-generation pipeline (AutoSurvey, SurveyX, PaperQA) belong in single_agent_pipeline. Papers whose primary contribution is a benchmark dataset, evaluation protocol, or metric (SurveyBench, SurGE, ReportBench, SurveyLens) belong in benchmark_evaluation even if they use a simple pipeline to demonstrate the benchmark. The distinction is intentional rather than architectural: a benchmark paper may use a single-agent pipeline internally, but the contribution to the field is the measurement tool, not the system.

### hybrid_interactive vs single_agent_pipeline
The boundary is **whether human interaction is a required, integral part of the workflow**. If the system requires human labels, judgments, or mid-generation feedback to function, it's hybrid_interactive. If a human could optionally review the output but isn't needed for operation, it's single_agent.

### iterative_refinement vs multi_agent_pipeline
Some papers could fit both categories. The tiebreaker is: if the **learning/refinement mechanism** is the defining contribution, place in iterative_refinement. If the **multi-agent coordination architecture** is the primary innovation, place in multi_agent_pipeline. Example: RL4F (2305.08844) uses 2 agents but the key contribution is RL-trained critique → placed in multi_agent_pipeline by its profile.
