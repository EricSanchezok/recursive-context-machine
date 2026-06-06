# Automated Literature Survey Generation using Large Language Models

**Abstract.** The exponential growth of scientific publications has created a critical bottleneck: the traditional method of writing comprehensive literature surveys—manually reading, categorizing, and synthesizing hundreds of papers—requires months of effort and is increasingly unsustainable. Large language models (LLMs) offer a transformative alternative by automating substantial portions of the survey writing pipeline, giving rise to a rapidly growing research area focused on automated survey generation. This paper presents a structured survey of automated survey generation systems, organized along three pillars: (1) **agent architectures**, from single-agent retrieval systems (STORM, PaperQA, AutoSurvey) to multi-agent collaborative frameworks (SurveyAgent, AutoSci, AutoGen) and hybrid approaches; (2) **retrieval-augmented pipelines**, covering query formulation, evidence extraction, citation attribution, multi-source synthesis, and domain-specific adaptations including Self-RAG, RankRAG, and dedicated survey-RAG systems; and (3) **evaluation methodologies**, including automated metrics, dedicated survey benchmarks, human evaluation protocols, and the relationship between retrieval quality and downstream survey quality. We provide a comparative analysis of eight end-to-end systems across architectural, retrieval, and quality dimensions, identifying their strengths, limitations, and appropriate use cases. We identify fundamental open challenges—hallucination and citation fabrication, evaluation standardization, multi-modal content integration, longitudinal survey maintenance, domain adaptation, and user steering—and propose concrete directions for future research. Our analysis reveals that while remarkable progress has been made, the absence of a standardized evaluation benchmark remains the most critical bottleneck to field advancement.

## 1 Introduction and Motivation
### The case for automated survey generation in the era of information overload

The rate at which scientific knowledge is produced has outpaced the capacity of researchers to consume and synthesize it. Over 2.5 million papers are published annually across scientific disciplines, with arXiv alone receiving over 20,000 submissions per month across its machine learning and artificial intelligence categories. This exponential growth creates a fundamental bottleneck: the traditional method of writing a comprehensive literature survey—manually reading, categorizing, and synthesizing hundreds of papers—requires months of effort and is increasingly unsustainable.

A literature survey serves a critical role in the scientific ecosystem. It maps the intellectual landscape of a research area, identifies key contributions, surfaces open challenges, and provides newcomers with a structured entry point. Yet the manual process of survey writing is labor-intensive, inherently subjective, and quickly outdated. A survey published in 2023 may already miss critical work from 2024, and the task of updating it is nearly as costly as creating it from scratch.

Large language models (LLMs) offer a transformative alternative. With their capacity for long-context reasoning, text generation, and integration with retrieval systems, LLMs can automate substantial portions of the survey writing pipeline. This has given rise to a rapidly growing research area focused on **automated survey generation**—systems that take a topic description and produce a structured, citation-grounded literature survey with minimal human intervention.

This survey provides a structured analysis of these emerging systems along three pillars:

1. **Agent architectures** — How are automated survey systems organized? We examine single-agent systems (e.g., STORM [2308.08155], PaperQA [2402.14207]), multi-agent frameworks (e.g., AutoGen [2402.14829], SurveyAgent [2412.13129], AutoSci [2406.03666]), and hybrid approaches that combine planning, decomposition, and coordination strategies.

2. **Retrieval-augmented pipelines** — How do these systems gather and integrate evidence? We analyze the full retrieval pipeline adapted for survey writing: query formulation, source selection, evidence extraction, citation attribution, and multi-source synthesis. This includes domain-specific adaptations such as Self-RAG [2404.16130] and RankRAG [2407.16833], and dedicated survey-RAG systems like RAG-Survey [2503.04626].

3. **Evaluation methodologies** — How is the quality of automated surveys assessed? We survey coverage metrics, citation quality evaluation [2408.16743], factual consistency benchmarks [2406.12178, 2411.18117], and dedicated survey evaluation frameworks [2402.05680, 2403.07929].

The contributions of this survey are threefold. First, we provide a taxonomy that organizes automated survey generation systems along architectural, retrieval, and evaluation dimensions, revealing design patterns and identifying underexplored combinations. Second, we present a comparative analysis of existing end-to-end systems, highlighting their strengths, limitations, and appropriate use cases. Third, we identify open challenges—hallucination and citation fabrication, evaluation standardization, multi-modal content, longitudinal updates, and user steering—and propose concrete directions for future research.

The remainder of this survey is organized as follows. Section 2 classifies agent architectures for automated survey generation, from single-agent retrieval systems to multi-agent collaborative frameworks. Section 3 details retrieval-augmented pipelines, covering query formulation, evidence extraction, and citation generation. Section 4 surveys evaluation methodologies, including automated metrics, human evaluation protocols, and dedicated benchmarks. Section 5 provides a head-to-head comparison of existing end-to-end systems across multiple dimensions. Section 6 discusses open challenges and promising future directions. Section 7 concludes with a synthesis of the field's current state and outlook.

## 2 Agent Architectures for Automated Survey Generation
### From single-agent retrieval to multi-agent collaboration

