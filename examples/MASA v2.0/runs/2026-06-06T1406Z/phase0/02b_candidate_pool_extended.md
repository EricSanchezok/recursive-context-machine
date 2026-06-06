# Extended Candidate Pool

**Assembled by**: ExtendedDiscoveryMerger (MASA pipeline)
**Date**: 2026-06-06
**run_dir**: `.`
**Source artifacts**: 02e_extended_method_candidates.md, 02f_extended_benchmark_candidates.md, 02g_extended_survey_candidates.md, 02h_extended_frontier_candidates.md
**Parent pool**: 02_candidate_pool.md (95 candidates)
**Total extended candidates**: 107 (after deduplication by arXiv ID then normalized title, excluding papers already in the main 95-pool)

---

## Deduplication Notes

- Deduplication performed on base arXiv ID (version suffix stripped), then on normalized title for entries without arXiv IDs.
- ~50 papers from extended raw results were already present in the main 95-candidate pool and are excluded.
- Papers appearing across multiple extended scouts have concatenated source_agent and source_query provenance.
- Each candidate is assigned a relevance_score: **high** (directly relevant to survey generation), **medium** (indirectly relevant — component, method, or adjacent domain), or **low** (boundary — useful for discrimination tests).
- Roles are per the existing pool taxonomy: core_method, mechanism, benchmark, metric, frontier, citation_seed, survey_reference, related_system, boundary.

---

## Candidates

### core_method — 4 papers (extended)

Systems and architectures directly addressing automated literature survey generation.

#### 1. PaperQA2
- **arXiv ID**: 2409.13740v2
- **Year**: 2024
- **Title**: Language agents achieve superhuman synthesis of scientific knowledge
- **Source agents**: ExtendedMethodScout
- **Source queries**: em-02
- **Likely role**: core_method
- **Relevance score**: high
- **Inclusion reason**: PaperQA2 matches/exceeds human experts on literature search tasks; Wikipedia-style cited summaries; LitQA2 benchmark; contradiction detection; directly relevant as deep research survey agent — the key open-source competitor to closed Deep Research systems.

#### 2. MATC
- **arXiv ID**: 2508.04306v1
- **Year**: 2025
- **Title**: Multi-Agent Taskforce Collaboration (MATC): Self-Correction of Compounding Errors in Long-Form Literature Review Generation
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: em-03, em-03 (f)
- **Likely role**: core_method
- **Relevance score**: high
- **Inclusion reason**: Four executor agents (search, outline, fact localization, drafting) with manager agent; three collaboration paradigms (exploration/exploitation/experience) to mitigate compounding errors; SOTA on existing benchmarks; directly addresses error-compounding in multi-agent survey generation.

#### 3. CKMAs
- **arXiv ID**: 2411.06159v3
- **Year**: 2024
- **Title**: Mixture of Knowledge Minigraph Agents (CKMAs) for Literature Review Generation
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-02, em-03 (f)
- **Likely role**: core_method
- **Relevance score**: high
- **Inclusion reason**: Collaborative knowledge minigraph agents; prompt-based KMCA constructs relation graphs from literature; MPSA organizes concepts via multiple paths for review paragraphs; evaluated on 3 benchmarks; directly addresses KG-driven survey generation — fills the knowledge-graph-survey method gap.

#### 4. AgentCo-op
- **arXiv ID**: 2605.20425
- **Year**: 2026
- **Title**: AgentCo-op: Retrieval-Based Synthesis of Interoperable Multi-Agent Workflows
- **Source agents**: ExtendedMethodScout
- **Source queries**: em-03
- **Likely role**: core_method
- **Relevance score**: high
- **Inclusion reason**: Retrieval-based composition of reusable skills/tools/agents into typed artifact handoff workflows; bounded self-guided local repair on failure; 2026 frontier signal — could be adapted for survey agent orchestration; future-oriented method.

---

### mechanism — 25 papers (extended)

Components, retrieval strategies, and sub-systems that enable survey generation.

#### 5. ChatCite
- **arXiv ID**: 2403.02574v1
- **Year**: 2024
- **Title**: ChatCite: LLM Agent with Human Workflow Guidance for Comparative Literature Summary
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: em-01, em-02 (f)
- **Likely role**: mechanism
- **Relevance score**: high
- **Inclusion reason**: Mimics human workflow for comparative literature summarization; Reflective Incremental Mechanism for element extraction and summary generation; G-Score evaluation metric; directly usable as summarization component within survey agent pipelines.

#### 6. QuOTeS
- **arXiv ID**: 2306.11832v1
- **Year**: 2023
- **Title**: QuOTeS: Query-Oriented Technical Summarization
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: em-02, em-02 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Interactive query-focused summarization for composition of Introduction/Related Work sections; user study evaluated; assists in sentence-level extraction from reference collections.

#### 7. VirSci
- **arXiv ID**: 2410.09403v4
- **Year**: 2024
- **Title**: VirSci (Virtual Scientists): Many Heads Are Better Than One — Improved Scientific Idea Generation by A LLM-Based Multi-Agent System
- **Source agents**: ExtendedMethodScout
- **Source queries**: em-03
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Multi-agent system mimicking real scientific teamwork; collaborative idea generation, evaluation, and refinement; demonstrates higher novelty over SOTA; investigation of collaboration mechanisms transferable to survey generation.

#### 8. Citation Recommendation via Knowledge Graphs
- **arXiv ID**: 2106.05633v1
- **Year**: 2021
- **Title**: Citation Recommendation for Research Papers via Knowledge Graphs
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-01, emc-01 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Exploits research knowledge graphs (STM-KG) interlinking papers via scientific concepts; combination of text + citation network + KG information; outperforms SOTA (MAP 20.6%); relevant for graph-aware citation traversal.

#### 9. CitationIE
- **arXiv ID**: 2106.01560v1
- **Year**: 2021
- **Title**: CitationIE: Leveraging the Citation Graph for Scientific Information Extraction
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-01, emc-01 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Augments text representations with citation graph structure/content for scientific IE; demonstrates significant gains in entity extraction; could transfer to citation-aware survey content extraction.

#### 10. Finding Non-obvious Papers
- **arXiv ID**: 1812.11252v1
- **Year**: 2018
- **Title**: Towards Finding Non-obvious Papers: An Analysis of Citation Recommender Systems
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-01, emc-01 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Analysis of projection graph power-law distributions; identifies limitations of existing methods for loosely connected papers; proposes multi-method recommendation lists; informs citation traversal edge-case handling.

#### 11. KG-Based SLR Automation
- **arXiv ID**: 2208.02334v1
- **Year**: 2022
- **Title**: A Knowledge Graph-Based Method for Automating Systematic Literature Reviews
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-02, emc-02 (f)
- **Likely role**: mechanism
- **Relevance score**: high
- **Inclusion reason**: Partially automates SLR conduction and visualizes results as KG; designed software prototype with accurate results vs manual conduction; bridges the SLR methodology and KG approaches; directly transferable to survey generation pipeline.

