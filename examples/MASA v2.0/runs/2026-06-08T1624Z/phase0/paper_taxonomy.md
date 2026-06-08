# Paper Taxonomy — Automated Survey Generation

**Generated**: 2026-06-08
**Agent**: TaxonomyBuilder
**Input**: phase0/paper_profiles/ — 35 core profiles + 02_candidate_pool.md
**Status**: complete

---

## Classification Dimensions

| Dimension | Possible Values | Criteria |
|-----------|---------------|---------|
| **agent_count** | 1, 2–5, 5+ | Number of distinct LLM agents in the system architecture |
| **graph_awareness_level** | none, citation_chaining, hierarchical_graph, bfs_graph | Whether and how the system uses citation graph structure for literature discovery |
| **writing_paradigm** | outline_driven, attribute_tree, parallel_sections, recurrent_outline, adaptive_planning, graph_structured, quality_driven, reflect_while_writing, plan_write_review, memory_guided_outline, step_by_step | How the survey content is organized and generated |
| **iteration_type** | none, single_pass, post_hoc_refinement, self_evaluation_loop, coarse_to_fine, multi_llm_evaluation, real_time_feedback, quality_estimation_feedback, memory_driven, interactive | The strategy for refining and improving the survey |
| **agent_specialization** | low, medium, high | Degree of role differentiation among agents |
| **human_in_loop** | none, semi_automatic, interactive | Whether human oversight is integrated into the generation process |
| **retrieval_method** | embedding, keyword, hybrid, graph_traversal | Primary mechanism for locating relevant literature |

---

## Category: Multi-Agent Pipeline

### Definition
Systems that use multiple specialized LLM agents with distinct roles (planner, writer, reviewer, searcher, etc.) that coordinate through structured communication to produce surveys. The division of labor enables parallel work, specialized expertise, and quality feedback loops.

### Papers

| arXiv ID | Paper | Graph Awareness | Writing Paradigm | Agent Count | Key Distinction |
|----------|-------|----------------|-----------------|-------------|-----------------|
| 2506.12689 | SciSage | citation_chaining | reflect_while_writing | 4 | Real-time reflection during writing; +32% citation F1 |
| 2509.18661 | Agentic AutoSurvey | none | plan_write_review | 4 | Shared task board coordination; 8.18/10 quality score |
| 2510.26012 | AutoSurvey2 | none | parallel_sections | 5+ | Parallel section generation; multi-LLM evaluation |
| 2508.04306 | MATC | bfs | manager_taskforce | 5 | Error-mitigation architecture with 4 specialized taskforces |
| 2411.06159 | KMCA / MPSA | bfs | mixture_of_experts | Configurable | Minigraph subgraph decomposition; multi-perspective synthesis |
| 2510.10890 | LLM×MapReduce-V3 | none | hierarchical_mapreduce | Configurable | MCP-driven hierarchical modular system; human-in-the-loop |
| 2510.05138 | LiRA | none | multi_agent_review | Multi | Multi-agent framework for reliable literature review |
| 2404.17017 | AutoGenesisAgent | none | self_generating_mas | Variable | Self-generating multi-agent systems for complex tasks |
| 2403.08399 | Multi-Agent SLR System | none | slr_pipeline | Multi | Multiple AI agents for systematic literature review |
| 2504.14822 | InsightAgent | bfs | human_centered_mas | 6 | Human orchestrator + 5 specialized agents; 27.2% quality improvement |
| 2510.15624 | Build Your Research Group | none | personalized_mas | Multi | Personalized multi-agent research group framework |

### Shared Characteristics
- All use **2+ agents**, typically 4–6, with distinct role definitions
- All employ **iterative or multi-round generation** rather than single-pass
- All separate **retrieval/search from writing** into distinct agent roles
- None achieve **graph_awareness_level=hierarchical** (except InsightAgent which uses bfs for citation tracking)

### Distinguishing Features (intra-category variation)
- **Coordination pattern**: Shared task board (Agentic AutoSurvey) vs. hierarchical manager (MATC, LLM×MapReduce-V3) vs. real-time feedback loop (SciSage) vs. mixture-of-experts (KMCA) vs. human-centered (InsightAgent)
- **Graph awareness**: Most use none; MATC, InsightAgent, and KMCA use bfs citation chaining
- **Parallelism**: AutoSurvey2 and KMCA generate sections in parallel; others generate sequentially
- **Error handling**: MATC has explicit error-mitigation taskforces; SciSage uses real-time reflection; others rely on post-hoc review
- **Human involvement**: InsightAgent has human orchestrator; most others are fully automated

---

## Category: Single-Agent Pipeline

### Definition
Systems that use a single LLM (or a single LLM instance) orchestrating a sequential pipeline of stages — planning, retrieval, writing, and refinement — without multi-agent coordination. Despite the multi-stage pipeline, a single model handles all cognitive tasks.

### Papers

