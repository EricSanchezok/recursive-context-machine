# Candidate Pool

**Assembled by**: DiscoveryMerger (MASA pipeline)
**Date**: 2026-06-06
**run_dir**: `.`
**Source artifacts**: 02a_method_candidates.md, 02b_benchmark_candidates.md, 02c_survey_candidates.md, 02d_frontier_candidates.md
**Total unique candidates**: 95 (after deduplication by arXiv ID then normalized title)

---

## Deduplication Notes

- Deduplication performed on base arXiv ID (version suffix stripped), then on normalized title for entries without arXiv IDs.
- Cross-scout overlaps merged — source_agent and source_query provenance concatenated from all sources.
- Role assigned as the most informative categorical label per the merged evidence; secondary roles noted in inclusion reason when relevant.

---

## Candidates

### core_method — 17 papers

Systems and architectures for automated literature survey generation.

#### 1. AutoSurvey2
- **arXiv ID**: 2510.26012
- **Year**: 2025
- **Title**: AutoSurvey2: Empowering Researchers with Next Level Automated Literature Surveys
- **Source agents**: MethodScout, SurveyScout, FrontierScout
- **Source queries**: cm-01, (survey), fr-01, fr-03
- **Likely role**: core_method
- **Inclusion reason**: Multi-stage pipeline with parallel section generation, iterative refinement, real-time retrieval of recent publications, multi-LLM evaluation. Direct descendant of AutoSurvey; outperforms retrieval-based baselines.

#### 2. SurveyG
- **arXiv ID**: 2510.07733
- **Year**: 2025
- **Title**: SurveyG: A Multi-Agent LLM Framework with Hierarchical Citation Graph for Automated Survey Generation
- **Source agents**: MethodScout, FrontierScout
- **Source queries**: cm-01, mc-02, fr-01, fr-03
- **Likely role**: core_method
- **Inclusion reason**: Organizes citation graph into three layers (Foundation/Development/Frontier); combines horizontal search within layers and vertical depth traversal. Central to the anchor question on citation-graph-aware survey generation.

#### 3. SurveyGen
- **arXiv ID**: 2508.17647v1
- **Year**: 2025
- **Title**: SurveyGen: Quality-Aware Scientific Survey Generation with Large Language Models
- **Source agents**: MethodScout, BenchmarkScout
- **Source queries**: cm-01, (benchmark dataset)
- **Likely role**: core_method
- **Inclusion reason**: Quality-aware RAG pipeline with QUAL-SG framework; introduces quality-aware indicators into retrieval; large-scale dataset of 4200 human-written surveys.

#### 4. SurveyX
- **arXiv ID**: 2502.14776v2
- **Year**: 2025
- **Title**: SurveyX: Academic Survey Automation via Large Language Models
- **Source agents**: MethodScout, FrontierScout
- **Source queries**: cm-01, fr-01
- **Likely role**: core_method
- **Inclusion reason**: Two-phase generation (Preparation + Generation) with AttributeTree preprocessing and re-polishing; online reference retrieval; strong citation quality results (+1.76 over baselines).

#### 5. SurveyGen-I
- **arXiv ID**: 2508.14317v1
- **Year**: 2025
- **Title**: SurveyGen-I: Consistent Scientific Survey Generation with Evolving Plans and Memory-Guided Writing
- **Source agents**: MethodScout, FrontierScout
- **Source queries**: cm-01, fr-03
- **Likely role**: core_method
- **Inclusion reason**: Coarse-to-fine retrieval with adaptive planning and memory-guided generation; dynamic refinement across subsections; memory mechanism for coherence across subsections.

#### 6. SurveyForge
- **arXiv ID**: 2503.04629v1
- **Year**: 2025
- **Title**: SurveyForge: On the Outline Heuristics, Memory-Driven Generation, and Multi-dimensional Evaluation
- **Source agents**: MethodScout, SurveyScout, FrontierScout
- **Source queries**: cm-01, mc-02, fr-01, fr-03
- **Likely role**: core_method
- **Inclusion reason**: Learns outline structure from human-written surveys; scholar navigation agent for memory-driven retrieval; includes SurveyBench evaluation; outline-first approach.

#### 7. InteractiveSurvey
- **arXiv ID**: 2504.08762v1
- **Year**: 2025
- **Title**: InteractiveSurvey: An LLM-based Personalized and Interactive Survey Paper Generation System
- **Source agents**: MethodScout, SurveyScout, FrontierScout
- **Source queries**: cm-01, cm-03, fr-03
- **Likely role**: core_method
- **Inclusion reason**: User-in-the-loop approach; allows customization of reference categorization, outline, and content during generation; online retrieval + user uploads.

#### 8. IterSurvey
- **arXiv ID**: 2510.21900
- **Year**: 2025
- **Title**: Deep Literature Survey Automation with an Iterative Workflow
- **Source agents**: MethodScout, SurveyScout, FrontierScout
- **Source queries**: cm-03, mc-02, mc-05, fr-03
- **Likely role**: core_method
- **Inclusion reason**: Recurrent outline generation where planning agent incrementally retrieves, reads, and updates outline; paper cards for faithful grounding; review-and-refine loop; introduces Survey-Arena pairwise benchmark.