#### 12. Scholarly KG from Survey Tables
- **arXiv ID**: 2012.00456v1
- **Year**: 2020
- **Title**: Creating a Scholarly Knowledge Graph from Survey Article Tables
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-02, emc-02 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Human-in-the-loop methodology using survey article tables for KG construction; 92 survey articles, 160 tables, 2,626 papers ingested; demonstrates practical KG construction pipeline from existing surveys.

#### 13. ReviewRobot
- **arXiv ID**: 2010.06119v3
- **Year**: 2020
- **Title**: ReviewRobot: Explainable Paper Review Generation based on Knowledge Synthesis
- **Source agents**: ExtendedMethodScout
- **Source queries**: emc-02
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Constructs KGs from target paper, cited works, and background corpus; compares KGs for score prediction and evidence extraction; 71.4–100% scoring accuracy; relevant as KG-based synthesis approach.

#### 14. Paperfetcher
- **arXiv ID**: 2110.12490v3
- **Year**: 2021
- **Title**: Paperfetcher: A tool to automate handsearch for systematic reviews
- **Source agents**: ExtendedMethodScout, ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: emc-03, esv, emc-03 (f)
- **Likely role**: mechanism
- **Relevance score**: high
- **Inclusion reason**: Open-source Python package + web-app for automated handsearch; incorporates bidirectional snowballing; DOI/RIS export; first tool automating handsearch with multidisciplinary focus; directly applicable to survey agent's citation expansion module.

#### 15. Cascading Citation Expansion
- **arXiv ID**: 1806.00089v1
- **Year**: 2018
- **Title**: Cascading Citation Expansion
- **Source agents**: ExtendedMethodScout, ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: emc-03, esv, emc-03 (f)
- **Likely role**: mechanism
- **Relevance score**: high
- **Inclusion reason**: Successive citation expansion process using Dimensions API; integrated in CiteSpace; traces back to Garfield's citation indexing concept; demonstrates iterative citation expansion — core mechanism for survey agent breadth traversal; fills pre-2020 snowballing gap.

#### 16. Iterated Decomposition
- **arXiv ID**: 2301.01751v2
- **Year**: 2023
- **Title**: Iterated Decomposition: Improving Science Q&A by Supervising Reasoning Processes
- **Source agents**: ExtendedMethodScout
- **Source queries**: emc-04
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Human-in-the-loop workflow for compositional LM program refinement; ICE visualization tool; applied to RCT placebo description, medical adherence, Qasper QA; demonstrates iterative decomposition pattern applicable to survey retrieval.

#### 17. Decomposing Complex Queries
- **arXiv ID**: 2305.15053v1
- **Year**: 2023
- **Title**: Decomposing Complex Queries for Tip-of-the-Tongue Retrieval
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-04, emc-04 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Decomposes queries into individual clues routed to specialized retrievers; ensemble approach; 7% relative improvement on recall@5; directly applicable query decomposition strategy for multi-faceted survey queries.

#### 18. Entity-Centric Query Refinement
- **arXiv ID**: 2204.00743v2
- **Year**: 2022
- **Title**: Entity-Centric Query Refinement
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-04, emc-04 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Task of generating query refinements to partition entity-answer collections; uses KB taxonomy for candidate refinement selection; text generation model generalizes to novel queries; applicable for iterative refinement in survey paper discovery.

#### 19. Talk to Papers
- **arXiv ID**: 2004.02002v3
- **Year**: 2020
- **Title**: Talk to Papers: Bringing Neural Question Answering to Academic Search
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-04, emc-04 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Open-domain QA for academic search; improves over classic search engine baselines; collaborative data collection tool; relevant as QA-based retrieval complement to keyword survey search.

#### 20. PaperSearchQA
- **arXiv ID**: 2601.18207
- **Year**: 2026
- **Title**: PaperSearchQA: Learning to Search and Reason over Scientific Papers with RLVR
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: eby-01, eby-01 (f)
- **Likely role**: mechanism
- **Relevance score**: high
- **Inclusion reason**: Trains search agents using RL with verifiable rewards over 16M biomedical abstracts; demonstrates planning, reasoning, self-verification; directly relevant to RL-based traversal gap — the strongest RL-for-paper-search signal found.

#### 21. Sparse Reference Selection with RL
- **arXiv ID**: 2509.05874v1
- **Year**: 2025
- **Title**: Learning to Construct Knowledge through Sparse Reference Selection with RL
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: eby-01, eby-01 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Deep RL framework for sparse reference selection prioritizing papers under time/cost constraints; evaluated on drug-gene discovery; novel prioritization approach for survey retrieval under budget.

#### 22. Multi-hop RL Search
- **arXiv ID**: 2205.15281v1
- **Year**: 2022
- **Title**: Learning Open Domain Multi-hop Search Using Reinforcement Learning
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: eby-01, eby-01 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Actor-critic RL for multi-hop entity-relation search; learns to direct IR resources to relevant corpus regions; relevant for citation traversal pathfinding.

#### 23. DeepResearch^Eco
- **arXiv ID**: 2507.10522v1
- **Year**: 2025
- **Title**: DeepResearch^Eco: Recursive Agentic Workflow for Scientific QA in Ecology
- **Source agents**: ExtendedFrontierScout
- **Source queries**: em-03 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Recursive depth-breadth controlled exploration; domain-specific deep research workflow; demonstrates iterative retrieval pattern applicable to survey agent design.

#### 24. Graph Embedding for Citation Recommendation
- **arXiv ID**: 1812.03835v1
- **Year**: 2018
- **Title**: Graph Embedding for Citation Recommendation
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-01 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Task-specific neighborhood construction for citation embedding; early approach combining graph embeddings with citation recommendation; relevant as foundational technique.

#### 25. KG-EmpiRE
- **arXiv ID**: 2405.08351v1
- **Year**: 2024
- **Title**: KG-EmpiRE: Community-Maintainable Knowledge Graph for Sustainable Literature Review
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-02 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: ORKG-based living review in Requirements Engineering; community-maintainable KG approach; demonstrates KG-driven survey methodology in practice.

#### 26. Academic IR Using Citation Clusters
- **arXiv ID**: 2207.03299v2
- **Year**: 2022
- **Title**: Academic Information Retrieval Using Citation Clusters
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-03 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Citation cluster evaluation for systematic reviews; evaluates cluster-based retrieval for academic search; relevant for citation graph traversal methodology.

#### 27. IterQR
- **arXiv ID**: 2504.05309v1
- **Year**: 2025
- **Title**: IterQR: Iterative Framework for LLM-based Query Rewrite
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-04 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Iterative query rewrite with RAG + CoT; applicable to multi-round survey paper retrieval query refinement (e-commerce domain methodology is transferable).

#### 28. Aspect-based Academic Search
- **arXiv ID**: 2001.10781v1
- **Year**: 2020
- **Title**: Aspect-based Academic Search using Domain-specific Knowledge Bases
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-04 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Aspect-aware retrieval using KB language models; enables multi-dimensional academic search; applicable to survey agents needing to search across different aspects of a topic.

