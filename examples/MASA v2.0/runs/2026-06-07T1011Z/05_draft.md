# Automated Survey Generation: From RAG Pipelines to Multi-Agent Deliberation

## A Systematic Survey and Critical Analysis

---

# 1 Introduction

The accelerating growth of scientific publishing — millions of papers indexed annually — has created an acute need for automated tools that can synthesize research literature into coherent, structured surveys. Automated survey generation, defined here as end-to-end systems that produce structured multi-section literature surveys from a topic query, has emerged as a rapidly evolving field spanning retrieval-augmented generation, multi-agent coordination, and citation graph analysis [arXiv:2406.10252, arXiv:2402.08565]. From foundational RAG paradigms in 2020 to deliberation-first multi-agent architectures in 2026, the field has seen remarkable architectural innovation in just six years.

This paper presents a systematic survey of automated survey generation systems, tracing their evolution through four phases: (1) foundational RAG and dataset infrastructure (2020–2023), (2) single-agent pipeline emergence (2024), (3) architectural proliferation with multi-agent, graph-enhanced, and human-in-the-loop systems (2025), and (4) the current frontier of deliberation-first and domain-expert architectures (2026). Across these phases, we identify a central tension: **architectural innovation has outpaced evaluation infrastructure**. The field has produced more evaluation benchmarks (8+) than architectural approaches, yet no two systems can be directly compared [arXiv:2510.03120, arXiv:2501.04306].

Four narrative threads run through this survey. **Thread 1 — The Evaluation Comparability Crisis**: claims of "SOTA," "human-competitive quality," and "superhuman performance" are unfalsifiable without a shared yardstick. **Thread 2 — The Automation–Control Tension**: as systems become more autonomous, they become less transparent and less controllable. **Thread 3 — Citation Graph Shallowness**: despite every system claiming to address citation quality, engagement with citation structure remains at single-hop BFS traversal. **Thread 4 — The Depth–Breadth Trade-Off**: no system simultaneously achieves broad coverage and deep citation verification.

We scope this survey to end-to-end systems generating multi-section surveys from a topic query. We exclude single-document summarization, non-scientific domains, and pure citation analysis without generation. Our goal is to provide researchers with a structured understanding of the design space, a critical assessment of claims against evidence, and a roadmap for the next generation of systems.

**Section 2** covers the foundational RAG paradigm and dataset infrastructure. **Section 3** examines the single-agent pipeline template and its training paradigms. **Section 4** surveys the architectural proliferation of 2025 across multi-agent, graph-enhanced, hybrid interactive, and iterative refinement systems. **Section 5** analyzes the 2026 frontier. **Section 6** provides a critical assessment of claims, gaps, and blind spots. **Section 7** outlines future directions.

---

# 2 The Foundations — RAG Paradigm and Dataset Infrastructure (2020–2023)

## 2.1 Core Enabling Methods — RAG and Retrieval Paradigm

The foundational technology underlying virtually every automated survey generation system is Retrieval-Augmented Generation (RAG), introduced by Lewis et al. [arXiv:2005.11401]. RAG combines parametric memory (a pre-trained language model) with non-parametric memory (a dense vector index of document passages) in a retrieve-then-generate architecture. Given a query, a retriever (typically Dense Passage Retrieval) finds the top-\(k\) relevant passages from a large corpus, and a generator conditions on both the query and the retrieved passages to produce text. This design was a breakthrough: it grounded LLM outputs in external knowledge without requiring fine-tuning, reducing hallucination on knowledge-intensive tasks.

The mechanism operates as follows. The input query \(x\) is encoded by the retriever \(E_Q\) into a query embedding, which is compared against a pre-computed index of passage embeddings \(E_P(d_i)\) via maximum inner product search. The top-\(k\) passages \([d_1, \ldots, d_k]\) are concatenated with the query to form the generator's input. The generator \(G\) produces output \(y\) by marginalizing over retrieved passages:

\[
P(y|x) \approx \sum_{i=1}^k P(y|x, d_i) \cdot P(d_i|x)
\]

where \(P(d_i|x)\) is approximated by the retriever's score. This formulation made RAG highly effective for short-form QA — achieving state-of-the-art performance on multiple QA benchmarks at the time of publication [arXiv:2005.11401].

However, RAG was designed for tasks where a single retrieval pass provides sufficient context: a single question maps to a small set of relevant passages. Survey generation fundamentally violates this assumption. A multi-topic survey requires synthesizing across dozens of papers spanning distinct sub-topics, each contributing different claims, methods, and findings. Single-pass retrieval cannot simultaneously cover breadth (all relevant sub-topics) and depth (sufficient detail per sub-topic). Moreover, RAG retrieves at the **passage level**, not the **citation graph level** — meaning it has no awareness of how papers relate to each other, which papers build on which, or whether a cited claim is actually supported by the referenced work. This passage-level retrieval paradigm, inherited by virtually all survey generation systems, is the root cause of what we later identify as the field's citation shallowness problem (Thread 3).

In summary, RAG provided the essential building block — grounded generation with external knowledge — but its fundamental assumptions (single-pass, passage-level retrieval for short-form tasks) made it an incomplete foundation for survey generation. The field's transition to multi-stage pipelines and citation-aware architectures was a direct response to these limitations.

## 2.2 Datasets and Evaluation Infrastructure

The foundation phase also produced the datasets that established evaluation infrastructure for survey generation. Table 1 summarizes the key resources.

**Table 1: Phase 1 Datasets and Their Role in Survey Generation**

| Dataset | Year | Task | Scale | Limitation for Full Surveys |
|---------|------|------|-------|-----------------------------|
| SciFact [arXiv:2004.14974] | 2020 | Claim verification (SUPPORT/REFUTE/NEI) | 1,409 claim-paper pairs | Biomedical only; binary classification, not generation |
| Multi-XScience [arXiv:2010.14235] | 2020 | Multi-document related-work generation | 30K+ examples | Produces one paragraph, not a full survey section |
| SciReviewGen [arXiv:2305.15186] | 2023 | Full review generation | 10K reviews, 690K cited papers | Largest dataset, but reviews are short and non-hierarchical |
| FActScore [arXiv:2305.14251] | 2023 | Atomic factuality evaluation | Wikipedia-based | Validated on biographies, not scientific surveys |
| SciTLDR [arXiv:2004.15011] | 2020 | Extreme summarization | 5.4K papers | Single-sentence output — incompatible with survey structure |
| MS² [arXiv:2104.06486] | 2021 | Contradiction-aware medical summarization | Medical domain | Domain-specific; contradiction handling not adopted by survey systems |

SciFact [arXiv:2004.14974] introduced the SUPPORT/REFUTE/NEI paradigm for claim-level verification against scientific abstracts. This dataset established the standard evaluation protocol for citation-claim alignment and is still used by survey systems for post-hoc citation verification. Multi-XScience [arXiv:2010.14235] provided the first large-scale dataset for multi-document related-work generation, pairing a target paper with its references and requiring the system to synthesize a related-work section. SciReviewGen [arXiv:2305.15186] expanded this to 10K full review-generation instances paired with 690K cited papers, becoming the standard training resource for supervised review generation.

FActScore [arXiv:2305.14251] introduced atomic fact decomposition — breaking a generated text into individual factual claims and verifying each against a knowledge source. While not designed for surveys, FActScore-style evaluation has been adopted by several survey systems for citation verification. The approach has a critical limitation: atomic decomposition quality depends on the LLM performing the decomposition, and the framework was never validated on survey-length text with hundreds of citations.

The common limitation across all these datasets is revealing: **none provides evaluation infrastructure for full multi-section surveys**. SciFact tests claim verification, not generation. Multi-XScience and SciReviewGen evaluate short related-work or review paragraphs, not structured multi-section surveys with integrated narrative. FActScore measures factuality of individual claims, not the coherence, coverage, or analytical depth of a full survey. The field entered its architectural proliferation phase (Phase 3) without any dataset designed to evaluate the task these systems were actually performing. This gap, established in Phase 1, is the root of the evaluation comparability crisis that the survey demonstrates across all subsequent phases.

## 2.3 Limitations That Drove the Transition

Phase 1 established the building blocks — RAG for grounded generation, datasets for evaluation, factuality metrics for quality assessment — but addressed only short-form tasks. No Phase 1 system could produce a coherent multi-section survey exceeding 1,000 words. The transition to Phase 2 was not a failure of Phase 1 methods but a recognition of a **phase boundary**: short-form methods do not trivially scale to long-form surveys.