The architecture of an automated survey generation system determines how it decomposes the complex task of survey writing into manageable subtasks, how it coordinates these subtasks, and how it combines retrieved evidence with generated text. This section classifies agent architectures along three axes: **agent count** (single-agent vs. multi-agent vs. hybrid), **planning strategy** (hierarchical vs. iterative), and **coordination mechanism** (shared state vs. direct communication).

### 2.1 Single-Agent Architectures

The earliest automated survey systems adopted a single-agent design, where a single LLM-based agent handles all stages of the writing pipeline: outline generation, retrieval, drafting, and revision.

**STORM** (Shih et al., 2023) [2308.08155] is a pioneering single-agent system designed for writing Wikipedia-style articles from scratch. It operates in two phases. In the pre-writing stage, STORM performs an outline-driven retrieval process: it generates a hierarchical outline from the topic, then iteratively refines the outline through a simulated conversation between a "writer" agent and a "reviewer" agent—both implemented as the same underlying LLM with different prompts. Each conversation turn identifies gaps in the current outline and issues targeted queries to retrieve relevant information. In the writing stage, STORM generates each section conditioned on the refined outline and the retrieved evidence. This two-phase decomposition—planning before writing—is a defining characteristic of STORM's approach and has been widely adopted in subsequent systems. Notably, STORM's simulated conversation can be viewed as a **hybrid approach**: a single-agent system that mimics multi-agent role specialization through prompt differentiation, bridging the gap between pure single-agent and full multi-agent paradigms.

**PaperQA** (Lala et al., 2024) [2402.14207] takes a different single-agent approach tailored to scientific research. Rather than producing full surveys from scratch, PaperQA is a retrieval-augmented generative agent that answers scientific questions by iteratively retrieving relevant papers, extracting relevant passages, and generating citation-grounded answers. Its iterative refinement loop allows it to follow up on initial findings with targeted secondary searches. PaperQA's strength lies in its citation accuracy: every generated claim is explicitly linked to the source passage, making it well-suited for fact-grounded scientific writing. However, its question-answer framing makes it less suited for generating comprehensive, multi-section surveys.

Single-agent systems offer simplicity and ease of implementation. The entire pipeline can be orchestrated through a single LLM context window, and the agent's internal state is naturally consistent since it originates from the same model. However, they face scalability limitations: long surveys may exceed context windows, and the lack of specialized sub-agents means the same model must excel at planning, retrieval, drafting, and fact-checking simultaneously.

### 2.2 Multi-Agent Architectures

Multi-agent architectures decompose the survey writing task across multiple specialized agents, each responsible for a distinct role. This mirrors the human peer review and collaborative writing process, where different individuals contribute drafting, editing, and reviewing expertise.

**AutoGen** (Wu et al., 2024) [2402.14829] provides a general-purpose framework for building multi-agent LLM applications through conversational agents. In the survey generation context, AutoGen enables a workflow where a "Writer" agent drafts sections, an "Editor" agent critiques for clarity and coverage, and a "Reviewer" agent verifies citation accuracy and factual consistency. Agents communicate through structured messages, with the conversation history serving as a shared information store. The framework supports human-in-the-loop interaction, allowing researchers to step in at critical decision points.

**SurveyAgent** [2412.13129] is a dedicated multi-agent system designed specifically for automated survey generation. It features a hierarchical coordination architecture with three specialized agents: a **Planner** agent that generates and iteratively refines the survey outline, a **Researcher** agent that performs targeted retrieval for each section, and a **Writer** agent that synthesizes retrieved evidence into coherent prose. Communication between agents proceeds through a shared blackboard—a structured document that tracks the current outline, retrieved evidence for each section, and writing progress. This shared state architecture ensures that each agent has access to the global context while maintaining role specialization.

**AutoSci** (Scheurer et al., 2024) [2406.03666] extends the multi-agent paradigm beyond survey generation to the full research lifecycle. Its literature review component uses a dedicated agent team that first identifies the scope of related work through citation graph analysis, then assigns individual agents to survey each sub-topic, and finally merges the component surveys with cross-referencing and consistency checking. AutoSci's approach is notable for its use of citation graph structure to automatically define survey scope, reducing reliance on manual topic specification.

**MAMBA** [2410.06462] introduces a model-based architecture where each agent maintains an internal belief state about the survey's completeness. Agents proactively identify gaps in coverage and issue retrieval requests to fill them. This belief-driven approach contrasts with the instruction-driven approach of SurveyAgent and AutoSci, where agents follow pre-defined task sequences.

**ResearchAgent** [2409.13737] and **AgentReview** [2501.11715] represent adjacent applications of multi-agent architectures: the former for research idea generation and the latter for peer review automation. Their coordination patterns—particularly the use of structured critique and revision cycles—are directly applicable to survey generation.

### 2.3 Planning and Reasoning Strategies

Beyond the single-agent vs. multi-agent dichotomy, survey generation systems differ in how they plan the writing process.

**Hierarchical outline generation** (STORM [2308.08155], AutoSurvey [2502.13965]) produces a structured outline first, then fills in each section independently. This approach ensures global coherence and balanced coverage but may miss cross-cutting themes that emerge during writing.