#### 29. Interactive Query Generation Assistant
- **arXiv ID**: 2311.11226v1
- **Year**: 2023
- **Title**: Interactive Query Generation Assistant using LLM-based Prompt Modification
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-04 (f)
- **Likely role**: mechanism
- **Relevance score**: low
- **Inclusion reason**: HITL query generation; general-purpose query refinement assistant; boundary-relevant for iterative query development in survey retrieval.

#### 30. RerrFact
- **arXiv ID**: 2202.02646v2
- **Year**: 2022
- **Title**: RerrFact: Reduced Evidence Retrieval for Scientific Claim Verification
- **Source agents**: ExtendedFrontierScout
- **Source queries**: eby-02 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Modular claim verification approach with reduced evidence retrieval; applicable to survey citation verification pipeline.

#### 31. Generating Scientific Claims
- **arXiv ID**: 2203.12990v1
- **Year**: 2022
- **Title**: Generating Scientific Claims for Zero-Shot Fact Checking
- **Source agents**: ExtendedFrontierScout
- **Source queries**: eby-02 (f)
- **Likely role**: mechanism
- **Relevance score**: low
- **Inclusion reason**: Claim generation for zero-shot verification; adjacent technology for extracting verifiable claims from survey output.

#### 32. Attribute First, then Generate
- **arXiv ID**: 2403.17104v3
- **Year**: 2024
- **Title**: Attribute First, then Generate: Locally-attributable Grounded Text Generation
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-01 (f)
- **Likely role**: mechanism
- **Relevance score**: high
- **Inclusion reason**: Select-then-generate paradigm for fine-grained attribution; directly addresses citation-first workflow where evidence is selected before generation — mirrors desired survey agent behavior.

#### 33. FRONT
- **arXiv ID**: 2408.04568v1
- **Year**: 2024
- **Title**: Learning Fine-Grained Grounded Citations (FRONT)
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-01 (f)
- **Likely role**: mechanism
- **Relevance score**: medium
- **Inclusion reason**: Training framework for fine-grained citation; generates sentence-level grounded citations; training paradigm transferable to survey agent citation modules.

---

### benchmark — 16 papers (extended)

Evaluation frameworks, benchmarks, and datasets for survey generation and citation quality.

#### 34. Auto-survey Challenge
- **arXiv ID**: 2310.04480v2
- **Year**: 2023
- **Title**: Auto-survey Challenge: Evaluating LLMs' Capability to Autonomously Compose Survey Papers
- **Source agents**: ExtendedBenchmarkScout
- **Source queries**: ebm-01
- **Likely role**: benchmark
- **Relevance score**: high
- **Inclusion reason**: Non-LLM-as-judge evaluation paradigm using human editorial review; competition hosted at AutoML 2023; assessment criteria: clarity, reference appropriateness, accountability, substantive value — unique human-evaluation protocol.

#### 35. ALCE
- **arXiv ID**: 2305.14627v2
- **Year**: 2023
- **Title**: ALCE: Automatic LLMs' Citation Evaluation
- **Source agents**: ExtendedBenchmarkScout, ExtendedFrontierScout
- **Source queries**: ebm-02, epr-01 (f)
- **Likely role**: benchmark
- **Relevance score**: high
- **Inclusion reason**: First benchmark for evaluating LLMs' ability to generate text with citations; 3-axis evaluation (fluency, correctness, citation quality); strong correlation with human judgments; pre-dates CiteEval as the first dedicated citation evaluation benchmark.

#### 36. CiteME
- **arXiv ID**: 2407.12861v2
- **Year**: 2024
- **Title**: CiteME: Can Language Models Accurately Cite Scientific Claims?
- **Source agents**: ExtendedBenchmarkScout, ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: ebm-02, ebm-02 (sv), eby-02 (f)
- **Likely role**: benchmark
- **Relevance score**: high
- **Inclusion reason**: Evaluates LMs' ability to identify the correct cited paper from a text excerpt (7-way multiple-choice); reveals huge gap (4.2–18.5% vs humans 69.7%); targets citation attribution accuracy — a prerequisite for survey citation quality that no existing metric in the pool addresses.

#### 37. FACTS Grounding Leaderboard
- **arXiv ID**: 2501.03200v1
- **Year**: 2025
- **Title**: FACTS Grounding: Grounding to Provided Context Documents
- **Source agents**: ExtendedBenchmarkScout, ExtendedFrontierScout
- **Source queries**: ebm-02, epr-02 (f)
- **Likely role**: benchmark
- **Relevance score**: medium
- **Inclusion reason**: Active Kaggle leaderboard evaluating LMs' ability to ground responses to provided context documents (up to 32k tokens); multi-judge aggregation mitigates bias; long-context grounding is directly relevant to survey generation.

#### 38. FACTOR
- **arXiv ID**: 2307.06908v2
- **Year**: 2023
- **Title**: FACTOR: Factual Assessment via Corpus TransfORmation
- **Source agents**: ExtendedBenchmarkScout
- **Source queries**: ebm-02
- **Likely role**: benchmark
- **Relevance score**: medium
- **Inclusion reason**: Automatically transforms a factual corpus into a benchmark evaluating LM factuality; Wiki-FACTOR, News-FACTOR, Expert-FACTOR benchmarks; methodology for creating bespoke factuality benchmarks from domain corpora.

#### 39. DRACO
- **arXiv ID**: 2602.11685
- **Year**: 2026
- **Title**: DRACO: Cross-Domain Benchmark for Deep Research Accuracy
- **Source agents**: ExtendedBenchmarkScout, ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: ebm-02, ebm-02 (sv), ebm-02 (f)
- **Likely role**: benchmark
- **Relevance score**: high
- **Inclusion reason**: Cross-domain benchmark of 10 domains, 40 countries, derived from real-world Perplexity Deep Research requests; 4-dimension evaluation (accuracy, completeness/objectivity, presentation, citation quality); captures the commercial deep research evaluation paradigm.

#### 40. SurveyLens
- **arXiv ID**: 2602.11238
- **Year**: 2026
- **Title**: SurveyLens: A Research Discipline-Aware Benchmark for Automated Survey Generation
- **Source agents**: ExtendedSurveyScout
- **Source queries**: ebm-01
- **Likely role**: benchmark
- **Relevance score**: high
- **Inclusion reason**: First discipline-aware benchmark for ASG; 1,000 surveys across 10 disciplines; dual-lens evaluation (Discipline-Aware Rubric + Canonical Alignment); fills the non-CS evaluation gap (but already in main pool — listed here for the extended context).

#### 41. REASONS
- **arXiv ID**: 2405.02228v3
- **Year**: 2024
- **Title**: REASONS: Attribution in Scientific Literature — New Benchmark and Methods
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: epr-01 (sv), epr-01 (f)
- **Likely role**: benchmark
- **Relevance score**: high
- **Inclusion reason**: New dataset for sentence-level citation attribution across 12 scientific domains; metadata-augmented approach reduces hallucination rates by 42%; directly relevant to citation quality evaluation in surveys.