Three specific gaps drove the transition. First, **single-pass retrieval** — adequate for QA — cannot provide the breadth of coverage a survey requires. A survey on "reinforcement learning from human feedback" needs papers on reward modeling, optimization, debiasing, and evaluation; a single retrieval pass cannot anticipate all sub-topics. Second, **flat document summarization** does not produce structured narrative: surveys require hierarchical organization with sections, subsections, and coherent transitions. Third, **no quality control** mechanism exists in single-pass generation: if the retrieval set has gaps or the generated text contains errors, there is no way to detect or correct them.

The enabling condition for Phase 2 was the emergence of sufficiently capable LLMs (GPT-4 and its contemporaries) with the reasoning capacity to decompose survey writing into sub-tasks and execute them sequentially. The key insight was that survey generation is not a summarization task — it is a **program synthesis task** where the program is a sequence of retrieval, outline, drafting, and evaluation operations [arXiv:2406.10252]. This insight defined the stage-decomposition template that became the field's dominant architecture from 2024 onward.

---

# 3 The Single-Agent Pipeline Emerges — Task Decomposition (2024)

## 3.1 The Stage-Decomposition Template — AutoSurvey and Its Contemporaries

### 3.1.1 The Archetype: AutoSurvey

AutoSurvey [arXiv:2406.10252] established the stage-decomposition template that became the dominant architecture for survey generation. The system decomposes survey writing into five sequential stages, each executed by a single LLM (GPT-4) via carefully engineered prompts:

1. **Retrieval**: Keyword + embedding search on arXiv yields an initial paper pool. Notably, this retrieval is **offline and one-shot** — performed before any generation begins, with no revisiting.
2. **Outline Generation**: The LLM analyzes the retrieved papers to produce a hierarchical outline (sections and subsections) reflecting the topic's topical structure.
3. **Subsection Drafting**: Each subsection is drafted independently, conditioned on the outline, the section heading, and the relevant subset of retrieved papers.
4. **Integration**: Subsection drafts are concatenated, cross-references are added, and narrative flow is checked.
5. **Evaluation**: Automatic metrics (ROUGE, BLEU) and LLM-based quality scoring assess coverage, coherence, and citation quality.

The mechanism's key insight is that **decomposition reduces task complexity**: each sub-task (outline a single section, draft one subsection) is easier than monolithic generation. However, the single-pass design means errors cascade — a poor outline degrades all subsequent drafts, and retrieval gaps cannot be fixed mid-generation.

### 3.1.2 Contemporaneous Systems: Architectural Variations

Table 2 compares AutoSurvey against five contemporaneous systems.

**Table 2: Single-Agent Pipeline Comparison**

| System | Preprocessing Strategy | Retrieval Method | Generation Scope | Iteration Strategy | Evaluation Approach |
|--------|----------------------|------------------|-----------------|-------------------|-------------------|
| AutoSurvey [2406.10252] | None (flat query → outline) | Keyword + embedding (one-shot) | Full multi-section survey | Single-pass (no iteration) | ROUGE, BLEU, LLM scoring |
| SurveyX [2502.14776] | AttributeTree (multi-level decomposition) | Per-attribute retrieval from paper DB | Full survey with structured sections | Re-polishing pass after draft | Human evaluation on structure & content |
| LitLLM [2402.01788] | Modular RAG toolkit | Pluggable retrievers (BM25, dense, hybrid) | Related-work section only | None (toolkit for human use) | Not evaluated (toolkit) |
| Instruct LLMs Step by Step [2408.07884] | Prompt decomposition into sub-questions | None (zero-shot LM generation) | Survey from model knowledge | None (single prompt chain) | Human evaluation of informativeness |
| DimInd [2504.18496] | Multi-level compression (dimension → indicator → facet) | Embedding + indicator-based filtering | Hierarchical survey with facets | Sequential (dimension → facet → synthesis) | Human evaluation on structure |

**SurveyX**[arXiv:2502.14776] introduces AttributeTree preprocessing: the topic is decomposed into attributes (e.g., for "attention mechanisms": types, applications, theoretical analysis), and retrieval is performed independently per attribute. This addresses AutoSurvey's flat-retrieval limitation — per-attribute retrieval ensures broader coverage — but the preprocessing requires human-defined attribute taxonomies, introducing a bottleneck.

**LitLLM**[arXiv:2402.01788] takes a modular RAG approach for related-work generation, offering pluggable retrievers (BM25, dense embeddings, hybrid). Its mechanism is a retrieval-generation pipeline where the user selects the retriever configuration. While flexible, LitLLM produces at most one related-work paragraph — not a full survey.

**Instruct LLMs Step by Step**[arXiv:2408.07884] uses prompt decomposition: the survey topic is broken into sub-questions (e.g., "What are the main methods in this area?"), each answered sequentially, with answers concatenated into a survey. The mechanism requires no external retrieval — the LLM relies entirely on its parametric knowledge. This eliminates retrieval quality as a variable but makes factual accuracy entirely dependent on model memorization.

**DimInd**[arXiv:2504.18496] proposes multi-level compression: from broad dimensions to specific indicators to granular facets, each level informing retrieval and generation at the next. The mechanism is a coarse-to-fine synthesis pipeline where higher levels constrain lower levels, producing hierarchically organized surveys with facet-level detail.

### 3.1.3 Assessment: Genuine Advances and Unfulfilled Claims

**Genuine advances**: Stage decomposition was a genuine breakthrough — it made multi-section survey generation tractable. AutoSurvey demonstrated that single-LLM pipelines could produce coherent, structured surveys exceeding 5,000 words, establishing the baseline architecture. AttributeTree (SurveyX) and multi-level compression (DimInd) showed that structured preprocessing improves coverage over flat retrieval.

**Unfulfilled claims**: No Phase 2 system achieved iterative refinement — once a stage was complete, errors could not be corrected in later stages. Citation graph awareness was entirely absent: paper discovery relied on keyword/embedding search, missing the citation chaining that human reviewers use. Critically, the **Automation–Control Tension** (Thread 2) emerges here: the single-agent pipeline maximizes automation (a single query produces a full survey) but minimizes control — the user cannot intervene in retrieval, organization, or drafting. This tension becomes sharper in the multi-agent systems of Phase 3, where autonomy increases without corresponding transparency.

## 3.2 Training Paradigms — Fine-Tuned vs. Zero-Shot

A fundamental divide separates Phase 2 systems into two camps: those that fine-tune models on domain-specific data and those that rely entirely on zero-shot prompting of general-purpose LLMs. This distinction has deep implications for citation accuracy, domain portability, and deployment cost.

### 3.2.1 Fine-Tuned Systems

**OpenScholar**[arXiv:2411.14199] trains a specialized retriever on 45M scientific papers using citation-supervised fine-tuning. The mechanism uses citation relationships as training signals: if Paper A cites Paper B, the retriever is trained to rank B higher for queries related to A. This citation-supervised signal is more informative than simple keyword co-occurrence because it encodes actual paper relevance as judged by human authors. OpenScholar reports state-of-the-art citation accuracy on multiple scientific QA datasets — a genuine advance over general-purpose retrieval.

**ScholarCopilot**[arXiv:2504.00824] fine-tunes the generator itself with a retrieval-token-gated architecture: special tokens in the input sequence control whether the model attends to retrieved passages or generates from parametric knowledge. This fine-grained control mechanism allows the model to cite sources faithfully when relevant and rely on internal knowledge when retrieval is noisy.

**AcademicGPT**[arXiv:2311.12315] is a domain-specific fine-tuned LLM for academic writing tasks, trained on a curated corpus of academic papers and reviews. Its mechanism is direct: pre-train on academic text, fine-tune on writing tasks. While it improves fluency and formatting, its citation accuracy depends entirely on what the model memorized — it has no retrieval component.

### 3.2.2 Zero-Shot Systems

In contrast, **AutoSurvey**[arXiv:2406.10252] and **SurveyX**[arXiv:2502.14776] use GPT-4 with zero-shot prompting — no fine-tuning, no domain-specific training. Their retrieval and generation quality depends on prompt engineering and the base model's capabilities.

**PaperQA2**[arXiv:2409.13740] bridges the two paradigms: it achieves superhuman QA performance without fine-tuning, using multi-step verification, contradiction detection, and iterative query formulation. Its mechanism involves generating multiple search queries for each question, retrieving full-text papers, extracting candidate answers, and cross-verifying across papers. The system achieved superhuman performance on multiple scientific QA benchmarks — though this applies to short-form QA, not survey generation.

### 3.2.3 Comparison and Assessment

**Table 3: Training Paradigms Comparison**