| arXiv ID | Paper | Graph Awareness | Writing Paradigm | Retrieval | Iteration | Key Distinction |
|----------|-------|----------------|-----------------|-----------|-----------|-----------------|
| 2406.10252 | AutoSurvey | none | outline_driven | embedding | post_hoc_refinement | Foundational seed paper; Outline → Retrieve → Draft → Refine |
| 2502.14776 | SurveyX | none | attribute_tree | hybrid | none | AttributeTree pre-processing for structured survey planning |
| 2508.17647 | SurveyGen | none | quality_driven | hybrid | quality_estimation | Quality-aware retrieval; trained on 4,200+ human surveys |
| 2503.04629 | SurveyForge | citation_chaining | memory_guided_outline | hybrid | memory_driven | Scholar Navigation Agent with citation-following memory |
| 2408.07884 | Step-by-Step Survey | none | step_by_step | keyword | none | Title → Abstract → Headings → Content prompt chain |
| 2302.04580 | BigSurvey / CAST | none | structured_summary | embedding | none | Structured multi-document summarization for surveys |
| 2402.01788 | LitLLM | none | toolkit_pipeline | hybrid | none | Toolkit for scientific literature review |
| 2408.13450 | vitaLITy 2 | none | review_pipeline | hybrid | none | Reviewing academic literature using LLMs |
| 2407.20906 | Auto Review Generation | none | review_pipeline | hybrid | post_hoc | Automated review generation method based on LLMs |
| 2510.26750 | ProfOlaf | none | semi_automated_slr | hybrid | interactive | Semi-automated tool for systematic literature reviews |
| 2412.08578 | ML IR for SLR | none | ir_pipeline | hybrid | none | ML-based information retrieval for systematic review support |
| 2407.18657 | SWARM-SLR | none | framework_analysis | N/A | N/A | Requirements framework for SLR automation (non-operational) |
| 1908.06676 | EDAM | none | se_slr_pipeline | keyword | none | Reducing effort for systematic reviews in SE |
| 2509.16599 | CASMA | none | slr_meta_analysis | hybrid | none | Computational-assisted systematic review and meta-analysis |
| 2510.11409 | LLM SLR Corpus Filtration | none | slr_filtration | hybrid | none | Semi-automatic corpus filtration for SLRs |
| 2509.19370 | Meow | none | end_to_end_outline | hybrid | multi_round | End-to-end outline writing for automatic academic survey |
| 2412.15249 | LitLLMs | none | lit_review_pipeline | hybrid | evaluation | Evaluation of LLMs for literature review tasks |
| 2410.15978 | PROMPTHEUS | none | human_centered_pipeline | hybrid | interactive | Human-centered pipeline for SLRs with LLMs |

### Shared Characteristics
- All use **1 agent** — a single LLM orchestrates all stages
- All follow a **sequential pipeline** model (no parallel agent execution)
- Most use **hybrid retrieval** (embedding + keyword); some use pure embedding or keyword
- Most lack **citation graph awareness** — SurveyForge is the only exception with bfs citation chaining

### Distinguishing Features (intra-category variation)
- **Planning mechanism**: Outline (AutoSurvey) vs. AttributeTree (SurveyX) vs. memory-guided (SurveyForge) vs. step-by-step prompt (Step-by-Step) vs. quality-driven (SurveyGen) vs. end-to-end outline (Meow)
- **Iteration**: Post-hoc refinement (AutoSurvey, Auto Review) vs. quality estimation feedback (SurveyGen) vs. memory-driven (SurveyForge) vs. none (SurveyX, Step-by-Step)
- **Domain specificity**: CASMA and EDAM target systematic reviews in SE/medical; others are general-purpose
- **Human involvement**: ProfOlaf, PROMPTHEUS, and InsightAgent are semi-automated; others are fully automated
- **Graph awareness**: SurveyForge uses bfs citation chaining; others use none

---

## Category: Iterative Refinement

### Definition
Systems where iterative improvement is the central architectural pattern — the survey structure and content evolve through multiple rounds of generation, evaluation, and revision. Unlike single-agent pipelines where refinement is one stage among many, these systems center their design around the iteration loop.

### Papers

| arXiv ID | Paper | Graph Awareness | Writing Paradigm | Iteration Strategy | Key Distinction |
|----------|-------|----------------|-----------------|-------------------|-----------------|
| 2508.14317 | SurveyGen-I | none | adaptive_planning | coarse_to_fine | Coarse-to-fine retrieval with plan evolution; memory-guided writing |
| 2510.21900 | IterSurvey | none | recurrent_outline | self_evaluation_loop | Outline adapts as content is generated; Survey-Arena eval |
| 2412.15249 | LitLLMs | none | lit_review_eval | evaluation_loop | Evaluates whether LLMs are ready for literature review |

### Shared Characteristics
- All use **1 agent** with **multi-round iteration**
- All adapt the **plan/outline** based on discovered content
- None use **citation graph traversal**
- All emphasize **self-evaluation** as the driver for iteration