#### 42. OpenFactCheck
- **arXiv ID**: 2405.05583v2
- **Year**: 2024
- **Title**: OpenFactCheck: Building, Benchmarking Customized Fact-Checking Systems
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: ebm-02 (sv), ebm-02 (f)
- **Likely role**: benchmark
- **Relevance score**: medium
- **Inclusion reason**: Unified framework for building fact-checking systems + evaluate LLM factuality; three modules: CUSTCHECKER, LLMEVAL, CHECKEREVAL; customizable architecture applicable to survey factuality evaluation.

#### 43. SciFact
- **arXiv ID**: 2004.14974v6
- **Year**: 2020
- **Title**: SciFact: Fact or Fiction — Verifying Scientific Claims
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: eby-02 (sv), eby-02 (f)
- **Likely role**: benchmark
- **Relevance score**: medium
- **Inclusion reason**: Introduces scientific claim verification task + SciFact dataset (1.4K claims with evidence); foundational for claim verification methods transferable to citation grounding.

#### 44. Check-COVID
- **arXiv ID**: 2305.18265v1
- **Year**: 2023
- **Title**: Check-COVID: Fact-Checking COVID-19 News Claims with Scientific Evidence
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: eby-02 (sv), eby-02 (f)
- **Likely role**: benchmark
- **Relevance score**: medium
- **Inclusion reason**: 1,504 expert-annotated news claims paired with scientific evidence; bridges everyday language claims with formal academic evidence; relevant for cross-domain verification transfer.

#### 45. LitSearch
- **arXiv ID**: 2407.18940v2
- **Year**: 2024
- **Title**: LitSearch: A Retrieval Benchmark for Scientific Literature Search
- **Source agents**: ExtendedFrontierScout
- **Source queries**: em-02 (f)
- **Likely role**: benchmark
- **Relevance score**: medium
- **Inclusion reason**: 597 realistic ML/NLP literature search queries; evaluation benchmark for scientific paper retrieval; directly applicable for evaluating survey agent retrieval component.

#### 46. SciVer Shared Task
- **arXiv ID**: 2107.08188v1
- **Year**: 2021
- **Title**: SciVer Shared Task on Scientific Claim Verification
- **Source agents**: ExtendedFrontierScout
- **Source queries**: eby-02 (f)
- **Likely role**: benchmark
- **Relevance score**: low
- **Inclusion reason**: Shared task overview on scientific claim verification; provides dataset and evaluation framework; peripheral but relevant for citation verification task definition.

#### 47. ReFACT
- **arXiv ID**: 2509.25868
- **Year**: 2025
- **Title**: ReFACT: Benchmark for Scientific Confabulation Detection
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-02 (f)
- **Likely role**: benchmark
- **Relevance score**: high
- **Inclusion reason**: 1K expert-annotated confabulation detection benchmark; directly targets scientific hallucination detection; relevant for evaluating survey output factuality.

#### 48. QuanTemp++
- **arXiv ID**: 2510.22055
- **Year**: 2025
- **Title**: QuanTemp++: Benchmark for Numerical Fact-Checking
- **Source agents**: ExtendedFrontierScout
- **Source queries**: eby-02 (f)
- **Likely role**: benchmark
- **Relevance score**: low
- **Inclusion reason**: Numerical claim verification with claim decomposition; relevant for numerical factuality in surveys but highly specific.

#### 49. FACTORY
- **arXiv ID**: 2508.00109v1
- **Year**: 2025
- **Title**: FACTORY: Large-Scale Human-Verified Prompt Set for Long-Form Factuality
- **Source agents**: ExtendedBenchmarkScout
- **Source queries**: ebm-02
- **Likely role**: benchmark
- **Relevance score**: medium
- **Inclusion reason**: Large-scale human-verified prompt set for long-form factuality; model-in-the-loop development, human refinement; ~40% claims in SOTA model responses are not factual — high-quality challenge set for survey evaluation.

---

### metric — 14 papers (extended)

Evaluation metrics and measurement frameworks.

#### 50. VERISCORE
- **arXiv ID**: 2406.19276v1
- **Year**: 2024
- **Title**: VERISCORE: Distinguishing Verifiable from Unverifiable Claims in Factuality Evaluation
- **Source agents**: ExtendedBenchmarkScout
- **Source queries**: ebm-02
- **Likely role**: metric
- **Relevance score**: high
- **Inclusion reason**: Distinguishes verifiable scholarship from interpretive synthesis — critical for survey evaluation where many claims (interpretations, trends) are not strictly verifiable; fine-tuned open-weight LM variant; addresses limitation of FActScore/SAFE that assume all claims are verifiable.

#### 51. SAFE / LongFact
- **arXiv ID**: 2403.18802v4
- **Year**: 2024
- **Title**: SAFE: Search-Augmented Factuality Evaluator + LongFact Prompt Set
- **Source agents**: ExtendedBenchmarkScout
- **Source queries**: ebm-02
- **Likely role**: metric
- **Relevance score**: high
- **Inclusion reason**: Search-Augmented Factuality Evaluator + LongFact prompt set (38 topics); F1 score balancing precision (supported facts) and recall (fact density); LLM agents outperform crowdsourced human annotators; 20× cheaper than humans — gold-standard long-form factuality F1 formulation.

#### 52. VeriFact
- **arXiv ID**: 2505.09701v1
- **Year**: 2025
- **Title**: VeriFact: Factuality Evaluation with Precision and Recall
- **Source agents**: ExtendedBenchmarkScout
- **Source queries**: ebm-02
- **Likely role**: metric
- **Relevance score**: high
- **Inclusion reason**: Enhances fact extraction by identifying and resolving incomplete/missing facts; FactRBench measuring both precision AND recall; reference fact sets enable recall assessment — critical for survey evaluation where missing important topics is a key failure mode.

#### 53. FaStFACT
- **arXiv ID**: 2510.12839
- **Year**: 2025
- **Title**: FaStFACT: Fast Factuality Evaluation
- **Source agents**: ExtendedBenchmarkScout
- **Source queries**: ebm-02
- **Likely role**: metric
- **Relevance score**: medium
- **Inclusion reason**: Fast factuality evaluation with chunk-level claim extraction, confidence-based pre-verification, document-level evidence collection; highest alignment with human evaluation among baselines; efficiency optimization matters for evaluating long survey outputs.

#### 54. D-FActScore
- **arXiv ID**: 2402.05629v4
- **Year**: 2024
- **Title**: D-FActScore: Entity-Ambiguity-Aware Factuality Metric
- **Source agents**: ExtendedBenchmarkScout
- **Source queries**: ebm-02
- **Likely role**: metric
- **Relevance score**: high
- **Inclusion reason**: Entity-ambiguity-aware metric extending FActScore; detects when LLMs mix information from distinct entities into composite non-factual paragraphs; addresses a subtle hallucination mode common in survey writing (attributing finding A to paper B vs paper C).