| System | Paradigm | Training Data | Retrieval | Citation Accuracy vs. Baselines | Portability |
|--------|----------|--------------|-----------|-------------------------------|-------------|
| OpenScholar [2411.14199] | Fine-tuned | 45M papers + citation graph | Specialized retriever | SOTA at publication | Requires retraining for new domains |
| ScholarCopilot [2504.00824] | Fine-tuned | Academic writing corpus | Token-gated retrieval-gated generator | Improved over GPT-4 | Domain-specific (CS) |
| AcademicGPT [2311.12315] | Fine-tuned | Academic paper corpus | None (parametric only) | Not evaluated on citation accuracy | Low (no retrieval) |
| AutoSurvey [2406.10252] | Zero-shot | None | Keyword + embedding | ~40% human win rate | High (any domain with retrieval) |
| PaperQA2 [2409.13740] | Zero-shot | None | Multi-step iterative retrieval | Superhuman on QA | High |

The key tension is between **performance** and **portability**. Fine-tuned systems achieve better citation accuracy because they have been optimized for the task — but they require significant compute resources and training data, and must be retrained for new domains or languages. Zero-shot systems are immediately deployable on any topic but rely on the base LLM's general capabilities, which may not capture domain-specific citation norms.

Critically, neither paradigm addresses **citation graph reasoning** (Thread 3). OpenScholar uses citation relationships only as training signals for its retriever — it does not trace claims through chains of citations. PaperQA2's multi-step verification checks claims against individual papers but does not model how papers relate to each other. The field would need fundamentally different architectures — graph traversal mechanisms, claim-level provenance tracking — to move beyond shallow citation verification, and these architectures would not emerge until Phase 3.

## 3.3 Phase 2 Assessment — Genuine Advances and the Ceiling

Phase 2 achieved three genuine advances. First, **coherent multi-section survey generation** was demonstrated for the first time: AutoSurvey showed that GPT-4 could produce a structured 5,000+ word survey on a specialized CS topic, organized into sections with citations [arXiv:2406.10252]. Second, **the stage-decomposition template** established a reusable architecture that all subsequent systems would inherit or extend. Third, **PaperQA2's superhuman QA result**[arXiv:2409.13740] proved that LLM-based systems could exceed human expert performance on well-defined literature synthesis tasks — even if those tasks were short-form QA rather than full survey generation.

However, Phase 2 hit a clear ceiling defined by three limitations. The **single-pass design** meant retrieval gaps discovered during writing could not be addressed — if a crucial paper was missed in the initial retrieval, it would not appear anywhere in the survey. The **lack of citation graph awareness** limited discovery to keyword/embedding search, missing the citation chaining that researchers rely on for comprehensive literature review. And the **single-LLM bottleneck** meant one model was responsible for all stages — retrieval analysis, outline planning, drafting, quality assessment — creating a multi-task conflict where optimizing for one stage (e.g., drafting fluency) could degrade another (e.g., citation accuracy).

PaperQA2's "superhuman" claim [arXiv:2409.13740] also requires precise contextualization. The system outperformed PhD scientists on scientific QA — answering factual questions about papers — but this is a fundamentally different task from writing a structured analytical survey. No Phase 2 system provided evidence that its outputs were competitive with human surveys on the criteria that actually matter: analytical depth, conflicting-finding synthesis, and methodological assessment.

These limitations drove two distinct responses in Phase 3. **Multi-agent architectures** would specialize different LLMs for different sub-tasks (retrieval analysis, drafting, evaluation), resolving the single-LLM bottleneck. **Graph-enhanced retrieval** would make citation graph traversal a core architectural component, addressing the discovery limitation. Both responses would introduce their own trade-offs and unfulfilled claims, which we examine in the next section.

---

# 4 Architectural Proliferation — Multi-Agent, Graph, and Human-in-the-Loop (2025)

## 4.1 Multi-Agent Architectures — Dividing the Labor

Phase 3's defining innovation is role specialization: instead of a single LLM driving all stages, multi-agent systems deploy specialized agents (Planner, Retriever, Writer, Evaluator, Reviewer) with distinct prompts, access patterns, and coordination mechanisms. This section compares eight representative multi-agent systems and critically assesses their genuine advances and unfulfilled claims.

### 4.1.1 Architectural Patterns and Mechanisms

**Table 4: Multi-Agent Architecture Comparison**

| System | Agent Count | Coordination Pattern | Iteration Strategy | Citation Graph Awareness | Key Innovation |
|--------|------------|---------------------|-------------------|------------------------|----------------|
| ARISE [2511.17689] | 5 (Plan, Retrieve, Draft, Evaluate, Revise) | Rubric-guided loop — Evaluation scores draft; Revision revises to improve rubric scores until threshold (92.48) met | Multi-round (eval → revise → eval) | BFS forward/backward chaining | Formal quality rubric as both evaluation tool and generation guide |
| SciSage [2506.12689] | 4 (Plan, Retrieve, Write, hierarchical Reflect) | Reflect-when-you-write — Reflector provides real-time feedback during drafting (local paragraph + global survey levels) | Multi-round (real-time reflection loop) | BFS forward/backward chaining | Real-time reflection during drafting, not after; Citation F1 metric |
| Agentic AutoSurvey [2509.18661] | 4 (Orchestrator, Search, Write, Review) | Orchestrator-driven — Orchestrator plans, Search retrieves, Write drafts, Review evaluates across 12 dimensions | Multi-round (review → refine loop) | None | 12-dimensional quality rubric; demonstrated on 75–443 papers/topic |
| AutoSurvey2 [2510.26012] | Multiple parallel LLM instances | Parallel section drafting — each section drafted by an independent LLM call, then centrally integrated; real-time re-retrieval during refinement | Multi-round (retrieve → draft → refine → re-retrieve → re-draft) | None | Parallel section generation with real-time re-retrieval solves the single-pass retrieval gap of its predecessor |
| MARCO [2410.21784] | 5+ (Searcher, Analyst, Writer, Critic, Summarizer + Conversation Manager) | Real-time chat-based — agents discuss and debate in structured chat; Conversation Manager tracks state | Multi-round (chat until consensus) | None | Real-time chat paradigm for dynamic information exchange |
| Federation of Agents [2509.20175] | Variable | Federated — semantics-aware agents share knowledge graph-based coordination | Multi-round (federated consensus) | None | Knowledge graph coordination for independent agent collaboration |
| OrchMAS [2603.03005] | Variable (Coordinator + domain experts) | Hierarchical — Coordinator creates work plan, assigns tasks to domain-specialized experts | Multi-round (Coordinator integrates, resolves conflicts) | None | Domain-specialized expert agents with pre-configured knowledge profiles |
| DOVA [2603.13327] | configurable (≥4) | Deliberation-first — agents discuss topic before retrieval, then targeted synthesis | Single-pass (deliberation → retrieve → synthesize) | None | Deliberation before retrieval inverts retrieve-then-generate paradigm |

**How the rubric-guided loop works in ARISE**: The formal quality rubric defines dimensions (coverage, organization, citation accuracy, clarity, novelty). The Evaluation Agent scores each section against these dimensions, producing dimension-specific scores with deficiency descriptions. The Revision Agent receives these scores and feedback, then revises the draft to improve each deficient dimension. The loop continues until all dimension scores exceed the quality threshold (92.48). This mechanism transforms evaluation from a post-hoc measurement into a quality driver — arguably the most important architectural innovation of Phase 3.

**SciSage's reflect-when-you-write** is a different iteration mechanism: rather than post-hoc evaluation → revision, the hierarchical Reflector provides feedback during drafting. The local Reflector checks each paragraph for citation correctness, factual accuracy, and relevance; the global Reflector monitors overall coherence and coverage. The Writer Agent incorporates feedback before proceeding to the next paragraph, preventing error accumulation.

**AutoSurvey2**[arXiv:2510.26012] is the direct successor to the foundational AutoSurvey. Its key architectural improvement is parallel section generation: rather than drafting sections sequentially, each section is produced by an independent LLM call with its own retrieved context, enabling parallel throughput. More critically, AutoSurvey2 introduces real-time re-retrieval during the refinement phase — if a coherence check or coverage analysis identifies a gap, the system can re-query paper databases mid-generation. This addresses the original AutoSurvey's fundamental limitation: single-pass retrieval that cannot recover from coverage gaps. However, AutoSurvey2's agents are homogeneous — they use the same base LLM with different context assignments rather than genuine role specialization. The system is conceptually a parallel dispatch architecture, not a true multi-agent system with differentiated expertise.

**DOVA's deliberation-first**[arXiv:2603.13327] inverts the standard pattern: perspective agents (empiricist, theoretician, methodologist) discuss the research topic before any retrieval occurs. This deliberation identifies key questions, information needs, and potential controversies. Only after consensus does retrieval begin — and retrieval is guided by the deliberation output. The mechanism is architecturally novel but its quality depends on the deliberation being productive, which is currently unmeasured.

### 4.1.2 Genuine Advances

