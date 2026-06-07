# SurveySpec — Automated Literature Survey Agents with Citation Graph Expansion

## run_dir
`.`

## topic
Automated literature survey agents that use citation graph expansion as a core retrieval and synthesis strategy.

## reader_need
Researchers and engineers building next-generation literature review tools. This brief should help them (a) understand the design space of citation-graph-driven survey agents, (b) identify open challenges in coverage, bias, and temporal drift, and (c) choose or extend an existing approach for their own domain.

## scope_include
- Systems that frame literature survey as an agentic pipeline (plan → retrieve → expand → synthesize → revise).
- Methods that use citation graph traversal (forward/backward citation chaining, co-citation, bibliographic coupling) as a retrieval expansion strategy.
- Benchmarks and evaluation protocols for automated survey generation (e.g., faithfulness, coverage, citation accuracy).
- Representations of citation graphs used for retrieval (e.g., graph neural network embeddings, adjacency-based relevance propagation).
- **Foundational works (3+ pre-2023):**
  - *Vaswani et al. (2017)* — "Attention Is All You Need": Transformer architecture that underlies modern survey agents.
  - *Carbonell & Goldstein (1998)* — "The Use of MMR, Diversity-Based Reranking for Reordering Documents": Maximum Marginal Relevance, a precursor to coverage-aware retrieval.
  - *Kleinberg (1999)* — "Authoritative Sources in a Hyperlinked Environment" (HITS algorithm): Early citation-graph authority propagation.
  - *Page et al. (1999)* — "The PageRank Citation Ranking": Foundational citation graph ranking.
- **Adjacent domain context (2+ works/traditions outside immediate topic):**
  - *PRISMA 2020 statement* (Page et al., 2021) — Preferred Reporting Items for Systematic Reviews and Meta-Analyses: Systematic review methodology from evidence synthesis.
  - *Hicks et al. (2015)* — "The Leiden Manifesto for Research Metrics": Scientometrics principles that guard against perverse incentives in citation-based evaluation.
- **Temporal balance:** Include foundational papers from early years (pre-2010 citation graph algorithms) alongside recent agent-based approaches (2023–2025).

## scope_exclude
- General-purpose RAG systems that do not specifically address literature survey or citation graph traversal.
- Single-paper summarization tools (e.g., paper digesters) that do not expand beyond one source.
- Bibliometric analysis of a specific field (e.g., "citation analysis in chemistry") — focus is on the *method*, not a discipline case study.
- Non-textual citation analysis (e.g., patent citation graphs, software dependency graphs) unless directly informing the textual survey methodology.

## cross_domain_context
- **Systematic review methodology** (PRISMA, Cochrane handbook): Pipeline stages (identification, screening, eligibility, inclusion) map onto agent planning + retrieval + filtering + synthesis stages. This domain informs the framing of survey generation as a structured review process with explicit reporting standards.
- **Scientometrics** (Leiden Manifesto, h-index, citation normalization): Provides tools to audit citation graph bias (e.g., field-normalised citation counts, discipline skew, Matthew effect). Informs a subsection on bias auditing for automated surveys.

## anchor_questions
- **Primary:** How can citation graph expansion be effectively combined with LLM-based reasoning to produce faithful, balanced, and comprehensive literature surveys?
- **Secondary:**
  1. What are the failure modes of citation-graph-driven survey agents (topic drift, citation echo chambers, recency overemphasis)?
  2. How do existing evaluation benchmarks measure and trade off coverage, faithfulness, and readability?
  3. What design decisions (graph construction method, expansion policy, synthesis architecture) most strongly affect survey quality?

## concept_seed
- Core: automated literature survey, citation graph expansion, citation chaining, bibliographic coupling, co-citation analysis, survey generation, agentic survey.
- Synonyms & abbreviations: LLM survey agent, citation graph traversal, literature review automation, forward/backward citation expansion.
- Boundary terms: systematic review automation, evidence synthesis, meta-analysis, scoping review.
- Method terms: retriever-reader architecture, plan-then-expand, graph-of-thought, hierarchical citation network.

## expected_dimensions
1. **Method** — Architecture components: planner, retriever, citation graph builder, expander, synthesizer.
2. **Benchmark** — Evaluation datasets (e.g., SurveyBench, LitQA, MIRAGE) and protocols (human rating, reference survey overlap).
3. **Metric** — Coverage (recall of relevant works), faithfulness (citation grounding), readability (automated and human), bias (temporal, disciplinary, citation-tier).
4. **Limitation** — Topic drift under deep expansion, citation echo chambers, LLM hallucination in citation synthesis, evaluation cost.
5. **Application** — Disciplinary adoption (CS, biomedicine, social sciences), tool/API integration (Semantic Scholar, OpenAlex, arXiv).
6. **Theory** — Citation graph properties relevant to retrieval: power-law degree distribution, clustering coefficient, structural holes, time-dependent citation decay.

## quality_bar
The final brief must enable a reader to:
- List the major current systems (e.g., AutoSurvey, PaperQA, Elicit, Scim, SurveyAgent) and classify their graph expansion strategy.
- Identify which graph construction choices (depth, breadth, recency bias, field restrictors) lead to which failure modes.
- Design an evaluation protocol for a new survey agent, specifying at least three metrics and a baseline comparison.
- Judge whether a given survey agent is likely to produce a biased review for a cross-disciplinary topic.