### Distinguishing Features (intra-category variation)
- **Granularity of iteration**: SurveyGen-I iterates at the retrieval level (coarse → fine); IterSurvey iterates at the outline level (recurrent regeneration); LitLLMs iterates at the evaluation level (are we there yet?)
- **Memory mechanism**: SurveyGen-I has explicit memory for narrative construction; IterSurvey does not
- **Convergence criteria**: Not explicitly specified in any — a shared limitation

---

## Category: Graph-Enhanced Retrieval

### Definition
Systems where citation graph structure is the primary mechanism for literature discovery and survey organization. These systems construct, traverse, or model citation relationships to guide retrieval and structure the generated survey.

### Papers

| arXiv ID | Paper | Graph Awareness | Graph Type | Writing Paradigm | Key Distinction |
|----------|-------|----------------|-----------|-----------------|-----------------|
| 2510.07733 | SurveyG | hierarchical_graph | Foundation/Development/Frontier | graph_structured | Three-layer hierarchical citation graph; horizontal + vertical traversal |
| 2506.12689 | SciSage | citation_chaining | bfs traversal | reflect_while_writing | Searcher agent uses forward/backward citation chaining |
| 2503.04629 | SurveyForge | citation_chaining | bfs traversal | memory_guided_outline | Scholar Navigation Agent follows citation trails |
| 2508.04306 | MATC | bfs | bfs traversal | manager_taskforce | Exploration taskforce uses citation chaining for coverage |
| 2409.12177 | LitFM | hierarchical | structure-aware FM | retrieval_augmented | Retrieval-augmented structure-aware foundation model for citation graphs |
| 2605.14790 | Graphs of Research (GoR) | hierarchical | paper-evolution DAG | graph_structured | Citation evolution DAG as structured supervision for idea generation |

### Shared Characteristics
- All use **some form of citation graph traversal** — the key distinguishing feature from other categories
- Most combine graph traversal with **hybrid retrieval** (embedding + keyword + graph)
- Graph awareness is used for **literature discovery**, not just citation placement

### Distinguishing Features (intra-category variation)
- **Graph depth**: SurveyG uses a 3-layer hierarchy (Foundation/Development/Frontier); others use bfs with configurable depth
- **Primary vs. secondary role**: SurveyG's graph is primary (drives both retrieval and organization); SciSage/SurveyForge/MATC use graph as a supplementary retrieval strategy
- **Integration with generation**: SurveyG's graph structure directly maps to survey outline; others use graph only for retrieval

---

## Category: Hybrid Interactive

### Definition
Systems that integrate human oversight, guidance, or interaction at multiple stages of the generation process. The human is not just a consumer but an active participant in shaping the survey's content, structure, and quality.

### Papers

| arXiv ID | Paper | Human Role | Graph Awareness | Iteration | Key Distinction |
|----------|-------|-----------|----------------|-----------|-----------------|
| 2504.08762 | InteractiveSurvey | Customize references, outline, content mid-generation | none | interactive | User customization at multiple generation stages |
| 2504.14822 | InsightAgent | Human orchestrator + 5 AI agents | bfs | interactive | Months→1.5 hours; +27.2% quality; human validates each stage |
| 2410.15978 | PROMPTHEUS | Human-centered pipeline for SLRs | none | interactive | Human-LLM collaboration for systematic literature reviews |
| 2510.26750 | ProfOlaf | Semi-automated SLR tool | none | interactive | Semi-automated tool with human oversight |

### Shared Characteristics
- All require **active human participation** at some stage of generation
- All combine **AI automation** with human judgment for quality control
- All use **interactive iteration** where the human provides feedback

### Distinguishing Features (intra-category variation)
- **Depth of human involvement**: InsightAgent requires human orchestration at every stage; InteractiveSurvey allows mid-generation customization; PROMPTHEUS uses human oversight at key checkpoints
- **Agent count**: InsightAgent uses 6 agents; InteractiveSurvey and PROMPTHEUS use 1 agent
- **Graph awareness**: InsightAgent uses bfs citation tracking; others use none

---

## Category: Automated Scientific Discovery

### Definition
Broader autonomous research systems that encompass survey generation as one component of a larger scientific discovery pipeline. These systems go beyond literature review to include hypothesis generation, experiment design, data analysis, and paper writing.

### Papers

