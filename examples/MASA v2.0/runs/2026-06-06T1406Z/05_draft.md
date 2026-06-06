# Automated Literature Survey Agents with Citation Graph Expansion

## Section 1: Introduction and Scope

**Defining the landscape of citation-graph-aware automated survey generation**

The volume of published scientific literature has grown at an accelerating pace that exceeds the capacity of individual researchers to track comprehensively. A recent large-scale audit of citation practices identified approximately 147,000 hallucinated citations across published papers in 2025 alone, underscoring the scale of the attribution challenge that motivates automated survey generation [arXiv:2605.07723]. Researchers face an estimated 4–6 weeks per systematic review in manual screening alone, and the cognitive burden of synthesizing dozens to hundreds of papers into a coherent survey remains a bottleneck in scientific communication [arXiv:2409.04600]. These pressures have motivated a wave of work on automated literature survey generation — systems that leverage large language models (LLMs) to autonomously retrieve, filter, synthesize, and cite academic papers into structured survey documents.

This survey focuses on LLM-based agents that incorporate **citation graph structure** — the network formed by direct citations, cocitation relationships, and bibliographic coupling among papers — to guide paper discovery, relevance ranking, and content synthesis. The scope encompasses systems that use citation graphs as retrieval signals, traversal strategies, or organizational frameworks for survey generation. We explicitly exclude pure recommender systems that suggest individual papers without synthesis, non-automated systematic review methodologies such as PRISMA-based manual meta-analysis, and graph neural network architectures applied to citation graphs in isolation without a survey-agent framing.

The survey is organized around two anchor questions defined in the survey specification: **(1) Primary question** — How do current automated survey agents incorporate citation graph structure to guide paper discovery, relevance ranking, and content synthesis? This question is addressed primarily in Sections 2 (architectural taxonomy) and 3 (citation graph expansion strategies). **(2) Secondary question** — What are the main failure modes — hallucination, coverage gaps, citation obsolescence — and what mitigation strategies exist? This question is addressed in Sections 6 (citation attribution and factuality) and 9 (open challenges). The quality bar for this survey is that each major approach must be accompanied by concrete method names, architectural patterns, failure mechanisms, and at least one quantitative benchmark result.

We make the following contributions: a structured taxonomy of survey agents organized by citation graph awareness (Section 2); a comprehensive comparison of graph expansion strategies from classical traversal to RL-optimized discovery (Section 3); a synthesis of evaluation benchmarks specifically designed for citation quality (Section 7); and a roadmap of open challenges including hallucination, evaluation standardization, and graph traversal trade-offs (Section 9). Prior surveys have reviewed AI tools for literature reviews at the screening and extraction stage [arXiv:2402.08565] and evaluated zero-shot LLM capabilities for literature review tasks [arXiv:2412.15249]. The broader LLM agent literature provides architectural context for the agent-based systems reviewed here [arXiv:2503.21460], but none of these prior efforts has focused specifically on architecturally integrated citation graph expansion within autonomous survey agents — the gap this survey fills.

The remainder of this survey proceeds as follows. Section 2 presents the architectural taxonomy. Section 3 covers citation graph expansion strategies. Sections 4 and 5 discuss planning and retrieval. Section 6 addresses citation attribution. Section 7 reviews evaluation benchmarks. Section 8 explores emerging interactive and living survey paradigms. Section 9 enumerates open challenges, and Section 10 concludes with recommendations.

---

## Section 2: Architectural Taxonomy of Citation-Graph-Aware Survey Agents

**Single-agent, multi-agent, and hybrid architectures for LLM-based survey writing with citation graph integration**

Automated survey generation systems can be classified by their agent architecture and the mechanism through which they incorporate citation graph information. We organize existing systems into three architectural categories — single-agent, multi-agent, and hybrid/interactive — and then compare them along citation graph awareness axes. Throughout this survey, we use the term **coverage breadth** to mean the fraction of relevant papers in a topic that a survey cites, a metric that will appear consistently across sections as a key quality dimension.

### 2.1 Single-Agent Architectures

Single-agent systems delegate the entire survey generation pipeline to a single LLM agent augmented with retrieval, planning, and writing capabilities. The foundational work in this category is **AutoSurvey** [arXiv:2406.10252], which introduced a three-stage pipeline: outline generation, section drafting, and integration. AutoSurvey operates without explicit citation graph traversal, relying on keyword-based semantic search over a static corpus.

**SurveyX** [arXiv:2502.14776] improves upon AutoSurvey by introducing a two-phase Preparation+Generation pipeline with an AttributeTree structure that organizes papers hierarchically by attributes. SurveyX achieves a +1.76 improvement in citation quality over AutoSurvey baselines, demonstrating the value of structured paper representation. **SurveyForge** [arXiv:2503.04629] adds outline heuristics learned from human-written surveys and a scholar navigation agent for memory-driven retrieval. The scholar navigation agent maintains a working memory of discovered papers, enabling a primitive form of citation graph awareness through iterative follow-up searches.

**LitLLM** [arXiv:2402.01788] provides a modular RAG-based toolkit for literature review with discrete stages: web search, keyword extraction, paper re-ranking, and related work generation. Its re-ranking pipeline uses cross-encoders for relevance assessment but does not leverage citation graph structure. **SurveyGen-I** [arXiv:2508.14317] introduces coarse-to-fine retrieval with adaptive planning and memory-guided writing, enabling dynamic refinement across subsections. Its memory mechanism allows the system to maintain coherence across sections but lacks explicit citation graph traversal.

A common limitation across single-agent systems is their reliance on semantic or keyword-based retrieval without structured navigation of citation networks. While this approach can surface relevant papers, it does not systematically exploit the citation relationships that define topical communities, foundational works, and frontier research.

### 2.2 Multi-Agent Architectures

Multi-agent architectures distribute survey generation across specialized agents, each responsible for a distinct subtask such as search, outline generation, writing, or evaluation. **Agentic AutoSurvey** [arXiv:2509.18661] employs four specialist agents — Paper Search, Topic Mining, Writer, and Quality Evaluator — processing 75–443 papers per topic and achieving the highest reported overall score of 8.18/10 across 12 evaluation dimensions. The Paper Search agent performs citation-aware retrieval by starting from seed papers and expanding through reference lists, though the specific traversal strategy is BFS-based rather than learned.

**SurveyG** [arXiv:2510.07733] is the most architecturally explicit in its citation graph usage. It organizes the citation graph into three hierarchical layers — Foundation, Development, and Frontier — and combines horizontal search within each layer with vertical depth traversal across layers. This design directly addresses the primary anchor question: citation graph structure guides both paper discovery (which layer to traverse) and relevance ranking (which papers at each layer are most relevant). SurveyG's hierarchical approach provides a concrete answer to how citation graphs can be structurally integrated into survey generation.