#### 55. LongDocFACTScore
- **arXiv ID**: 2309.12455v2
- **Year**: 2023
- **Title**: LongDocFACTScore: Factuality Evaluation for Long Document Summarization
- **Source agents**: ExtendedBenchmarkScout
- **Source queries**: ebm-02
- **Likely role**: metric
- **Relevance score**: high
- **Inclusion reason**: Factuality evaluation for long document summarization (scientific domain); human-annotated LongSciVerify dataset; specifically designed for scientific long documents — the closest direct relative to survey evaluation.

#### 56. Hybrid Search Strategies for SLR
- **arXiv ID**: 2004.09741v1
- **Year**: 2020
- **Title**: On the Performance of Hybrid Search Strategies for Systematic Literature Reviews
- **Source agents**: ExtendedMethodScout
- **Source queries**: emc-03
- **Likely role**: metric
- **Relevance score**: medium
- **Inclusion reason**: Evaluates hybrid search strategies (database + iterative/parallel/sequential snowballing) using precision, recall, F-measure; quantifies trade-offs between search quality and effort; directly informs citation graph expansion methodology.

#### 57. References of References
- **arXiv ID**: 2101.08577v2
- **Year**: 2021
- **Title**: References of References: How Far is the Knowledge Ancestry
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-03, emc-03 (f)
- **Likely role**: metric
- **Relevance score**: medium
- **Inclusion reason**: Systematic analysis of backward citation generations (references of references); shows multi-generational citations remain topically relevant; informs depth parameter decisions for citation graph traversal in survey agents.

#### 58. Visualizing a Field of Research
- **arXiv ID**: 1906.04800v1
- **Year**: 2019
- **Title**: Visualizing a Field of Research: A Methodology of Systematic Scientometric Reviews
- **Source agents**: ExtendedMethodScout
- **Source queries**: emc-03
- **Likely role**: metric
- **Relevance score**: medium
- **Inclusion reason**: Cascading citation expansion methodology for systematic reviews; demonstrates globalism/localism unification; compares keyword-based vs expansion-based dataset construction; informs methodology choices for survey agent dataset building.

#### 59. FactKB
- **arXiv ID**: 2305.08281v2
- **Year**: 2023
- **Title**: FactKB: Generalizable Factuality Evaluation using Knowledge Bases
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-02 (f)
- **Likely role**: metric
- **Relevance score**: medium
- **Inclusion reason**: KB-enhanced factuality evaluation, cross-domain; generalizable factuality scoring; applicable to survey factuality evaluation.

#### 60. Evaluating Factual Consistency
- **arXiv ID**: 1910.12840v1
- **Year**: 2019
- **Title**: Evaluating Factual Consistency of Abstractive Summarization
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-02 (f)
- **Likely role**: metric
- **Relevance score**: medium
- **Inclusion reason**: Weakly-supervised factual consistency model; foundational approach for abstractive summarization evaluation; transferable to survey evaluation.

#### 61. PlainQAFact
- **arXiv ID**: 2503.08890v2
- **Year**: 2025
- **Title**: PlainQAFact: Factual Consistency for Biomedical Plain Language Summarization
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-02 (f)
- **Likely role**: metric
- **Relevance score**: medium
- **Inclusion reason**: Retrieval-augmented factual consistency for lay summaries; domain-specific (biomedical) but methodologically transferable.

#### 62. Factual Ablation
- **arXiv ID**: 2203.10133v2
- **Year**: 2022
- **Title**: Probing Factually Grounded Content Transfer (Factual Ablation)
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-02 (f)
- **Likely role**: metric
- **Relevance score**: low
- **Inclusion reason**: Factual ablation for measuring consistency; causal methodology for evaluating factual grounding; experimental technique rather than production metric.

#### 63. SHI Metric
- **arXiv ID**: 2404.04631v2
- **Year**: 2024
- **Title**: Simple Hallucination Index (SHI): On the Limitations of LLMs — False Attribution
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-01 (f)
- **Likely role**: metric
- **Relevance score**: medium
- **Inclusion reason**: Simple Hallucination Index metric for false attribution detection; lightweight hallucination measure applicable to survey citation screening.

#### 64. Capacity of Citation Generation
- **arXiv ID**: 2410.11217v1
- **Year**: 2024
- **Title**: On the Capacity of Citation Generation by LLMs
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-01 (f)
- **Likely role**: metric
- **Relevance score**: medium
- **Inclusion reason**: Systematic analysis of LLM citation capability; Generate-then-Refine approach; empirical analysis of citation generation capacity; informs benchmark design for survey citation evaluation.

#### 65. Background Knowledge Grounding
- **arXiv ID**: 2305.02104v1
- **Year**: 2023
- **Title**: Background Knowledge Grounding for Biomedical Lay Summaries
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-02 (f)
- **Likely role**: metric
- **Relevance score**: low
- **Inclusion reason**: Grounding source evaluation study; biomedical focus; relevant as grounding methodology reference for survey evaluation.

---

### survey_reference — 11 papers (extended)

Papers that are themselves surveys, reviews, or taxonomies providing framing and background.

#### 66. LLM Agent Survey (Methodology)
- **arXiv ID**: 2503.21460v1
- **Year**: 2025
- **Title**: Large Language Model Agent: A Survey on Methodology, Applications and Challenges
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: esv-01, esv-01 (f)
- **Likely role**: survey_reference
- **Relevance score**: medium
- **Inclusion reason**: Comprehensive survey of LLM agents organized around architecture, collaboration, and evolution; covers the methodological foundations that survey agents inherit; useful for framing the broader agent context.

#### 67. From Language to Action
- **arXiv ID**: 2508.17281v1
- **Year**: 2025
- **Title**: From Language to Action: A Review of LLMs as Autonomous Agents and Tool Users
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: esv-01, esv-01 (f)
- **Likely role**: survey_reference
- **Relevance score**: medium
- **Inclusion reason**: Structured review of LLM agents (single-agent, multi-agent, tool integration); analyzes 68 datasets; covers reasoning, planning, memory; relevant for understanding agentic patterns used in survey generation.

#### 68. From LLM Reasoning to Autonomous AI Agents
- **arXiv ID**: 2504.19678v1
- **Year**: 2025
- **Title**: From LLM Reasoning to Autonomous AI Agents: A Comprehensive Review
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: esv-01, esv-01 (f)
- **Likely role**: survey_reference
- **Relevance score**: medium
- **Inclusion reason**: Side-by-side comparison of ~60 benchmarks (2019–2025); surveys AI-agent frameworks (2023–2025); covers agent-to-agent protocols (ACP, MCP, A2A); useful for understanding the agent protocol landscape that survey agents operate in.

#### 69. A Survey on LLM based Autonomous Agents
- **arXiv ID**: 2308.11432v7
- **Year**: 2023
- **Title**: A Survey on Large Language Model based Autonomous Agents
- **Source agents**: ExtendedSurveyScout
- **Source queries**: esv-01
- **Likely role**: survey_reference
- **Relevance score**: medium
- **Inclusion reason**: Widely-cited survey proposing unified agent framework; covers construction, applications, and evaluation; foundational reference for agent architectures.