Multi-agent systems genuinely advance the field in three ways. First, **role specialization improves output quality** — the evidence, though not from controlled comparisons, is consistent across papers that multi-agent systems produce more coherent and better-cited surveys than single-agent baselines. Second, **rubric-guided evaluation** (ARISE, SciSage, Agentic AutoSurvey) moves quality assessment into the generation loop, enabling targeted improvement. Third, **deliberation-first** (DOVA) represents a genuine architectural insight: understanding intellectual context before gathering papers may produce more focused, analytical surveys. AutoSurvey2's **real-time re-retrieval** solves a concrete problem that had limited its predecessor since 2024.

### 4.1.3 Critical Assessment

Several limitations demand scrutiny. **ARISE's 92.48 quality score** is the field's most salient unvalidated number: it is a system-defined aggregate from a rubric with no established correlation to human-judged survey quality. The same system could define a 100-point rubric and report a higher score. Without human calibration, this number is meaningless for cross-system comparison — a symptom of the **Evaluation Comparability Crisis** (Thread 1).

Second, **agent specialization is prompt-deep**. With the partial exception of OrchMAS (which configures domain knowledge profiles), all agents in all multi-agent systems use the same underlying LLM with different system prompts. The "specialization" is linguistic, not architectural — a prompt difference, not a capability difference. What evidence would confirm or falsify this claim? A controlled experiment could compare retrieval choices made by agents with the same base model but different role prompts (e.g., "You are a retriever" vs. "You are an evaluator"): if the prompts produce measurably different search query formulations or paper selection patterns, then prompt-deep specialization has genuine behavioral consequences. If the outputs are statistically indistinguishable, the specialization is surface-level only. This experiment has not been conducted — a gap that leaves the field's most widespread architectural assumption untested.

Third, **DOVA's deliberation quality is unmeasured** — and the claim that deliberation improves retrieval relevance is circular without a deliberation quality metric. This echoes the rubric-quality problem: both ARISE and DOVA make unvalidated claims about intermediate process quality (rubric scores and deliberation productivity, respectively), highlighting a pattern where systems assert quality improvements without establishing measurement frameworks.

The **Automation–Control Tension** (Thread 2) deepens here: multi-agent systems are more autonomous than single-agent pipelines but also less transparent — a human user cannot inspect or intervene in the deliberation process. AutoSurvey2's parallel architecture, while efficient, introduces a coordination opacity that single-agent pipelines did not have. The frontier systems of 2026 would push this tension further, as we examine in Section 5.

## 4.2 Graph-Enhanced Retrieval — Beyond Keyword Search

While multi-agent systems specialized the *generation* side, graph-enhanced retrieval systems tackled the *discovery* problem. Instead of relying on keyword/embedding search alone, these systems make citation graph traversal a core architectural component — discovering papers through forward/backward chaining from seed papers, and in some cases organizing discovered papers by their role in the research lineage.

### 4.2.1 Architectural Patterns and Mechanisms

**Table 5: Graph-Enhanced Retrieval Systems Comparison**

| System | Graph Traversal Strategy | Graph Role | Agent Count | Iteration Strategy | Evaluation Approach |
|--------|------------------------|------------|-------------|-------------------|-------------------|
| SurveyForge [2503.04629] | Bidirectional BFS chaining from seed papers | Retrieval expansion — discovers papers beyond keyword search | 1 (Single LLM + memory buffer) | Single-pass | SurveyBench win-rate; multi-dimensional evaluation |
| SurveyG [2510.07733] | Bidirectional BFS with three-tier hierarchical organization | Retrieval expansion + outline organization — tiers directly inform survey structure | 4 (Plan, Retrieve, Write, Validate) | Single-pass | Self-evaluation on CS topics; human evaluation of organization |
| SurveyGen [2508.17647] | Citation graph traversal + quality estimation | Retrieval expansion with guided refinement — quality scores filter retrieved papers | 1 | Multi-round (quality-guided refinement) | Self-evaluation on retrieval coverage |
| ProfOlaf [2510.26750] | Explicit snowballing (forward + backward) per SLR protocols | Protocol-driven discovery — follows SLR methodology for systematic paper collection | 1 (Semi-automated) | Single-pass with human validation | SLR compliance assessment |
| GEAR-Up [2312.09948] | Knowledge graph-based query expansion | KG for entity-level query expansion — improves search queries, not paper discovery | 1 | Single-pass | Retrieval quality metrics |

**How SurveyG's three-tier graph works**: Starting from a seed paper set, backward chaining follows references to identify **Foundation** tier papers (highly cited, early works that the seed papers build upon) and **Development** tier papers (intermediate works that extended foundations). Forward chaining finds papers that cite the seed set to identify **Frontier** tier papers (recent work that builds on the topic). The tiers are not just organizational categories — they directly inform the survey outline: Foundation papers are described in a background section, Development papers in a methods section, and Frontier papers in a current-advances section [arXiv:2510.07733]. This is the first system to model citation graph *depth* rather than treating all retrieved papers uniformly.

**SurveyForge's bidirectional citation chaining** extends retrieval from the initial keyword/embedding search: each seed paper's references (backward) and citations (forward) are collected, creating an expanded pool. The cross-section memory buffer stores key claims from each written section; subsequent sections can reference this buffer to maintain coherence [arXiv:2503.04629]. Unlike SurveyG, SurveyForge does not organize retrieved papers hierarchically — all papers, regardless of graph position, are treated equally.

**ProfOlaf**[arXiv:2510.26750] follows explicit snowballing protocols from systematic literature review (SLR) methodology: starting from a known set of relevant papers, forward snowballing (finding papers that cite them) and backward snowballing (finding papers they cite) are performed iteratively with human validation at each step. This is the only system that operationalizes established SLR methodology for paper discovery in survey generation.

### 4.2.2 Genuine Advances

Graph-enhanced retrieval demonstrates that **citation chaining discovers papers that keyword/embedding search misses** — a genuine advance. A paper about "transformer attention mechanisms" may use different terminology than the seed topic "attention is all you need," and only citation chaining can bridge this vocabulary gap. SurveyG's **hierarchical tiering** is the most principled approach to organizing discovered papers: instead of a flat list, papers are categorized by their role in the research narrative (foundational, developmental, frontier), which directly improves survey structure.

### 4.2.3 Critical Assessment

Despite these advances, graph-enhanced retrieval exposes **Thread 3 (Citation Graph Shallowness)** in its starkest form: **all graph traversal is single-hop BFS**. No system performs multi-hop reasoning through citation chains. SurveyG's "hierarchical" tiers are determined by one step of backward/forward chaining. Understanding *why* this limitation persists — why no system has attempted multi-hop traversal — requires decomposing the barriers into four distinct categories: engineering, relevance degradation, infrastructural, and evaluation.

**Engineering barrier — scalable path-finding at citation-graph scale**: Multi-hop traversal requires efficient algorithms for exploring citation paths of depth >1 across large paper corpora. At the scale of academic citation graphs — the Semantic Scholar Academic Graph alone indexes 200M+ papers with 2B+ citation edges [arXiv:2605.22878] — exhaustive multi-hop expansion is computationally prohibitive. Current systems (SurveyForge, SurveyG, SurveyGen) use one-shot BFS chaining from a fixed seed set, limiting graph exploration to direct neighbors. This is a deliberate design choice for latency: even single-hop BFS on a large seed set (e.g., 50 papers × 50 references each = 2,500 candidates) strains API-based retrieval budgets. Multi-hop traversal would compound this exponentially (50 × 50 × 50 = 125,000 candidates for two-hop expansion), requiring priority-based path pruning or learned relevance signals to remain feasible — neither of which any current system implements.

**Relevance degradation — noise accumulation per hop**: Each citation hop introduces a relevance filtering problem. Papers that cite a relevant paper may be addressing entirely different sub-topics, using different methodologies, or citing it for a minor tangential point. Non-topical citations — references cited for related-work positioning rather than substantive contribution — further dilute relevance. At hop 1, a retriever can filter by keyword overlap with the seed topic; at hop 2, the filtering target becomes diffuse — should papers at depth 2 match the original query, the intermediate query (what the hop-1 paper is about), or both? No current system addresses this compounding ambiguity. Even SciAtlas [arXiv:2605.22878], which constructs a 43M-paper knowledge graph with 3B triplets across 26 disciplines to enable topological reasoning, acknowledges that "current academic retrieval tools predominantly rely on superficial keyword matching or vector-space semantic retrieval, which lack the topological reasoning capabilities required to navigate complex logical connections" — and SciAtlas's own neuro-symbolic retrieval (tri-path collaborative recall + graph reranking) is demonstrated on single-paper retrieval, not multi-hop citation chain traversal.

