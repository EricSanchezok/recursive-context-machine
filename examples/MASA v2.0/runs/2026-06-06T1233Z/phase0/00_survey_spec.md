# SurveySpec

**run_dir**: `.`

**topic**: Automated literature survey generation using large language models: agent architectures, retrieval-augmented pipelines, and evaluation methodologies

**reader_need**: A researcher or practitioner who needs a structured overview of methods, architectures, and evaluation practices for LLM-based automated survey generation, to inform system design or identify open challenges.

**scope_include**:
- LLM-based systems that produce structured literature surveys from paper corpora (AutoSurvey, STORM, PaperQA, SurveyAgent, AutoSci, RAG-Survey)
- Agent architectures: single-agent, multi-agent, and hybrid (AutoGen, Self-RAG, RankRAG)
- Planning strategies: hierarchical outline decomposition, iterative refinement
- Retrieval pipelines: query formulation, evidence extraction, citation attribution, multi-source synthesis
- Evaluation: automatic metrics, human evaluation protocols, dedicated survey benchmarks
- Citation graph expansion techniques for coverage
- Time range: 2023–2025

**scope_exclude**:
- General-purpose RAG systems not evaluated on survey or article writing tasks
- Single-paper summarization tools unless embedded in a survey pipeline
- Traditional pre-LLM systematic review tools and meta-analysis methodology
- LLM benchmarking unrelated to writing quality or citation fidelity
- Non-English survey generation

**anchor_questions**:
- Primary: What are the dominant architectural patterns for LLM-based automated survey generation, and how do they handle planning, retrieval, drafting, and quality assurance?
- Secondary 1: How do existing systems evaluate survey quality — what metrics, benchmarks, and human evaluation protocols are used, and what are their limitations?
- Secondary 2: What role does citation graph expansion play in improving coverage, relevance, and citation fidelity in automated surveys?
- Secondary 3: What are the key limitations and open challenges in automated survey generation today (hallucination, evaluation standardization, multi-modal content, longitudinal updates, user steering)?

**concept_seed**:
- Core: automated survey generation, LLM-based literature review, agent architectures for survey writing, citation-grounded survey
- Synonyms: machine-generated surveys, AI-assisted review writing, automated literature survey
- Abbreviations: RAG, MASA, Self-RAG, AutoSurvey, STORM, SurveyAgent, AutoSci
- Related: systematic review, meta-analysis, evidence synthesis, knowledge aggregation, multi-agent coordination, retrieval-augmented generation
- Boundary terms: single-paper summarization, question answering, general RAG

**expected_dimensions**:
- **method**: architectural taxonomy (single-agent, multi-agent, hybrid), planning (hierarchical, iterative), retrieval strategies (query planning, evidence extraction, citation attribution), synthesis (outline-driven, iterative refinement)
- **benchmark**: human evaluation datasets, survey quality benchmarks (coverage, citation accuracy, factuality, coherence)
- **metric**: ROUGE, BLEU, citation precision/recall, factual consistency (Self-Checker, FactScore), human preference ratings, coverage completeness
- **limitation**: hallucination and citation fabrication, evaluation subjectivity, stale knowledge, domain adaptation costs, lack of standardized benchmarks
- **application**: academic survey writing, technology monitoring, competitive intelligence, educational material generation
- **theory**: LLM reasoning for synthesis, RAG theory for scientific domains, multi-agent coordination, knowledge aggregation and citation fidelity

**quality_bar**: The final survey brief must enable a practitioner to (a) describe at least 3 distinct architectural approaches with named systems, (b) identify the dominant evaluation metrics and their known limitations, (c) list the top-3 open challenges with named references, and (d) select a baseline architecture for building a new survey generation system.