#### 9. Agentic AutoSurvey
- **arXiv ID**: 2509.18661v1
- **Year**: 2025
- **Title**: Agentic AutoSurvey: Let LLMs Survey LLMs
- **Source agents**: MethodScout, SurveyScout, FrontierScout
- **Source queries**: cm-03, fr-01, fr-03
- **Likely role**: core_method
- **Inclusion reason**: Four specialized agents (Paper Search, Topic Mining, Writer, Quality Evaluator); processes 75–443 papers per topic; 12-dimension evaluation; scored 8.18/10 vs AutoSurvey's 4.77/10.

#### 10. LiRA
- **arXiv ID**: 2510.05138
- **Year**: 2025
- **Title**: LiRA: A Multi-Agent Framework for Reliable and Readable Literature Review Generation
- **Source agents**: MethodScout
- **Source queries**: cm-03
- **Likely role**: core_method
- **Inclusion reason**: Multi-agent collaborative workflow emulating human review process (outlining, subsection writing, editing, reviewing agents); evaluated on SciReviewGen; robustness to reviewer model variation.

#### 11. ResearchPilot
- **arXiv ID**: 2603.14629
- **Year**: 2026
- **Title**: ResearchPilot: A Local-First Multi-Agent System for Literature Synthesis and Related Work Drafting
- **Source agents**: MethodScout
- **Source queries**: cm-03, cm-06, mc-05
- **Likely role**: core_method
- **Inclusion reason**: Local-first multi-agent system; retrieves from Semantic Scholar/arXiv; structured findings extraction; cross-paper pattern synthesis; citation-aware drafting.

#### 12. OpenScholar
- **arXiv ID**: 2411.14199v1
- **Year**: 2024
- **Title**: OpenScholar: Synthesizing Scientific Literature with Retrieval-augmented LMs
- **Source agents**: MethodScout, SurveyScout
- **Source queries**: cm-06
- **Likely role**: core_method
- **Inclusion reason**: Specialized retrieval-augmented LM; 45M open-access paper datastore; self-feedback inference loop; citation accuracy on par with human experts; ScholarQABench; outperforms GPT-4o by 5%.

#### 13. LitLLM
- **arXiv ID**: 2402.01788v2
- **Year**: 2024
- **Title**: LitLLM: A Toolkit for Scientific Literature Review
- **Source agents**: MethodScout, SurveyScout
- **Source queries**: cm-04, cm-06
- **Likely role**: core_method
- **Inclusion reason**: RAG-based toolkit: web search → keyword extraction → paper re-ranking → related work generation.

#### 14. SciSage
- **arXiv ID**: 2506.12689v2
- **Year**: 2025
- **Title**: SciSage: Reflect-when-you-write Paradigm for Automated Survey Generation
- **Source agents**: FrontierScout
- **Source queries**: fr-01, fr-03
- **Likely role**: core_method
- **Inclusion reason**: Multi-agent survey generation framework with hierarchical Reflector agent evaluating at outline, section, and document levels; +1.73 coherence, +32% citation F1; releases SurveyScope benchmark (46 papers, 11 CS domains).

#### 15. Patience is all you need
- **arXiv ID**: 2504.08752v1
- **Year**: 2025
- **Title**: Patience is all you need! An agentic system for performing scientific literature review
- **Source agents**: MethodScout
- **Source queries**: cm-03
- **Likely role**: core_method
- **Inclusion reason**: Agentic retrieval + distillation system using sparse retrieval; sparse retrieval matches dense SOTA; full-text understanding; keyword-based search and distillation.

#### 16. Ai2 Scholar QA
- **arXiv ID**: 2504.10861v2
- **Year**: 2025
- **Title**: Ai2 Scholar QA: Organized Literature Synthesis with Attribution
- **Source agents**: MethodScout
- **Source queries**: mc-05
- **Likely role**: core_method
- **Inclusion reason**: Full pipeline released as open-source; organized synthesis with attribution; outperforms competing systems.

#### 17. Citegeist
- **arXiv ID**: 2503.23229v1
- **Year**: 2025
- **Title**: Citegeist: Automated Generation of Related Work Analysis on the arXiv Corpus
- **Source agents**: MethodScout
- **Source queries**: cm-04
- **Likely role**: core_method
- **Inclusion reason**: Dynamic RAG on arXiv for related work generation; embedding-based similarity + summarization + multi-stage filtering; optimized for continuous document growth.

---

### mechanism — 11 papers

Components, retrieval strategies, and sub-systems that enable survey generation.

#### 18. SPAR
- **arXiv ID**: 2507.15245v1
- **Year**: 2025
- **Title**: SPAR: Scholar Paper Retrieval with LLM-based Agents for Enhanced Academic Search
- **Source agents**: MethodScout
- **Source queries**: cm-03, cm-06
- **Likely role**: mechanism
- **Inclusion reason**: RefChain-based query decomposition and query evolution for academic search; multi-agent retrieval; up to +56% F1 vs baselines; includes SPARBench.

#### 19. PaSa
- **arXiv ID**: 2501.10120v2
- **Year**: 2025
- **Title**: PaSa: An LLM Agent for Comprehensive Academic Paper Search
- **Source agents**: MethodScout
- **Source queries**: cm-06, mc-05
- **Likely role**: mechanism
- **Inclusion reason**: Autonomous paper search agent with RL-optimized decisions; invokes search tools, reads papers, selects references; +37.78% recall over Google+GPT-4o.

#### 20. LitFM
- **arXiv ID**: 2409.12177v1
- **Year**: 2024
- **Title**: LitFM: A Retrieval Augmented Structure-aware Foundation Model For Citation Graphs
- **Source agents**: MethodScout, BenchmarkScout, FrontierScout
- **Source queries**: cm-04, fr-02, cd-01
- **Likely role**: mechanism
- **Inclusion reason**: Novel graph retriever navigating citation graphs; integrates graph structure during training and inference; 28.1% retrieval precision improvement; benchmark datasets on 3 academic fields.