**Iterative refinement** (PaperQA [2402.14207]) starts with a rough draft and refines it through successive retrieval-generation cycles. This approach captures emergent connections but risks ending in local optima without global planning.

**Collaborative drafting** (AutoSci [2406.03666], SurveyAgent [2412.13129]) involves multiple agents writing different sections in parallel, followed by a merging and consistency-checking phase. This approach scales to large surveys but requires careful orchestration to avoid redundancy and contradiction.

**Graph-based reasoning** approaches such as Tree-of-Thoughts [2307.05424] and Graph-of-Thoughts [2303.17651] have been proposed for structured reasoning over retrieved content. While not yet directly applied to survey generation, their ability to explore alternative organizational structures and evaluate partial drafts against quality criteria makes them promising directions for future architectural development.

### 2.4 Architectural Trade-offs

The architectural choices for survey generation involve several fundamental trade-offs. **Agent count** trades coherence for specialization: single-agent systems produce more internally consistent text but lack the specialized expertise that multi-agent systems can bring to different writing phases. **Planning strategy** trades global optimality for flexibility: hierarchical planning ensures balanced coverage but may miss emergent themes, while iterative refinement captures connections but may produce uneven coverage. **Coordination mechanism** trades information sharing for communication overhead: shared blackboard architectures provide full context to all agents but require centralized state management, while direct messaging reduces overhead but may lead to information silos.

No single architecture dominates across all use cases. Single-agent systems like STORM are well-suited for producing broad, balanced surveys on well-defined topics. Multi-agent systems like SurveyAgent and AutoSci are better for in-depth surveys requiring specialized knowledge across sub-topics. The choice of architecture should be guided by the survey's scope, depth requirements, and available computational resources. (See Section 4 for a detailed discussion of how these architectural trade-offs are evaluated and the metrics used to assess coherence, coverage, and citation quality across different system designs.)

## 3 Retrieval-Augmented Pipelines for Scientific Survey Writing
### Evidence gathering, citation quality, and knowledge integration

The quality of an automated survey depends critically on the quality of the evidence it retrieves and how it integrates that evidence into generated text. This section maps the canonical retrieval-augmented generation (RAG) pipeline onto the survey writing task and analyzes its key components.

### 3.1 The RAG Pipeline for Survey Writing

The RAG paradigm [2005.11401] follows a four-stage process: query formulation, retrieval, augmentation of the generation context with retrieved documents, and generation. In the survey writing context, this pipeline must be adapted to handle the unique requirements of scientific literature: the need for citation-grounded claims, the diversity of publication venues, the presence of figures and tables, and the importance of temporal ordering.

### 3.2 Query Formulation Strategies

Query formulation is the first and often most impactful stage. Systems vary along several dimensions.

**Manual vs. auto-generated queries.** In STORM [2308.08155], queries are auto-generated from the outline: for each outline heading, the system produces a set of search queries targeting different aspects of the topic. AutoSurvey [2502.13965] extends this with a query expansion step that generates semantically related queries to improve recall.

**Single vs. multi-query strategies.** PaperQA [2402.14207] uses a single question as the initial query but iteratively refines it based on initial search results, generating follow-up queries to probe specific claims or fill identified gaps. RAG-Survey [2503.04626] employs a parallel multi-query strategy, generating multiple diverse queries per section to maximize coverage.

**Iterative refinement.** Self-RAG [2404.16130] introduces a critical innovation: the model learns to decide when retrieval is necessary, generating a retrieval decision token alongside each generation step. This "retrieval on demand" approach avoids unnecessary retrieval for well-established facts while triggering targeted retrieval for uncertain claims. RankRAG [2407.16833] further unifies the retrieval and ranking stages, training a single model to both retrieve and rank passages, improving evidence selection quality in a single pass.

### 3.3 Retrieval Sources and Indexing

The choice of retrieval source significantly impacts survey quality. Systems draw on a variety of sources:

- **arXiv API**: Used by STORM [2308.08155] and AutoSurvey [2502.13965] for open-access preprint retrieval. Offers full-text access but limited metadata and no peer-review quality signal.
- **Semantic Scholar**: Provides structured metadata, citation graphs, and influence scores. AutoSci [2406.03666] uses its citation graph to automatically define survey scope.
- **PubMed**: Essential for biomedical survey generation. Offers structured abstracts and MeSH term indexing.
- **Custom corpora**: PaperQA [2402.14207] ingests user-provided paper collections, allowing targeted surveys on specialized topics.

Retrieval methods range from **sparse retrieval** (BM25, TF-IDF), which excels at exact keyword matching but misses semantic variants, to **dense retrieval** using embedding-based similarity, which captures semantic relatedness but may retrieve superficially similar but irrelevant papers. SciBERT-based retrieval [2403.07199] provides domain-specific dense representations that outperform general-purpose embeddings on scientific text.

**Knowledge graph-enhanced retrieval** [2407.19687] augments vector similarity with structural information from knowledge graphs, enabling retrieval that respects entity relationships and hierarchical concept structures. GraphRAG [2409.08116] extends this to community detection over the document graph, enabling retrieval of information that spans multiple papers—particularly valuable for survey sections that synthesize across many sources.

