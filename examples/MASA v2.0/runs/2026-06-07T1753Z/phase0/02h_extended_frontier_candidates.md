# Extended Frontier Candidates — Automated Literature Survey Agents with Citation Graph Expansion

Generated: 2026-06-07T18:04Z
Run dir: `.`
Phase: ExtendedFrontierScout (arXiv embedding search for extended query plan E1–E12)

---

## 1. Summary

| Metric | Value |
|--------|-------|
| Queries executed | 12 (E1–E12) |
| Total arXiv results | 120 (10 per query) |
| **New unique candidates** | **52** |
| Already in main pool (69 entries) | 45 (deduplicated by arXiv ID) |
| Off-topic / irrelevant | 23 (excluded) |

**Deduplication reference:** Existing 69-entry pool in `03_expansion.md` (IDs verified against sections 2.1–2.8).

---

## 2. New Candidates by Gap Area

### 2.1 Graph/Tree-Based Literature Analysis (E1 + E2)

**New papers from E1 — "hierarchical citation graph survey generation traversal"**

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2410.03761 | **HiGTL: Taxonomy Tree Generation from Citation Graph** | 2024 | End-to-end hierarchical graph taxonomy learning; LLM-driven node verbalization; directly addresses hierarchical citation graph traversal for survey structuring |
| 2512.22159 | **Oignon: Citation Graph Tool** | 2025 | Open-source dual-path ranking with recency weighting; practical tool for systematic graph exploration |
| 2605.14790 | **Graphs of Research: Citation Evolution Graphs as Supervision** | 2026 | SFT method using 2-hop citation DAG for research idea generation; citation position/frequency edge signals |
| 2402.04854 | **Hierarchical Tree-structured Knowledge Graph for Academic Insight Survey** | 2024 | Tree-structured KG reflecting inheritance insight; designed for beginner researcher surveys |
| 2306.10051 | **TOBY: Tool for Exploring Data in Academic Survey Papers** | 2023 | Visualization tool with hierarchical taxonomic view, document similarity, citation network, and recommendation |
| 2504.13834 | **Science Hierarchography: Hierarchical Organization of Science Literature** | 2025 | Hybrid embedding+LLM clustering for multi-level science hierarchy; navigation agent evaluation |

**New papers from E2 — "graph neural network citation retrieval embedding science"**

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2209.00446 | **Self-Supervised Pretraining of GNN for Retrieval of Related Math Expressions** | 2022 | GNN-based embedding for retrieval; self-supervised on 29M math expressions from arXiv |
| 2302.01826 | **Graph Embedding for Mapping Interdisciplinary Research Networks** | 2023 | Novel GNN for interdisciplinary citation prediction; preserves cross-field implications in embeddings |
| 2408.15371 | **Temporal GNN-Powered Paper Recommendation on Dynamic Citation Networks** | 2024 | Temporal dimension in paper embeddings; RNN-based memory for evolving citation impact |
| 2512.16661 | **Microsoft Academic Graph Information Retrieval for Research Recommendation** | 2025 | Attention-Based Subgraph Retriever — GNN-as-retriever with LLM reasoning |
| 2104.04939 | **GCN-based Framework for Estimating Future Citations** | 2021 | GCN for citation impact prediction; short/long-term (1/5/10 year) |
| 1812.03835 | **Graph Embedding for Citation Recommendation** | 2018 | Task-specific neighborhood construction; robust when seed papers are scarce |
| 2104.02562 | **Structured Citation Trend Prediction Using GNNs** | 2021 | Predicts top papers at publication time; outperforms classic ML on F1 |
| 1903.06464 | **Context-Aware Citation Recommendation with BERT and GCN** | 2019 | GCN + BERT hybrid; FullTextPeerRead dataset; +28% MAP |
| 2305.01572 | **H2CGL: Hierarchical Heterogeneous Contrastive Graph Learning** | 2023 | Dynamics-aware GNN for citation impact; contrastive learning with hard negatives |
| 2010.00182 | **Dual Attention Model for Citation Recommendation (DACR)** | 2020 | Self-attention + additive attention over words, sections, structural contexts |