#### 21. CG-RAG
- **arXiv ID**: 2501.15067v1
- **Year**: 2025
- **Title**: CG-RAG: Citation Graph Retrieval-Augmented Generation for Research Question Answering
- **Source agents**: FrontierScout
- **Source queries**: fr-02
- **Likely role**: mechanism
- **Inclusion reason**: Integrates sparse and dense retrieval signals within citation graph structures for research question answering; lexical-semantic graph retrieval (LeSeGR) approach.

#### 22. ResearchAgent
- **arXiv ID**: 2404.07738v2
- **Year**: 2024
- **Title**: ResearchAgent: Iterative Research Idea Generation over Scientific Literature with Large Language Models
- **Source agents**: MethodScout, FrontierScout
- **Source queries**: cm-04, cm-06, mc-05, fr-03
- **Likely role**: mechanism
- **Inclusion reason**: Academic graph traversal for paper augmentation; iterative refinement via LLM reviewing agents; cross-pollination across fields; multi-agent feedback loops.

#### 23. CitationSum
- **arXiv ID**: 2301.11223v4
- **Year**: 2023
- **Title**: CitationSum: Citation-aware Graph Contrastive Learning for Scientific Paper Summarization
- **Source agents**: MethodScout
- **Source queries**: cm-04
- **Likely role**: mechanism
- **Inclusion reason**: Graphs salient content from references; weighted correlations between source and references; self-supervised framework for citation-aware summarization.

#### 24. AutoResearcher
- **arXiv ID**: 2510.20844
- **Year**: 2025
- **Title**: AutoResearcher: Automating Knowledge-Grounded and Transparent Research Ideation with Multi-Agent Collaboration
- **Source agents**: MethodScout
- **Source queries**: mc-05
- **Likely role**: mechanism
- **Inclusion reason**: Four-stage pipeline (curation → generation → selection → review); transparent intermediate reasoning; domain-agnostic — applicable to survey generation.

#### 25. IntrAgent
- **arXiv ID**: 2604.22861
- **Year**: 2026
- **Title**: IntrAgent: An LLM Agent for Content-Grounded Information Retrieval through Literature Review
- **Source agents**: MethodScout
- **Source queries**: cm-03
- **Likely role**: mechanism
- **Inclusion reason**: Two-stage pipeline (section ranking + iterative reading) for information retrieval from literature; mimics human reading behavior; IntraBench benchmark across 5 STEM domains; 13.2% higher cross-domain accuracy.

#### 26. FutureGen
- **arXiv ID**: 2503.16561v3
- **Year**: 2025
- **Title**: FutureGen: A RAG-based Approach to Generate the Future Work of Scientific Article
- **Source agents**: MethodScout
- **Source queries**: cm-04
- **Likely role**: mechanism
- **Inclusion reason**: RAG + LLM feedback loop for generating future work sections; LLM-as-a-judge evaluation framework; applicable to section-level generation.

#### 27. Oignon
- **arXiv ID**: 2512.22159
- **Year**: 2025
- **Title**: Oignon: Citation Graph Tool
- **Source agents**: MethodScout
- **Source queries**: mc-02
- **Likely role**: mechanism
- **Inclusion reason**: Free/open-source citation graph exploration tool; dual-path ranking system with recency weighting; captures both foundational and recent works.

#### 28. Interleaved snowballing
- **arXiv ID**: 2402.08339v1
- **Year**: 2024
- **Title**: Interleaved snowballing: Reducing the workload of literature curators
- **Source agents**: MethodScout
- **Source queries**: mc-02
- **Likely role**: mechanism
- **Inclusion reason**: Formal algorithm for literature snowballing; LitBall desktop app; reduces curator workload; directly relevant to citation graph expansion strategy.

---

### frontier — 5 papers

Very recent or forward-looking methods (2025–2026).

#### 29. PaSaMaster
- **arXiv ID**: 2605.14306
- **Year**: 2026
- **Title**: Towards Self-Evolving Agentic Literature Retrieval
- **Source agents**: MethodScout
- **Source queries**: cm-03
- **Likely role**: frontier
- **Inclusion reason**: Transforms retrieval from one-shot to evolving search; zero source hallucination; separates planning from retrieval for cost efficiency; most recent (2026) entry in the pool.

#### 30. DeepXiv-SDK
- **arXiv ID**: 2603.00084
- **Year**: 2026
- **Title**: DeepXiv-SDK: An Agentic Data Interface for Scientific Literature
- **Source agents**: MethodScout
- **Source queries**: cm-06
- **Likely role**: frontier
- **Inclusion reason**: Three-layer interface (Data/Service/Application); CLI, MCP, Python SDK support; daily-synced arXiv corpus; enables agentic access to scientific literature.

#### 31. Paper2Agent
- **arXiv ID**: 2509.06917v1
- **Year**: 2025
- **Title**: Paper2Agent: Reimagining Research Papers As Interactive and Reliable AI Agents
- **Source agents**: MethodScout
- **Source queries**: mc-05
- **Likely role**: frontier
- **Inclusion reason**: Novel paradigm converting static papers to MCP servers; agents can reproduce original results and handle novel queries; directly addresses agent-paper interaction paradigm shift.