**Infrastructural gap — no claim-level citation graph**: The deepest barrier is that existing citation graphs are paper-to-paper, not claim-to-claim. Multi-hop reasoning about scientific claims requires knowing not just that Paper A cites Paper B, but what specific claim from Paper B is being cited by Paper A, and how that claim relates to claims in Paper C. Current infrastructure — Semantic Scholar, OpenAlex, SciAtlas — models citation relationships at the paper level. No system provides claim-level provenance data at the scale needed for multi-hop survey generation. Building this infrastructure would require: (a) claim extraction from each cited paper (identifying specific findings, methods, or results that later papers build upon); (b) claim-level citation linking (mapping which citations correspond to which claims); (c) cross-paper claim alignment (determining when claims in different papers address the same concept). None of these capabilities exist in any current survey generation system.

**Evaluation gap — no benchmark measures multi-hop citation accuracy**: Even if a system could perform multi-hop citation reasoning, there is no benchmark to evaluate it. Current evaluation metrics (Citation F1, ROUGE, FActScore) all measure surface-level citation attribution — does the system cite the right papers in the right places? None measures whether the system correctly traces claims through citation chains, identifies intellectual lineages, or discovers non-obvious connections across a multi-hop path. Creating such a benchmark would require human annotation of claim-level provenance through citation chains — a labor-intensive process at a scale that existing evaluation efforts (SurveyBench's 4,947 surveys, SurveyScope's 46 papers) have not attempted.

**Secondary critiques**: Beyond the multi-hop barrier, two additional limitations apply. First, **hierarchical tiering (SurveyG) is citation-position-based, not intellectual-lineage-based**. A paper's tier depends on where it sits in the citation graph relative to the seed set, not on the nature of its contribution. A foundational methodological contribution and a minor follow-up can be in the same Foundation tier if they are cited by the same number of seed papers. Second, claims of **"improved coverage"** lack comparison against strong baselines. SurveyG compares against "flat retrieval" (keyword + embedding), but not against the strongest available system — which would be ARISE or SurveyForge, both of which also use BFS chaining. Without controlled comparison, the marginal benefit of hierarchical organization over flat BFS chaining is unknown — a direct symptom of the **Evaluation Comparability Crisis** (Thread 1).

## 4.3 Human-in-the-Loop and Iterative Refinement

Not all systems pursue full automation. Two alternative patterns — hybrid interactive and iterative refinement — address the **Automation–Control Tension** (Thread 2) by preserving human oversight or quality-driven iteration.

### 4.3.1 Hybrid Interactive Systems

Hybrid interactive systems embed human decisions at critical generation stages. **InteractiveSurvey**[arXiv:2504.08762] involves the user in three stages: (1) **reference categorization** — the user assigns semantic categories to seed papers (e.g., "foundational work," "methodological," "related but not central"), directly shaping the survey's emphasis; (2) **outline refinement** — the system generates an outline, the user edits it; (3) **draft review** — the user reviews the draft and requests specific revisions. The mechanism gives the user fine-grained control over the survey's intellectual framing — the user decides which papers are foundational and which are peripheral — while the system handles retrieval, drafting, and formatting.

**PROMPTHEUS**[arXiv:2410.15978] follows PRISMA guidelines for systematic literature reviews, requiring human validation at each stage: search strategy validation, screening decisions, data extraction, and synthesis. **CRUISE-Screening**[arXiv:2309.01684] focuses on living literature review maintenance, with NLP-based screening that flags new papers for human review.

### 4.3.2 Iterative Refinement Systems

Iterative refinement systems replace human intervention with quality-driven loops. **ReClaim**[arXiv:2407.01796] operates at the per-sentence level: for each generated sentence, the system retrieves supporting evidence, generates a claim, then verifies it against the retrieved source. The mechanism is an interleaved retrieve-generate-verify loop that runs for every sentence. This ensures fine-grained citation attribution — every claim is verified before it is written.

**Quantifying ReClaim's computational cost**: The per-sentence retrieval-verify cycle requires approximately 10 API calls per sentence — one for retrieval query formulation, one for passage retrieval, one for evidence extraction, one for claim generation, one for claim verification, plus potential retries on failure. Scaling this to a full survey of 100 sentences would require ~1,000 API calls per survey. At current pricing for frontier LLM APIs (GPT-4-class), this represents significant cost: ~$50–100 per survey in API fees, with latency of several minutes per sentence reducing throughput to hours per survey. This concrete estimate anchors the **Depth–Breadth Trade-Off** (Thread 4): ReClaim achieves the field's deepest citation verification but at a cost that makes full-survey deployment prohibitive at current pricing.

**IterSurvey**[arXiv:2510.21900] operates at the per-draft level: the system generates a survey, evaluates coverage and outline quality, identifies gaps, refines the outline, and re-generates. The mechanism uses **paper cards** — structured records for each citation that store title, key claims, method description, and findings. Cards are reused across iterations, ensuring consistent attribution and reducing hallucinated citations.

### 4.3.3 Comparison and Assessment

**Table 6: Interactive and Iterative Systems Comparison**

| System | Interaction Type | Granularity | Scope | Citation Graph Awareness |
|--------|----------------|-------------|-------|------------------------|
| InteractiveSurvey [2504.08762] | Human guides (categorization, outline, review) | Per-section (outline → draft) | Full survey | BFS (from seed papers) |
| PROMPTHEUS [2410.15978] | Human validates (per PRISMA stage) | Pipeline stage | Full SLR | None |
| CRUISE-Screening [2309.01684] | Human validates (screening decisions) | Per-paper (screening) | Living review | None |
| ReClaim [2407.01796] | Quality-driven (per-sentence verification) | Per-sentence | QA / summarization | None |
| IterSurvey [2510.21900] | Quality-driven (outline refinement) | Per-draft (outline revision) | Full survey | BFS (via paper cards) |

**Genuine advances**: Human-in-the-loop provides transparency and control that fully automated systems cannot match — the user can correct errors, adjust framing, and ensure the survey meets their standards. IterSurvey's paper cards ensure citation faithfulness by grounding each citation in a structured record. ReClaim's sentence-level verification is the most rigorous citation-attribution mechanism in the field.

**Unfulfilled claims**: These systems expose the **Depth–Breadth Trade-Off** (Thread 4) in concrete terms. ReClaim achieves deep verification (per-sentence) but at ~1,000 API calls per 100-sentence survey — a cost that precludes at-scale deployment. IterSurvey achieves breadth (full surveys) but its per-draft iteration cannot verify individual claims with the same granularity as ReClaim. Human-in-the-loop systems achieve control but sacrifice scalability — InteractiveSurvey requires active user engagement across three stages, which does not scale to at-scale deployment. No system simultaneously achieves broad coverage, deep citation verification, and scalable automation.

## 4.4 The Evaluation Benchmark Explosion

The 2025–2026 period saw an explosion of evaluation benchmarks — 8+ in two years — as the field recognized that it lacked adequate evaluation infrastructure. However, instead of converging on a shared standard, each benchmark defined different metrics, protocols, and reference sets, deepening the **Evaluation Comparability Crisis** (Thread 1).

### 4.4.1 The Benchmark Landscape

**Table 7: Evaluation Benchmarks Comparison**

| Benchmark | Year | Scale | Metric Types | Discipline Coverage | Reference Surveys |
|-----------|------|-------|-------------|-------------------|-----------------|
| SurveyBench [2510.03120] | 2025 | 11K+ papers, 4.9K+ surveys | Quiz answerability, outline, content, non-textual quality | Multi-discipline | 4,947 human-written |
| SurveyEval [2512.02763] | 2025 | Cross-subject | Rubric-based subject-specific scoring | Multi-discipline | Subject-specific references |
| SurveyScope (SciSage) [2506.12689] | 2025 | 46 papers, 11 CS domains | Citation F1, overall quality | CS only | 11 human-written |
| Survey-Arena (IterSurvey) [2510.21900] | 2025 | Topic-level | Pairwise win-rate (A vs B) | CS only | None (pairwise only) |
| SurGE [2508.15658] | 2025 | Survey-level | Quality rubric dimensions | CS | Human-written references |
| SGSimEval [2508.11310] | 2025 | Survey-level | Similarity-based metrics | CS | Reference surveys |
| SurveyLens [2602.11238] | 2026 | Cross-discipline | Discipline-adaptive rubrics | Multi-discipline | Discipline-specific |
| DeepSurvey-Bench [2601.15307] | 2026 | Depth-focused | Academic depth assessment | CS | Expert-written depth surveys |
| CiteRAG [2601.14949] | 2026 | Citation-level | Citation prediction accuracy | CS | Ground-truth citation sets |