**LiRA** [arXiv:2510.05138] implements a multi-agent collaborative workflow that emulates the human review process through outlining, subsection writing, editing, and reviewing agents. Evaluated on the SciReviewGen dataset, LiRA demonstrates robustness to reviewer model variation. **ResearchPilot** [arXiv:2603.14629] adopts a local-first approach for literature synthesis and related work drafting, retrieving from Semantic Scholar and arXiv with structured findings extraction and cross-paper pattern synthesis.

Two mechanism-level contributions support multi-agent architectures. **MATC** (Multi-Agent Taskforce Collaboration) [arXiv:2508.04306] formalizes three collaboration paradigms — exploration, exploitation, and experience — for error correction in long-form literature review generation. MATC shows that explicit coordination patterns reduce compounding errors across agents. **CKMAs** (Collaborative Knowledge Minigraph Agents) [arXiv:2411.06159] construct relation graphs from literature and organize concepts via multiple paths for review paragraphs, directly leveraging graph structure for content synthesis.

### 2.3 Hybrid and Interactive Architectures

Hybrid systems combine automated generation with iterative retrieval loops or user interaction. **STORM** [arXiv:2402.14207] is the seminal hybrid system for Wikipedia-like article generation, using retrieval from the web combined with outline-driven drafting. While not designed specifically for academic surveys, STORM's architecture has influenced subsequent survey generation systems.

**IterSurvey** [arXiv:2510.21900] implements a recurrent outline generation process where a planning agent incrementally retrieves, reads, and updates the outline. Paper cards provide faithful grounding for each claim, and a review-and-refine loop allows revision based on retrieved evidence. IterSurvey introduces Survey-Arena, a pairwise benchmark that positions machine-generated surveys relative to human-written ones.

**InteractiveSurvey** [arXiv:2504.08762] enables user customization at multiple stages: reference categorization, outline structure, and content during generation. Users can upload papers and guide citation graph traversal decisions interactively. **SciSage** [arXiv:2506.12689] introduces a "reflect-when-you-write" paradigm with a hierarchical Reflector agent that evaluates at the outline, section, and document levels. SciSage achieves +1.73 coherence improvement and +32% citation F1 over baselines, and releases SurveyScope — a benchmark of 46 papers across 11 CS domains (cross-referenced in Section 7.1).

**AutoSurvey2** [arXiv:2510.26012] also falls in this category due to its parallel section generation with real-time retrieval, though it lacks the iterative refinement of other hybrid systems.

These hybrid systems occupy different points on an automation-versus-user-control spectrum. InteractiveSurvey provides maximum user control, allowing manual specification of reference categories and outline structure before and during generation. IterSurvey balances automation with iteration, enabling users to guide the refinement loop without specifying every detail upfront. STORM operates with minimal user intervention, relying entirely on automated retrieval and outline generation. The trade-off is structural: user control improves citation precision and topic alignment by incorporating domain expertise, but it limits scalability to batch processing of many topics and requires researcher time that automated approaches save.

### 2.4 Citation Graph Awareness Comparison

Table 1 synthesizes the three architectural categories along citation graph awareness axes, including evaluation scores where available.

| Architecture | System | Graph Traversal | Graph Construction | Depth Control | Relevance Ranking | Evaluation Score |
|---|---|---|---|---|---|---|
| Single-agent | AutoSurvey [2406.10252] | None | N/A | N/A | Semantic search | 4.77/10 [2509.18661] |
| Single-agent | SurveyX [2502.14776] | None | Pre-built attribute tree | Fixed | Attribute matching | +1.76 vs AutoSurvey |
| Single-agent | SurveyForge [2503.04629] | Memory-driven follow-up | On-the-fly | Adaptive | Scholar agent memory | — |
| Multi-agent | Agentic AutoSurvey [2509.18661] | BFS on references | On-the-fly | Fixed (2–3 hops) | Citation count + recency | 8.18/10 |
| Multi-agent | SurveyG [2510.07733] | Hierarchical (3-layer) | Pre-built + on-the-fly | Adaptive (layer-adaptive) | Recency-weighted per layer | — |
| Multi-agent | LiRA [2510.05138] | Reference expansion | On-the-fly | Fixed | Cross-encoder relevance | — |
| Hybrid | STORM [2402.14207] | Web search (not citation) | N/A | N/A | Search engine ranking | — |
| Hybrid | IterSurvey [2510.21900] | Iterative retrieval | On-the-fly | Adaptive | Paper card relevance | — |
| Hybrid | SciSage [2506.12689] | Hierarchical reflection | On-the-fly | Adaptive | Reflector-guided | +1.73 coherence, +32% cit. F1 |

Single-agent architectures with no graph traversal are suitable for focused topics where keyword search suffices. Hierarchical multi-agent systems (SurveyG) are appropriate for broad-coverage surveys requiring structured coverage of foundational, contemporary, and frontier work. Hybrid systems with iterative retrieval (IterSurvey, SciSage) offer the best balance for topics where citation structure significantly informs paper discovery.

---

## Section 3: Citation Graph Expansion Strategies for Paper Discovery

**From direct citation traversal to hierarchical graph-aware retrieval — methods for expanding and navigating citation networks**

Citation graph expansion is the process of traversing the network of paper references to discover relevant literature beyond the initial seed set. The choice of expansion strategy directly determines coverage breadth, relevance precision, and frontier identification in automated survey generation.

### 3.1 Classical Citation Graph Traversal Methods

Three foundational citation analysis techniques form the basis of classical graph traversal. **Direct citation** follows reference lists from a seed paper to its cited sources. **Cocitation analysis** identifies papers that are frequently cited together, revealing topical clusters. **Bibliographic coupling** groups papers that share common references, indicating similar research directions. A systematic comparison of these three methods for seed-based retrieval [arXiv:2403.09295] found that cocitation achieves the highest precision for focused retrieval, while the combination of all three methods yields the best overall recall. The study, conducted across multiple systematic review corpora, provides quantitative evidence that no single method dominates across all metrics.

**Interleaved Snowballing** [arXiv:2402.08339] formalizes the snowballing process as an algorithm that interleaves backward and forward citation traversal. The method reduces curator workload by presenting papers in batches ordered by relevance, with the LitBall desktop application providing an interactive interface. **Cascading Citation Expansion** [arXiv:1806.00089] implements successive citation expansion through multiple generations using the Dimensions API, integrated into the CiteSpace tool. This approach traces back to Garfield's foundational concept of citation indexing and demonstrates that multi-generational citation expansion can surface topically relevant distant works.