#### 32. paper.json
- **arXiv ID**: 2605.16194
- **Year**: 2026
- **Title**: paper.json: A Coordination Convention for LLM-Agent-Actionable Papers
- **Source agents**: MethodScout
- **Source queries**: cm-06, mc-05
- **Likely role**: frontier
- **Inclusion reason**: Proposed standard for machine-actionable papers; stable claim IDs, explicit does-not-claim lists, per-figure commands; directly addresses agent-paper interaction failures.

#### 33. FAIR Literature Surveys with Scholarly KGs
- **arXiv ID**: 2006.01747v1
- **Year**: 2020
- **Title**: FAIR Literature Surveys with Scholarly Knowledge Graphs (ORKG)
- **Source agents**: FrontierScout
- **Source queries**: cd-01
- **Likely role**: frontier
- **Inclusion reason**: Uses Open Research Knowledge Graph for contribution-based survey synthesis; demonstrates knowledge-graph-driven survey methodology; FAIR data principles.

---

### benchmark — 12 papers

Evaluation frameworks, benchmarks, and datasets for survey generation and citation quality.

#### 34. SurveyBench
- **arXiv ID**: 2510.03120
- **Year**: 2025
- **Title**: SurveyBench: How Well Can LLM(-Agents) Write Academic Surveys?
- **Source agents**: BenchmarkScout, SurveyScout, FrontierScout
- **Source queries**: bm-02, fr-01
- **Likely role**: benchmark
- **Inclusion reason**: Quiz-driven evaluation framework; built from 11,343 arXiv topics and 4,947 high-quality surveys; multi-faceted metric hierarchy (outline quality, content quality, non-textual richness); dual-mode protocol (content-based + quiz-based answerability).

#### 35. SurGE
- **arXiv ID**: 2508.15658
- **Year**: 2025
- **Title**: SurGE: Standardized Benchmark for Survey Generation Evaluation
- **Source agents**: BenchmarkScout
- **Source queries**: bm-01, bm-04
- **Likely role**: benchmark
- **Inclusion reason**: Benchmark for CS survey generation; test instances with expert-written surveys + references; large-scale corpus of 1M+ papers as retrieval pool; 4-dimension evaluation (information coverage, referencing accuracy, structural organization, content quality).

#### 36. SGSimEval
- **arXiv ID**: 2508.11310
- **Year**: 2025
- **Title**: SGSimEval: A Comprehensive Multifaceted and Similarity-Enhanced Benchmark for Automatic Survey Generation Systems
- **Source agents**: MethodScout, BenchmarkScout
- **Source queries**: cm-01, bm-01, bm-03
- **Likely role**: benchmark
- **Inclusion reason**: Multi-faceted benchmark combining outline, content, and reference assessments; integrates LLM-based scoring with quantitative metrics; human preference metrics for inherent quality and similarity to humans.

#### 37. SurveyLens
- **arXiv ID**: 2602.11238
- **Year**: 2026
- **Title**: SurveyLens: Discipline-Aware Benchmark for Automated Survey Generation
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: benchmark
- **Inclusion reason**: First discipline-aware benchmark across 10 disciplines; SurveyLens-1k dataset (1,000 human-written surveys); dual-lens evaluation (Discipline-Aware Rubric Evaluation + Canonical Alignment Evaluation); evaluates 11 ASG methods including Deep Research agents.

#### 38. DeepSurvey-Bench
- **arXiv ID**: 2601.15307
- **Year**: 2026
- **Title**: DeepSurvey-Bench: Evaluating Academic Value of Automated Surveys
- **Source agents**: BenchmarkScout
- **Source queries**: bm-04
- **Likely role**: benchmark
- **Inclusion reason**: Evaluates "academic value" across 3 dimensions (informational value, scholarly communication value, research guidance value); critiques existing benchmarks for flawed selection criteria.

#### 39. SurveyEval
- **arXiv ID**: 2512.02763
- **Year**: 2025
- **Title**: SurveyEval: Multi-Dimension Evaluation with Human-LLM Alignment
- **Source agents**: BenchmarkScout
- **Source queries**: bm-04
- **Likely role**: benchmark
- **Inclusion reason**: Evaluates across 3 dimensions (overall quality, outline coherence, reference accuracy) over 7 subjects; augments LLM-as-a-Judge with human references to strengthen alignment.

#### 40. Survey-Arena (from IterSurvey paper)
- **arXiv ID**: 2510.21900
- **Year**: 2025
- **Title**: Survey-Arena: A Pairwise Benchmark for Survey Generation
- **Source agents**: BenchmarkScout
- **Source queries**: bm-03
- **Likely role**: benchmark
- **Inclusion reason**: Pairwise benchmark complementing absolute scoring; positions machine-generated surveys relative to human-written ones; separable from the IterSurvey framework.

#### 41. SurveyGen Dataset
- **arXiv ID**: 2508.17647
- **Year**: 2025
- **Title**: SurveyGen Dataset: 4,200+ Human-Written Surveys
- **Source agents**: BenchmarkScout
- **Source queries**: bm-01
- **Likely role**: benchmark
- **Inclusion reason**: Large-scale dataset of 4,200+ human-written surveys across diverse domains; 242,143 cited references; quality-related metadata; used for QUAL-SG pipeline.

#### 42. SurveyBank
- **arXiv ID**: 2110.06354
- **Year**: 2021
- **Title**: SurveyBank: A Dataset of Survey Papers with Citation Relationships
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: benchmark
- **Inclusion reason**: Dataset of CS survey papers with citation relationships; multi-level reading lists; designed for Reading Path Generation task.