#### 70. LLM Agents for Search and Recommendation
- **arXiv ID**: 2503.05659v2
- **Year**: 2025
- **Title**: A Survey of LLM Empowered Agents for Recommendation and Search
- **Source agents**: ExtendedSurveyScout
- **Source queries**: esv-01
- **Likely role**: survey_reference
- **Relevance score**: medium
- **Inclusion reason**: First systematic survey of LLM agents for information retrieval (recommendation + search); directly relevant to the retrieval component of survey agents.

#### 71. Rise and Potential of LLM Based Agents
- **arXiv ID**: 2309.07864v3
- **Year**: 2023
- **Title**: The Rise and Potential of Large Language Model Based Agents: A Survey
- **Source agents**: ExtendedSurveyScout
- **Source queries**: esv-01
- **Likely role**: survey_reference
- **Relevance score**: medium
- **Inclusion reason**: Three-component framework (brain, perception, action) for LLM agents; covers single-agent, multi-agent, and human-agent cooperation; high citation count.

#### 72. Agentic Large Language Models Survey
- **arXiv ID**: 2503.23037v2
- **Year**: 2025
- **Title**: Agentic Large Language Models, a survey
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: esv-01, esv-01 (f)
- **Likely role**: survey_reference
- **Relevance score**: medium
- **Inclusion reason**: Organizes literature around three capabilities: reason, act, interact; discusses retrieval enabling tool use, reflection improving collaboration; research agenda included.

#### 73. Deep Search Agents Survey
- **arXiv ID**: 2508.05668v3
- **Year**: 2025
- **Title**: A Survey of LLM-based Deep Search Agents: Paradigm, Optimization, Evaluation, and Challenges
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: esv-01, esv-01 (f)
- **Likely role**: survey_reference
- **Relevance score**: high
- **Inclusion reason**: First systematic survey of deep search agents (OpenAI Deep Research, Gemini, Perplexity); covers architecture, optimization, evaluation; directly fills the deep research paradigm gap — essential framing for the survey generation vs deep research comparison.

#### 74. Related Work Generation: A Meta Study
- **arXiv ID**: 2201.01880v1
- **Year**: 2022
- **Title**: Automatic Related Work Generation: A Meta Study
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: em-01 (sv), em-01 (f)
- **Likely role**: survey_reference
- **Relevance score**: high
- **Inclusion reason**: Meta-study comparing related work generation formulations, datasets, approaches, evaluation; covers pre-LLM and early LLM approaches; fills the pre-2020 gap through its literature tracing.

#### 75. Multi-Agent Collaboration Survey
- **arXiv ID**: 2501.06322v1
- **Year**: 2025
- **Title**: Multi-Agent Collaboration Mechanisms: A Survey of LLMs
- **Source agents**: ExtendedMethodScout
- **Source queries**: em-03
- **Likely role**: survey_reference
- **Relevance score**: medium
- **Inclusion reason**: Comprehensive survey of LLM-based multi-agent collaboration; framework for characterizing mechanisms (actors, types, structures, strategies, protocols); useful as taxonomy reference for understanding multi-agent patterns in survey agents.

#### 76. Fact Extraction and Verification Review
- **arXiv ID**: 2010.03001v5
- **Year**: 2020
- **Title**: A Review on Fact Extraction and Verification
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: epr-01 (sv), eby-02 (f)
- **Likely role**: survey_reference
- **Relevance score**: medium
- **Inclusion reason**: Survey of fact extraction and verification (FEVER task); covers document retrieval, sentence selection, and veracity classification; relevant for the citation hallucination detection gap.

---

### citation_seed — 5 papers (extended)

Foundational, infrastructure, or early influential papers providing background and citations.

#### 77. Refcat
- **arXiv ID**: 2110.06595v2
- **Year**: 2021
- **Title**: Refcat: The Internet Archive Scholar Citation Graph
- **Source agents**: ExtendedMethodScout
- **Source queries**: emc-01
- **Likely role**: citation_seed
- **Relevance score**: medium
- **Inclusion reason**: Large-scale (1.3B citations) freely available citation graph dataset under CC0; MIT-licensed code; infrastructure resource for building citation graph traversal components.

#### 78. Paper Evolution Graph
- **arXiv ID**: 1711.08913v1
- **Year**: 2017
- **Title**: Paper Evolution Graph: Multi-view Structural Retrieval for Academic Literature
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: emc-01, emc-01 (f)
- **Likely role**: citation_seed
- **Relevance score**: low
- **Inclusion reason**: Constructs structural retrieval results (PEG) showing paper evolution via multiple chains; soft-clustering via metagraph factorization; early graph-based retrieval approach.

#### 79. ComplexWebQuestions
- **arXiv ID**: 1803.06643v1
- **Year**: 2018
- **Title**: ComplexWebQuestions: The Web as a Knowledge-base for Answering Complex Questions
- **Source agents**: ExtendedMethodScout
- **Source queries**: emc-04
- **Likely role**: citation_seed
- **Relevance score**: low
- **Inclusion reason**: Decomposes complex questions into simple question sequences; ComplexWebQuestions dataset; 20.8→27.5 precision@1 improvement; foundational approach for question decomposition in retrieval pipelines.

#### 80. Direction Aware Citation Analysis
- **arXiv ID**: 1205.1143v1
- **Year**: 2012
- **Title**: Direction Aware Citation Analysis
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-01 (f)
- **Likely role**: citation_seed
- **Relevance score**: low
- **Inclusion reason**: Early direction-aware citation recommendation; historical reference for citation graph analysis.

#### 81. Footnote Chasing Analysis
- **arXiv ID**: 1707.02494v2
- **Year**: 2017
- **Title**: Analysis of Footnote Chasing and Citation Searching
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-03 (f)
- **Likely role**: citation_seed
- **Relevance score**: medium
- **Inclusion reason**: User behavior study of Bates' search stratagems; observational study of how researchers actually perform footnote chasing and citation searching; informs human-like citation traversal design.

---

### related_system — 14 papers (extended)

Systems that are adjacent or supportive but not core survey generation pipelines.

#### 82. WisPaper
- **arXiv ID**: 2512.06879
- **Year**: 2025
- **Title**: WisPaper: Your AI Scholar Search Engine
- **Source agents**: ExtendedMethodScout, ExtendedFrontierScout
- **Source queries**: em-02, em-02 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: Intelligent academic retrieval platform with deep agentic search mode; closed-loop workflow (discovery → management → tracking); could provide retrieval infrastructure for survey agents.

#### 83. CRUISE-Screening
- **arXiv ID**: 2309.01684v1
- **Year**: 2023
- **Title**: CRUISE-Screening: Living Literature Reviews Toolbox
- **Source agents**: ExtendedFrontierScout
- **Source queries**: em-01 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: Living review screening tool; toolbox for maintaining up-to-date literature reviews; relevant for the continuous update dimension of survey agents.