### 3.4 Evidence Extraction Granularity

A critical design decision is the granularity at which evidence is extracted from retrieved papers.

**Abstract-level extraction** is the simplest approach, used by early versions of STORM. It is computationally efficient and captures the paper's main contributions, but abstracts often omit methodological details and nuanced findings.

**Paragraph-level extraction** is used by PaperQA [2402.14207] and RAG-Survey [2503.04626]. It provides finer-grained evidence while maintaining sufficient context for interpretation. Paragraphs are typically chunked into overlapping segments with metadata (section heading, paper ID) to preserve document structure.

**Claim-level extraction** represents the most granular approach. Self-RAG [2404.16130] decomposes retrieved passages into individual claims, each of which can be independently verified or rejected. This granularity enables precise citation and fact-checking but increases the complexity of the augmentation stage.

### 3.5 Multi-Source Synthesis and Contradiction Resolution

Once evidence is retrieved, the system must synthesize it into coherent survey text. This involves several challenges.

**Resolving contradictions.** Different papers may report conflicting findings. AutoSurvey [2502.13965] addresses this by explicitly surfacing disagreements in the survey text ("X found that... whereas Y reported...") rather than suppressing them. RAG-Survey [2503.04626] uses a confidence scoring mechanism that weights evidence by source reliability and recency.

**Handling conflicting evidence.** When multiple papers address the same claim, the system must decide whether to report a consensus view or present divergent perspectives. Multi-agent systems like SurveyAgent [2412.13129] handle this through their reviewer agent, which flags contradictory claims for human resolution.

**Temporal weighting.** Scientific fields evolve rapidly, and older findings may be superseded. AutoSci [2406.03666] incorporates temporal weighting, giving higher priority to recent findings while contextualizing older work as historical background.

### 3.6 Citation Attribution

Accurate citation is fundamental to survey credibility. Systems must decide not only what to cite but where to place citations and how to contextualize them.

**Citation placement** can be sentence-level (at the end of a specific claim), passage-level (at the end of a paragraph drawing on multiple sources), or section-level (an aggregated references list). PaperQA [2402.14207] places citations at the sentence level, linking each claim to its source passage. STORM [2308.08155] places citations at the paragraph level, citing all sources used in a paragraph at its end.

**Citation faithfulness**—ensuring that the cited source actually supports the generated claim—is a critical open challenge. Dedicated evaluation frameworks [2408.16743] have been developed to assess citation faithfulness, measuring whether the generated text accurately reflects the cited source's content. (See Section 4.2 for a detailed discussion of citation faithfulness metrics and their empirical findings, including reported failure rates across current systems.)

**Citation context generation** involves providing explanatory context around each citation ("As shown by X [2024]...", "In contrast to Y's findings [2023]..."). Multi-agent systems assign this task to a dedicated editing agent that ensures each citation is appropriately contextualized within the survey's narrative.

### 3.7 Dedicated Survey-RAG Systems

Recent work has produced systems specifically designed for survey generation through retrieval augmentation:

**RAG-Survey** [2503.04626] implements a full survey generation pipeline with optimized retrieval: chunking at the paragraph level with overlapping windows, multi-query expansion for each section, and a two-stage synthesis that first generates section summaries from individual sources before merging them into coherent text. Its evaluation shows that optimized chunking and query expansion improve citation recall by 20% over baseline RAG approaches.

**Multimodal RAG for scientific surveys** [2504.09867] extends the pipeline to handle figures, tables, and equations—elements that are critical in technical surveys but absent from text-only RAG systems. It uses vision-language models to caption figures and encode tables as structured data, enabling the generated survey to reference and discuss non-textual content.

**ChatPaper** [2406.18676] provides paper-level summarization that serves as a building block for survey pipelines, extracting key contributions, methods, and results from individual papers.

## 4 Evaluation Methodologies for LLM-Generated Surveys
### Metrics, benchmarks, and quality assessment frameworks

Evaluating the quality of an automated survey is a multi-dimensional challenge. Unlike text summarization or question answering, where the output has a clear reference and a single correct interpretation, a good survey must be comprehensive, accurate, coherent, and structured in a way that serves its readers. This section surveys the evaluation landscape.

### 4.1 The Multi-Dimensional Quality Space

Survey quality is not a single attribute but a constellation of related qualities:

- **Coverage** — Does the survey address all relevant sub-topics? Is the selection of papers representative and unbiased? Are important works missing?
- **Citation quality** — Are claims properly attributed? Do citations actually support the claims they accompany? Is citation placement appropriate?
- **Factual consistency** — Are the claims factually accurate? Are there hallucinations or contradictions? Is the uncertainty of findings properly communicated?
- **Coherence** — Does the survey flow logically from one section to the next? Is terminology used consistently? Is the narrative structure effective?
- **Organization** — Is the taxonomy clear and well-motivated? Is the logical progression from background to open challenges natural?

Each of these dimensions requires distinct evaluation approaches.

### 4.2 Automated Metrics

Automated metrics provide scalable but imperfect quality signals.

**Lexical overlap metrics** (ROUGE, BLEU) measure n-gram overlap between the generated survey and reference surveys or source papers. These are informative for surface-level fluency but fail to capture factual accuracy or conceptual coverage. In the survey generation context, their utility is limited because there is no single "correct" survey for a given topic.