**Oignon** [arXiv:2512.22159] provides a free/open-source citation graph exploration tool with a dual-path ranking system that weights recency alongside relevance. By assigning higher scores to more recent work while maintaining connection to foundational papers, Oignon balances the exploration-exploitation trade-off in citation traversal.

### 3.2 Graph-Based Retrieval Augmentation

Recent work moves beyond classical traversal by using citation graph structure to enhance retrieval representations. **LitFM** [arXiv:2409.12177] introduces a structure-aware foundation model for citation graphs that integrates graph structure during both training and inference. LitFM achieves a 28.1% improvement in retrieval precision over dense retriever baselines by learning paper representations that incorporate local graph neighborhoods. The model is evaluated on benchmark datasets spanning three academic fields with sentence-level citation information.

**CG-RAG** (Citation Graph Retrieval-Augmented Generation) [arXiv:2501.15067] combines sparse and dense retrieval signals within citation graph structures for research question answering. Its lexical-semantic graph retrieval (LeSeGR) approach encodes both textual content and citation relationships, outperforming standard RAG for research QA tasks. **CitationIE** [arXiv:2106.01560] demonstrates that augmenting text representations with citation graph structure improves scientific information extraction tasks, suggesting transfer potential to survey paper discovery.

The key insight from graph-based retrieval approaches is that citation graph structure provides a complementary signal to semantic similarity. Papers that are topically similar may not be citation-linked, and papers that are citation-linked may not be semantically similar in surface text — the graph captures relationships that dense retrieval misses.

### 3.3 Hierarchical and Multi-Layer Citation Traversal

Hierarchical traversal organizes the citation graph into layers corresponding to different levels of research maturity. **SurveyG**'s three-layer architecture (Foundation → Development → Frontier) is the most fully realized example [arXiv:2510.07733]. Foundation layer papers are highly-cited, older works that define the field. Development layer papers represent contemporary research building on foundations. Frontier layer papers capture very recent work at the research edge. SurveyG performs horizontal search within each layer to find relevant papers at that maturity level, and vertical traversal to connect across layers.

**PUREsuggest** [arXiv:2408.02508] supports citation-based literature search with keyword-steerable rankings, enabling users to control whether retrieved papers emphasize foundational or recent work. Its visual exploration interface allows interactive navigation of the citation graph, though it targets human-in-the-loop rather than fully autonomous traversal.

The motivation for hierarchical layering is explicit in the outline: foundational works provide context and background, contemporary developments provide the current state of the art, and frontier research identifies emerging directions. This structure mirrors how human-written surveys organize their references, making hierarchical traversal particularly suited for survey generation.

### 3.4 Agent-Driven and RL-Optimized Graph Traversal

The frontier of citation graph expansion replaces fixed traversal parameters (depth, breadth) with learned policies. **PaSa** [arXiv:2501.10120] introduces an autonomous paper search agent that uses reinforcement learning to optimize traversal decisions. PaSa invokes search tools, reads papers, selects references, and decides when to stop expanding. It achieves +37.78% recall over the Google+GPT-4o baseline, demonstrating that learned traversal policies substantially outperform fixed BFS or depth-limited approaches.

**PaperSearchQA** [arXiv:2601.18207] extends this paradigm with RLVR (Reinforcement Learning from Verifiable Rewards) training over 16 million abstracts. The agent learns to formulate search queries and select which papers to examine, with rewards tied to retrieval recall. **PaSaMaster** [arXiv:2605.14306] further advances toward self-evolving retrieval with zero source hallucination, separating planning from retrieval for cost efficiency. PaSaMaster's architecture allows the agent to adapt its traversal strategy based on the topic's citation structure without human intervention.

Query decomposition offers an alternative to learned graph traversal. **SPAR** [arXiv:2507.15245] uses a RefChain-based approach that decomposes complex academic search queries into sub-queries, with each sub-query targeting a different facet of the research question. SPAR achieves up to +56% F1 improvement over monolithic query baselines. Decomposition-based retrieval [arXiv:2305.15053] routes individual query components to specialized retrievers, providing a complementary strategy for expanding coverage without deep citation traversal.

The evolution from fixed BFS to learned traversal represents a fundamental shift. Fixed strategies require domain expertise to set depth parameters; learned strategies adapt to the citation structure of each topic automatically.

---

## Section 4: Planning and Outline Generation Strategies

**From hierarchical decomposition to iterative refinement — how survey systems structure content**

Survey generation requires a plan that determines the topics to cover, their organization, and the depth of coverage for each subtopic. Planning strategies range from fixed templates to dynamically evolving outlines informed by citation graph signals.

### 4.1 Hierarchical Outline Decomposition

The earliest approach to outline generation uses fixed or learned hierarchical templates. **AutoSurvey** [arXiv:2406.10252] employs a fixed outline template where the system fills section slots based on retrieved content. **SurveyForge** [arXiv:2503.04629] improves upon this by learning outline heuristics from thousands of human-written surveys. The system extracts section headings, subsections, and their typical ordering to produce outlines that resemble human writing patterns.

**SurveyX** [arXiv:2502.14776] introduces the AttributeTree, a structured pre-processing step that organizes papers by attributes such as method, dataset, and evaluation metric. The AttributeTree serves as both an outline structure and a retrieval target — each attribute maps to a section or subsection. **IterSurvey** [arXiv:2510.21900] implements a Section-Outline-to-Subsection-Detail pipeline where the planning agent starts with coarse section titles and iteratively decomposes each into finer-grained subsections.

The precursor literature includes **Generating Related Work** [arXiv:2104.08668], which demonstrated that section generation benefits from explicit structure, and **SurveySum** [arXiv:2408.16444], which applied multi-document summarization techniques to survey generation. **HierCat** [arXiv:2304.03512] provides a large-scale dataset of 7,600 hierarchical catalogues and 389,000 reference papers, supporting the development of data-driven outline generation methods (cross-referenced from Section 7.3 for its role in dataset provision).

A key open question is whether citation graph topology can inform outline structure. Clusters in the citation graph naturally map to topical communities, suggesting that community detection could provide an unsupervised outline structure. No current system fully exploits this connection.

### 4.2 Adaptive and Evolving Planning

Adaptive planning systems modify their outline as new information is retrieved. **SurveyGen-I** [arXiv:2508.14317] introduces evolving plans with memory-guided writing: the system maintains a working memory of which topics have been covered, which papers have been cited, and which claims need additional support. The plan evolves from broad section headings to detailed subsection outlines as retrieval progresses.

**CogWriter** [arXiv:2502.12568] implements cognitive writing principles with hierarchical planning and monitoring. The system tracks its own coverage completeness and adjusts the outline to fill identified gaps. **SuperWriter** [arXiv:2506.04180] extends this with reflection-driven planning, hierarchical DPO (Direct Preference Optimization), and Monte Carlo Tree Search (MCTS) for exploring alternative outline structures. These cognitive approaches view planning not as a one-shot template but as a dynamic process that co-evolves with retrieval.