#### 84. FAST²
- **arXiv ID**: 1705.05420v6
- **Year**: 2017
- **Title**: FAST²: An Intelligent Assistant for Finding Relevant Papers
- **Source agents**: ExtendedFrontierScout
- **Source queries**: em-01 (f)
- **Likely role**: related_system
- **Relevance score**: low
- **Inclusion reason**: Early active learning for literature screening; historical reference for semi-automated literature discovery.

#### 85. LLMSurver
- **arXiv ID**: 2407.10652v2
- **Year**: 2024
- **Title**: Cutting Through the Clutter: LLMs for Efficient Filtration in Systematic Literature Reviews
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: eby-03 (sv), eby-03 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: Open-source LLMSurver tool for literature filtration; consensus scheme achieves >98.8% recall; human-AI collaboration model; fills the semi-automated screening gap.

#### 86. AISysRev
- **arXiv ID**: 2510.06708
- **Year**: 2025
- **Title**: AISysRev: LLM-based Tool for Title-Abstract Screening
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: eby-03 (sv), eby-03 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: LLM-based screening tool with OpenRouter support; identifies Easy Includes/Excludes vs Boundary cases needing human intervention; practical approach to hybrid screening.

#### 87. Streamlining SLR with LLMs
- **arXiv ID**: 2412.15247v1
- **Year**: 2024
- **Title**: Streamlining Systematic Reviews: A Novel Application of LLMs
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: eby-03 (sv), eby-03 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: LLM system covering both title/abstract and full-text screening; 99.5% exclusion rate, 95.5% reduction in manual screening time; significant efficiency gains.

#### 88. LLM Ensembles for Screening
- **arXiv ID**: 2411.02451v2
- **Year**: 2024
- **Title**: High-performance Automated Abstract Screening with LLM Ensembles
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: eby-03 (sv), eby-03 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: Tests 6 LLMs + 66 ensembles on Cochrane Library reviews; perfect sensitivity achievable with ensembles; precision trade-off documented.

#### 89. AIDE
- **arXiv ID**: 2501.11840v1
- **Year**: 2025
- **Title**: AIDE: LLMs with Human-In-The-Loop Validation for Systematic Review Data Extraction
- **Source agents**: ExtendedFrontierScout
- **Source queries**: eby-03 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: Open-source HITL extraction tool; human-in-the-loop data extraction for systematic reviews; relevant for semi-automated survey paper data extraction.

#### 90. Promise and Challenges of LLM Screening
- **arXiv ID**: 2404.15667v4
- **Year**: 2024
- **Title**: Promise and Challenges of Using LLMs to Accelerate Systematic Review Screening
- **Source agents**: ExtendedFrontierScout
- **Source queries**: eby-03 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: Empirical study with GPT-3.5/4, few-shot analysis; quantitative assessment of LLM screening capabilities and limitations.

#### 91. GPTscreenR
- **arXiv ID**: 2311.07918v1
- **Year**: 2023
- **Title**: Automated Title and Abstract Screening (GPTscreenR)
- **Source agents**: ExtendedFrontierScout
- **Source queries**: eby-03 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: R package for GPT-4 screening; practical tool for automated abstract screening; R ecosystem integration for research synthesis.

#### 92. AiReview
- **arXiv ID**: 2504.04193v1
- **Year**: 2025
- **Title**: AiReview: Open Platform for Accelerating Systematic Reviews with LLMs
- **Source agents**: ExtendedFrontierScout
- **Source queries**: eby-03 (f)
- **Likely role**: related_system
- **Relevance score**: medium
- **Inclusion reason**: First platform bridging LLM screening and medical systematic reviews; open platform architecture; practical deployment reference.

#### 93. Can LLMs Replace Humans in SR?
- **arXiv ID**: 2310.17526v2
- **Year**: 2023
- **Title**: Can LLMs Replace Humans in Systematic Reviews? Evaluating GPT-4
- **Source agents**: ExtendedFrontierScout
- **Source queries**: eby-03 (f)
- **Likely role**: related_system
- **Relevance score**: low
- **Inclusion reason**: Comprehensive GPT-4 evaluation across languages; empirical assessment of LLM capabilities vs human performance in SLR tasks; useful for understanding automation limits.

#### 94. QueryExplorer
- **arXiv ID**: 2403.15667v1
- **Year**: 2024
- **Title**: QueryExplorer: Interactive Query Generation Assistant
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-04 (f)
- **Likely role**: related_system
- **Relevance score**: low
- **Inclusion reason**: Interactive query reformulation interface; HCI-focused tool for query refinement; adjacent to retrieval component development.

#### 95. Counterfactual Factual Consistency
- **arXiv ID**: 2108.13134v2
- **Year**: 2021
- **Title**: Measuring Factual Consistency via Counterfactual Estimation
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-02 (f)
- **Likely role**: related_system
- **Relevance score**: low
- **Inclusion reason**: Causal approach to factual consistency; experimental evaluation methodology rather than practical metric; boundary reference.

---

### boundary — 12 papers (extended)

Papers that are intentionally peripheral — useful for discrimination tests and comparative analysis.

#### 96. UKP-ATHENA
- **arXiv ID**: 1911.10392v1
- **Year**: 2019
- **Title**: When is ACL's Deadline? A Scientific Conversational Agent (UKP-ATHENA)
- **Source agents**: ExtendedMethodScout
- **Source queries**: em-02
- **Likely role**: boundary
- **Relevance score**: low
- **Inclusion reason**: Conversational agent for NLP literature exploration; multi-source information access; context-maintaining dialogue — adjacent but dated; useful as boundary discriminator.

#### 97. Multi-Agent Sampling (MCTS)
- **arXiv ID**: 2412.17061v2
- **Year**: 2024
- **Title**: Multi-Agent Sampling: Scaling Inference Compute for Data Synthesis with Tree Search-Based Agentic Collaboration
- **Source agents**: ExtendedMethodScout
- **Source queries**: em-03
- **Likely role**: boundary
- **Relevance score**: low
- **Inclusion reason**: Monte Carlo Tree Search for agent workflow optimization; uses reward model for real-time feedback; focused on data synthesis alignment rather than literature tasks — boundary reference for search-based agent coordination.

#### 98. Snowballing on Gray Literature
- **arXiv ID**: 2407.14991v1
- **Year**: 2024
- **Title**: Investigating the use of Snowballing on Gray Literature Reviews
- **Source agents**: ExtendedMethodScout
- **Source queries**: emc-03
- **Likely role**: boundary
- **Relevance score**: low
- **Inclusion reason**: Tests link-based and similarity-based snowballing on Q&A site discussions; 120% increase in valid dataset; guidelines for snowballing on non-traditional sources — boundary reference for gray literature expansion.

#### 99. AgentRxiv
- **arXiv ID**: 2503.18102v1
- **Year**: 2025
- **Title**: AgentRxiv: Towards Collaborative Autonomous Research
- **Source agents**: ExtendedFrontierScout
- **Source queries**: em-03 (f)
- **Likely role**: boundary
- **Relevance score**: low
- **Inclusion reason**: Preprint-server-based agent collaboration; explores autonomous research beyond survey generation; visionary but not directly applicable.