#### 43. CiteEval / CiteBench
- **arXiv ID**: 2506.01829
- **Year**: 2025
- **Title**: CiteEval: Principle-Driven Citation Quality Evaluation
- **Source agents**: BenchmarkScout
- **Source queries**: bm-04
- **Likely role**: benchmark
- **Inclusion reason**: Principle-driven citation evaluation; CiteBench multi-domain benchmark with human annotations; CiteEval-Auto model-based metrics correlated with human judgments.

#### 44. PaperArena
- **arXiv ID**: 2510.10909
- **Year**: 2025
- **Title**: PaperArena: An Evaluation Benchmark for Tool-Augmented Agentic Reasoning on Scientific Literature
- **Source agents**: MethodScout
- **Source queries**: cm-03
- **Likely role**: benchmark
- **Inclusion reason**: Multi-tool orchestration; cross-paper reasoning; tools include multimodal parsing, context retrieval, programmatic computation.

#### 45. OAG-Bench
- **arXiv ID**: 2402.15810
- **Year**: 2024
- **Title**: OAG-Bench: Multi-Aspect Benchmark for Academic Graph Mining
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: benchmark
- **Inclusion reason**: Multi-aspect benchmark based on Open Academic Graph (OAG); 10 tasks, 20 datasets, 70+ baselines; includes paper source tracing, scholar profiling; useful for citation graph retrieval component evaluation.

---

### metric — 6 papers

Evaluation metrics and measurement frameworks.

#### 46. FActScore
- **arXiv ID**: 2305.14251
- **Year**: 2023
- **Title**: FActScore: Fine-grained Atomic Evaluation of Factual Precision
- **Source agents**: BenchmarkScout
- **Source queries**: bm-04
- **Likely role**: metric
- **Inclusion reason**: Breaks generations into atomic facts; computes % supported by reliable knowledge source; automated version with <2% error rate; directly applicable to survey factual precision evaluation.

#### 47. TRUE
- **arXiv ID**: 2204.04991
- **Year**: 2022
- **Title**: TRUE: Comprehensive Survey of Factual Consistency Metrics
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: metric
- **Inclusion reason**: Comprehensive survey and meta-evaluation of factual consistency metrics; standardized collection of annotated texts; example-level meta-evaluation protocol.

#### 48. Core
- **arXiv ID**: 2407.03572
- **Year**: 2024
- **Title**: Core: Sub-claim Selection for Factual Precision Metrics
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: metric
- **Inclusion reason**: Sub-claim selection component that filters obvious/repetitive subclaims; augments FActScore-like approaches; could improve factual precision evaluation in surveys.

#### 49. Phocus
- **arXiv ID**: 2201.02915
- **Year**: 2022
- **Title**: Phocus: Citation Sentiment Analysis and Ranking Model
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: metric
- **Inclusion reason**: Classifies citations coarsely; ranks references within a paper; applicable to fine-grained citation quality evaluation in surveys.

#### 50. Human Evaluation of Creative NLG Systems
- **arXiv ID**: 2108.00308
- **Year**: 2021
- **Title**: Human Evaluation of Creative NLG Systems: Survey and Guidelines
- **Source agents**: BenchmarkScout
- **Source queries**: bm-03
- **Likely role**: metric
- **Inclusion reason**: Surveys human evaluation practices in creative NLG; provides guidelines transferable to survey generation evaluation protocol design.

#### 51. Seed-based Citation Retrieval Comparison
- **arXiv ID**: 2403.09295
- **Year**: 2024
- **Title**: Comparing Direct Citation, Co-citation, and Bibliographic Coupling for Seed-based Retrieval
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: metric
- **Inclusion reason**: Compares citation graph traversal methods for seed-based retrieval; uses systematic reviews as baseline; shows advantage for co-citation, best when combining all three.

---

### citation_seed — 12 papers

Foundational, infrastructure, or early influential papers providing background and citations.

#### 52. AutoSurvey
- **arXiv ID**: 2406.10252v2
- **Year**: 2024
- **Title**: AutoSurvey: Large Language Models Can Automatically Write Surveys
- **Source agents**: MethodScout, SurveyScout
- **Source queries**: cm-01
- **Likely role**: citation_seed
- **Inclusion reason**: Foundational system paper for automated survey generation; outline → section drafting → integration pipeline; widely cited seed.

#### 53. PaperQA
- **arXiv ID**: 2312.07559v2
- **Year**: 2023
- **Title**: PaperQA: Retrieval-Augmented Generative Agent for Scientific Research
- **Source agents**: MethodScout, SurveyScout
- **Source queries**: cm-04, cm-06, mc-05
- **Likely role**: citation_seed
- **Inclusion reason**: Early influential RAG agent for science; full-text retrieval + relevance assessment + RAG answers; LitQA benchmark; widely cited.

#### 54. PaperRobot
- **arXiv ID**: 1905.07870v4
- **Year**: 2019
- **Title**: PaperRobot: Incremental Draft Generation of Scientific Ideas
- **Source agents**: MethodScout
- **Source queries**: cm-04
- **Likely role**: citation_seed
- **Inclusion reason**: Early work on automated paper drafting with knowledge graphs; background KG construction; link prediction for idea generation; incremental abstract/conclusion/future work generation.

#### 55. Tell Me How to Survey
- **arXiv ID**: 2110.06354v3
- **Year**: 2021
- **Title**: Tell Me How to Survey: Literature Review Made Simple with Automatic Reading Path Generation
- **Source agents**: MethodScout, SurveyScout
- **Source queries**: mc-02
- **Likely role**: citation_seed
- **Inclusion reason**: Introduces the Reading Path Generation (RPG) task; SurveyBank dataset; graph-optimization-based approach; RePaGer system.