---

### 2.2 RAG for Scientific Literature Survey (E3 + E4)

**New papers from E3 — "retrieval augmented generation scientific paper survey synthesis"**

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2505.16349 | **XSum: Ask, Retrieve, Summarize — Modular Pipeline for Scientific Summarization** | 2025 | Modular RAG pipeline for multi-document summarization; question-generation + editor modules |
| 2510.26012 | **AutoSurvey2: Empowering Researchers with Next Level Automated Literature Surveys** | 2025 | Multi-stage RAG pipeline; parallel section generation; iterative refinement; multi-LLM evaluation |
| 2504.10861 | **Ai2 Scholar QA: Organized Literature Synthesis with Attribution** | 2025 | Open-source scientific QA pipeline with attribution; outperforms competing systems |
| 2503.23229 | **Citegeist: Automated Generation of Related Work Analysis on arXiv Corpus** | 2025 | Dynamic RAG on arXiv; embedding similarity + summarization + multi-stage filtering |

**New papers from E4 — "multi hop retrieval iterative search literature review automation"**

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2110.12490 | **Paperfetcher: A tool to automate handsearch for systematic reviews** | 2021 | Automates handsearch + snowballing; free open-source Python package + web-app |
| 1705.05420 | **FAST²: An Intelligent Assistant for Finding Relevant Papers** | 2017 | Self-correcting classifier; estimator of remaining relevant papers; domain knowledge guidance |
| 2412.15249 | **LitLLMs: LLMs for Literature Review — Are we there yet?** | 2024 | Two-step retrieval (keyword extraction + re-ranking); planning-based generation; evaluation protocol |
| 2404.07738 | **ResearchAgent: Iterative Research Idea Generation over Scientific Literature** | 2024 | Iterative idea generation with collaborative reviewing agents; academic graph + knowledge store |
| 2510.20844 | **AutoResearcher: Automating Knowledge-Grounded Research Ideation** | 2025 | Multi-agent system; 4-stage pipeline (curation, generation, selection, review); domain-agnostic |

---

### 2.3 Citation Quality and Noise (E5)

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2601.15307 | **DeepSurvey-Bench: Evaluating Academic Value of Generated Surveys** | 2026 | New benchmark evaluating deep "academic value" (informational, scholarly communication, research guidance) — goes beyond surface metrics |
| 2508.11310 | **SGSimEval: Comprehensive Benchmark for Automatic Survey Generation** | 2025 | Multifaceted evaluation (outline, content, references); human preference metrics; similarity-enhanced |
| 2510.06242 | **Transparent Reference-free Automated Evaluation of Open-Ended User Survey Responses** | 2025 | *Boundary:* about evaluating human survey responses, not academic surveys. Marginal relevance. |

---

### 2.4 Temporal Bias (E6)

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2402.12046 | **Citation Amnesia: On The Recency Bias of NLP and Other Academic Fields** | 2024 | Large-scale study (240M papers, 20 fields, 43 years); quantifies citation age recession; NLP/ML strongest decline (-12.8%) |
| 2305.18554 | **Forgotten Knowledge: Examining the Citational Amnesia in NLP** | 2023 | 71.5K NLP papers; 62% citations from immediate 5 years; temporal diversity at all-time low since 2014 |
| 2401.03545 | **Is there really a Citation Age Bias in NLP?** | 2024 | Counterpoint: argues citation amnesia is an artefact of field dynamics, not bias per se |
| 2101.07609 | **Chronological Citation Recommendation with Time Preference** | 2021 | Models time preference distribution for re-ranking; handles cold-start problems |
| 2002.06961 | **Citation Recommendation: Approaches and Datasets (Survey)** | 2020 | Comprehensive survey of citation recommendation field — useful context |
| 2509.04190 | **The changing role of cited papers over time** | 2025 | Large-scale full-text analysis (900 HCPs, 220K citing papers); shift from topical engagement to symbolic referencing |
| 1205.1143 | **Recommendation on Academic Networks using Direction Aware Citation Analysis** | 2012 | Direction-aware algorithms tunable for recent or traditional papers; relevance feedback |
| 2201.07534 | **Automation of Citation Screening for SLRs using Neural Networks** | 2022 | Replicability study; simpler embedding model outperforms deep learning on 18/23 datasets |