| arXiv ID | Paper | Focus | Graph Awareness | Relation to Survey Gen |
|----------|-------|-------|----------------|----------------------|
| 2408.06292 | The AI Scientist | Full pipeline: ideation → experiments → paper writing | none | Survey generation is one module in full discovery pipeline |
| 2602.07040 | Aster | Autonomous discovery 20x faster | none | Accelerated scientific discovery with automated literature review |
| 2510.26887 | Denario | Deep knowledge AI agents for discovery | none | Knowledge agents for scientific discovery |
| 2510.26144 | The FM Agent | Foundation model agent for research | none | Agent for general research tasks |
| 2504.03424 | AI Cosmologist I | Agentic data analysis | none | Domain-specific (cosmology) automated analysis with lit review |
| 2507.07257 | Open Source Planning | Planning & control for autonomous discovery | none | Open-source system for autonomous scientific discovery |
| 2605.20025 | AutoResearchClaw | Self-reinforcing autonomous research | none | Self-reinforcing research loop with survey generation |
| 2510.20844 | AutoResearcher | Knowledge-grounded research ideation | none | Automating ideation with literature grounding |
| 2504.18765 | Vision for Auto Research | Vision paper for LLM agent research | none | Vision/proposal for future autonomous research agents |
| 2510.15624 | Build Your Research Group | Personalized multi-agent research group | none | Multi-agent framework for personalized research assistance |

### Shared Characteristics
- All go **beyond survey generation** to encompass broader research tasks
- Most are from **FrontierScout** (future/emerging systems)
- None use **citation graph awareness** for literature discovery
- Most have **high agent counts** or configurable multi-agent architectures

### Distinguishing Features (intra-category variation)
- **Maturity**: The AI Scientist is operational; Vision for Auto Research is a vision paper; others are in development
- **Domain specificity**: AI Cosmologist is cosmology-specific; others are general-purpose
- **Autonomy level**: AutoResearchClaw is self-reinforcing; Build Your Research Group requires user configuration

---

## Category: Mechanism & Citation Graph Papers

### Definition
Papers that describe fundamental mechanisms for citation graph traversal, influence propagation, bibliometric analysis, or retrieval enhancement — without being full survey generation systems. These provide building blocks that survey generation systems can incorporate.

### Papers

| arXiv ID | Paper | Year | Mechanism Type | Key Contribution |
|----------|-------|------|---------------|-----------------|
| 2408.15371 | Temporal GNN Paper Recommendation | 2024 | citation_graph_dynamics | GNN-powered recommendation on dynamic citation networks |
| 2305.01572 | H2CGL | 2023 | citation_impact_prediction | Modeling dynamics of citation networks for impact prediction |
| 2104.02562 | Structured Citation Trend Prediction | 2021 | citation_trend_analysis | GNN-based citation trend prediction |
| 2512.22159 | Oignon | 2025 | citation_graph_tool | Citation graph visualization and analysis tool |
| 2106.01560 | CitationIE | 2021 | citation_information_extraction | Leveraging citation graphs for scientific IE |
| 1805.02262 | Literature Graph (Semantic Scholar) | 2018 | citation_graph_infrastructure | Construction of the literature graph (infrastructure) |
| 1806.00089 | Cascading Citation Expansion | 2018 | citation_expansion | Cascading forward/backward citation expansion |
| 1904.07579 | Go Wide, Go Deep | 2019 | influence_dispersion | Quantifying impact through influence dispersion trees |
| 1310.8224 | Transitive Reduction of Citation Networks | 2013 | graph_reduction | Transitive reduction for simplifying citation networks |
| 2602.12206 | OpenAIRE Citation Graph | 2026 | citation_graph_infrastructure | Making complete OpenAIRE citation graph accessible |
| 2210.03629 | ReAct | 2022 | agent_reasoning | Synergizing reasoning and acting in language models |
| 2310.04406 | LATS | 2023 | agent_reasoning | Language Agent Tree Search for reasoning + planning |
| 2409.12177 | LitFM | 2024 | citation_graph_fm | Structure-aware foundation model for citation graphs |
| 1205.1143 | Direction Aware Citation Analysis | 2012 | citation_analysis | Recommendation on academic networks using direction-aware analysis |
| 2104.04939 | GCN Citation Count Prediction | 2021 | citation_prediction | GCN-based framework for estimating future citations |
| 2003.12042 | Heterogeneous Dynamical GNN | 2020 | citation_prediction | Heterogeneous dynamical GNN for quantifying scientific impact |
| 1903.06464 | Context-Aware Citation Recommendation | 2019 | citation_recommendation | BERT + GCN for context-aware citation recommendation |
| 2402.08339 | Interleaved Snowballing | 2024 | slr_methodology | Reducing workload of literature curators through interleaved snowballing |
| 2004.09741 | Hybrid Search Strategies for SLRs | 2020 | slr_methodology | Performance of hybrid search strategies for systematic reviews |
| 2010.04665 | Scaling SLRs with ML Pipelines | 2020 | slr_methodology | Scaling systematic literature reviews with ML pipelines |

### Shared Characteristics
- All are **mechanism papers** — they describe a technique, tool, or infrastructure rather than a full survey generation system
- Most focus on **citation graph analysis**, traversal, or prediction
- Many predate the LLM era (2012–2022) — foundational infrastructure for survey generation