### 4.3 Iterative Refinement and Self-Feedback

Iterative refinement applies feedback loops to improve outline and content quality. **Self-Refine** [arXiv:2303.17651] provides the foundational framework: generate, receive feedback, refine. **EIPE-text** (Evaluation-guided Iterative Plan Extraction) [arXiv:2310.08185] applies this specifically to outline planning, where an evaluator assesses the current plan and suggests improvements.

**SuperWriter**'s structured thinking-through process [arXiv:2506.04180] combines iterative refinement with multi-level feedback — outline quality, section coherence, and citation correctness are evaluated at different granularities. The system applies MCTS to explore alternative outline structures, selecting the one that maximizes expected quality.

### 4.4 Planning Quality and Coverage

Planning quality directly affects survey coherence and coverage completeness. **SurveyBench**'s evaluation framework includes outline quality as a standalone dimension, finding that outline quality is a significant predictor of overall survey quality [arXiv:2510.03120]. Systems with higher outline coherence scores consistently produce surveys with better content quality and reference accuracy, suggesting that improving planning should be a priority for survey system development.

Citation graph awareness at the planning stage can improve coverage completeness by identifying under-explored subtopics. Community detection algorithms (e.g., Louvain, spectral clustering) applied to citation graphs naturally partition papers into topical communities. These communities could serve as unsupervised section headings, with each community forming one outline section — a promising direction for citation-graph-informed planning that current systems do not exploit. For example, if a citation graph reveals a cluster of papers on a specific methodology not represented in the current outline, the planner can add a corresponding section. This capability remains largely unrealized in current systems, representing an opportunity for future work.

---

## Section 5: Retrieval-Augmented Pipelines for Evidence Collection

**Query formulation, evidence extraction, and multi-source synthesis for survey content**

Retrieval-augmented pipelines provide the evidence that grounds survey content. The pipeline design — how queries are formulated, how results are extracted and re-ranked, and how evidence is synthesized — determines the factual quality and coverage breadth of the generated survey.

### 5.1 Query Formulation Strategies

Survey systems differ in how they translate outline topics into search queries. **LitLLM** [arXiv:2402.01788] uses keyword extraction from the topic and outline to construct Boolean search queries, followed by a re-ranking step. **SurveyForge** [arXiv:2503.04629] employs a scholar navigation agent that formulates queries based on its memory of discovered papers, enabling iterative refinement through follow-up searches.

**SurveyX** [arXiv:2502.14776] implements attribute-aware retrieval where queries are constructed from the AttributeTree's attribute-value pairs. Each attribute (e.g., "method = Transformer", "dataset = SciReach") generates a targeted query. **SurveyGen-I** [arXiv:2508.14317] introduces coarse-to-fine retrieval: broad queries for initial paper discovery, followed by targeted queries for specific subsections.

Citation graph context can inform query formulation in ways currently under-utilized. Citation contexts — the sentences in which papers cite other papers — provide natural query seeds that encode how papers relate to each other. No current survey system systematically exploits citation contexts for query formulation.

### 5.2 Evidence Extraction and Re-Ranking

Retrieved papers undergo extraction and re-ranking to isolate relevant evidence. **LitLLM**'s re-ranking pipeline [arXiv:2402.01788] applies cross-encoders to assess query-paper relevance, selecting the top-k papers for each section. **LitFM**'s graph retriever [arXiv:2409.12177] achieves 28.1% precision improvement over cross-encoder-only approaches by incorporating citation graph structure into the relevance assessment. This contrast is instructive: LitFM uses citation graph structure to enhance retrieval while LitLLM relies on dense passage representations without graph awareness.

**PUREsuggest** [arXiv:2408.02508] provides interactive citation suggestion during the writing process, enabling researchers to discover papers they might have missed. Evidence granularity varies across systems — some extract full-text paragraphs, others use abstracts, and a few operate at the sentence level.

### 5.3 Adaptive Retrieval and Self-Reflection

Adaptive retrieval systems decide when and what to retrieve based on the current generation state. **Self-RAG** [arXiv:2310.11511] is the foundational paradigm, where the model generates a retrieval token to indicate whether retrieval is needed. **Self-Routing RAG** [arXiv:2504.01018] makes dynamic routing decisions, achieving 29% fewer retrievals while maintaining quality. **FoRAG** [arXiv:2406.13779] optimizes factuality via RLHF, and **InstructRAG** [arXiv:2406.13629] uses self-synthesized rationales for retrieval decisions. **RA-RAG** [arXiv:2410.22954] estimates source reliability to weight retrieved evidence.

*Caveat*: These adaptive retrieval methods have been validated primarily on QA and summarization tasks. Their transfer to multi-section survey generation — where each section requires different evidence at different granularities — remains an empirical question that current literature does not fully address.

### 5.4 Multi-Source Synthesis and Citation Grounding

Synthesis combines evidence from multiple sources into coherent narrative text. **OpenScholar** [arXiv:2411.14199] operates at the largest scale with a 45-million paper datastore and SciFact-based retriever, achieving citation accuracy on par with human experts and outperforming GPT-4o by 5%. **DimInd** [arXiv:2504.18496] uses facet-based synthesis with LLM assistance, organizing evidence along pre-defined facets such as methods, findings, and limitations.

The contrast between OpenScholar and DimInd illustrates a design spectrum. OpenScholar's datastore-centric approach prioritizes breadth of coverage, synthesizing evidence from a massive corpus through a modular retriever-reader pipeline. DimInd's taxonomy-driven approach prioritizes structured organization, constraining synthesis along defined dimensions. Both approaches trace their lineage to the foundational RAG framework [arXiv:2005.11401] that established retrieval followed by generation as a paradigm.

---

## Section 6: Citation Attribution and Factuality Mechanisms

**Ensuring groundedness through sentence-level citations, verification, and factuality evaluation**

Accurate citation attribution is critical for survey credibility. Generated surveys must not only produce factual claims but also correctly attribute those claims to their source papers. This section covers methods for citation generation, verification, and factuality evaluation.

### 6.1 Sentence-Level and Sub-Sentence Citation

Citation granularity has progressed from document-level attribution to sub-sentence precision. **ReClaim** [arXiv:2407.01796] introduces interleaved reference-claim generation, achieving 90% citation accuracy by generating claims and references jointly. **VeriCite** [arXiv:2510.11394] implements a three-stage verification pipeline: claim extraction, evidence selection using NLI, and citation validity assessment.