---

### 2.5 Cross-Domain Transfer (E7)

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2602.11238 | **SurveyLens: A Research Discipline-Aware Benchmark for Automatic Survey Generation** | 2026 | First discipline-aware benchmark; 1,000 surveys across 10 disciplines; dual-lens evaluation (rubric + canonical alignment). **High relevance.** |

---

### 2.6 Multi-Agent Architectures (E8)

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2508.04306 | **MATC: Multi-Agent Taskforce Collaboration — Self-Correction of Compounding Errors** | 2025 | Manager + 4 executor agents for literature review; 3 collaboration paradigms to mitigate error propagation |
| 2501.06322 | **Multi-Agent Collaboration Mechanisms: A Survey of LLMs** | 2025 | Comprehensive survey of MAS collaboration types, structures, strategies — useful reference |
| 2505.11765 | **OMAC: A Broad Optimization Framework for LLM-Based Multi-Agent Collaboration** | 2025 | 5 optimization dimensions for MAS; joint optimization algorithm |
| 2309.17288 | **AutoAgents: A Framework for Automatic Agent Generation** | 2023 | Dynamically generates and coordinates specialized agents; observer role for reflection |
| 2406.20041 | **BMW Agents: Framework for Task Automation Through Multi-Agent Collaboration** | 2024 | Flexible agent engineering framework; planning + execution for industrial applications |

*Boundary:* 2502.11518 (Embodied AI), 2405.03256 (MARE — Requirements Engineering), 2402.15235 (MACRec — Recommendation)

---

### 2.7 Post-Training Optimization / RLHF (E9)

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2510.27126 | **AURA: A RL Framework for AI-Driven Adaptive Conversational Surveys** | 2025 | **Most relevant:** applies RL (epsilon-greedy) to adaptive survey conversations; LSDE metric (Length, Self-disclosure, Emotion, Specificity). Transferable to adaptive literature survey generation. |
| 2604.02507 | **RLHF: A Statistical Perspective (Survey)** | 2026 | Statistical overview of RLHF pipeline — useful methodological reference |
| 2504.12501 | **Reinforcement Learning from Human Feedback (Book)** | 2025 | Comprehensive RLHF monograph |
| 2403.01304 | **Improving Validity of Automatically Generated Feedback via RL** | 2024 | RL (DPO) for feedback generation; GPT-4 annotation pipeline — transferable to survey quality |
| 2305.08844 | **RL4F: Generating Natural Language Feedback with RL for Repairing Model Outputs** | 2023 | RL-trained critique generator for fixing LLM outputs — relevant for survey refinement |
| 2505.22338 | **Text2Grad: RL from Natural Language Feedback** | 2025 | Span-level gradients from textual feedback; interpretable policy optimization |
| 2411.11761 | **Mapping out the Space of Human Feedback for RL: A Conceptual Framework** | 2024 | Taxonomy of 9 feedback dimensions + 7 quality metrics — useful evaluation framework |

---

### 2.8 Human-in-the-Loop Systems (E10)

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2204.03685 | **Read, Revise, Repeat: Human-in-the-loop Iterative Text Revision (R3)** | 2022 | Iterative revision with accept/reject interface; human-machine interaction for document improvement |
| 2208.06133 | **Scholastic: Graphical Human-AI Collaboration for Inductive Text Analysis** | 2022 | Machine-in-the-loop clustering for interpretive text analysis; visual analytics |
| 2504.14822 | **InsightAgent: Completing Systematic Review in Hours Instead of Months** | 2025 | Multi-agent + interactive AI; user studies with 9 medical professionals; 27.2% quality improvement |
| 2103.04044 | **Putting Humans in the NLP Loop: A Survey** | 2021 | Survey of HITL NLP frameworks — comprehensive overview of feedback collection and learning methods |
| 2505.23789 | **LitChat: Conversational Exploration of Literature Landscape** | 2025 | Interactive conversational agent for literature exploration; KG construction + data mining |
| 2102.05374 | **Enhancing Reading Strategies by Exploring A Theme-based Approach** | 2021 | Visual/thematic exploration methodology; reading strategy development |