### Distinguishing Features (intra-category variation)
- **Purpose**: Citation prediction vs. graph infrastructure vs. agent reasoning vs. SLR methodology
- **Technique**: GNN-based vs. PageRank-based vs. rule-based vs. LLM-based
- **Granularity**: Paper-level (citation prediction) vs. corpus-level (graph construction) vs. field-level (bibliometrics)

---

## Category: Benchmark & Evaluation

### Definition
Dedicated benchmarks, evaluation frameworks, and datasets for assessing the quality of automated survey generation systems. These provide standardized metrics, evaluation protocols, and comparison baselines. Evaluation metrics and factuality evaluation tools are also included.

### Papers

| arXiv ID | Paper | Year | Evaluation Focus | Key Metrics |
|----------|-------|------|-----------------|-------------|
| 2510.03120 | SurveyBench | 2025 | Quiz-driven evaluation | 11,343 arXiv topics + 4,947 surveys for QA-based eval |
| 2512.02763 | SurveyEval | 2025 | Multi-dimension evaluation | Overall quality, outline coherence, reference accuracy (7 subjects) |
| 2508.15658 | SurGE | 2025 | 4-dimension evaluation | 1M-paper corpus; coverage, accuracy, structure, readability |
| 2602.11238 | SurveyLens | 2026 | Discipline-aware evaluation | 1,000 human surveys across 10 disciplines |
| 2601.15307 | DeepSurvey-Bench | 2026 | Academic value evaluation | Information value, scholarly communication quality |
| 2406.10291 | ResearchArena | 2024 | 3-stage agent evaluation | Paper discovery, selection, organization; 12M paper offline env |
| 2308.10410 | Wikipedia-Style Survey Eval | 2023 | Human vs. GPT comparison | 99 NLP topics; GPT-written vs. human-written survey quality |
| 2508.11310 | SGSimEval | 2025 | Multi-faceted ASG benchmark | Comprehensive benchmark for automated survey generation systems |
| 2402.16009 | PST-Bench | 2024 | Source tracing | Tracing and benchmarking the source of publications |
| 2601.14949 | CiteRAG | 2026 | Citation prediction | RAG benchmark for citation prediction |

### Evaluation Metrics & Factuality Tools

| arXiv ID | Paper | Year | Focus | Application to Survey Gen |
|----------|-------|------|-------|--------------------------|
| 1904.09675 | BERTScore | 2019 | Text generation similarity | Semantic overlap between generated and reference surveys |
| 2305.14251 | FActScore | 2023 | Atomic fact precision | Verifying factual accuracy of survey claims |
| 2103.12693 | QuestEval | 2021 | QA-based factuality | Reference-free factuality evaluation for surveys |
| 2309.12455 | LongDocFACTScore | 2023 | Long-document factuality | Factuality evaluation for long survey documents |
| 2406.19276 | VERISCORE | 2024 | Verifiable claim evaluation | Evaluating factuality of verifiable claims in survey text |
| 2403.18802 | SAFE | 2024 | Long-form factuality | Long-form factuality evaluation applicable to surveys |
| 2208.01030 | SMART | 2022 | Sentence-level evaluation | Sentence-level text quality evaluation |
| 2403.02270 | FENICE | 2024 | NLI-based factuality | SOTA factuality metric based on NLI and claim extraction |
| 2305.08281 | FactKB | 2023 | Generalizable factuality | Factuality evaluation using language models |
| 2510.17853 | CiteGuard | 2025 | Citation attribution | Faithful citation attribution for LLM-generated text |
| 2305.14627 | ALCE | 2023 | Citation generation | Benchmark for LLMs generating text with citations |
| 2407.12861 | CiteME | 2024 | Citation accuracy | Can language models accurately cite scientific claims? |
| 2411.02448 | REC | 2024 | Explanation + attribution | Rate, explain, and cite framework |
| 2408.11832 | OpenFactCheck | 2024 | Fact-checking framework | Unified framework for factuality evaluation |
| 2405.05583 | OpenFactCheck Custom | 2024 | Custom fact-checking | Building and benchmarking customized fact-checking |
| 2509.21557 | Generation-Time vs Post-hoc Citation | 2025 | Citation strategy evaluation | Holistic evaluation of LLM attribution strategies |
| 2510.12839 | FaStFACT | 2025 | Fast factuality | Faster, stronger long-form factuality evaluations |
| 2411.16638 | Do Auto Metrics Measure Factuality? | 2024 | Critique | Critical evaluation of automatic factuality metrics |
| 2010.12495 | Understanding Summarization Metrics | 2020 | Critique | ROUGE/BERTScore analysis and limitations |
| 2305.18201 | LFQA Evaluation Critique | 2023 | Critique | Critical evaluation of long-form QA evaluation |
| 2210.16732 | Long Summary Robustness | 2022 | Critique | Robustness of long abstractive summarization evaluation |
| 2010.12834 | GO FIGURE | 2020 | Critique | Meta-evaluation of factuality in summarization |
| 2510.04311 | Task Complexity in MAS | 2025 | Critique | Importance of task complexity in evaluating LLM MAS |

