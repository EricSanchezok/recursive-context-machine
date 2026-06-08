# SurveySpec

## run_dir
.

## topic
Automated literature survey generation using multi-agent systems and citation-graph expansion.

## reader_need
A practitioner or researcher evaluating the landscape of automated survey systems — what architectures exist, how citation-graph expansion is used for breadth/depth coverage, how quality is measured, and what gaps remain for production use in scientific discovery support.

## scope_include

- **Automated survey-generation systems**: any system that produces a structured literature survey from a corpus (multi-agent, single-agent, LLM-native).
- **Citation-graph expansion methods**: retrieval strategies leveraging forward/backward citation traversal, PageRank-style influence propagation, or graph neural networks for literature discovery.
- **Evaluation protocols**: human-judged and automated metrics for survey quality (coverage, coherence, factuality, citation accuracy).
- **Benchmark tasks**: SurveyBench, Multi-Survey, and any ad-hoc evaluation corpora used in the literature.
- **Foundational works (pre-2023)**:
  1. Vaswani et al. (2017) — Attention Is All You Need (transformer architecture foundational to all surveyed LLM-based systems).
  2. Devlin et al. (2019) — BERT (pre-training for language understanding).
  3. Brown et al. (2020) — GPT-3 (scaling laws and in-context learning).
- **Adjacent domain context**:
  1. PRISMA (Moher et al., 2009; Page et al., 2021) — systematic review reporting standard from biomedicine; informs survey completeness and transparency expectations.
  2. Garfield (1955, 1972) — citation indexing theory; informs the intellectual basis of citation-graph expansion.
- **Temporal balance**: foundational papers from early years (1955–2022) must be included alongside recent results to prevent recency bias.

## scope_exclude

- **Pure summarisation tasks** (e.g., CNN/DailyMail, XSum) that lack a survey structure with bibliography and section organisation.
- **Single-paper or single-topic literature reviews** written manually or without an automated pipeline.
- **Non-English corpora** unless the method is language-agnostic and explicitly claimed as such.
- **Plagiarism detection** in academic text unless directly used as a survey evaluation tool.
- **Generic RAG/retrieval systems** that are not specifically designed for survey generation.

## scope_inclusion_budget

1. **Foundational works**: At least 3 pre-2023 works establishing the paradigm (transformer attention, pre-training, scaling laws).
2. **Adjacent domain context**: At least 2 works from traditions outside the immediate topic (PRISMA for review methodology; Garfield for citation theory).
3. **Temporal balance**: At least 3 works from before 2023 for every 7 works from 2023 onward, to prevent recency bias in fast-moving topic.

## cross_domain_context

- **Systematic review methodology**: PRISMA guidelines and the Cochrane handbook tradition. These inform the expected structure of a survey (study selection flow, inclusion/exclusion criteria, risk-of-bias assessment) and should frame at least one subsection on evaluation gaps.
- **Scientometrics**: Citation analysis theory (Garfield, Price, Small) provides the theoretical grounding for citation-graph expansion algorithms. This perspective should inform the section on retrieval / expansion strategies.
- **Evaluation standards from NLP**: The broader NLP evaluation framework (BLEU, ROUGE, BERTScore, FactKB) contextualises the automated survey metrics proposed in the corpus.

## anchor_questions

1. **Primary**: What architectural patterns exist for automated survey generation that incorporate citation-graph expansion, and how do these systems compare in coverage, coherence, and factuality?
2. **Secondary**: How is survey quality evaluated in the current literature, and what are the most pressing gaps for deployment in real scientific workflows?

## concept_seed

- Core: automated survey generation, literature survey, citation graph, multi-agent system, LLM agent, retrieval-augmented generation, paper retrieval, survey quality
- Synonyms: survey automation, paper survey, literature review automation, survey writer, citation network traversal
- Abbreviations: LLM, RAG, GNN, GNN4Science, SciLit
- Boundary terms: text summarisation (exclude generic), systematic review (include only ML-auto variants), meta-analysis (exclude unless bundled with survey)

## expected_dimensions

- **Method**: single-agent vs. multi-agent; fully autonomous vs. human-in-the-loop; LLM-native vs. pipeline with external retrieval; graph expansion algorithm type (BFS, DFS, PageRank, GNN).
- **Benchmark**: SurveyBench, Multi-Survey, human ratings, user studies; task-specific vs. general-domain.
- **Metric**: ROUGE, BERTScore, factuality (FactKB, entity precision), coverage (citation recall, section overlap), coherence (discourse score).
- **Limitation**: hallucination, citation fabrication, shallow coverage, repetition, evaluation subjectivity, cost, latency.
- **Application**: targeted scientific discovery, research assistants, peer-review aids, curriculum generation.
- **Theory**: citation graph theory, information foraging theory, discourse structure, survey taxonomy.

## quality_bar

A final brief should be able to answer both anchor questions with supporting citations from at least 8 distinct systems/methods, report at least one quantitative comparison from a shared benchmark, and identify at least two unresolved challenges that are actively discussed in the community.