**MIRAGE** [arXiv:2406.13663] applies saliency-based attribution methods to identify which source sentences support each generated claim. **Sub-Sentence Citations** [arXiv:2509.20859] further refines granularity using credit model filtering to attribute sub-claims to specific source passages. **ARC-JSD** [arXiv:2505.16415] uses Jensen-Shannon Divergence to drive attribution without requiring fine-tuning, making it applicable to any LLM.

The progression from document-level to sub-sentence granularity reflects increased precision requirements. Document-level citation can verify that a paper was consulted but not that a specific claim originated from that paper. Sub-sentence citation provides a verifiable link between each atomic claim and its source.

### 6.2 Citation Capacity and Quality

A systematic analysis of citation generation capacity [arXiv:2410.11217] introduces a Generate-then-Refine method and evaluates factors affecting citation quality: source document access (full-text vs. abstract-only), model scale, training data quality, and citation graph awareness. The Generate-then-Refine approach first generates claims with placeholder citations, then refines the citation assignments using retrieved evidence — a two-stage pipeline that separates claim generation from citation selection. The study finds that access to full-text sources substantially improves citation accuracy (by 15–20% over abstract-only access), while larger model scales (70B+ parameter models) correlate with better citation selectivity but not necessarily higher accuracy. Coverage breadth — the fraction of claims that can be attributed to retrieved sources — is identified as a key quality dimension that current evaluation frameworks under-emphasize. The systematic finding that citation capacity is bounded by source access rather than model scale has practical implications: investing in full-text retrieval pipelines may yield greater returns than upgrading to larger models.

### 6.3 Citation Graph Expansion for Attribution Coverage

Citation graph expansion directly supports citation attribution by surfacing papers that can serve as evidence sources. **SurveyG**'s hierarchical graph [arXiv:2510.07733] identifies relevant citations across all three layers (Foundation, Development, Frontier), ensuring that claims about foundational work, contemporary developments, and frontier research all have supporting citations. **LitFM**'s graph retriever [arXiv:2409.12177] surfaces non-obvious citations — papers that are topically relevant but not retrieved by semantic search alone. **PUREsuggest** [arXiv:2408.02508] provides interactive citation gap detection, suggesting papers that fill uncovered evidence needs.

To illustrate: if a survey claims that "method X achieves SOTA accuracy on benchmark Y" citing only paper A, and graph traversal reveals that papers B and C also report similar results for method X, the system can attribute the claim to multiple corroborating sources, increasing attribution density and robustness. This capability — expanding attribution coverage through citation graph traversal — directly addresses the primary anchor question: citation graph structure guides both discovery (finding papers) and attribution (assigning claims to found papers). Systems that expand citation graphs effectively also achieve better attribution coverage.

### 6.4 Factuality Evaluation in Long-Form Text

Methods for evaluating factuality in long-form generated text can be grouped by approach. **Decomposition-based** methods break generated text into atomic facts. **FActScore** [arXiv:2305.14251] decomposes generations into atomic facts and computes the percentage supported by a reliable knowledge source, achieving <2% error rate in its automated version.

**Search-augmented** methods verify claims by searching external sources. **SAFE** (Search-Augmented Factuality Evaluation) [arXiv:2403.18802] uses a search engine to verify each claim against retrieved evidence. **Claim-verification** methods assess whether claims are verifiable or unverifiable. **VERISCORE** [arXiv:2406.19276] distinguishes verifiable claims (those that can be checked against sources) from unverifiable ones (opinions, speculation), enabling more nuanced evaluation.

**Entity-grounded** methods identify specific entities that may be hallucinated. **WildHallucinations** [arXiv:2407.17468] provides a benchmark for entity-level hallucination detection. **D-FActScore** [arXiv:2402.05629] incorporates entity-ambiguity awareness into the decomposition framework. For long scientific documents, **LongDocFACTScore** [arXiv:2309.12455] extends factuality evaluation to multi-page scientific texts, providing a direct evaluation tool for survey-length outputs.

---

## Section 7: Evaluation Methodologies and Benchmarks

**Metrics, datasets, and human evaluation protocols for assessing survey quality and citation graph effectiveness**

The evaluation of automated survey generation is itself an active research area. Multiple benchmarks have been proposed, each with different evaluation dimensions, topic coverage, and protocols. This section reviews dedicated survey benchmarks, citation-specific benchmarks, datasets, metrics, factuality evaluations, and human evaluation protocols.

### 7.1 Dedicated Survey-Generation Benchmarks

Seven major benchmarks have been proposed for survey generation evaluation. **SurveyBench** [arXiv:2510.03120] is a quiz-driven evaluation framework built from 11,343 arXiv topics and 4,947 high-quality surveys. It introduces a multi-faceted metric hierarchy covering outline quality, content quality, and non-textual richness, with a dual-mode protocol combining content-based and quiz-based answerability evaluation. **SurGE** [arXiv:2508.15658] provides a standardized benchmark for computer science survey generation with a large-scale retrieval pool of 1M+ papers and 4-dimension evaluation (information coverage, referencing accuracy, structural organization, content quality).

**SurveyEval** [arXiv:2512.02763] evaluates across three dimensions (overall quality, outline coherence, reference accuracy) over 7 subjects, augmenting LLM-as-a-Judge scoring with human references to strengthen alignment. **DeepSurvey-Bench** [arXiv:2601.15307] introduces three "academic value" dimensions — informational value, scholarly communication value, and research guidance value — and critiques existing benchmarks for flawed selection criteria. **SGSimEval** [arXiv:2508.11310] combines LLM-based scoring with quantitative metrics and human preference data. **SurveyLens** [arXiv:2602.11238] is the first discipline-aware benchmark, covering 1,000 human-written surveys across 10 disciplines with dual-lens evaluation (Discipline-Aware Rubric Evaluation + Canonical Alignment Evaluation). **SurveyScope** [arXiv:2506.12689], released with the SciSage system, provides a benchmark of 46 papers across 11 computer science domains with a focus on coherence and citation F1 evaluation.

| Benchmark | Topics | Dimensions | Protocol | Year |
|---|---|---|---|---|
| SurveyBench [2510.03120] | 11K arXiv | Outline, content, non-textual | Quiz + content | 2025 |
| SurGE [2508.15658] | CS domain | Coverage, referencing, structure, quality | Expert-written references | 2025 |
| SurveyEval [2512.02763] | 7 subjects | Overall, outline, reference | LLM + human alignment | 2025 |
| DeepSurvey-Bench [2601.15307] | Multi-domain | Academic value (3 dimensions) | Human evaluation | 2026 |
| SGSimEval [2508.11310] | Multi-domain | Outline, content, references | LLM + human preference | 2025 |
| SurveyLens [2602.11238] | 10 disciplines | Discipline-aware + canonical alignment | Dual-lens rubric | 2026 |
| SurveyScope [2506.12689] | 11 CS domains | Coherence, citation F1 | Reflector-guided | 2025 |