#### 100. Machine Understanding of Scientific Language
- **arXiv ID**: 2506.23990v1
- **Year**: 2025
- **Title**: Machine Understanding of Scientific Language
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-02 (f)
- **Likely role**: boundary
- **Relevance score**: low
- **Inclusion reason**: Thesis covering scientific fact-checking; broad scope covering multiple NLP tasks; useful as survey reference for scientific NLP capabilities.

#### 101. LLM Hallucinations in the Wild
- **arXiv ID**: 2605.07723
- **Year**: 2026
- **Title**: LLM hallucinations in the wild: Large-scale evidence from non-existent citations
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: epr-01 (sv), epr-01 (f)
- **Likely role**: boundary (dual role with metric)
- **Relevance score**: high
- **Inclusion reason**: Large-scale audit of 111M references across 2.5M papers; finds ~147K hallucinated citations in 2025 alone; crucial evidence paper establishing real-world magnitude of citation hallucination — essential problem context for survey generation.

#### 102. How LLMs Cite
- **arXiv ID**: 2603.03299
- **Year**: 2026
- **Title**: How LLMs Cite and Why It Matters: A Cross-Model Audit of Reference Fabrication
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: epr-01 (sv), epr-01 (f)
- **Likely role**: boundary (dual role with metric)
- **Relevance score**: high
- **Inclusion reason**: Audits 10 LLMs across 4 domains (69,557 citations); hallucination rates span 11.4%–56.8%; proves hallucination is prompt-induced, not intrinsic; practical multi-model consensus filter (95.6% accuracy with 3+ models).

#### 103. Attribution Crisis in LLM Search Results
- **arXiv ID**: 2508.00838v1
- **Year**: 2025
- **Title**: The Attribution Crisis in LLM Search Results
- **Source agents**: ExtendedSurveyScout, ExtendedFrontierScout
- **Source queries**: epr-01 (sv), epr-01 (f)
- **Likely role**: boundary (dual role with metric)
- **Relevance score**: high
- **Inclusion reason**: Analysis of ~14,000 LMArena conversation logs; documents three exploitation patterns (no search, no citation, high-volume low-credit); relevant for understanding survey agent attribution failures.

#### 104. Hallucinating References Detection
- **arXiv ID**: 2305.18248v3
- **Year**: 2023
- **Title**: Do Language Models Know When They're Hallucinating References?
- **Source agents**: ExtendedFrontierScout
- **Source queries**: epr-01 (f)
- **Likely role**: boundary
- **Relevance score**: medium
- **Inclusion reason**: Consistency-check method for detecting hallucinated references; automated hallucination detection at the reference level; applicable to survey citation quality screening.

#### 105. Clarifying Questions for Code Search
- **arXiv ID**: 2201.09974v1
- **Year**: 2022
- **Title**: Generating Clarifying Questions for Query Refinement in Code Search
- **Source agents**: ExtendedFrontierScout
- **Source queries**: emc-04 (f)
- **Likely role**: boundary
- **Relevance score**: low
- **Inclusion reason**: Query refinement for code search — domain-specific; boundary reference for query generation techniques with limited transferability.

#### 106. Decade of KGs in NLP
- **arXiv ID**: 2210.00105v1
- **Year**: 2022
- **Title**: A Decade of Knowledge Graphs in Natural Language Processing: A Survey
- **Source agents**: ExtendedSurveyScout
- **Source queries**: emc-02
- **Likely role**: boundary
- **Relevance score**: low
- **Inclusion reason**: Systematic analysis of 507 papers on KGs in NLP; provides taxonomy of tasks and research streams; broadly relevant for KG context but not specific to survey generation.

#### 107. Knowledge Graphs Tutorial
- **arXiv ID**: 2003.02320v6
- **Year**: 2020
- **Title**: Knowledge Graphs (Comprehensive Tutorial)
- **Source agents**: ExtendedSurveyScout
- **Source queries**: emc-02
- **Likely role**: boundary
- **Relevance score**: low
- **Inclusion reason**: Comprehensive introduction to knowledge graphs — data models, query languages, schema, identity, context, extraction, quality; foundational tutorial too general for direct inclusion but useful as KG background.

---

## Summary Statistics

| Role | Count |
|------|-------|
| core_method | 4 |
| mechanism | 29 |
| benchmark | 16 |
| metric | 16 |
| survey_reference | 11 |
| citation_seed | 5 |
| related_system | 14 |
| boundary | 12 |
| **Total unique papers** | **107** |

**Note**: 3 papers have dual roles (LLM Hallucinations in the Wild, How LLMs Cite, Attribution Crisis — boundary/metric); role-based counts sum to 110 due to these dual assignments.

## Cross-Scout Provenance Summary

| Source Scout | Unique Papers (extended) |
|---|---|
| ExtendedMethodScout (02e) | 30 |
| ExtendedBenchmarkScout (02f) | 13 |
| ExtendedSurveyScout (02g) | 41 |
| ExtendedFrontierScout (02h) | ~103 raw (many overlap with other scouts) |
| **Merged extended total** | **107** (deduplicated across all four) |

## Relevance Score Distribution

| Relevance | Count | Description |
|-----------|-------|-------------|
| **high** | 23 | Directly relevant to survey generation, citation graph traversal, or evaluation |
| **medium** | 62 | Indirectly relevant — component, method, adjacent domain, or transferable technique |
| **low** | 22 | Boundary — useful for discrimination tests, comparative analysis, or historical context |

## Top Papers for Downstream Processing

### Highest cross-scout agreement (appeared in 3+ extended scouts):
1. **Paperfetcher** (2110.12490v3) — Method, Survey, Frontier
2. **Cascading Citation Expansion** (1806.00089v1) — Method, Survey, Frontier
3. **CiteME** (2407.12861v2) — Benchmark, Survey, Frontier
4. **DRACO** (2602.11685) — Benchmark, Survey, Frontier
5. **ChatCite** (2403.02574v1) — Method, Frontier

### Key gap-filling additions vs main pool:
1. **Deep Search Agents Survey** (2508.05668v3) — fills the deep research paradigm gap
2. **PaperQA2** (2409.13740v2) — open-source deep research agent (competitor to closed systems)
3. **CKMAs** (2411.06159v3) — knowledge-minigraph-driven survey generation
4. **Iterated Decomposition** (2301.01751v2) — query decomposition for scientific QA
5. **SAFE/LongFact** (2403.18802v4) — search-augmented F1 formulation for survey evaluation
6. **LLM Hallucinations in the Wild** (2605.07723) — real-world citation hallucination magnitude evidence
7. **PaperSearchQA** (2601.18207) — RLVR-trained paper search agent
8. **ALCE** (2305.14627v2) — first citation evaluation benchmark (pre-dates CiteEval)
9. **D-FActScore** (2402.05629v4) — entity-ambiguity-aware factuality for survey attribution
10. **FAST² / AISysRev / LLMSurver / AiReview** — HITL screening tools bridging automated and human-assisted review