**SurveyBench**[arXiv:2510.03120] is the largest and most ambitious, providing 4,947 human-written surveys across multiple disciplines with quiz-driven evaluation — testing whether a generated survey enables readers to answer domain-specific questions. **SurveyScope** (associated with SciSage) [arXiv:2506.12689] introduced Citation F1, a dedicated metric for citation quality. **Survey-Arena** (associated with IterSurvey) [arXiv:2510.21900] provides pairwise comparison, enabling A vs B evaluation without reference surveys. **SurveyLens**[arXiv:2602.11238] and **DeepSurvey-Bench**[arXiv:2601.15307] address discipline adaptation and academic depth respectively, adding more dimensions to an already fragmented landscape.

### 4.4.2 The Crisis Deepens

The benchmark explosion has created four distinct fragmentation problems:

**Metric fragmentation**: ROUGE-L, BLEU, FActScore, Citation F1, rubric-based quality scores (varying dimension counts: 5 in ARISE, 12 in Agentic AutoSurvey), win-rate, quiz answerability, coverage breadth, user satisfaction — no two systems report the same set of metrics. A system could rank first on one metric and last on another, and there is no agreed prioritization.

**Reference survey fragmentation**: AutoSurvey uses its own human-written surveys on 10 CS topics. SurveyForge uses SurveyBench (100 human surveys). SciSage uses SurveyScope (46 papers). SurveyG evaluates on self-created CS topics. No system evaluates against another system's reference set, making direct cross-comparison impossible.

**Human evaluation inconsistency**: When human evaluation is used — which is rare — protocols vary wildly: win-rate comparison (AutoSurvey), Likert-scale ratings (SurveyG), user satisfaction surveys (InteractiveSurvey). None follow standardized human evaluation guidelines from the summarization community (e.g., SummEval protocols).

**Benchmark proliferation without consolidation**: Each new benchmark adds evaluation dimensions but none subsumes the others. The result is that every paper can find a benchmark on which its system performs well — which is a recipe for confirmation bias, not scientific progress.

The consequences are stark: the field has produced more evaluation benchmarks than architectural approaches, yet **no benchmark has achieved consensus adoption**. Every generation system reports on a different combination. Claims of "SOTA" and "human-competitive quality" are unfalsifiable — not because the claims might be false, but because there is no shared yardstick to verify them. This crisis, which the next section develops in full, is the single most important challenge facing the field of automated survey generation.

---

# 5 Current Frontier — Deliberation, Domain Expertise, and Local Deployment (2026)

The 2026 frontier introduces three architectural innovations that push beyond Phase 3: deliberation-first synthesis (DOVA), domain-specialized expert agents (OrchMAS), and local-first deployment (ResearchPilot). Each addresses a distinct limitation of prior systems while inheriting and deepening the evaluation crisis.

## 5.1 Frontier Systems

**DOVA (Deliberation-first Multi-Agent Architecture)** [arXiv:2603.13327] inverts the standard retrieve-then-generate pipeline. In DOVA, perspective agents (empiricist, theoretician, methodologist, domain specialist) engage in structured **deliberation before any retrieval occurs**. The mechanism involves three phases: (1) **Deliberation** — agents discuss the research topic, identifying key questions, potential approaches, methodological considerations, and information needs. Each agent contributes from its configured perspective, producing a structured deliberation output that captures the intellectual framework for the survey. (2) **Targeted retrieval** — the deliberation output guides keyword queries and paper selection criteria, focusing retrieval on the specific information needs identified during deliberation. (3) **Collaborative synthesis** — agents collaborate to produce a survey based on the retrieved papers, guided by the deliberation framework. The architecture is single-pass: once the survey is synthesized, there is no iterative refinement loop. DOVA's key architectural claim is that deliberation before retrieval produces more focused, analytical surveys than retrieve-then-deliberate approaches. However, **deliberation quality is entirely unmeasured** — there is no metric to determine whether productive deliberation has occurred, whether the identified information needs are genuinely informative, or whether perspective agents produce differentiated contributions rather than surface-level variation. Cross-reference: this echoes the rubric-quality problem identified in §4.1 — both ARISE's 92.48 score and DOVA's deliberation quality are unvalidated claims about intermediate process quality, revealing a pattern where systems assert quality improvements without establishing measurement frameworks.

**OrchMAS (Orchestrated Multi-Agent System)** [arXiv:2603.03005] deploys domain-specialized expert agents with pre-configured knowledge profiles. The mechanism: (1) **Work plan creation** — a Coordinator Agent analyzes the research question and creates a structured work plan identifying which sub-topics require which expertise (e.g., ML, statistics, neuroscience). (2) **Task assignment** — N Expert Agents, each pre-configured with domain-specific knowledge profiles, are assigned tasks matching their configured expertise. (3) **Domain-specific retrieval and analysis** — each expert retrieves papers using domain-specific configurations (terminology, query templates, filtering criteria) and produces structured summaries. (4) **Coordinated integration** — the Coordinator integrates expert outputs, resolving conflicts between domain perspectives and identifying areas of consensus. The hierarchical orchestration mechanism is architecturally distinct from the flat coordination of ARISE or SciSage. Critical caveats: OrchMAS requires upfront domain expertise configuration — how the system scales to new domains, and whether the expertise is genuinely different from what a general-purpose LLM already knows, is not addressed. The knowledge profiles may be prompt-level customization rather than genuine domain specialization.

**ResearchPilot** [arXiv:2603.14629] is a local-first multi-agent system (4 agents: Retriever, Analyst, Writer, Reviewer) running entirely on consumer hardware using quantized local LLMs (e.g., Llama, Mistral at 4-bit or 8-bit quantization). The mechanism is a sequential pipeline: (1) **Retriever** performs local embedding search over a manually curated paper index — no cloud APIs, no arXiv search. (2) **Analyst** extracts structured information from each paper (claims, methods, findings) using the local LLM. (3) **Writer** synthesizes a survey from the extracted information. (4) **Reviewer** evaluates the draft for coherence and coverage. The system operates at near-zero cost (no API fees) and preserves full privacy. Reported quality is acknowledged to be lower than cloud-based frontier models — the system uses quantized 7B–13B parameter models which produce less fluent and less accurate output than GPT-4 or Claude-class systems. No quantitative comparison against cloud baselines is provided, and the local paper index requires manual curation — the user must download and format papers before generation. The trade-off is steep: privacy and cost savings versus a significant quality gap and manual preprocessing burden.

## 5.2 Cross-Phase Comparison

**Table 8: Frontier Systems vs. Strongest Phase 3 Baselines**

| System | Approach | Agent Count | Retrieval Method | Citation Graph Awareness | Iteration Strategy | Deployment | Key Innovation |
|--------|----------|------------|-----------------|------------------------|-------------------|------------|----------------|
| ARISE [2511.17689] | Rubric-guided iterative | 5 | Hybrid + BFS | BFS | Multi-round (eval-score loop) | Cloud | Formal quality rubric |
| SciSage [2506.12689] | Reflect-when-you-write | 4 | Hybrid + BFS | BFS | Real-time reflection | Cloud | Hierarchical Reflector |
| SurveyG [2510.07733] | Hierarchical graph | 4 | Graph traversal | Hierarchical (3-tier) | Single-pass | Cloud | Three-tier citation graph |
| DOVA [2603.13327] | Deliberation-first | configurable (≥4) | Deliberation-guided | None | Single-pass | Cloud | Inverted retrieve-deliberate |
| OrchMAS [2603.03005] | Domain-expert orchestration | Variable | Per-expert hybrid | None | Multi-round (coordinator iteration) | Cloud | Domain-specific agents |
| ResearchPilot [2603.14629] | Local-first pipeline | 4 | Local embedding | None | Single-pass | Local (consumer HW) | Privacy-preserving local deployment |

## 5.3 Critical Assessment

The frontier systems introduce genuine innovations but inherit — and in some cases deepen — the field's unresolved problems.

**DOVA's deliberation quality is fundamentally unmeasured**. The claim that "deliberation before retrieval improves output quality" is intuitive but untested: there is no deliberation quality metric, no ablation study comparing deliberation-first vs. retrieve-then-deliberate on identical topics, and no evidence that perspective agents produce genuinely different information needs rather than surface variation. As noted above, this mirrors the rubric-quality problem from §4.1 — both are unvalidated claims about intermediate process quality. The **Automation–Control Tension** (Thread 2) deepens: deliberation-first increases autonomy (agents decide what information is relevant through self-driven discussion) without increasing transparency (the user cannot observe or intervene in the deliberation).

**OrchMAS's domain expertise — is it real?** Domain-specialized agents require upfront configuration of knowledge profiles. Whether this produces genuinely different behavior from a general-purpose LLM prompted to "act as a domain expert" is an open question. In other multi-agent systems (ARISE, SciSage, Agentic AutoSurvey), agents use the same underlying LLM with different prompts — the "specialization" is prompt-deep. OrchMAS's knowledge profiles may represent a genuine advance, but the paper provides no analysis of whether agent behavior changes measurably with different profile configurations.