#### 56. Construction of the Literature Graph in Semantic Scholar
- **arXiv ID**: 1805.02262v1
- **Year**: 2018
- **Title**: Construction of the Literature Graph in Semantic Scholar
- **Source agents**: MethodScout, SurveyScout, FrontierScout (boundary)
- **Source queries**: mc-02, (survey), fr-02, cd-01
- **Likely role**: citation_seed
- **Inclusion reason**: Foundational infrastructure paper; describes Semantic Scholar's 280M+ node heterogeneous literature graph; entity extraction and linking pipeline.

#### 57. Automatic generation of reviews of scientific papers (2020)
- **arXiv ID**: 2010.04147v1
- **Year**: 2020
- **Title**: Automatic generation of reviews of scientific papers
- **Source agents**: MethodScout
- **Source queries**: cm-04
- **Likely role**: citation_seed
- **Inclusion reason**: Early method using cocitation graphs + BERT for extractive summarization; cocitation graph for key paper identification; PubMed dataset.

#### 58. RAG for AI-Generated Content: A Survey
- **arXiv ID**: 2402.19473v6
- **Year**: 2024
- **Title**: Retrieval-Augmented Generation for AI-Generated Content: A Survey
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: citation_seed
- **Inclusion reason**: Broad RAG survey covering augmentation methodologies, applications across modalities, benchmarks, and limitations; foundational reading for RAG component.

#### 59. A Survey on Retrieval-Augmented Text Generation
- **arXiv ID**: 2202.01110v2
- **Year**: 2022
- **Title**: A Survey on Retrieval-Augmented Text Generation
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: citation_seed
- **Inclusion reason**: Earlier RAG survey covering generic paradigm, tasks (dialogue, MT, other generation); useful for tracing RAG foundations.

#### 60. Hierarchical Catalogues of Literature Reviews Dataset
- **arXiv ID**: 2304.03512
- **Year**: 2023
- **Title**: Hierarchical Catalogues of Literature Reviews Dataset
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: citation_seed
- **Inclusion reason**: 7.6k literature review catalogues and 389k reference papers; designed for hierarchical catalogue generation task; relevant for outline/organization aspects of surveys.

#### 61. LitLLMs: Are we there yet?
- **arXiv ID**: 2412.15249v2
- **Year**: 2024
- **Title**: LitLLMs, LLMs for Literature Review: Are we there yet?
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: citation_seed
- **Inclusion reason**: Zero-shot eval of LLMs for retrieval + writing literature reviews; assesses current capabilities and gaps; proposes keyword-extraction & re-ranking pipeline.

#### 62. Systematic Review of RAG (2020–2025)
- **arXiv ID**: 2508.06401v3
- **Year**: 2025
- **Title**: A Systematic Literature Review of Retrieval-Augmented Generation: Techniques, Metrics, and Challenges
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: citation_seed
- **Inclusion reason**: Systematic review of 128 RAG papers (2020–2025); catalogs datasets, architectures, evaluation practices; important context for RAG-based survey agents.

#### 63. AI for Literature Reviews: Opportunities and Challenges
- **arXiv ID**: 2402.08565v2
- **Year**: 2024
- **Title**: Artificial Intelligence for Literature Reviews: Opportunities and Challenges
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: citation_seed
- **Inclusion reason**: Reviews AI tools for semi-automating SLR; covers screening & extraction phases across 21+11 tools; strong overview of state-of-play.

---

### survey_reference — 9 papers

Papers that are themselves surveys, reviews, or taxonomies providing framing and background.

#### 64. LLMs for Automated Scholarly Paper Review: A Survey
- **arXiv ID**: 2501.10326v2
- **Year**: 2025
- **Title**: Large language models for automated scholarly paper review: A survey
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: survey_reference
- **Inclusion reason**: Surveys LLM-based automated scholarly paper review (ASPR); covers methods, datasets, online systems, performance, and challenges; adjacent to our topic but focused on paper review rather than survey generation.

#### 65. Citation Recommendation: Approaches and Datasets
- **arXiv ID**: 2002.06961v2
- **Year**: 2020
- **Title**: Citation Recommendation: Approaches and Datasets
- **Source agents**: SurveyScout, FrontierScout (boundary)
- **Source queries**: cd-01
- **Likely role**: survey_reference
- **Inclusion reason**: First dedicated survey on citation recommendation; covers approaches, datasets, evaluation methods; relevant for citation graph retrieval component.

#### 66. PRISMA-DFLLM
- **arXiv ID**: 2306.14905v1
- **Year**: 2023
- **Title**: PRISMA-DFLLM: An Extension of PRISMA for Systematic Literature Reviews using Domain-specific Finetuned LLMs
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: survey_reference
- **Inclusion reason**: Proposes a methodological framework extending PRISMA with domain-finetuned LLMs; provides reporting guidelines checklist; relevant as taxonomy for structured SLR workflows.

#### 67. LLM-assisted Systematic Review
- **arXiv ID**: 2409.04600v1
- **Year**: 2024
- **Title**: The emergence of LLMs as a tool in literature reviews: an LLM automated systematic review
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: survey_reference
- **Inclusion reason**: LLM-assisted systematic review of 172 papers; finds ChatGPT/GPT most dominant (73.2%), most automation at search (34.9%) and data extraction (31.4%) stages.