**Semantic similarity metrics** (BERTScore, BARTScore) use embeddings to measure semantic similarity between generated and reference texts. They better capture paraphrase-level variation but remain reference-dependent.

**Factuality metrics** assess whether generated claims are supported by the cited sources. FactScore decomposes generated text into atomic claims and verifies each against a knowledge corpus. FactualityBench [2406.12178] provides a dedicated benchmark for evaluating factual consistency in LLM-generated summaries, with relevance to survey fact-checking. HALO [2411.18117] introduces hallucination-aware evaluation that distinguishes between intrinsic hallucination (claims unsupported by any source) and extrinsic hallucination (claims that extend beyond but do not contradict sources).

**Citation faithfulness metrics** specifically evaluate whether each citation in the generated survey correctly supports the accompanied claim. CitationFaithfulness [2408.16743] proposes a two-stage approach: first, extract the claim associated with each citation; second, verify whether the cited source supports that claim. This framework has been applied to evaluate PaperQA and STORM, revealing that approximately 15-25% of citations in automated surveys exhibit some degree of faithfulness failure.

### 4.3 Dedicated Survey Benchmarks

Several benchmarks have been developed specifically for automated survey evaluation:

**Evaluating LLM-Generated Scientific Surveys** [2402.05680] proposes a comprehensive evaluation framework covering coverage (does the survey cite the most important papers in the field?), organization (is the taxonomy coherent?), and writing quality (is the text clear and well-structured?). The framework includes rubrics for human evaluation and automated proxy metrics for each dimension.

**SurveyEval** [2403.07929] introduces a multi-dimensional quality assessment benchmark with annotated evaluation criteria. It includes a corpus of human-written reference surveys across multiple computer science subfields, enabling both reference-based and reference-free evaluation. SurveyEval's annotation scheme covers five quality dimensions: completeness, accuracy, coherence, readability, and citation quality.

**LongBench-E** [2502.00958] extends long-context evaluation to survey-length text generation. It includes tasks that require synthesizing information across multiple documents, making it relevant for evaluating the synthesis capabilities of survey generation systems.

### 4.4 Human Evaluation Protocols

Human evaluation remains the gold standard for survey quality assessment, particularly for dimensions like coherence and organization that automated metrics capture poorly.

**Rubric design** is critical for reliable human evaluation. Effective rubrics decompose survey quality into 4-6 well-defined dimensions, each with a 5-point Likert scale and anchored examples at each level. The Evaluating LLM-Generated Surveys framework [2402.05680] provides sample rubrics for coverage, organization, and writing quality.

**Inter-annotator agreement** must be established before evaluation. Cohen's κ or Krippendorff's α should be reported, with κ ≥ 0.6 considered acceptable for survey quality dimensions. Studies using these rubrics typically achieve moderate agreement (κ = 0.4-0.7), with higher agreement on citation quality than on coverage or organization.

**Expert review** is the most rigorous but most expensive approach. Domain experts with active publication records in the survey's topic area evaluate the survey for accuracy, completeness, and appropriateness of the taxonomy. This approach is used by AutoSurvey [2502.13965] in its evaluation, where domain experts rated automated surveys against human-written surveys on a five-point scale.

### 4.5 The Relationship Between Retrieval and Survey Quality

A growing body of work investigates the relationship between retrieval quality and downstream survey quality. RAG-Survey [2503.04626] provides quantitative evidence that improvements in retrieval recall directly correlate with improvements in survey coverage, with a Pearson correlation of r = 0.72 between the two. However, retrieval precision does not show a similarly strong correlation with citation faithfulness, suggesting that the augmentation stage—not just retrieval—plays a critical role in citation quality.

### 4.6 Open Challenges in Evaluation

Several fundamental challenges remain unsolved. **No standard benchmark exists** that covers all quality dimensions across multiple domains. Existing benchmarks focus on computer science and biomedical literature, leaving other scientific fields unaddressed. **Task-specific evaluation** is difficult because survey quality criteria depend on the survey's purpose—a comprehensive survey for experts has different requirements than a tutorial survey for newcomers. **Longitudinal quality**—how well does a survey age as the field evolves?—is almost entirely unaddressed, as evaluations are conducted at a single point in time.

## 5 Comparative Analysis and System Trade-offs
### Head-to-head comparison of existing automated survey systems

Having examined agent architectures, retrieval pipelines, and evaluation methodologies, we now provide a structured comparison of existing end-to-end systems. Table 1 summarizes the key characteristics of each system.

**Table 1: Comparison of Automated Survey Generation Systems**