**ResearchPilot trades quality for privacy** — and the trade-off is steep but measurable. Quantized 7B–13B models typically achieve 60–75% of the benchmark performance of the unquantized 70B+ cloud models they distill from, with proportionally higher hallucination rates and lower citation accuracy. No formal comparison against cloud baselines (ARISE, SciSage) is provided — the evaluation is qualitative and self-reported. The manual curation requirement (user must collect and format papers) effectively limits the system to scenarios where the user already has a curated paper collection, which is a fraction of the use cases automated survey generation aims to serve.

**Comparing quality control paradigms**. Across the field, three fundamentally distinct approaches to quality control have emerged, but no study directly compares them. **Deliberation-first (DOVA)** positions quality control *before* retrieval: agents identify information needs proactively, preventing wasted retrieval on irrelevant directions. The quality problem it solves is "retrieve first, think later" — the efficiency gain of avoiding irrelevant paper collection. The failure mode it introduces is that deliberation quality is entirely unvalidated: there is no evidence that DOVA's agents identify genuinely better information needs than what a standard query expansion would produce, and no mechanism to recover if the deliberation goes off-track. **Rubric-guided iteration (ARISE)** positions quality control *during* generation: each draft is scored against a multi-dimensional rubric, and the system iterates until scores converge. The quality problem it solves is "no measurable quality signal" — the rubric provides a target and a convergence criterion. The failure mode is that the rubric is system-defined and uncalibrated against human judgment: the 92.48 score that ARISE reports as its quality claim is a self-consistent number with no external reference. **Reflect-when-you-write (SciSage)** positions quality control *during drafting*: the Hierarchical Reflector catches errors and inconsistencies in real-time as sections are written, preventing error accumulation across sections. The quality problem it solves is "error propagation in sequential drafting" — mistakes made in early sections contaminating later ones. The failure mode is that the reflection is generated by the same model doing the drafting, introducing a self-confirmation bias: the model may fail to detect errors it systematically produces. This mirrors the general concern in self-improving LLM systems where the generator and evaluator share the same parametric knowledge and failure modes.

None of the three paradigms addresses the fundamental question: **does quality control in the generation loop actually improve the final survey, measured against human expert judgment?** DOVA has no measured deliberation-outcome link. ARISE's rubric scores lack human calibration. SciSage's reflection quality is untested against independent evaluation. This question is unfalsifiable without a unified evaluation protocol (see §7, Direction 1) that connects process metrics (deliberation quality, rubric scores, reflection density) to outcome quality (human-judged analytical depth, citation accuracy, narrative coherence).

Critically, **none of the frontier systems address citation graph reasoning beyond BFS** (Thread 3). DOVA's deliberation is about information needs, not about tracing claims through citation chains. OrchMAS's experts are domain-configured but not citation-graph-aware. ResearchPilot's local retrieval is embedding-search only with no graph traversal. The field has not progressed beyond single-hop forward/backward chaining.

The **Depth–Breadth Trade-Off** (Thread 4) also remains unresolved. DOVA's single-pass design and OrchMAS's hierarchical coordination both prioritize breadth (comprehensive coverage via guided retrieval) over depth (individual claim verification). ResearchPilot's local-first approach prioritizes accessibility over both breadth and depth. No frontier system attempts the per-sentence verification that ReClaim demonstrated, nor the iterative refinement that ARISE formalized.

---

# 6 Critical Assessment — Claims, Gaps, and Blind Spots

This section systematically examines what the field claims versus what the evidence actually shows, identifies methodological weaknesses that persist across all phases, diagnoses the evaluation comparability crisis, and exposes blind spots the field is collectively ignoring.

## 6.1 Claim vs. Evidence Gap

The following table compares 7 key claims from representative papers against the supporting evidence, with explicit source tracing for each claim.

**Table 9: Claim vs. Evidence Gap**

| Claim | Claim Source | Supporting Evidence | Critical Assessment |
|-------|-------------|-------------------|-------------------|
| "AutoSurvey achieves human-competitive quality" | AutoSurvey [2406.10252, §5] | ROUGE-L ~0.35; ~40% human win rate | ROUGE-L measures n-gram overlap, not analytical depth. 40% win rate means humans prefer the human baseline in 60% of cases. The claim overstates the evidence. |
| "ARISE achieves 92.48 quality score" | ARISE [2511.17689, §4] | Self-evaluation on system-defined rubric | Rubric dimensions and scoring are defined by the system; no human calibration or cross-validation. Without a mapping to human-judged quality, this is not a meaningful absolute score. |
| "PaperQA2 is superhuman" | PaperQA2 [2409.13740, Abstract] | Outperforms PhD scientists on scientific QA benchmarks | QA is not survey generation. The "superhuman" claim applies to short-form factual question answering, not to multi-section analytical synthesis. |
| "SciSage achieves SOTA Citation F1" | SciSage [2506.12689, §6] | Evaluation on SurveyScope benchmark (46 papers, 11 CS domains) | No concurrent system evaluated on the same benchmark. SOTA claim is unfalsifiable without direct comparison under identical conditions. |
| "Multi-agent systems outperform single-agent" | Claim common across multi-agent papers [2511.17689, 2506.12689, 2509.18661] | Cross-paper comparison across different benchmarks | Each multi-agent paper uses different topics, metrics, and reference surveys. No controlled ablation study compares single vs. multi-agent on identical conditions. |
| "Citation graph traversal improves coverage" | SurveyG [2510.07733, §5], SurveyForge [2503.04629, §4] | Self-evaluation shows more papers retrieved | The question is not quantity but quality — does graph traversal discover genuinely relevant papers? No paper measures relevance gain over keyword/embedding baseline with controlled evaluation. |
| "Survey generation is mature enough for practical use" | Implicit across multiple papers | Fragmented evaluation on small topic samples | The largest evaluation covers ~100 surveys. Human-written surveys exist for thousands of topics. Scalability and reliability at practical deployment scale are undemonstrated. |

## 6.2 Methodological Weaknesses Across All Phases

Eight persistent weaknesses afflict the field. **Self-evaluation epidemic**: of the representative papers analyzed, nearly all rely on self-created evaluation benchmarks with no independent validation. There is no third-party evaluation in the entire corpus.

**Small topical samples**: The field-wide picture reveals consistently narrow evaluation scopes. Across 12 representative systems (AutoSurvey, SurveyX, ARISE, SciSage, Agentic AutoSurvey, SurveyForge, SurveyG, DOVA, OrchMAS, ResearchPilot, IterSurvey, InteractiveSurvey), the mean evaluation scope is approximately 20 topics, the median is roughly 15 topics, and the maximum reaches ~100 topics (SurveyForge on SurveyBench). This is not an individual weakness of a few papers — it is a systemic field-wide constraint. Every evaluation risks overfitting to its specific set, and no system has demonstrated generalization across hundreds of topics spanning multiple disciplines.

**Single-discipline bias**: Computer science dominates evaluation across all phases. Systems tested on CS topics cannot be assumed to generalize to biomedicine, social sciences, or engineering. Only SurveyBench includes multi-discipline reference surveys, and even then, generation systems are rarely evaluated on non-CS topics.

**No standardized benchmark**: Despite 8+ benchmark papers (SurveyBench, SurveyEval, SGSimEval, SurGE, SurveyLens, DeepSurvey-Bench, SurveyScope, Survey-Arena), no single benchmark has achieved consensus adoption. Each new system reports on a different combination of benchmarks, making cross-comparison impossible.

**Superficial citation verification**: Systems claim "high citation accuracy" but verification is typically: (a) post-hoc and NLI-based, not during generation; (b) evaluated at the paper level, not the claim level; (c) self-evaluated, not independently audited. No system uses FActScore-style atomic decomposition for systematic citation audit of survey-length output.

**No measure of analytical depth**: The field evaluates coverage (are all relevant papers cited?), coherence (does the narrative flow?), and citation accuracy (are claims supported by cited papers?). No system or benchmark measures analytical depth — does the survey identify conflicting findings, evaluate methodological quality, identify open questions, or provide novel synthesis? These are the distinguishing features of high-quality human surveys, and they are entirely absent from current evaluation frameworks.