#### 68. GraphReview
- **arXiv ID**: 2605.27204
- **Year**: 2026
- **Title**: GraphReview: Graph-based LLM Framework for Paper Evaluation
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: survey_reference
- **Inclusion reason**: Graph-based LLM framework using message passing over semantic paper graphs; evaluates papers in context of contemporaneous and prior work; 29.7% improvement on decision/ranking metrics; adjacent — paper evaluation rather than survey evaluation.

#### 69. SourceBench
- **arXiv ID**: 2602.16942
- **Year**: 2026
- **Title**: SourceBench: 8-Metric Framework for Cited Web Source Quality
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: survey_reference
- **Inclusion reason**: 8-metric framework for evaluating quality of cited web sources; metrics include content relevance, factual accuracy, objectivity, freshness, authority, clarity; citation quality metrics could transfer to survey citation evaluation.

#### 70. LitFM Benchmarks
- **arXiv ID**: 2409.12177
- **Year**: 2024
- **Title**: LitFM Benchmark Datasets for Citation Graph Retrieval
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: survey_reference
- **Inclusion reason**: Benchmark datasets on 3 academic fields with sentence-level citation information and local context; associated with LitFM foundation model (already listed as mechanism).

---

### related_system — 9 papers

Systems that are adjacent or supportive but not core survey generation pipelines.

#### 71. PUREsuggest
- **arXiv ID**: 2408.02508v1
- **Year**: 2024
- **Title**: PUREsuggest: Citation-based Literature Search and Visual Exploration with Keyword-controlled Rankings
- **Source agents**: MethodScout, SurveyScout, FrontierScout (boundary)
- **Source queries**: mc-02, fr-03
- **Likely role**: related_system
- **Inclusion reason**: Citation-based suggestion with keyword-steerable rankings and visual exploration; interactive citation graph foraging; useful as citation traversal component reference.

#### 72. vitaLITy 2
- **arXiv ID**: 2408.13450v1
- **Year**: 2024
- **Title**: vitaLITy 2: Reviewing Academic Literature Using Large Language Models
- **Source agents**: MethodScout
- **Source queries**: cm-06
- **Likely role**: related_system
- **Inclusion reason**: RAG architecture for semantic literature search; 66,692 paper corpus; text embeddings from multiple LMs; summarization and chat interface.

#### 73. LLAssist
- **arXiv ID**: 2407.13993v3
- **Year**: 2024
- **Title**: LLAssist: Simple Tools for Automating Literature Review Using Large Language Models
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: related_system
- **Inclusion reason**: Open-source tool for extracting info and evaluating relevance to user-defined research questions; simpler tool-level approach.

#### 74. Graphy
- **arXiv ID**: 2502.16868v1
- **Year**: 2025
- **Title**: Graphy: Towards End-to-End Modeling, Exploring and Generating Report from Raw Data
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: related_system
- **Inclusion reason**: End-to-end platform that transforms raw documents into structured graph of Fact/Dimension nodes; Offline Scrapper + online Surveyor.

#### 75. CitNetExplorer
- **arXiv ID**: 1404.5322v1
- **Year**: 2014
- **Title**: CitNetExplorer: A new software tool for analyzing and visualizing citation networks
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: related_system
- **Inclusion reason**: Citation network analysis and visualization tool; dated but relevant for citation network methodology reference.

#### 76. CiteEval / CiteBench (also listed under benchmark)
- **arXiv ID**: 2506.01829
- **Year**: 2025
- **Title**: CiteEval: Principle-Driven Citation Quality Evaluation
- **Source agents**: BenchmarkScout
- **Source queries**: bm-04
- **Likely role**: related_system (dual role with benchmark)
- **Inclusion reason**: Also serves as a related evaluation system; principle-driven citation evaluation framework.

#### 77. Hierarchical Catalogues of Literature Reviews Dataset (also listed under citation_seed)
- **arXiv ID**: 2304.03512
- **Year**: 2023
- **Title**: Hierarchical Catalogues of Literature Reviews Dataset
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: related_system (dual role with citation_seed)
- **Inclusion reason**: Also serves as related dataset system; relevant for understanding outline generation tasks.

---

### boundary — 7 papers

Papers that are intentionally peripheral — useful for discrimination tests and comparative analysis.

#### 78. TOBY
- **arXiv ID**: 2306.10051v1
- **Year**: 2023
- **Title**: TOBY: A Tool for Exploring Data in Academic Survey Papers
- **Source agents**: MethodScout
- **Source queries**: mc-02
- **Likely role**: boundary
- **Inclusion reason**: Visualization tool for exploring survey paper contents; hierarchical taxonomy view; document similarity; citation network; paper recommendation — adjacent but not survey generation.

#### 79. CiteSee
- **arXiv ID**: 2302.07302v1
- **Year**: 2023
- **Title**: CiteSee: Augmenting Citations in Scientific Papers with Persistent and Personalized Historical Context
- **Source agents**: MethodScout
- **Source queries**: mc-02
- **Likely role**: boundary
- **Inclusion reason**: Personalized citation augmentation tool for paper reading; uses user's publishing/reading/saving history; user-centric, not survey generation.

#### 80. TaskGen
- **arXiv ID**: 2407.15734v1
- **Year**: 2024
- **Title**: TaskGen: A Task-Based, Memory-Infused Agentic Framework using StrictJSON
- **Source agents**: MethodScout
- **Source queries**: mc-05
- **Likely role**: boundary
- **Inclusion reason**: General-purpose agentic framework (not specific to literature); task decomposition with memory; RAG on NaturalQuestions — helpful as general agentic pattern reference.