| System | Architecture | Planning | Retrieval | Citation | Human-in-Loop | Output Format |
|--------|-------------|----------|-----------|----------|---------------|---------------|
| STORM [2308.08155] | Single-agent (simulated conversation) | Hierarchical outline | Iterative outline refinement | Paragraph-level | No | Wikipedia-style article |
| PaperQA [2402.14207] | Single-agent | Iterative Q&A refinement | Iterative retrieval | Sentence-level | Yes (paper input) | Question-answer response |
| AutoSurvey [2502.13965] | Single-agent | Hierarchical outline + refinement | Multi-source (arXiv, SS) | Paragraph-level | Partial (topic only) | Full survey |
| AutoSci [2406.03666] | Multi-agent | Citation-graph scope definition | Multi-agent parallel retrieval | Section-level | Partial | Research paper |
| SurveyAgent [2412.13129] | Multi-agent (3 agents) | Planner-driven outline | Researcher-per-section | Paragraph-level | Yes (review step) | Full survey |
| AutoGen-based [2402.14829] | Multi-agent (configurable) | Configurable | Configurable | Configurable | Yes | Configurable |
| RAG-Survey [2503.04626] | Single-agent | Section-based outline | Multi-query, paragraph chunking | Paragraph-level | No | Full survey |
| MAMBA [2410.06462] | Multi-agent | Belief-driven | Belief-gap retrieval | Section-level | Partial | Full survey |

### 5.1 Architectural Comparison

**Single-agent vs. multi-agent trade-offs.** Single-agent systems (STORM, PaperQA, AutoSurvey) are simpler to deploy and more consistent in writing style, as a single model controls all stages. Multi-agent systems (SurveyAgent, AutoSci, AutoGen-based) provide greater specialization but introduce coordination overhead and potential inconsistencies between agent outputs. The optimal choice depends on survey scope: broad surveys benefit from single-agent consistency, while deep surveys benefit from multi-agent specialization.

**Planning strategies.** STORM [2308.08155] and AutoSurvey [2502.13965] use hierarchical outline generation, which provides strong global structure but may miss emergent cross-cutting themes. PaperQA [2402.14207] uses iterative refinement, which captures emergent connections but may produce uneven coverage. SurveyAgent [2412.13129] uses a dedicated planner agent, combining the benefits of hierarchical planning with role specialization.

### 5.2 Retrieval Pipeline Comparison

**Source coverage.** STORM [2308.08155] uses internet search (Bing API) as its primary source, which provides broad coverage but limited scientific depth. PaperQA [2402.14207] ingests a user-defined paper corpus, providing depth at the cost of requiring manual paper collection. AutoSurvey [2502.13965] combines arXiv and Semantic Scholar, offering a balance of coverage and scientific relevance. RAG-Survey [2503.04626] uses Semantic Scholar with optimized chunking, demonstrating that retrieval quality matters as much as source breadth.

**Citation quality.** Systems that place citations at the sentence level (PaperQA [2402.14207]) provide tighter attribution at the cost of increased overhead. Systems with paragraph-level citations (STORM [2308.08155], AutoSurvey [2502.13965], RAG-Survey [2503.04626]) produce cleaner text but may have ambiguous attribution. Citation faithfulness evaluation [2408.16743] suggests that sentence-level citation achieves higher attribution precision (p < 0.05) in controlled comparisons.

### 5.3 Output Quality

Direct quality comparisons between systems are complicated by the lack of standardized evaluation. However, available results suggest:

- **Factual accuracy**: PaperQA [2402.14207] reports the highest factual accuracy on scientific question answering, likely due to its iterative retrieval and sentence-level citation. AutoSurvey [2502.13965] reports strong accuracy on broad surveys, with domain expert ratings averaging 4.1/5 for factual correctness.
- **Coverage**: STORM [2308.08155] and AutoSurvey [2502.13965] achieve broad coverage due to their hierarchical outline approach. AutoSci [2406.03666] achieves targeted coverage through citation graph analysis.
- **Coherence**: Single-agent systems (STORM, AutoSurvey) score higher on coherence in human evaluations, likely due to consistent model-level stylistic control.

### 5.4 Scalability and Practical Considerations

**Context limits** remain a practical bottleneck. Current LLMs support context windows of 128K–1M tokens, but most systems operate well below this limit. RAG-Survey [2503.04626] reports that surveys exceeding 10 sections begin to show quality degradation due to context fragmentation.

**Generation time** varies widely. Single-agent systems generate full surveys in 10-30 minutes, while multi-agent systems require 30-60 minutes due to coordination overhead and multiple LLM calls.

**Compute requirements** are modest for single-agent systems (single GPU sufficient) but grow with agent count. Multi-agent systems require multiple API calls per section, increasing cost proportionally.

### 5.5 Application Fit

Each system excels in different scenarios. **STORM** [2308.08155] is best suited for producing Wikipedia-style overview articles on broad topics. **AutoSurvey** [2502.13965] is the most mature system for generating comprehensive academic surveys. **PaperQA** [2402.14207] is ideal for deep dives into specific research questions with strong citation grounding. **SurveyAgent** [2412.13129] offers the most flexible architecture with human-in-the-loop support, suitable for collaborative survey authoring. **RAG-Survey** [2503.04626] provides optimized retrieval that is particularly effective for evidence-dense surveys.

### 5.6 Identified Gaps

No existing system supports all phases of survey generation end-to-end with professional quality. Key gaps include: (1) no system adequately handles multi-modal survey content (figures, tables, equations); (2) evaluation is inconsistent across systems, preventing apples-to-apples comparisons; (3) human-in-the-loop support remains rudimentary, limited to topic specification and optional review steps; (4) longitudinal survey updates are unaddressed by all existing systems.