### Shared Characteristics
- All provide **evaluation infrastructure** rather than generation methods
- Most focus on **survey-specific** evaluation (SurveyBench, SurveyEval, SurGE, SurveyLens)
- Metrics papers address **cross-cutting concerns** (factuality, citation accuracy, text quality)

### Distinguishing Features (intra-category variation)
- **Scope**: Survey-specific (SurveyBench) vs. general NLP (BERTScore, FActScore)
- **Evaluation approach**: Quiz-based (SurveyBench) vs. dimension-rating (SurveyEval) vs. reference-free (QuestEval) vs. human comparison (Wiki Survey)
- **Factuality granularity**: Atomic fact (FActScore) vs. claim-level (VERISCORE) vs. sentence-level (SMART) vs. document-level (LongDocFACTScore)
- **Citation focus**: Attribution (CiteGuard, ALCE, REC) vs. accuracy (CiteME) vs. prediction (CiteRAG)

---

## Category: Survey / Review / Taxonomy Papers

### Definition
Surveys, reviews, taxonomies, and vision papers that serve as citation seeds or contextual references. These papers survey the field of automated survey generation, LLM agents, or related areas, and are included for citation and context rather than as primary method papers.

### Papers

| arXiv ID | Paper | Year | Topic | Relevance |
|----------|-------|------|-------|-----------|
| 2409.04600 | LLMs as a tool in literature reviews | 2024 | LLMs for lit review | Direct survey of LLM applications in literature review |
| 2402.08565 | AI for Literature Reviews: Opportunities and Challenges | 2024 | AI for lit review | Comprehensive review of AI opportunities in lit review |
| 2401.10917 | AI to automate systematic review of scientific literature | 2024 | AI for SLR | Survey of AI automation for systematic reviews |
| 2211.15397 | Automating SLRs with NLP and Text Mining | 2022 | NLP for SLR | Earlier survey of NLP-based SLR automation |
| 2308.11432 | LLM-based Autonomous Agents | 2023 | LLM agents | Broad survey of LLM-based autonomous agents |
| 2309.07864 | Rise and Potential of LLM-based Agents | 2023 | LLM agents | Survey of LLM agent capabilities |
| 2508.05668 | LLM-based Deep Search Agents | 2025 | Deep search agents | Survey of LLM-based deep search paradigms |
| 2601.01743 | AI Agent Systems | 2026 | Agent architectures | Survey of AI agent architectures and evaluation |
| 2601.12560 | Agentic AI | 2026 | Agent taxonomies | Taxonomy of LLM agent architectures |
| 2604.18133 | Multi-Agent Systems: Classical to LFM | 2026 | Multi-agent systems | Survey from classical to LLM-enabled multi-agent systems |
| 2503.21460 | LLM Agent: Methodology, Applications, Challenges | 2025 | LLM agents | Comprehensive survey of LLM agent methodology |
| 2508.17692 | LLM-based Agentic Reasoning Frameworks | 2025 | Agentic reasoning | Survey of agentic reasoning frameworks |
| 2406.05804 | Prominent Paradigms for LLM-Based Agents | 2024 | Agent paradigms | Review of prominent LLM agent paradigms |
| 2505.23252 | Multiple Classification Dimensions for Papers | 2025 | Paper classification | Automatic construction of paper classification dimensions |
| 2605.16475 | GenAI for Literature Reviews | 2026 | GenAI for lit review | Frontier survey of generative AI for literature reviews |
| 2111.07533 | Automated scholarly paper review | 2021 | AI-assisted review | Survey of automated paper review concepts |
| 2504.19678 | LLM Reasoning to Autonomous AI Agents | 2025 | Agent evolution | Comprehensive review from LLM reasoning to autonomous agents |

---

## Category: Citation Seed & Foundational Papers

### Definition
Classic or domain-specific papers providing theoretical underpinnings (PageRank, GNNs) or cross-domain inspiration (information foraging, discourse structure, PRISMA guidelines). These are not directly about survey generation but inform the design space.

### Papers