**Reproducibility crisis**: None of the 12+ representative systems (AutoSurvey, SurveyX, ARISE, SciSage, Agentic AutoSurvey, SurveyForge, SurveyG, DOVA, OrchMAS, ResearchPilot, IterSurvey, InteractiveSurvey) provide publicly available code that generates reproducible surveys. Most systems evaluate on self-created datasets with unreleased code, making independent verification impossible. This is not a field-wide inevitability — the Reproducible Pipeline for literature synthesis [arXiv:2508.04612] demonstrates that reproducible automated literature synthesis is achievable: it provides open-source code, achieves F1>0.85 for relevance classification on 50 manually-annotated papers, and demonstrates near-linear scalability to 1,000 papers using 8 CPU workers. Three case studies confirm faithful reproduction within 1–3% of original results. The absence of comparable reproducibility infrastructure in any ASG system is therefore a methodological choice, not a technical constraint. The field cannot claim progress toward practical deployment while its results remain fundamentally unverifiable.

**Computational cost blindness**: No system reports standardized compute costs. GPU-hours, API calls per survey, total inference time, and cost-per-section are unreported across all compared systems. This makes practical feasibility assessment impossible: a system that achieves marginally higher quality at 10× the cost may be worse in deployment terms. For context, ReClaim's per-sentence verification (§4.3) would require ~1,000 API calls for a typical 100-sentence survey, but even this estimate is architectural — no system provides actual cost accounting. Without standardized cost reporting, the field cannot evaluate the practical trade-off between architectural complexity and deployment feasibility.

## 6.3 Evaluation Comparability Crisis

The crisis has four dimensions. **Metric fragmentation**: ROUGE-L, BLEU, FActScore, Citation F1, rubric scores (5–12 dimensions), win-rate, quiz answerability — no two systems report the same set. **Reference survey fragmentation**: AutoSurvey uses its own 10-topic set; SurveyForge uses SurveyBench (100 surveys); SciSage uses SurveyScope (46 papers) — no system evaluates against another's reference set. **Human evaluation inconsistency**: protocols vary from win-rate to Likert scales to user satisfaction, with no standardized guidelines or inter-annotator agreement reporting. **Benchmark proliferation without consolidation**: 8+ benchmarks in two years, each adding evaluation dimensions but none subsuming the others. The result: every paper can find a benchmark on which its system performs well. Cross-comparison is impossible.

## 6.4 Blind Spots

Seven blind spots are collectively ignored. **(1) Multi-hop citation reasoning**: no system traces claims through chains of citations. **(2) Temporal analysis**: no system tracks how research areas evolve over time. **(3) Methodological quality assessment**: no automated system evaluates the methodological rigor of cited papers. **(4) Figure/table generation**: SurveyBench explicitly includes "non-textual quality" as a dimension — every system fails it because none attempts it. **(5) Cross-lingual surveys**: most surveyed papers address English-language surveys only, with no exploration of generating surveys in other languages or covering non-English literature. **(6) Longitudinal trustworthiness**: no system addresses survey maintenance as the literature evolves. **(7) User trust and calibration**: no paper studies whether users detect hallucinated citations, notice coverage gaps, or over-rely on system outputs.

These blind spots converge on a single uncomfortable fact: the field has focused on architectural innovation (more agents, better retrieval, richer rubrics) while neglecting the fundamental question of whether its outputs are trustworthy, maintainable, and analytically useful. The next section outlines how a next-generation system could address these gaps.

---

# 7 Future Directions — Toward Next-Generation Survey Generation

Based on the gaps identified in Section 6, we outline six concrete directions for next-generation systems. Each direction is grounded in a specific blind spot, describes what a solution would look like, and identifies preliminary work. Directions are not independent — we explore tensions between them after presenting each direction.

**1. Unified evaluation protocol** (Addresses: Evaluation Comparability Crisis, §6.3). The field's most urgent need is a shared evaluation framework: a corpus of 500+ topics across multiple disciplines, a multi-dimensional metric suite covering coverage, citation faithfulness (via atomic decomposition), analytical depth, organization quality, and readability; and independent third-party evaluation on held-out topics. Preliminary work: SurveyBench [arXiv:2510.03120] provides the largest corpus (4,947 surveys) but has not achieved adoption as a shared standard. The community must converge on a single benchmark to enable meaningful progress measurement.

**2. Multi-hop citation reasoning** (Addresses: Citation Graph Shallowness, §6.4-1). Current systems traverse citation graphs at single-hop BFS. Next-generation systems need multi-hop expansion with relevance filtering per hop, enabling claim-level provenance tracking through chains of citations. This requires modeling not just which papers cite which, but what claims each paper makes and how they relate to claims in cited papers. Preliminary work: SurveyG's hierarchical tiers gesture at this but are citation-position-based rather than intellectual-lineage-based.

**3. Analytical synthesis capability with structured outputs** (Addresses: No measure of analytical depth, §6.2-6; Figure/table generation blind spot, §6.4-4). The field's most transformative direction: systems that identify conflicting findings across papers, evaluate methodological quality, and produce genuine synthesis rather than descriptive summaries. This requires claim extraction and comparison across papers, methodological feature extraction, and confidence/provenance tracking per claim. Critically, **analytical synthesis naturally subsumes non-textual content generation** — comparison tables, methodology taxonomies, and evidence maps are structured outputs that serve analytical synthesis, not an independent capability. A system that can identify "Paper A uses method X; Paper B uses method Y; they disagree on outcome Z" can automatically generate a comparison table. Similarly, methodology taxonomies emerge naturally from feature extraction, and evidence maps from claim-provenance tracking. Rather than treating non-textual content as a separate generation task, it should be the **structured output extension** of a system that already performs analytical synthesis.

**4. Domain-adaptive expertise** (Addresses: Single-discipline bias, §6.2-3). Rather than relying on a single general-purpose system, future systems should have the ability to adapt to different disciplinary conventions — citation density norms, argumentation structures, and evaluation standards, which vary significantly across fields. OrchMAS [arXiv:2603.03005] provides preliminary work with domain-configured expert agents. A mature implementation would include discipline-specific retrieval configurations, prompt templates, and quality rubrics, with automatic discipline detection rather than manual configuration.

**5. Temporal grounding and maintenance** (Addresses: Longitudinal trustworthiness, §6.4-6). Mechanisms for updating surveys as new literature accumulates: change detection (flagging when new papers change the evidentiary landscape), selective re-generation (updating affected sections without rewriting the entire survey), and version tracking. No preliminary work exists in the automated survey generation literature — this direction is entirely unaddressed.

**6. Calibrated uncertainty communication** (Addresses: User trust, §6.4-7). Systems should express confidence in their claims — flagging weak evidence, contested findings, and information synthesized from partial sources. Current systems produce uniformly confident prose regardless of underlying evidence quality, which is misleading to readers. Preliminary work: no survey generation system addresses uncertainty communication, though related work in QA calibration provides a starting point.

### Cross-Cutting Trade-Offs

These directions are not independent — pursuing one may constrain or complicate another. Three tensions merit explicit consideration.

**Multi-hop reasoning vs. the Depth–Breadth Trade-Off (Thread 4)**. Direction 2 (multi-hop citation reasoning) requires significantly more retrieval per claim — each hop in a citation chain requires re-querying paper databases, extracting claims from new papers, and verifying alignment. This amplifies the computational cost that currently limits per-sentence verification (quantified in §4.3 as ~1,000 API calls for a 100-sentence survey). Multi-hop reasoning could push this to 3,000–5,000+ calls per survey. Without efficiency innovations, the field risks a multi-hop system that is architecturally impressive but practically inoperable at survey scale.

**Domain adaptation vs. evaluation comparability (Thread 1)**. Direction 4 (domain-adaptive expertise) multiplies the number of discipline-specific configurations — each domain may need different retrieval templates, quality rubrics, and evaluation criteria. This directly complicates Direction 1 (unified evaluation protocol): a single benchmark corpus of 500 topics must already span multiple disciplines, but domain-adaptive systems would introduce additional variance by tuning to discipline-specific conventions. The evaluation protocol must be flexible enough to accommodate domain adaptation while remaining standardized enough for cross-system comparison.

**Local deployment vs. evaluation protocol support**. ResearchPilot (§5) demonstrates that local-first deployment constrains the evaluation protocol a system can support. Local models with limited capacity cannot run complex multi-dimensional rubrics, atomic fact decomposition, or multi-hop graph traversal. Systems optimized for local deployment may not be evaluable on the full protocol that Direction 1 proposes — creating a de facto split between "evaluable" and "deployable" systems.

### Prioritization

The unified evaluation protocol (Direction 1) is the most urgent — without it, the field cannot meaningfully measure progress. Multi-hop citation reasoning (Direction 2) is the most technically challenging — it requires fundamentally new architectures for claim-level provenance. Analytical synthesis (Direction 3) is the most transformative — it would move the field from descriptive to genuinely analytical survey generation, with structured outputs (tables, taxonomies, evidence maps) emerging naturally as a byproduct. The field must consolidate evaluation before it can meaningfully measure progress — architectural innovation without evaluation standards is exploration without a compass.