### 7.2 Citation-Specific Benchmarks

Benchmarks focused specifically on citation quality provide fine-grained evaluation. **CiteEval/CiteBench** [arXiv:2506.01829] introduces principle-driven citation quality evaluation with a multi-domain human-annotated benchmark and CiteEval-Auto, a model-based metric correlated with human judgments. **ALCE** [arXiv:2305.14627] is the first citation evaluation benchmark, establishing baseline citation quality metrics for RAG systems.

**CiteME** [arXiv:2407.12861] uses a multiple-choice citation identification task, revealing that LLMs achieve 4.2–18.5% accuracy versus humans at 69.7%, highlighting the significant gap in citation attribution capability. **REASONS** [arXiv:2405.02228] evaluates sentence-level citation attribution, demonstrating a 42% reduction in hallucination when training with citation-aware objectives. **Survey-Arena** [arXiv:2510.21900] provides a pairwise comparison benchmark that positions machine-generated surveys relative to human-written ones.

### 7.3 Datasets for Survey Generation

Training and evaluation rely on datasets of human-written surveys and their reference structures. **SciReviewGen** [arXiv:2305.15186] provides 10K+ reviews with 690K cited papers, the most widely used dataset for survey generation research. **HierCat** [arXiv:2304.03512] contains 7.6K hierarchical catalogues with 389K references, supporting outline generation tasks (cross-referenced from Section 4.1 for its use in hierarchical outline decomposition). **Gen-Review** [arXiv:2510.21192] offers 81K LLM-generated reviews for studying synthetic data. The **SurveyGen dataset** [arXiv:2508.17647] provides 4,200+ human-written surveys with quality-related metadata, enabling the QUAL-SG quality-aware pipeline.

### 7.4 Evaluation Metrics and Protocols

Automatic metrics include ROUGE and BLEU, but their limitations for survey evaluation are well-documented — they correlate poorly with human judgments of survey quality. More sophisticated protocols include **PROXYQA** [arXiv:2401.15042], which uses meta-questions to evaluate coverage, and the **LitLLMs evaluation study** [arXiv:2412.15249], which systematically benchmarks LLM capabilities for literature review tasks. The **Auto-survey Challenge** [arXiv:2310.04480] provides a competition framework with simulated peer-review evaluation. **Outcome-based Evaluation** [arXiv:2306.17614] assesses survey quality by downstream utility. **AutoEvalMetrics** [arXiv:2503.05712] predicts citation count and review scores as proxies for quality.

Metrics specifically designed for citation graph expansion evaluation — precision@k for citation graph retrieval, recall of relevant papers found through traversal, and coverage depth (number of citation hops traversed) — remain under-developed, with most evaluations focusing on the quality of the generated text rather than the quality of the graph traversal.

### 7.5 Hallucination and Factuality Benchmarks

General hallucination benchmarks provide tools for evaluating survey factuality. **HaluEval** [arXiv:2305.11747] offers a large-scale hallucination benchmark with multi-faceted error classification. **HALoGEN** [arXiv:2501.08292] provides 10K prompts with fine-grained hallucination type labels. **SelfCheckGPT** [arXiv:2303.08896] uses sampling-based consistency checking without requiring reference sources. **TRUE** [arXiv:2204.04991] provides a meta-evaluation framework for factual consistency metrics. **Provenance** [arXiv:2411.01022] uses NLI-based attribution verification. **DAHL** [arXiv:2411.09255] focuses on domain-specific hallucination detection for biomedicine. **ReFACT** [arXiv:2509.25868] targets scientific confabulation detection, distinguishing plausible-sounding fabrications from genuine claims.

### 7.6 Human Evaluation Protocols