| arXiv ID | Paper | Year | Domain | Contribution |
|----------|-------|------|--------|-------------|
| 1810.00826 | How Powerful are GNNs? | 2018 | GNN theory | Foundational GNN expressiveness theory |
| 1407.5107 | PageRank beyond the Web | 2014 | Ranking theory | PageRank for general citation analysis |
| 1805.02262 | Semantic Scholar Literature Graph | 2018 | Infrastructure | Construction of the literature graph |
| 2301.10140 | Semantic Scholar Open Data | 2023 | Infrastructure | Open data platform for citation graph |
| 2110.06595 | Refcat: Internet Archive Citation Graph | 2021 | Infrastructure | Citation graph from Internet Archive |
| 1902.05170 | GrapAL | 2019 | KG infrastructure | Connecting the dots in scientific literature |
| 1404.5322 | CitNetExplorer | 2014 | Visualization | Tool for analyzing and visualizing citation networks |
| 2006.05542 | Guidelines for Search Strategy to Update SLRs | 2020 | SLR methodology | Search strategy guidelines for SLR updates |
| 2001.08988 | Framework for Methodology Scoping Reviews | 2020 | SLR methodology | Framework for scoping review methodology |
| 2112.09424 | Search Strategy Formulation for SLRs | 2021 | SLR methodology | Formulating search strategies for systematic reviews |
| 2304.13556 | Systematic Review-lution: A Manifesto for HCI | 2023 | SLR methodology | Manifesto for systematic review methodology in HCI |
| 2306.17614 | Outcome-based Evaluation of SLR Automation | 2023 | SLR evaluation | Framework for evaluating SLR automation outcomes |
| 2508.15043 | LitForager | 2025 | Info foraging | Multimodal literature foraging strategies |
| 2408.02508 | PUREsuggest | 2024 | Citation search | Citation-based literature search and visual exploration |
| 2304.00121 | ManuScript | 2023 | Writing analysis | Decoding end-to-end writing trajectory in scholarly manuscripts |
| 2310.15077 | Discourse Structure for Science Journalism | 2023 | Discourse | Discourse structure framework for science communication |
| 1903.04427 | Rhetorical structure of science | 2019 | Discourse | Multidisciplinary analysis of rhetorical structure in science |
| 1710.03094 | In-text citations in scientific articles | 2017 | Citation analysis | Large-scale analysis of in-text citation characteristics |
| 2509.04190 | Changing role of cited papers over time | 2025 | Citation dynamics | Analyzing how the role of cited papers evolves over time |
| 2605.14790 | Graphs of Research (GoR) | 2026 | citation_graph_generation | Citation evolution DAG as supervision for idea generation |
| 2605.07723 | LLM hallucinations in the wild | 2026 | citation_hallucination_audit | Large-scale evidence from non-existent citations (111M refs) |
| 2604.22750 | How Do AI Agents Spend Your Money? | 2026 | token_cost_analysis | First systematic token consumption analysis for agentic tasks |

---

## Cross-Cutting Comparison

### Graph Awareness vs. Method Category

| Method Category | none | citation_chaining | bfs | hierarchical_graph |
|----------------|------|-------------------|-----|-------------------|
| **Single-Agent Pipeline** | AutoSurvey, SurveyX, SurveyGen, SurveyX, Step-by-Step, LitLLM, vitaLITy, ProfOlaf, CASMA, Meow | SurveyForge | — | — |
| **Multi-Agent Pipeline** | Agentic AutoSurvey, AutoSurvey2, LLM×MapReduce-V3, LiRA, AutoGenesisAgent, Multi-Agent SLR | SciSage | MATC, InsightAgent, KMCA | — |
| **Iterative Refinement** | SurveyGen-I, IterSurvey | — | — | — |
| **Graph-Enhanced Retrieval** | — | SciSage*, SurveyForge* | MATC*, LitFM | SurveyG |
| **Hybrid Interactive** | InteractiveSurvey, PROMPTHEUS, ProfOlaf | — | InsightAgent | — |
| **Automated Discovery** | AI Scientist, Aster, Denario, FM Agent, AutoResearcher, etc. | — | — | — |

\* Papers that appear in multiple rows have graph awareness as a secondary mechanism.

### Writing Paradigm vs. Iteration Strategy

| Writing Paradigm | none | post_hoc | multi_round | self_eval | real_time | interactive |
|-----------------|------|---------|-------------|-----------|-----------|-------------|
| **outline_driven** | — | AutoSurvey | — | — | — | — |
| **attribute_tree** | SurveyX | — | — | — | — | — |
| **parallel_sections** | — | — | AutoSurvey2 | — | — | — |
| **reflect_while_writing** | — | — | — | — | SciSage | — |
| **plan_write_review** | — | — | Agentic AutoSurvey | — | — | — |
| **adaptive_planning** | — | — | SurveyGen-I | — | — | — |
| **recurrent_outline** | — | — | — | IterSurvey | — | — |
| **graph_structured** | SurveyG | — | — | — | — | — |
| **quality_driven** | — | — | SurveyGen | — | — | — |
| **step_by_step** | Step-by-Step | — | — | — | — | — |
| **memory_guided** | — | — | SurveyForge | — | — | — |
| **human_centered** | — | — | — | — | — | InsightAgent, InteractiveSurvey |

---

## Boundary / Mixed Papers

