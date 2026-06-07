# Paper Taxonomy

## Classification Dimensions

### Primary Dimension: Architecture Type

| Dimension | Possible Values | Criteria |
|-----------|---------------|---------|
| Architecture Type | single_agent_pipeline, multi_agent_pipeline, graph_enhanced_retrieval, hybrid_interactive, iterative_refinement | How the system is organized architecturally — number of agents, coordination pattern, and role specialization |

### Secondary Dimensions

| Dimension | Possible Values | Criteria |
|-----------|---------------|---------|
| Citation Graph Awareness | none, bfs (simple forward/backward chaining), graph_traversal, hierarchical (structured tiers) | Whether and how the system uses citation graph structure for retrieval or organization |
| Retrieval Method | none, keyword, dense (embedding), hybrid, graph_traversal | How papers and content are retrieved |
| Iteration Strategy | single_pass, multi_round (refinement loops), interactive (human-in-loop) | Whether the system refines its output through iteration |
| Agent Count | 0 (N/A), 1, 2-5, 5+ | Number of distinct agent roles in the architecture |
| Human Involvement | none, seed_papers_only, human_validates, human_guides | Degree of user/human interaction in the generation process |
| Generation Scope | outline_only, related_work, section_level, full_survey | What the system produces |
| Paper Role | core_method, mechanism, benchmark/dataset, survey/reference, frontier_method | The paper's role in the ecosystem |

---

## Category: Single-Agent Pipeline

### Definition
Systems where a single LLM drives all stages of the pipeline through sequential prompting or staged processing. There is no multi-agent coordination, no role specialization, and typically no iterative refinement. This is the foundational category encompassing early survey generation systems, RAG-based approaches, and most benchmark/dataset papers.

### Papers