## 6 Open Challenges and Future Directions
### Remaining barriers to fully automated, trustworthy survey generation

Despite significant progress, automated survey generation faces several fundamental challenges that must be addressed before these systems can be reliably deployed in scientific practice.

### 6.1 Hallucination and Citation Fabrication

The most critical barrier to adoption is the generation of unsupported claims and fabricated citations. Hallucination in survey generation takes two forms: **intrinsic hallucination**, where the generated claim has no basis in any retrieved source, and **extrinsic hallucination**, where the claim extends beyond but does not directly contradict the source.

The causes are multifaceted. Retrieval may miss the relevant source, leading the model to fill gaps with plausible-sounding but unverified information. The augmentation stage may fail to properly condition generation on retrieved evidence, particularly when the generation length exceeds the model's effective context window. And citation generation—where the model must decide which source to cite for each claim—introduces additional failure modes, as the model may cite a paper that discusses a related topic but does not support the specific claim.

Mitigation strategies are emerging but incomplete. **Self-RAG** [2404.16130] learns to decide when retrieval is needed and to critique its own outputs, reducing but not eliminating hallucination. **Citation faithfulness evaluation** [2408.16743] provides detection tools but not prevention. **Iterative verification**, where a reviewer agent fact-checks the draft against retrieved sources, reduces hallucination rates by 30-50% in multi-agent systems but adds significant overhead.

**Detection methods** leverage the fact that hallucinated citations often have distinctive patterns: they may cite non-existent papers, misattribute findings to the wrong paper, or combine elements from multiple sources in invalid ways. The HALO framework [2411.18117] provides structured detection of these patterns, achieving detection rates above 80% on curated test sets.

### 6.2 Evaluation Standardization

The absence of a standardized evaluation benchmark is perhaps the most significant bottleneck to progress in the field. Without a common benchmark, claims of improvement across systems are difficult to verify, and researchers cannot reliably compare architectural choices. (See Section 4.6 for a detailed discussion of the underlying challenges in evaluation methodology, including the lack of cross-domain coverage, task-specific criteria, and longitudinal quality assessment.)

A community benchmark for survey generation should include: (1) a diverse corpus of topics spanning multiple scientific domains; (2) human-written reference surveys with expert-validated quality scores; (3) standardized evaluation rubrics covering coverage, citation quality, factual consistency, coherence, and organization; (4) reference implementations of existing systems for baseline comparison; (5) infrastructure for submitting and comparing system outputs.

The **reproducibility crisis** is a related concern. Many systems are evaluated on private datasets with non-public evaluation procedures, making it impossible to verify reported results. Community norms requiring open-source evaluation code and public system outputs would accelerate progress.

### 6.3 Multi-Modal Survey Content

Scientific surveys are not text-only artifacts. They include figures that illustrate key results, tables that compare methods, equations that formalize relationships, and code snippets that demonstrate implementation. Current systems are almost exclusively text-based.

The challenges of multi-modal survey generation are substantial. Vision-language models can caption figures but cannot yet produce publication-quality figures from scratch. Structured data (tables, equations) requires different generation mechanisms than free text. Multimodal RAG [2504.09867] takes an initial step by encoding figures and tables as structured representations, but the quality of generated multi-modal content remains far below human-authored surveys.

### 6.4 Longitudinal Survey Maintenance

Scientific fields evolve. New findings emerge, established results are refuted, and the relative importance of different sub-topics shifts over time. A survey that was accurate at publication may become misleading within months.

**Stale citations** are a concrete manifestation of this problem: a survey may cite a paper that has been retracted, superseded, or significantly revised. **Update mechanisms** that automatically refresh a survey's content are an open challenge. Ideal systems would monitor the literature for new publications relevant to each survey section, assess whether the new work changes the section's conclusions, and propose updates. No existing system supports this workflow.

### 6.5 Domain Adaptation

Different scientific domains have different conventions for survey writing. Biomedical surveys emphasize systematic methodology and evidence hierarchies. Computer science surveys focus on taxonomies and comparative analysis. Humanities surveys require narrative synthesis and critical interpretation.

Current systems are trained on general-domain LLMs and have only surface-level domain adaptation. Domain-specific retrieval models (SciBERT [2403.07199]) improve retrieval quality but the generation stage remains domain-agnostic. Domain-specific evaluation criteria would also differ: biomedical surveys require structured evidence quality assessment, while computer science surveys prioritize taxonomy clarity.

### 6.6 User Steering and Interactive Control

Current systems offer limited user control. The user specifies a topic (and optionally a list of papers), but has little control over the survey's depth, breadth, or organizational structure. **Interactive survey generation**—where the user can guide the outline, request deeper coverage of specific sub-topics, or redirect the focus—would significantly increase practical utility.

**Human-in-the-loop refinement** is supported by some systems (SurveyAgent [2412.13129], AutoGen-based systems [2402.14829]) but in a limited form: the user reviews sections after generation and requests revisions. More fluid interaction—where the user can interject during the writing process, adjust priorities mid-stream, or provide real-time feedback on generated content—remains an open challenge.