| arXiv ID | Paper | Issue | Recommended Placement |
|----------|-------|-------|---------------------|
| 2410.15978 | PROMPTHEUS | Between `single_agent_pipeline` and `hybrid_interactive` — has human-in-the-loop but primarily a single pipeline | **hybrid_interactive** — human oversight at key checkpoints is central to design |
| 2510.26750 | ProfOlaf | Between `single_agent_pipeline` and `hybrid_interactive` — semi-automated SLR tool | **single_agent_pipeline** but annotated as semi-automated (human-in-loop flag) |
| 2407.18657 | SWARM-SLR | Framework paper, not an operational system; method_category assigned as `single_agent_pipeline` | **single_agent_pipeline** (as framework/guidelines) — annotate as non-operational |
| 2504.14822 | InsightAgent | Between `multi_agent_pipeline` and `hybrid_interactive` — 5 AI agents + human orchestrator | **multi_agent_pipeline** with human_in_loop=interactive — primarily defined by agent architecture |
| 2510.10890 | LLM×MapReduce-V3 | General-purpose agent framework, not survey-specific; included due to relevance | **multi_agent_pipeline** — general-purpose but applicable to survey generation |
| 2408.06292 | The AI Scientist | Between `multi_agent_pipeline` and `automated_discovery` — includes survey gen but much broader | **automated_discovery** — scope extends far beyond survey generation |
| 2510.20844 | AutoResearcher | Between `multi_agent_pipeline` and `automated_discovery` — focuses on ideation with literature grounding | **automated_discovery** — research ideation is primary, not survey generation |

---

## Summary Statistics

| Category | Count | Includes |
|----------|-------|----------|
| **Multi-Agent Pipeline** | 11 | SciSage, Agentic AutoSurvey, AutoSurvey2, MATC, KMCA, LLM×MapReduce-V3, LiRA, AutoGenesisAgent, Multi-Agent SLR, InsightAgent, Build Your Research Group |
| **Single-Agent Pipeline** | 18 | AutoSurvey, SurveyX, SurveyGen, SurveyForge, Step-by-Step, BigSurvey/CAST, LitLLM, vitaLITy, Auto Review, ProfOlaf, ML IR for SLR, SWARM-SLR, EDAM, CASMA, LLM SLR Corpus, Meow, LitLLMs, PROMPTHEUS |
| **Iterative Refinement** | 3 | SurveyGen-I, IterSurvey, LitLLMs |
| **Graph-Enhanced Retrieval** | 5 | SurveyG, SciSage*, SurveyForge*, MATC*, LitFM |
| **Hybrid Interactive** | 4 | InteractiveSurvey, InsightAgent*, PROMPTHEUS, ProfOlaf* |
| **Automated Scientific Discovery** | 10 | AI Scientist, Aster, Denario, FM Agent, AI Cosmologist, Open Source Planning, AutoResearchClaw, AutoResearcher, Vision for Auto Research, Build Your Research Group* |
| **Mechanism & Citation Graph** | 23 | Temporal GNN, H2CGL, CitationIE, Oignon, Cascading Citation, LitFM, ReAct, LATS, Graphs of Research (GoR), LLM hallucinations in the wild, How Do AI Agents Spend Your Money, etc. |
| **Benchmark & Evaluation** | 31 | SurveyBench, SurveyEval, SurGE, SurveyLens, DeepSurvey-Bench, ResearchArena, SGSimEval, PST-Bench, CiteRAG, BERTScore, FActScore, VERISCORE, etc. |
| **Survey / Review Papers** | 17 | LLMs for lit review, AI for Lit Rev, LLM Agent surveys, etc. |
| **Citation Seed & Foundational** | 19 | GNN theory, PageRank, Semantic Scholar, CitNetExplorer, PRISMA guidelines, etc. |

\* Papers appearing in multiple categories (with * ) are counted in their primary category.

---

## Notes

1. **Primary dimension is `method_category`**: This captures the most fundamental architectural choice — how agents are organized and how the generation pipeline is structured.

2. **Secondary dimensions enable cross-cutting analysis**: `graph_awareness_level` and `writing_paradigm` reveal patterns that cut across method categories. For example, citation chaining appears in both single-agent (SurveyForge) and multi-agent (SciSage, MATC) systems.

3. **Graph awareness is the key architectural differentiator**: Only 5 out of 35 core method papers use any form of citation graph traversal. Most rely on embedding/keyword retrieval, suggesting a significant gap in the field. _(Note: Three supervisor-retrieved papers — GoR [2605.14790], LLM hallucinations [2605.07723], Token Cost [2604.22750] — were added in Round 2 for depth improvement in Sections 4.3, 5.4, and 6.)_

4. **The `iterative_refinement` category is small but conceptually distinct**: These papers center their design around the iteration loop, unlike single-agent pipelines where refinement is one stage among many.

5. **Mechanism papers provide the building blocks**: The 20+ mechanism/citation graph papers describe the foundational techniques (citation chaining, PageRank, snowballing) that survey generation systems can incorporate.

6. **Benchmark coverage is growing rapidly**: SurveyBench, SurveyEval, SurGE, and SurveyLens all appeared in 2025–2026, indicating the field is maturing toward standardized evaluation.

7. **Automated discovery systems are frontier but immature**: Most are vision/prototype papers. Only The AI Scientist and Aster have operational systems.