| arXiv ID | Paper | Year | Category-Specific Notes |
|----------|-------|------|------------------------|
| 2004.14974 | SciFact: A Dataset for Scientific Claim Verification | 2020 | Dataset/benchmark for claim verification — foundational evaluation resource |
| 2004.15011 | SciTLDR: Extreme Summarization of Scientific Papers | 2020 | Dataset for one-sentence paper summarization |
| 2005.11401 | Retrieval-Augmented Generation for Knowledge-Intensive NLP Tasks | 2020 | Foundational RAG paradigm — single retriever-generator pair |
| 2006.12166 | ASReview: Active Learning for Systematic Literature Review Screening | 2020 | Traditional ML-based SLR screening tool |
| 2010.04147 | AutoReviewGen: Automatic Generation of Literature Review Sections | 2020 | Pre-LLM BERT-based extractive review generation |
| 2010.14235 | Multi-XScience: A Dataset for Related-Work Generation | 2020 | Dataset for multi-document related-work summarization |
| 2011.08072 | MAG-20: Topic-Centric Multi-Document Summarization | 2020 | Dataset for topic-centric scientific summarization |
| 2104.06486 | MS²: Multi-Document Summarization of Medical Studies | 2021 | Dataset for contradiction-aware medical summarization |
| 2107.00414 | MultiCite: Multi-Sentence Citation Analysis | 2021 | Dataset for citation function analysis |
| 2203.01769 | PeerSum: Meta-Review Summarization | 2022 | Dataset for synthesizing multiple peer reviews |
| 2204.04991 | TRUE: Meta-Evaluation of Factual Consistency | 2022 | Meta-evaluation benchmark for factuality metrics |
| 2302.07302 | CiteSee: Augmenting Citations with Persistent Historical Context | 2023 | Single-pipeline system for citation context display |
| 2305.14251 | FActScore: Fine-grained Atomic Factuality Evaluation | 2023 | Standard factuality evaluation framework |
| 2305.15186 | SciReviewGen: 10K Reviews, 690K Cited Papers | 2023 | Large-scale review generation dataset |
| 2308.10410 | Wikipedia-style Survey Evaluation with GPT-4 | 2023 | Evaluation study of LLM survey capabilities |
| 2309.09727 | When LLMs Meet Citation: A Survey | 2023 | Survey paper on citation generation landscape |
| 2310.04480 | Auto-survey Challenge: Competition Platform | 2023 | Competition/evaluation platform |
| 2311.09182 | ContraDoc: Self-Contradiction Detection in Long Documents | 2023 | Dataset for intra-document consistency |
| 2311.12315 | AcademicGPT: Domain-Specific LLM for Academic Tasks | 2023 | Single fine-tuned model for academic writing |
| 2312.07559 | PaperQA: Retrieval-Augmented Generation Agent for Scientific Research | 2023 | Foundational scientific RAG agent |
| 2402.01788 | LitLLM: A Toolkit for Scientific Literature Review | 2024 | Modular RAG toolkit for related-work generation |
| 2402.08565 | AI for Literature Reviews: Opportunities and Challenges | 2024 | Survey paper on AI-assisted literature review |
| 2402.16063 | Citation-Enhanced Generation for LLM-based Chatbots | 2024 | Post-hoc citation verification plugin |
| 2403.05303 | ACLSum: Aspect-Based Summarization | 2024 | Dataset for aspect-specific paper summarization |
| 2403.18802 | LongFact + SAFE: Long-form Factuality Evaluation | 2024 | Automated factuality evaluation methodology |
| 2405.01930 | OARelatedWork: Large-Scale Related-Work Dataset | 2024 | Large-scale dataset with full-text access |
| 2406.10252 | AutoSurvey: Large Language Models Can Automatically Write Surveys | 2024 | **Foundational** end-to-end survey generation pipeline |
| 2407.12861 | CiteME: Citation Attribution Benchmark | 2024 | Benchmark for single-claim citation attribution |
| 2407.18940 | LitSearch: Scientific Literature Retrieval Benchmark | 2024 | Benchmark for retrieval quality evaluation |
| 2407.20906 | Auto Review Generation (PDH Catalysis Domain) | 2024 | Domain-specific extraction-to-generation pipeline |
| 2408.07884 | Instruct LLMs to Generate Literature Survey Step by Step | 2024 | Prompt decomposition for survey generation |
| 2408.16444 | SurveySum: Multi-Article Summarization Dataset | 2024 | Dataset for section-level survey summarization |
| 2409.13740 | PaperQA2: Superhuman Scientific QA with Citations | 2024 | Advanced single-agent RAG with multi-step verification |
| 2411.14199 | OpenScholar: Open-Source Scientific RAG | 2024 | Single retriever-generator with domain-specific training |
| 2411.16638 | Factuality Metrics Critique | 2024 | Meta-analysis of factuality evaluation metrics |
| 2411.18583 | Automated Literature Review Using NLP + RAG | 2024 | Hybrid NLP+RAG pipeline for lit review |
| 2412.13612 | LLMs for Automated Literature Review (Multi-dim Eval) | 2024 | Evaluation study of LLM survey capabilities |
| 2412.15249 | LitLLMs: Evaluation Protocol for Survey Generation | 2024 | Evaluation framework for zero-shot survey generation |
| 2501.04306 | LLM4SR: Survey of LLMs for Scientific Research | 2025 | Comprehensive survey of LLM applications in research |
| 2502.14776 | SurveyX: Academic Survey Automation with AttributeTree | 2025 | AttributeTree preprocessing for structured retrieval |
| 2503.08506 | ReviewBench / Review-CoT | 2025 | Benchmark for peer review generation |
| 2504.00824 | ScholarCopilot: Training LLMs for Academic Writing with Citations | 2025 | Fine-tuned model with retrieval-token-gated architecture |
| 2504.14822 | InsightAgent: SLR in Hours | 2025 | Parallelized SLR pipeline with multiple LLM instances |
| 2504.18496 | DimInd: Facets, Taxonomies, and Syntheses | 2025 | Multi-level compression pipeline for literature review |
| 2508.11310 | SGSimEval: Survey Generation Similarity Evaluation | 2025 | Evaluation methodology paper |
| 2508.14317 | SurveyGen-I: Memory-Guided Iterative Survey Generation | 2025 | Memory-guided writing with coarse-to-fine retrieval |
| 2508.15658 | SurGE: Survey Generation Evaluation Benchmark | 2025 | Benchmark for survey quality evaluation |
| 2509.19370 | Meow: Metadata-Driven Outline Writing | 2025 | Metadata-only outline generation |
| 2509.25868 | ReFACT: Scientific Confabulation Evaluation | 2025 | Benchmark for confabulation detection |
| 2510.03120 | SurveyBench: Benchmark for Survey Generation | 2025 | Comprehensive survey evaluation benchmark |
| 2512.02763 | SurveyEval: Cross-Subject Survey Evaluation | 2025 | Discipline-aware survey evaluation benchmark |
| 2601.14949 | CiteRAG: Citation Prediction Benchmark | 2026 | Benchmark for citation prediction |
| 2601.15307 | DeepSurvey-Bench: Academic Depth Evaluation | 2026 | Benchmark focused on academic depth assessment |
| 2602.11238 | SurveyLens: Discipline-Aware Evaluation | 2026 | Discipline-adaptation evaluation benchmark |