Human evaluation remains the gold standard for survey quality assessment, despite its cost and scalability limitations. Dimensions assessed include coverage, coherence, accuracy, readability, and usefulness. **SurveyBench**, **SurGE**, and **SurveyEval** each include human evaluation protocols with inter-annotator agreement (Cohen's κ) reporting. The **QUAL-SG framework** from SurveyGen [arXiv:2508.17647] provides a structured quality assessment protocol for survey-specific dimensions. The **Auto-survey Challenge** [arXiv:2310.04480] introduces a non-LLM-as-judge paradigm using simulated peer review, offering an alternative to the increasingly criticized LLM-as-a-Judge approach.

---

## Section 8: Emerging Frontiers — Interactive, Living, and Coordinated Survey Paradigms

**Personalized generation, continuous updating, and advanced multi-agent coordination for citation-graph-aware surveys**

Beyond the static, one-shot generation paradigm, several emerging directions push toward more adaptive, persistent, and coordinated survey systems. Each subsection in this section meets a minimum of 150 words to ensure sufficient technical depth.

### 8.1 Interactive and Personalized Survey Generation

Interactive systems place the researcher in the generation loop, enabling human guidance at critical decision points. **InteractiveSurvey** [arXiv:2504.08762] (cross-referenced from Section 2.3) allows users to customize reference categorization, outline structure, and content during generation through an explicit user interface. Users can upload their own paper collections and guide citation graph traversal decisions interactively, effectively steering which papers are incorporated. **ChatCite** [arXiv:2403.02574] provides human workflow guidance for comparative literature summarization through a reflective incremental mechanism that extracts and summarizes elements with user guidance. Its conversation-driven interface allows researchers to refine queries and focus areas iteratively.

**DimInd** [arXiv:2504.18496] (cross-referenced from Section 5.4) enables facet-based synthesis where users specify which facets to cover, and the system organizes evidence along those dimensions. **Synergi** [arXiv:2308.07517] implements mixed-initiative scholarly synthesis, where the system proposes structures and the user refines them through an interactive notebook interface. The spectrum ranges from fully automatic (no user input beyond the topic) to fully manual (user specifies every paper and section), with most interactive systems operating in the middle where the system generates initial drafts and the user guides revision. Interactive approaches improve citation precision by incorporating domain expertise into traversal decisions, but they require researcher time and do not scale to batch processing of many topics.

### 8.2 Living and Continuously Updated Surveys

Living survey paradigms maintain citation freshness over time by periodically re-retrieving and re-synthesizing evidence. **vitaLITy 2** [arXiv:2408.13450] uses a RAG architecture for iterative literature search across a 66,692-paper corpus, with periodic re-retrieval to incorporate new publications as they appear. The system maintains a persistent index that can be updated incrementally, avoiding full regeneration when new papers are added. **Evolving Literature Analysis** [arXiv:2502.18791] proposes semi-automated longitudinal tracking of research topics, monitoring citation graph changes to detect emerging work and flagging when new evidence contradicts earlier survey claims.

**Completing Systematic Reviews in Hours** [arXiv:2504.14822] introduces InsightAgent, which uses an interactive workflow to complete reviews rapidly while maintaining update capability. Its architecture separates the initial review from ongoing monitoring, allowing the system to flag when a review section has become outdated due to new publications. Living surveys require continuous citation graph monitoring — tracking new papers, updated citations, and shifting citation patterns. This requirement motivates the development of citation-aware retrieval systems that can detect when new evidence renders a survey claim stale. The key technical challenge is efficient incremental updating: re-running full retrieval for each update is cost-prohibitive, but selective re-retrieval risks missing important developments.

### 8.3 Advanced Multi-Agent Coordination Patterns

As multi-agent survey systems grow more complex, coordination mechanisms become critical for dividing the search space and integrating findings. **AgensFlow** [arXiv:2605.27466] introduces learned routing between agents, optimizing the flow of information based on task requirements. The routing policy learns which agent should handle which subtopic, effectively partitioning the citation graph based on detected topic clusters. **KABB** (Knowledge-Aware Bayesian Bandits) [arXiv:2502.07350] applies bandit-based selection to agent coordination, learning which agent configurations work best for different literature domains. The bandit prioritizes agents that have found high-relevance papers in specific graph regions, effectively directing exploration toward promising neighborhoods.

**Federation of Agents** [arXiv:2509.20175] proposes semantics-aware communication protocols that reduce information loss during agent handoffs. By communicating through structured representations (shared paper lists, evidence summaries) rather than natural language, the federation minimizes redundancy across agents working on overlapping citation neighborhoods. **AgentCoord** [arXiv:2404.11943] provides visual exploration tools for understanding coordination dynamics, enabling human oversight of multi-agent workflows.

LiRA's multi-agent workflow [arXiv:2510.05138] (cross-referenced from Section 2.2) exemplifies these patterns applied to survey generation, with dedicated agents for outlining, writing, editing, and reviewing. Coordination patterns directly affect citation graph partitioning — how the search space is divided among agents — and thus influence coverage breadth and redundancy. AgensFlow's learned routing could assign different citation subgraphs to different agents based on detected topic clusters from the citation graph; KABB's bandit selection could prioritize agents that have found high-relevance papers in specific graph regions, effectively partitioning exploration; Federation of Agents' semantics-aware communication could reduce redundant expansion across agents working on overlapping citation neighborhoods. These mechanisms are critical because uncoordinated multi-agent traversal risks either overlapping coverage (wasting resources) or missing entire subtopics (reducing coverage breadth).

### 8.4 Graph-Aware Deep Research Paradigms

The convergence between automated survey generation and deep research agents is producing systems that blur the line between survey writing and research synthesis. **PaperQA2** [arXiv:2409.13740] achieves superhuman performance on literature search tasks, matching or exceeding human experts on the LitQA2 benchmark and performing contradiction detection across multiple papers. PaperQA2's modular retriever-reader architecture mirrors the coordination patterns from Section 8.3: its retriever agent iterates over citation neighborhoods while the reader agent synthesizes findings across papers, achieving superhuman performance on LitQA2 benchmarks. This demonstrates that the boundary between "survey generation" and "deep research" is blurring, as both require scalable citation graph traversal with robust evidence aggregation.

**OpenScholar** [arXiv:2411.14199] synthesizes scientific literature at scale with a modular retriever-reader pipeline, achieving citation accuracy on par with human experts. Its architecture — separate retriever, reader, and synthesizer components — parallels the coordination patterns in Section 8.3, suggesting that modular, composable architectures are emerging as the dominant paradigm for both survey generation and deep research. The **Deep Search Agents Survey** [arXiv:2508.05668] provides a comprehensive overview of deep research paradigms, positioning survey agents within the broader landscape of automated scientific inquiry. The survey catalogs architectures including OpenAI Deep Research, Gemini Deep Research, and Perplexity Deep Research, comparing their retrieval strategies, synthesis approaches, and evaluation protocols against academic survey generation systems.

---

## Section 9: Open Challenges and Future Directions

**Key limitations, research gaps, and promising avenues in citation-graph-aware automated survey generation**

Despite significant progress, citation-graph-aware survey generation faces several open challenges. We identify six areas requiring further research.

### 9.1 Hallucination and Citation Fabrication

Hallucination manifests in survey generation as fabricated citations, misattributed claims, and outdated facts presented as current. **SciReviewGen** [arXiv:2305.15186] reports 15–25% hallucination rates in generated surveys, a level that undermines trust. The real-world citation hallucination audit referenced in Section 1 identified approximately 147K hallucinated citations across published literature in 2025 alone [arXiv:2605.07723], indicating that the problem extends beyond survey generation to the broader scientific publishing ecosystem. Detection methods such as **FActScore** [arXiv:2305.14251], **SAFE** [arXiv:2403.18802], and **SelfCheckGPT** [arXiv:2303.08896] (cross-referenced from Section 6.4) provide evaluation tools but do not solve the generation-side problem.

An integrated mitigation pipeline would combine verification (checking each claim against retrieved sources), citation graph cross-referencing (checking that cited papers actually support the attributed claim), and human oversight for high-stakes claims. Notably, citation graph traversal can both help (by surfacing corroborating evidence across multiple papers) and hurt (by propagating errors through citation chains when one paper's incorrect claim is cited by others). Developing error-propagation-aware traversal strategies that detect and quarantine unreliable citation chains remains an open problem.

### 9.2 Evaluation Standardization

The proliferation of benchmarks (Section 7) testifies to the need for standardized evaluation, but no unified framework yet exists. **SurGE**, **SurveyEval**, and **SurveyLens** measure different dimensions with different protocols, making cross-system comparison difficult. LLM-as-a-Judge evaluation is increasingly criticized [arXiv:2412.15249] for bias toward LLM-generated text, and data contamination remains a concern when training and evaluation data share topics. Coverage breadth — the fraction of relevant papers in a topic that a survey cites — should be standardized as a key evaluation dimension, as it is currently measured inconsistently across benchmarks. Some benchmarks measure recall against a fixed reference set, while others use expert annotations, making direct comparison impossible. The history of NLP suggests that a standardized evaluation framework akin to **GLUE (General Language Understanding Evaluation)** could catalyze progress by enabling apples-to-apples comparison across architectures and traversal strategies.

### 9.3 Knowledge Freshness and Temporal Dynamics

Surveys become stale as new research is published. A temporal-ordering failure example: citing a 2025 result as "recent" without noting that an earlier 2024 survey already covered the 2020–2023 work in that area. Living survey paradigms like **vitaLITy 2** [arXiv:2408.13450] and **Evolving Literature Analysis** [arXiv:2502.18791] (cross-referenced from Section 8.2) address this but are not yet widely adopted.

Specific temporal reasoning gaps include: outdated claims not flagged when newer evidence contradicts them, versioning of cited papers (preprint vs. published version), and temporal ordering of evidence (which papers built on which). Citation graph timestamps provide a natural signal for temporal relevance ranking — newer papers that cite foundational work are likely to represent contemporary understanding — but this signal is not systematically exploited. A further challenge is temporal attribution: when multiple papers report the same finding, the system should cite the earliest source rather than a later derivative, but current systems lack this provenance awareness.

### 9.4 Citation Graph Traversal Trade-offs

Citation graph traversal involves a fundamental exploration-exploitation trade-off. Deep traversal (many citation hops) can uncover foundational work and distant but relevant papers, but risks topic drift as each hop adds noise. Broad traversal (many siblings at the same hop) improves coverage of the current topic area but increases computational cost proportionally. **PaSa** [arXiv:2501.10120] (cross-referenced from Section 3.4) directly addresses this trade-off through reinforcement learning, learning when to expand deeply versus broadly based on the citation structure encountered. Its +37.78% recall improvement demonstrates that learned policies outperform fixed strategies.

Hybrid search strategies for systematic literature reviews [arXiv:2004.09741] quantify these trade-offs using precision, recall, and F-measure. The study finds that no single traversal strategy dominates across all topics — optimal strategy depends on the field's citation density, the topic's breadth, and the desired balance between recall and precision. Open questions include: how to determine optimal depth parameters automatically, how to design adaptive stopping criteria that halt traversal when new papers no longer improve coverage, and how to formulate multi-objective optimization for coverage vs. relevance vs. cost. The connection to coverage breadth is direct: deeper traversal improves recall of foundational work, while broader traversal captures more contemporary developments, and the optimal balance depends on the survey's purpose.

### 9.5 Domain Adaptation and Cost

Survey systems adapted for one domain (e.g., computer science) may not transfer to others with different citation practices, discourse structures, and terminology. Domain-specific systems like **PaperQA2** [arXiv:2409.13740] and **OpenScholar** [arXiv:2411.14199] have been validated primarily on CS and biomedical literature. Domain-agnostic approaches must handle varied citation densities (physics papers cite hundreds of references; humanities papers cite fewer), different publication timelines, and field-specific evaluation expectations.

Cost remains a practical barrier. Generating a single survey costs $10–50 in API fees for current multi-agent systems processing 75–443 papers per topic, depending on model choice and retrieval volume [arXiv:2509.18661]. For living surveys that require periodic regeneration, these costs multiply. Whether citation graph structure varies systematically across disciplines — and how traversal strategies should adapt to disciplinary citation norms — is an open empirical question. Computer science citation graphs are dense with short citation chains, while biomedical graphs are sparse with longer chains; optimal BFS depth likely differs accordingly.

### 9.6 Multi-Modal and Non-Textual Content

Current survey systems operate almost exclusively on text, missing tables, figures, equations, and code. Concrete examples of content types that current systems miss include: architecture diagrams in systems papers that convey design rationale not captured in text, mathematical derivations in theoretical work where the equations encode the core contribution, empirical result tables with statistical comparisons that provide the evidentiary foundation, and algorithm pseudocode that defines the method. Multi-modal LLMs such as GPT-4V and LLaVA demonstrate the feasibility of processing visual content, but their integration into survey generation pipelines is nascent.

**PaperArena** [arXiv:2510.10909] provides a multi-tool orchestration benchmark that includes multimodal parsing capabilities (OCR for figures, table extraction, code execution), demonstrating that these technologies are mature enough for integration. The **Deep Search Agents Survey** [arXiv:2508.05668] discusses multi-modal evaluation gaps, noting that current benchmarks evaluate only text-based understanding. Citation graphs primarily connect textual papers; figure and table citation links remain under-explored. A paper's figure may be cited by another paper's text, but this relationship is typically not encoded in citation graph structure. Extending citation graphs to include non-textual artifacts would require new infrastructure for extraction, indexing, and linking — but would unlock the ability to generate surveys that synthesize visual and quantitative evidence alongside textual claims.

---

## Section 10: Conclusion

**Synthesis, practical recommendations, and a research agenda for citation-graph-aware survey agents**

This survey has examined automated literature survey agents through the lens of citation graph expansion, addressing two anchor questions: how citation graph structure guides paper discovery and content synthesis (primary), and what failure modes and mitigations exist (secondary). We review the architectural landscape and provide actionable recommendations for practitioners and researchers.

**Architectural recommendations.** Single-agent systems (AutoSurvey, SurveyX, LitLLM) are suitable for focused topics where keyword search suffices and citation structure is secondary. Multi-agent systems with hierarchical citation graph integration (SurveyG, Agentic AutoSurvey) are recommended for broad-coverage surveys requiring structured coverage across foundational, contemporary, and frontier work. Hybrid systems with iterative refinement (IterSurvey, SciSage) offer the best balance for topics where citation structure significantly informs paper discovery and where user guidance is valuable. When citation graph depth is important — for example, tracing a field's intellectual lineage — SurveyG's hierarchical traversal [arXiv:2510.07733] or LitFM's graph retriever [arXiv:2409.12177] are the current best options.

**Evaluation checklist.** Practitioners evaluating a citation-graph-aware survey system should measure: citation precision (what fraction of cited papers actually support their attributed claims), coverage breadth (what fraction of relevant papers in the topic are cited), and factual consistency (what fraction of claims are supported by retrieved sources). Recommended benchmarks include SurveyBench for general evaluation [arXiv:2510.03120], SurGE for CS-domain evaluation [arXiv:2508.15658], and CiteEval for citation quality specifically [arXiv:2506.01829]. For hallucination detection, FActScore [arXiv:2305.14251] provides a reliable automated baseline.

**Research agenda.** The three most impactful directions for future work are: (1) developing optimal citation graph traversal policies that learn depth and breadth parameters automatically, replacing the fixed strategies used by current systems; (2) establishing a standardized evaluation framework for citation-aware survey systems, analogous to **GLUE (General Language Understanding Evaluation)** in NLP, to enable systematic comparison across architectures and traversal strategies; and (3) advancing interactive and living survey paradigms that combine continuous citation graph monitoring with user-in-the-loop refinement, enabling surveys that remain fresh as the research landscape evolves.

The convergence of citation graph expansion, survey generation, and deep research paradigms points toward a future where automated survey agents serve not just as writing assistants but as active participants in scientific discovery — identifying gaps in the literature, tracing intellectual lineages, and synthesizing knowledge at a scale beyond human capacity. Realizing this vision requires addressing the open challenges identified in this survey, particularly hallucination mitigation, evaluation standardization, and efficient citation graph traversal.