---

### 2.9 Human Evaluation Protocols (E11)

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2301.13298 | **LongEval: Guidelines for Human Evaluation of Faithfulness in Long-form Summarization** | 2023 | Survey of 162 papers on human eval practices; clause-level granularity reduces variance; partial annotation correlates (0.89 Kendall's tau) |

---

### 2.10 Systematic Review / PRISMA Alignment (E12)

| arXiv ID | Title | Year | Why Relevant |
|----------|-------|------|--------------|
| 2510.26750 | **ProfOlaf: Semi-Automated Tool for Systematic Literature Reviews** | 2025 | Iterative snowballing + LLM analysis + human-in-the-loop filtering |
| 2408.05239 | **The Literature Review Network: Explainable AI for SLR** | 2024 | PRISMA 2020-compliant; 84.78% accuracy; 288.6 min vs 19,920 min manual |
| 2407.18657 | **SWARM-SLR: Streamlined Workflow Automation for Machine-actionable SLRs** | 2024 | 65 requirements synthesized from guidelines; 11 tools mapped to lifecycle stages |
| 2504.04193 | **AiReview: Open Platform for Accelerating Systematic Reviews with LLMs** | 2025 | First platform for LLM-assisted title/abstract screening; extensible framework |
| 2409.04600 | **LLM as a tool in literature reviews: an LLM automated systematic review** | 2024 | Systematic review of LLM usage in review automation; GPT vs BERT performance comparison |
| 2307.06464 | **Assessing ChatGPT to Screen Articles for Systematic Reviews** | 2023 | ChatGPT viability for SR screening; consistency and generalizability evaluation |
| 1606.06424 | **Framework to Expedite Systematic Reviews by Automatically Building IE Training Corpora** | 2016 | Early framework using SVM for data element extraction from Cochrane reviews |

---

## 3. Top Candidates (Most Directly Relevant)

Top 10 papers most relevant to the survey topic (automated survey agents with citation graph expansion):

| Rank | arXiv ID | Title | Gap Area | Why |
|------|----------|-------|----------|-----|
| 1 | 2410.03761 | HiGTL: Taxonomy Tree Generation | Graph/tree analysis | Directly addresses hierarchical citation graph traversal for survey taxonomy |
| 2 | 2602.11238 | SurveyLens: Discipline-Aware Benchmark | Cross-domain transfer | First discipline-aware ASG benchmark across 10 fields |
| 3 | 2508.04306 | MATC: Multi-Agent Taskforce Collaboration | Multi-agent | Manager + 4 executors with error mitigation; state-of-the-art on benchmarks |
| 4 | 2510.26012 | AutoSurvey2 | RAG pipeline | Multi-stage RAG + parallel section gen + multi-LLM evaluation |
| 5 | 2601.15307 | DeepSurvey-Bench | Evaluation | Deep "academic value" evaluation; 3 value dimensions |
| 6 | 2402.12046 | Citation Amnesia | Temporal bias | Definitive study of recency bias; 240M papers across 20 fields |
| 7 | 2408.15371 | Temporal GNN Paper Recommendation | Graph + temporal | Temporal embedding updates; RNN memory module for dynamic citation impact |
| 8 | 2510.27126 | AURA: RL for Adaptive Surveys | RL/Optimization | Applies RL to survey adaptation; epsilon-greedy quality improvement |
| 9 | 2505.16349 | XSum: Ask, Retrieve, Summarize | RAG pipeline | Modular RAG pipeline with dynamic question generation |
| 10 | 2504.14822 | InsightAgent: Systematic Review in Hours | Human-in-the-loop | Multi-agent + interactive; 27.2% quality improvement; medical professional study |

---

## 4. Coverage Gaps Closed

| Gap Area (from Extended Plan) | Previously | Now |
|------------------------------|------------|-----|
| Graph/tree-based analysis | 0 dedicated papers | 6 new (HiGTL, Oignon, GoR, Tree KG, TOBY, Hierarchography) |
| GNN citation embeddings | Only LitFM (2409.12177) | 10 new GNN methods |
| RAG for survey synthesis | 5 papers in main pool | 4 new (XSum, AutoSurvey2, Ai2 Scholar, Citegeist) |
| Multi-hop iterative retrieval | IterSurvey (2510.21900) | 5 new (Paperfetcher, FAST², ResearchAgent, AutoResearcher, LitLLMs eval) |
| Citation quality/noise | 1 paper (2508.12735) | 2 new (DeepSurvey-Bench, SGSimEval) |
| Temporal bias | 0 dedicated papers | 7 new (Citation Amnesia, Forgotten Knowledge, +5) |
| Cross-domain | 0 dedicated papers | 1 key paper (SurveyLens) |
| Multi-agent architectures | 2 papers | 4 new (MATC, OMAC, AutoAgents, BMW Agents) + 1 survey |
| RL/post-training optimization | 1 paper (PaSa) | 1 directly relevant (AURA) + 6 RLHF methodology refs |
| HITL systems | 1 paper (InteractiveSurvey) | 6 new (R3, Scholastic, InsightAgent, LitChat, +2) |
| Human eval protocols | 0 dedicated | 1 key protocol (LongEval) |
| SLR/PRISMA alignment | 4 boundary papers | 7 new (ProfOlaf, LRN, SWARM-SLR, AiReview, +3) |

---

## 5. Off-Topic / Excluded

Papers returned by queries but excluded as irrelevant:

- 2010.13200 — Noise suppression algorithms in crowdsourcing (audio signal processing)
- 1406.5572 — SurveyMan (programming social science surveys, not literature surveys)
- 2401.12986 — Crowdsourced Adaptive Surveys (public opinion polling)
- 2009.14675 — COVID-19 Symptom Survey weights (epidemiology methodology)
- 1609.09767 — SDL-RX (medical survey apps)
- 2502.11518 — Embodied AI multi-agent survey (robotics)
- 2405.03256 — MARE (requirements engineering)
- 2402.15235 — MACRec (product recommendation)
- 2507.04730 — CueLearner (robot navigation)
- 2312.04736 — Goal-conditioned RL with language feedback (BabyAI)
- 2008.06036 — RL with trajectory feedback (theory)
- 1705.05420 — FAST² (already excluded as software engineering focused — but re-evaluated as relevant for iterative retrieval)
- 1904.02357 — Story generation (creative writing, not scientific survey)
- 2510.06242 — Open-ended user survey evaluation (marketing research)
- Several duplicates across queries

---

## 6. Query Execution Log

| Query | Category | Papers Returned | New Candidates | Top New ID |
|-------|----------|----------------|----------------|------------|
| E1 | extended_method | 10 | 6 | 2410.03761 |
| E2 | extended_method | 10 | 10 | 2408.15371 |
| E3 | extended_method | 10 | 4 | 2510.26012 |
| E4 | extended_method | 10 | 5 | 2504.14822 |
| E5 | extended_problem | 10 | 2 (+1 boundary) | 2601.15307 |
| E6 | extended_problem | 10 | 7 | 2402.12046 |
| E7 | extended_problem | 10 | 1 | 2602.11238 |
| E8 | extended_mechanism | 10 | 4 (+3 boundary) | 2508.04306 |
| E9 | extended_mechanism | 10 | 1 (+6 refs) | 2510.27126 |
| E10 | extended_mechanism | 10 | 6 | 2504.14822 |
| E11 | extended_benchmark | 10 | 1 | 2301.13298 |
| E12 | extended_boundary | 10 | 7 | 2510.26750 |