### Shared Characteristics
- Single LLM drives all stages — no role specialization or agent coordination
- Typically sequential pipeline (retrieval → outline → drafting → integration)
- Limited or no iterative refinement between stages
- Most benchmark/dataset papers default to this category

### Distinguishing Features (intra-category variation)
- **Retrieval sophistication ranges** from none (Instruct LLMs Step by Step) to hybrid multi-stage (PaperQA2, OpenScholar)
- **Generation scope varies** from related-work only (LitLLM, AutoReviewGen) to full survey (AutoSurvey, SurveyX)
- **Structured preprocessing** differs: AttributeTree (SurveyX), multi-level compression (DimInd), metadata analysis (Meow)
- **Training approach**: fine-tuned models (ScholarCopilot, AcademicGPT, OpenScholar) vs zero-shot prompting (AutoSurvey, SurveyX)
- **Post-hoc verification**: Citation-Enhanced Gen adds NLI verification; SurveyX adds re-polishing

---

## Category: Multi-Agent Pipeline

### Definition
Systems that employ multiple specialized LLM agents with distinct roles and coordination patterns. Agents may have specialized prompts, access patterns, or domain expertise. Coordination can be sequential, hierarchical, chat-based, or federated.

### Papers

| arXiv ID | Paper | Year | Category-Specific Notes |
|----------|-------|------|------------------------|
| 2403.08399 | A System for SLR using Multiple AI Agents | 2024 | 4 sequential SLR agents (Search, Screening, Extraction, Synthesis) |
| 2404.07738 | ResearchAgent: Iterative Idea Generation with Multi-Agent Review | 2024 | Generator + multiple Reviewers in iterative refine loop |
| 2410.21784 | MARCO: Multi-Agent Real-time Chat Orchestration | 2024 | 5+ agents with chat-based coordination and Conversation Manager |
| 2506.12689 | SciSage: Multi-Agent Framework with Hierarchical Reflector | 2025 | 4 agents (Planner, Retriever, Writer, hierarchical Reflector) with real-time reflection |
| 2507.07257 | Open Source Planning & Control (30 Agents) | 2025 | 30 agents with hierarchical Planner → Controller → Specialist structure |
| 2509.18661 | Agentic AutoSurvey: Let LLMs Survey LLMs | 2025 | 4 agents (Orchestrator, Search, Writing, Review) with iterative review-refine loop |
| 2509.20175 | Federation of Agents: Semantics-Aware Collaboration | 2025 | Federated agents with shared knowledge graph coordination |
| 2510.15624 | Build Your Personalized Research Group | 2025 | Research group simulation (Director, Senior/Junior Researchers, Critical Reviewer) |
| 2510.26012 | AutoSurvey2: Parallel Section Generation | 2025 | Multiple parallel LLM instances with centralized integration |
| 2511.17689 | ARISE: Rubric-Guided Iterative Survey Engine | 2025 | 5 agents (Planning, Retrieval, Drafting, Evaluation, Revision) with rubric guidance |
| 2603.03005 | OrchMAS: Multi-Agent Scientific Experts | 2026 | Coordinator + domain-expert agents with knowledge profiles |
| 2603.13327 | DOVA: Deliberation-First Multi-Agent Architecture | 2026 | 4+ perspective agents (empiricist, theoretician, etc.) with deliberation-before-retrieval |
| 2603.14629 | ResearchPilot: Local-First Multi-Agent Synthesis | 2026 | 4 local agents (Retriever, Analyst, Writer, Reviewer) on consumer hardware |

### Shared Characteristics
- Multiple specialized agent roles with distinct responsibilities
- Coordination mechanisms (sequential, hierarchical, chat-based, federated)
- Role specialization enables division of labor and parallel processing
- All use hybrid retrieval (search + dense) with no citation graph awareness, except SciSage and IterSurvey which use BFS

### Distinguishing Features (intra-category variation)
- **Agent count**: ranges from 4 (SciSage, DOVA) to 30 (Open Source Planning & Control)
- **Coordination pattern**: sequential (Multi-Agent SLR), hierarchical (30 Agents, OrchMAS), chat-based (MARCO), federated (Federation of Agents), rubric-guided (ARISE), deliberation-first (DOVA)
- **Homogeneity**: some use same LLM with different prompts (Agentic AutoSurvey, AutoSurvey2) vs domain-specialized agents (OrchMAS, DOVA perspectives)
- **Hardware profile**: cloud-based (most) vs local-first (ResearchPilot)
- **Citation graph awareness**: ARISE and SciSage use BFS chaining; most others have none

---

## Category: Graph-Enhanced Retrieval