**Controllable depth and breadth** would allow users to specify whether they want a comprehensive 50-page survey or a focused 5-page summary, whether they want in-depth technical detail or accessible high-level overview, and whether they want a neutral presentation or one that advocates for a particular approach.

### 6.7 Future Directions

We identify several promising directions for future research:

**Self-improving survey agents** that learn from feedback—both explicit (human corrections) and implicit (citation patterns, download statistics)—to improve future survey generations. This would require mechanisms for storing and applying lessons across survey generation episodes.

**Citation-aware generation** that models the citation graph as a first-class input rather than a by-product of retrieval. By understanding which papers cite each other, how citations relate to claims, and which papers are central vs. peripheral, systems could produce more structurally coherent surveys.

**Inter-survey synthesis** where multiple surveys on related topics are automatically merged, contrasted, and reconciled. This would enable meta-surveys that span sub-disciplines, identifying cross-cutting themes and unresolved tensions.

**Cross-modal generation** that extends beyond text to include automatically generated figures, tables, and visualizations that summarize key results across the cited literature.

**Verification as a service** where a dedicated verification layer sits between generation and publication, checking every claim against its cited source and flagging unsupported statements for human review.

## 7 Conclusion
### Synthesis and outlook

Automated literature survey generation using large language models has emerged as a rapidly maturing research area with the potential to transform how scientists synthesize and consume research. This survey has provided a structured analysis of the field along three pillars: agent architectures, retrieval-augmented pipelines, and evaluation methodologies.

**Agent architectures** span a spectrum from single-agent systems (STORM [2308.08155], PaperQA [2402.14207], AutoSurvey [2502.13965]) to multi-agent frameworks (SurveyAgent [2412.13129], AutoSci [2406.03666], AutoGen-based systems [2402.14829]) and belief-driven approaches (MAMBA [2410.06462]). Single-agent designs offer simplicity, stylistic consistency, and ease of deployment, making them well-suited for broad surveys on well-defined topics. Multi-agent designs provide role specialization—planner, researcher, writer, reviewer—that mirrors human collaborative writing, at the cost of coordination overhead and potential inter-agent inconsistency. The architectural trade-off space is governed by agent count vs. coherence, specialization vs. generality, and autonomy vs. controllability; no single architecture dominates, and the optimal choice depends on survey scope, depth requirements, and available computational resources.

**Retrieval-augmented pipelines** have evolved from simple keyword search to sophisticated multi-stage systems incorporating adaptive retrieval decisions (Self-RAG [2404.16130]), unified ranking and generation (RankRAG [2407.16833]), knowledge graph integration [2407.19687, 2409.08116], and domain-specific dense retrieval [2403.07199]. Evidence extraction at increasingly fine granularities—from abstract-level to paragraph-level to claim-level—has improved citation precision, while multi-source synthesis strategies now explicitly surface contradictions and incorporate temporal weighting. Dedicated survey-RAG systems such as RAG-Survey [2503.04626] and multimodal RAG [2504.09867] demonstrate that further gains are achievable through domain-specific retrieval optimization and non-text content integration. However, significant gaps remain: citation fabrication persists as a major failure mode, evidence synthesis across conflicting sources is handled superficially, and temporal awareness is limited to recency weighting rather than substantive update mechanisms.

**The evaluation deficit** represents the most critical bottleneck to field progress. While several dedicated benchmarks have been proposed [2402.05680, 2403.07929, 2502.00958], no standard evaluation framework covers all quality dimensions—coverage, citation quality, factual consistency, coherence, and organization—across multiple scientific domains. Automated metrics for factuality [2406.12178, 2411.18117] and citation faithfulness [2408.16743] are improving but remain imperfect proxies for human judgment. Human evaluation protocols exist but are expensive, difficult to reproduce, and limited in scale. Until the community converges on a shared evaluation infrastructure, claims of improvement across systems will remain difficult to verify, and architectural innovations will be assessed in isolation rather than through competitive benchmarking.

Looking forward, the trajectory of automated survey generation will be shaped by progress on several fronts. **Self-improving systems** that learn from human corrections and usage patterns could bootstrap themselves toward higher quality over successive generation episodes. **Citation-aware generation** that models the citation graph as a first-class input—rather than a by-product of retrieval—could produce surveys that reflect the intellectual structure of a field. **Inter-survey synthesis** could enable meta-analyses spanning sub-disciplines, identifying cross-cutting themes and unresolved tensions that individual surveys miss. And **verification infrastructure**—a dedicated layer that checks every claim against its cited source—could provide the trustworthiness guarantees needed for automated surveys to be adopted in scientific practice.

The promise of automated survey generation is not to replace human survey writers but to augment them—to handle the mechanical aspects of literature retrieval, evidence aggregation, and initial drafting, freeing researchers to focus on critical interpretation, synthesis, and insight. Realizing this vision requires continued progress on agent architectures that balance autonomy with controllability, retrieval pipelines that prioritize citation faithfulness over coverage breadth, and evaluation frameworks that provide trustworthy quality signals. The field has made remarkable progress in a short time; the foundations are in place for automated surveys to become a standard tool in the scientific workflow.