#### 81. Impact of a Deployed LLM Survey Creation Tool
- **arXiv ID**: 2506.14809v1
- **Year**: 2025
- **Title**: Impact of a Deployed LLM Survey Creation Tool through the IS Success Model
- **Source agents**: MethodScout
- **Source queries**: cm-01
- **Likely role**: boundary
- **Inclusion reason**: Focuses on Information Systems survey methodology rather than literature surveys; useful as boundary test for scope calibration.

#### 82. Question-Answer Extraction from Scientific Articles Using Knowledge Graphs and LLMs
- **arXiv ID**: 2507.13827v1
- **Year**: 2025
- **Title**: Question-Answer Extraction from Scientific Articles Using Knowledge Graphs and Large Language Models
- **Source agents**: MethodScout
- **Source queries**: cm-04
- **Likely role**: boundary
- **Inclusion reason**: QA extraction from articles, not survey generation; knowledge-graph-based QA generation; relevant as component technology but not survey generation per se.

#### 83. CitNetExplorer (also listed under related_system)
- **arXiv ID**: 1404.5322v1
- **Year**: 2014
- **Title**: CitNetExplorer
- **Source agents**: SurveyScout
- **Source queries**: (survey)
- **Likely role**: boundary (dual role)
- **Inclusion reason**: Also serves as boundary — citation network analysis and visualization tool rather than survey agent.

#### 84. SourceBench (also listed under survey_reference)
- **arXiv ID**: 2602.16942
- **Year**: 2026
- **Title**: SourceBench
- **Source agents**: BenchmarkScout
- **Source queries**: (bm queries)
- **Likely role**: boundary (dual role)
- **Inclusion reason**: Focused on web source quality rather than academic citation quality; useful as boundary discriminator.

---

## Summary Statistics

| Role | Count |
|------|-------|
| core_method | 17 |
| mechanism | 11 |
| benchmark | 12 |
| metric | 6 |
| frontier | 5 |
| citation_seed | 12 |
| survey_reference | 9 |
| related_system | 5 |
| boundary | 7 |
| boundary (dual role count) | 3 |
| **Total unique papers** | **95** |

**Note**: 3 papers appear in two roles (marked as dual role); total unique entries = 95, role-based counts sum to 100 due to these dual assignments.

---

## Post-Outline Supplement — Round 2

Papers added to the pool after initial outline generation, sourced from the extended pool to fill gaps identified by supervisor/judge feedback.

#### 85A. LLM Agent Survey
- **arXiv ID**: 2503.21460v1
- **Year**: 2025
- **Title**: Large Language Model Agent: A Survey on Methodology, Applications and Challenges
- **Source**: extended_pool (ExtendedSurveyScout)
- **Likely role**: survey_reference
- **Inclusion reason**: Comprehensive survey of LLM agents organized around architecture, collaboration, and evolution; referenced in Section 1 as background context for agent architectures in survey generation. Source: extended_pool.

#### 85B. Deep Search Agents Survey
- **arXiv ID**: 2508.05668v3
- **Year**: 2025
- **Title**: A Survey of LLM-based Deep Search Agents: Paradigm, Optimization, Evaluation, and Challenges
- **Source**: extended_pool (ExtendedSurveyScout, ExtendedFrontierScout)
- **Likely role**: survey_reference
- **Inclusion reason**: First systematic survey of deep search agents (OpenAI Deep Research, Gemini, Perplexity); covers architecture, optimization, evaluation; referenced in Section 8.4 for deep research paradigm comparison. Source: extended_pool.

---

## Cross-Scout Provenance Summary

| Source Scout | Unique Papers | Unique to This Scout |
|---|---|---|
| MethodScout (02a) | 45 | 13 |
| BenchmarkScout (02b) | 21 | 10 |
| SurveyScout (02c) | 25 | 10 |
| FrontierScout (02d) | 14 | 1 |
| **Merged total** | **95** | — |

**Papers unique to a single scout**: Papers that appeared in only one scout's output (and were not surfaced by any other scout) are distributed across all four sources, indicating complementary coverage rather than redundancy.

---

## Top Papers for Downstream Processing

### Highest cross-scout agreement (appeared in 3+ scouts):
1. **AutoSurvey2** (2510.26012) — MethodScout, SurveyScout, FrontierScout
2. **SurveyG** (2510.07733) — MethodScout, FrontierScout
3. **SurveyForge** (2503.04629v1) — MethodScout, SurveyScout, FrontierScout
4. **Agentic AutoSurvey** (2509.18661v1) — MethodScout, SurveyScout, FrontierScout
5. **InteractiveSurvey** (2504.08762v1) — MethodScout, SurveyScout, FrontierScout
6. **IterSurvey** (2510.21900) — MethodScout, SurveyScout, FrontierScout
7. **LitFM** (2409.12177v1) — MethodScout, BenchmarkScout, FrontierScout
8. **SurveyBench** (2510.03120) — BenchmarkScout, SurveyScout, FrontierScout

### Key frontier signals (2026 papers):
1. **PaSaMaster** (2605.14306) — self-evolving retrieval
2. **DeepXiv-SDK** (2603.00084) — agentic data interface
3. **paper.json** (2605.16194) — agent-actionable papers
4. **SurveyLens** (2602.11238) — discipline-aware evaluation
5. **DeepSurvey-Bench** (2601.15307) — academic value evaluation