### Definition
Systems where citation graph traversal (forward/backward chaining from seed papers) is a core architectural component for paper discovery. The graph structure may also inform survey organization. These systems contrast with purely search-based retrieval approaches.

### Papers

| arXiv ID | Paper | Year | Category-Specific Notes |
|----------|-------|------|------------------------|
| 2312.09948 | GEAR-Up: Query Expansion + Knowledge Graph | 2023 | KG-based query expansion for retrieval enhancement |
| 2503.04629 | SurveyForge: Outline Heuristics + Memory-Driven Generation | 2025 | Bidirectional citation chaining + cross-section memory |
| 2508.17647 | SurveyGen: Quality-Aware RAG Framework | 2025 | Citation graph traversal + quality estimation model for guided refinement |
| 2510.07733 | SurveyG: Multi-Agent with Hierarchical Citation Graph | 2025 | Three-tier (Foundation/Development/Frontier) hierarchical graph |
| 2510.26750 | ProfOlaf: Semi-Automated SLR with Snowballing | 2025 | Explicit citation snowballing following SLR protocols |

### Shared Characteristics
- Citation graph traversal is a core retrieval strategy, not an add-on
- Graph awareness enables discovery of papers beyond keyword search
- All use BFS-level chaining; SurveyG adds hierarchical organization
- All are single-pass or have limited iteration

### Distinguishing Features (intra-category variation)
- **Graph depth**: BFS single-hop (SurveyForge, SurveyGen, ProfOlaf) vs hierarchical tiers (SurveyG)
- **Agent count**: single-agent (GEAR-Up, SurveyForge, SurveyGen, ProfOlaf) vs multi-agent (SurveyG)
- **Graph role**: retrieval expansion only (SurveyForge, SurveyGen) vs outline organization (SurveyG) vs snowballing protocol (ProfOlaf) vs query expansion (GEAR-Up)
- **KG vs citation graph**: GEAR-Up uses a knowledge graph for entities; others use citation graph for paper discovery

---

## Category: Hybrid Interactive

### Definition
Systems where the human user plays an active role in the generation process. The human may validate intermediate outputs, provide guidance, or make decisions that shape the final survey. These systems trade full automation for transparency and user control.

### Papers

| arXiv ID | Paper | Year | Category-Specific Notes |
|----------|-------|------|------------------------|
| 2309.01684 | CRUISE-Screening: Living Literature Review Toolbox | 2023 | Continuous living review with human oversight; NLP-based screening |
| 2410.15978 | PROMPTHEUS: Human-Centered SLR Pipeline | 2024 | PRISMA-compliant pipeline with human validation at every stage |
| 2504.08762 | InteractiveSurvey: Personalized and Interactive Survey Generation | 2025 | Three-stage interaction (reference categorization, outline refinement, draft review) |

### Shared Characteristics
- Human-in-the-loop at critical decision points
- User can guide, validate, or override system outputs
- Designed for transparency and user control rather than full automation
- Follow established SLR or review methodology (PRISMA for PROMPTHEUS, living review protocols for CRUISE-Screening)

### Distinguishing Features (intra-category variation)
- **Interaction depth**: validation-only (PROMPTHEUS, CRUISE-Screening) vs active guidance (InteractiveSurvey)
- **Automation level**: CRUISE-Screening is mostly automated with human validation; PROMPTHEUS requires human decisions at each stage; InteractiveSurvey has deep user engagement throughout
- **Domain focus**: CRUISE-Screening is SLR-focused; PROMPTHEUS is software engineering; InteractiveSurvey is general-purpose

---

## Category: Iterative Refinement

### Definition
Systems where iterative refinement is the core architectural pattern — output is generated, evaluated, and revised in multiple rounds. This can operate at different granularities: per-sentence, per-section, or per-draft. The refinement distinguishes these from single-pass pipelines.

### Papers

| arXiv ID | Paper | Year | Category-Specific Notes |
|----------|-------|------|------------------------|
| 2407.01796 | ReClaim: Interleaved Reference-Claim Generation | 2024 | Per-sentence iteration (retrieve → generate → verify) |
| 2510.21900 | IterSurvey: Deep Literature Survey with Iterative Workflow | 2025 | Recurrent outline refinement with paper card grounding |

### Shared Characteristics
- Iterative refinement is the defining architectural feature
- Refinement is driven by explicit quality signals (verification, coverage analysis)
- Loop continues until quality criteria are met

### Distinguishing Features (intra-category variation)
- **Granularity**: per-sentence (ReClaim) vs per-draft outline (IterSurvey)
- **Scope**: ReClaim operates at micro-level (sentence-by-sentence verification); IterSurvey operates at macro-level (outline refinement)
- **Scale**: ReClaim is designed for QA/summarization; IterSurvey is designed for full survey generation

