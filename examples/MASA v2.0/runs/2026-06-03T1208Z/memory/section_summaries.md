## Section 1: Introduction and Motivation
- **Key Words**: information overload, automated survey generation, three-pillar taxonomy, agent architectures, retrieval-augmented pipelines, evaluation methodologies
- **Key Points**: Established exponential growth in scientific publications as motivation; defined the three-pillar taxonomy (agent architectures, retrieval-augmented pipelines, evaluation methodologies); stated contributions (taxonomy, comparative analysis, open challenges); provided section-by-section reading roadmap.
- **Status**: Complete. Introduces scope, motivation, taxonomy, and contributions.
- **Needs**: None.

## Section 2: Agent Architectures for Automated Survey Generation
- **Key Words**: single-agent, multi-agent, hybrid, STORM, PaperQA, AutoGen, SurveyAgent, AutoSci, MAMBA, planning strategies, architectural trade-offs
- **Key Points**: Classified architectures along agent count, planning strategy, and coordination mechanism axes; covered single-agent systems (STORM, PaperQA) and multi-agent systems (AutoGen, SurveyAgent, AutoSci, MAMBA); analyzed planning strategies (hierarchical outline, iterative refinement, collaborative drafting, graph-based reasoning); discussed trade-offs (agent count vs. coherence, specialization vs. generality, autonomy vs. controllability). STORM's simulated conversation explicitly labeled as a hybrid approach bridging single-agent and multi-agent paradigms. Forward reference to Section 4 added in trade-offs discussion. 9 reference papers cited.
- **Status**: Complete. All three architectural categories (single-agent, multi-agent, hybrid) now explicitly covered. Cross-reference to evaluation section added.
- **Needs**: None.

## Section 3: Retrieval-Augmented Pipelines for Scientific Survey Writing
- **Key Words**: RAG pipeline, query formulation, retrieval sources, evidence extraction, citation attribution, multi-source synthesis, Self-RAG, RankRAG, RAG-Survey, multimodal RAG
- **Key Points**: Mapped canonical RAG pipeline onto survey writing; covered query formulation strategies (manual vs. auto-generated, single vs. multi-query, iterative refinement); discussed retrieval sources (arXiv, Semantic Scholar, PubMed, custom corpora) and methods (sparse vs. dense, SciBERT, KG-enhanced, GraphRAG); analyzed evidence extraction granularity (abstract-level, paragraph-level, claim-level); presented Self-RAG and RankRAG; covered citation placement, faithfulness, and context generation with forward reference to Section 4.2 for citation faithfulness metrics; presented dedicated survey-RAG systems (RAG-Survey, Multimodal RAG). 9 reference papers cited.
- **Status**: Complete. Full pipeline coverage with cross-reference to evaluation section added.
- **Needs**: None.

## Section 4: Evaluation Methodologies for LLM-Generated Surveys
- **Key Words**: coverage, citation quality, factual consistency, coherence, organization, automated metrics, benchmarks, human evaluation, SurveyEval, CitationFaithfulness, HALO
- **Key Points**: Defined multi-dimensional quality space (coverage, citation quality, factual consistency, coherence, organization); presented automated metrics (lexical overlap, semantic similarity, factuality metrics, citation faithfulness); covered dedicated benchmarks (Evaluating LLM-Generated Surveys, SurveyEval, LongBench-E); discussed human evaluation protocols (rubric design, inter-annotator agreement, expert review); analyzed retrieval-quality relationship; identified open challenges (no standard benchmark, task-specific evaluation, longitudinal quality). 6 reference papers cited.
- **Status**: Complete. Comprehensive coverage of evaluation landscape including metrics, benchmarks, and human evaluation protocols.
- **Needs**: None.

## Section 5: Comparative Analysis and System Trade-offs
- **Key Words**: comparison matrix, STORM, AutoSurvey, PaperQA, AutoSci, SurveyAgent, RAG-Survey, MAMBA, architectural comparison, retrieval comparison, scalability, application fit
- **Key Points**: Created comparison matrix across 8 systems covering architecture, planning, retrieval, citation, human-in-loop, and output format; compared architectural features (single vs. multi-agent, planning strategy); compared retrieval pipelines (source coverage, citation quality); compared output quality (factual accuracy, coverage, coherence); discussed scalability (context limits, generation time, compute); analyzed application fit for each system; identified gaps (no end-to-end system, inconsistent evaluation, limited human-in-loop, no longitudinal updates). 7 reference papers cited.
- **Status**: Complete. Head-to-head comparison with summary table and application-specific recommendations.
- **Needs**: None.

## Section 6: Open Challenges and Future Directions
- **Key Words**: hallucination, citation fabrication, evaluation standardization, multi-modal content, longitudinal maintenance, domain adaptation, user steering, future directions
- **Key Points**: Addressed hallucination and citation fabrication (intrinsic vs. extrinsic, causes, detection, mitigation); discussed evaluation standardization (need for community benchmark, reproducibility crisis); covered multi-modal surveys (figures, tables, equations); discussed longitudinal survey maintenance (stale citations, update mechanisms); addressed domain adaptation (domain-specific conventions, retrieval, evaluation); discussed user steering (interactive generation, human-in-the-loop, controllable depth/breadth); proposed future directions (self-improving agents, citation-aware generation, inter-survey synthesis, cross-modal generation, verification as a service). 6 reference papers cited.
- **Status**: Complete. Comprehensive identification of challenges with concrete future research directions.
- **Needs**: None.

## Abstract
- **Key Words**: automated survey generation, LLMs, information overload, three-pillar taxonomy, agent architectures, retrieval-augmented pipelines, evaluation, open challenges
- **Key Points**: Established information overload as motivation; defined three-pillar taxonomy; previewed comparative analysis of 8 systems; identified evaluation standardization as critical bottleneck.
- **Status**: Complete. Concise summary of survey scope, methodology, and findings.
- **Needs**: None.

## Section 7: Conclusion
- **Key Words**: synthesis, outlook, architectural trade-offs, retrieval pipeline gaps, evaluation deficit, future directions
- **Key Points**: Recapitulated three architectural families and their trade-offs (single-agent vs. multi-agent); summarized retrieval pipeline improvements (Self-RAG, RankRAG, KG-enhanced retrieval) and remaining gaps (citation fabrication, conflict resolution, temporal awareness); restated evaluation deficit as most critical bottleneck; proposed forward-looking directions (self-improving systems, citation-aware generation, inter-survey synthesis, verification infrastructure); positioned automated surveys as augmentation of human survey writers.
- **Status**: Complete. Synthesizes findings from all sections, emphasizes evaluation bottleneck, and provides outlook.
- **Needs**: None.