---

## Cross-Category Comparison Matrix

### Primary Dimension: Architecture Type

| Architecture | Papers | Core Pattern |
|-------------|--------|-------------|
| Single-Agent Pipeline | 54 | Sequential single-LLM pipeline |
| Multi-Agent Pipeline | 13 | Specialized agent roles with coordination |
| Graph-Enhanced Retrieval | 5 | Citation graph traversal as core retrieval |
| Hybrid Interactive | 3 | Human-in-the-loop at decision points |
| Iterative Refinement | 2 | Multi-round generate-evaluate-revise loop |

### Secondary Dimension: Citation Graph Awareness x Architecture

| | None | BFS (Simple Chaining) | Graph Traversal | Hierarchical |
|---|---|---|---|---|
| **Single-Agent** | 50 papers (majority) | — | 1 (GEAR-Up — KG-based) | — |
| **Multi-Agent** | 11 (most) | 2 (ARISE, SciSage) | — | — |
| **Graph-Enhanced** | — | 3 (SurveyForge, SurveyGen, ProfOlaf) | 1 (GEAR-Up) | 1 (SurveyG) |
| **Hybrid Interactive** | 1 (PROMPTHEUS) | 1 (InteractiveSurvey) | — | — |
| **Iterative Refinement** | 1 (ReClaim) | 1 (IterSurvey) | — | — |

### Secondary Dimension: Iteration Strategy x Architecture

| | Single Pass | Multi-Round | Interactive |
|---|---|---|---|
| **Single-Agent** | 48 | 5 (SurveyGen, ScholarCopilot, CiteSee, DimInd, SurveyX) | — |
| **Multi-Agent** | 2 (DOVA, ResearchPilot) | 11 (most) | — |
| **Graph-Enhanced** | 4 (GEAR-Up, SurveyForge, SurveyG, SurveyGen) | 1 (ProfOlaf) | — |
| **Hybrid Interactive** | — | — | 3 (all) |
| **Iterative Refinement** | — | 2 (ReClaim, IterSurvey) | — |

---

## Category Rationale Notes

### Why specific papers were placed where they are

**SurveyGen** (2508.17647) and **SurveyForge** (2503.04629) are placed in `graph_enhanced_retrieval` rather than `single_agent_pipeline` because their citation graph traversal is an integral retrieval component that distinguishes them from purely search-based approaches. SurveyG (2510.07733) is placed here because its three-tier hierarchical graph is the defining architectural contribution.

**AutoSurvey2** (2510.26012) is placed in `multi_agent_pipeline` because it uses multiple parallel LLM instances for section drafting, qualifying as multi-agent even though agents are homogeneous (same LLM, different contexts). This contrasts with AutoSurvey (2406.10252) which is single-agent.

**ReClaim** (2407.01796) and **IterSurvey** (2510.21900) form a natural `iterative_refinement` category because iteration is their defining characteristic — ReClaim at the per-sentence level, IterSurvey at the per-draft outline level.

**InteractiveSurvey** (2504.08762), **PROMPTHEUS** (2410.15978), and **CRUISE-Screening** (2309.01684) are placed in `hybrid_interactive` because the human user is an essential component of the generation process, not an optional reviewer.

### Benchmark/Dataset Papers
All benchmark, dataset, and evaluation papers default to `single_agent_pipeline` as they provide evaluation infrastructure rather than novel survey generation methods. They are classified by their content type (benchmark, dataset, evaluation methodology) rather than by architectural contribution.

---

## Supplementary: Infrastructure & Theory Papers (Supervisor Retrieval R2)

These papers were retrieved by the Supervisor during Round 2 to fill knowledge gaps about (a) why multi-hop citation reasoning is hard and (b) reproducibility in automated literature synthesis.

| arXiv ID | Paper | Year | Category | Relevance |
|----------|-------|------|----------|-----------|
| 2605.22878 | SciAtlas: Large-Scale KG for Automated Scientific Research | 2026 | KG Infrastructure | Explicitly identifies that current retrieval lacks topological reasoning needed for multi-hop citation analysis. Provides neuro-symbolic retrieval as alternative. |
| 2508.04612 | A Reproducible, Scalable Pipeline for Synthesizing Autoregressive Model Literature | 2025 | Reproducibility Infrastructure | First fully open-source, reproducible pipeline for automated literature synthesis. Demonstrates F1>0.85 for relevance classification and near-linear scalability, highlighting that current ASG systems lack comparable reproducibility infrastructure. |
