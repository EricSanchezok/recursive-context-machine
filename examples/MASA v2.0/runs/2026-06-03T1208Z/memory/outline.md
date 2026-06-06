# Survey Outline: Automated Literature Survey Generation using Large Language Models

## Section 1: Introduction and Motivation
- **Subtitle**: The case for automated survey generation in the era of information overload
- **Writing Requirements**: Establish the problem of exponential growth in scientific publications; motivate the need for automated survey generation; define the scope of the survey (agent architectures, retrieval-augmented pipelines, evaluation methodologies); provide a reading roadmap for the paper.
- **Refinement Guidelines**:
  1. Open with quantitative growth statistics (e.g., arXiv submission rates, PubMed growth) to establish information overload.
  2. Contrast traditional manual survey writing (labor-intensive, slow, subjective) with the promise of LLM-assisted automation.
  3. Introduce the three-pillar taxonomy: agent architectures, retrieval-augmented pipelines, evaluation methodologies.
  4. State the survey's contributions concisely (taxonomy, comparative analysis, open challenges).
  5. Provide a section-by-section reading guide.
- **Reference Papers**: [2005.11401, 2502.13965, 2308.08155, 2402.14207]

## Section 2: Agent Architectures for Automated Survey Generation
- **Subtitle**: From single-agent retrieval to multi-agent collaboration
- **Writing Requirements**: Classify and compare agent architectures used in automated survey systems; cover single-agent systems (STORM, PaperQA), multi-agent frameworks (AutoGen, SurveyAgent, AutoSci), and hybrid approaches; analyze the role of planning, decomposition, and coordination in the writing process.
- **Refinement Guidelines**:
  1. Define the architectural design space: single-agent vs. multi-agent vs. hybrid vs. hierarchical.
  2. Describe single-agent pioneers: STORM (2308.08155) — outline-driven RAG with pre-writing stage using a simulated conversation between "writer" and "reviewer" personas; explicitly label this simulated conversation as a **hybrid approach** (single-agent implementation that mimics multi-agent role specialization through prompt differentiation). PaperQA (2402.14207) — iterative retrieval with citation generation.
  3. Present multi-agent paradigms: AutoGen (2402.14829) — conversational agents with specialized roles (writer, editor, reviewer); AutoSci (2406.03666) — full research pipeline including literature review; SurveyAgent (2412.13129) — dedicated survey generation agents.
  4. Discuss research ideation agents: ResearchAgent (2409.13737) — iterative research question formulation with multi-agent collaboration, relevant for survey scope definition.
  5. Compare architectural patterns: role specialization, communication topology, coordination mechanisms (shared blackboard vs. direct messaging).
  6. Analyze trade-offs: agent count vs. coherence, specialization vs. generality, autonomy vs. controllability. Forward-reference Section 4 for how these trade-offs are evaluated.
  7. Discuss planning approaches: hierarchical outline generation (STORM), iterative refinement (PaperQA), collaborative drafting (AutoSci).
- **Reference Papers**: [2308.08155, 2402.14207, 2402.14829, 2406.03666, 2412.13129, 2409.13737, 2410.06462, 2501.11715, 2303.17651, 2307.05424]

## Section 3: Retrieval-Augmented Pipelines for Scientific Survey Writing
- **Subtitle**: Evidence gathering, citation quality, and knowledge integration
- **Writing Requirements**: Detail the retrieval pipeline components specific to survey generation: query formulation, source selection, evidence extraction, citation attribution, and multi-source synthesis; cover domain-specific retrieval challenges; analyze RAG variants adapted for scientific literature.
- **Refinement Guidelines**:
  1. Map the canonical RAG pipeline (query → retrieve → augment → generate) onto survey writing.
  2. Discuss query formulation strategies: manual vs. auto-generated, single vs. multi-query, iterative refinement.
  3. Cover retrieval sources and indexing strategies: arXiv API, Semantic Scholar, PubMed, custom corpora; dense vs. sparse retrieval for scientific text.
  4. Analyze evidence extraction granularity: abstract-level, paragraph-level, claim-level.
  5. Present Self-RAG (2404.16130) — learned retrieval decisions with self-critique; RankRAG (2407.16833) — unified ranking and generation.
  6. Cover citation attribution methods: citation placement, citation context generation, citation faithfulness evaluation (forward-reference Section 4.2 for evaluation metrics for citation faithfulness).
  7. Discuss multi-source synthesis: resolving contradictions, handling conflicting evidence, temporal weighting.
  8. Present dedicated survey-RAG systems: RAG-Survey (2503.04626), KG-enhanced retrieval (2407.19687), multimodal RAG (2504.09867).
- **Reference Papers**: [2005.11401, 2404.16130, 2407.16833, 2503.04626, 2407.19687, 2504.09867, 2406.18676, 2409.08116, 2403.07199]

## Section 4: Evaluation Methodologies for LLM-Generated Surveys
- **Subtitle**: Metrics, benchmarks, and quality assessment frameworks
- **Writing Requirements**: Survey the evaluation landscape for automated surveys; cover coverage metrics, citation quality, factual consistency, coherence, and organization; present existing benchmarks and evaluation frameworks; discuss human evaluation protocols and automated metrics.
- **Refinement Guidelines**:
  1. Define the multi-dimensional quality space: coverage (completeness, relevance), citation quality (accuracy, placement, context), factual consistency (hallucination, contradiction), coherence (structure, flow, terminology), organization (taxonomy clarity, logical progression).
  2. Present automated metrics: ROUGE, BLEU, BERTScore for survey sections; factuality metrics (FactScore, HALO); citation faithfulness metrics (2408.16743).
  3. Cover dedicated survey benchmarks: Evaluating LLM-Generated Surveys (2402.05680), SurveyEval (2403.07929).
  4. Discuss human evaluation protocols: rubric design, inter-annotator agreement, expert review.
  5. Analyze the relationship between retrieval quality and survey quality.
  6. Present open challenges: no standard benchmark, task-specific evaluation, longitudinal quality (citation decay).
- **Reference Papers**: [2402.05680, 2403.07929, 2406.12178, 2408.16743, 2502.00958, 2411.18117]

## Section 5: Comparative Analysis and System Trade-offs
- **Subtitle**: Head-to-head comparison of existing automated survey systems
- **Writing Requirements**: Provide a structured comparison of existing end-to-end systems; compare on dimensions: architecture type, retrieval strategy, output quality, scalability, usability; include a summary table; discuss application scenarios where each system excels.
- **Refinement Guidelines**:
  1. Create a comparison matrix across systems: STORM, AutoSurvey, PaperQA, AutoSci, SurveyAgent, RAG-Survey.
  2. Compare architectural features: single vs. multi-agent, planning strategy, human-in-the-loop support.
  3. Compare retrieval pipelines: source coverage, citation quality, synthesis strategy.
  4. Compare output quality: factual accuracy, coverage, coherence (cite available evaluation results).
  5. Discuss scalability: context limits, generation time, compute requirements.
  6. Analyze application fit: broad survey (AutoSurvey), specific topic (PaperQA), collaborative drafting (AutoGen-based).
  7. Identify gaps: no system supports all phases end-to-end; evaluation is inconsistent across systems.
- **Reference Papers**: [2502.13965, 2308.08155, 2402.14207, 2406.03666, 2412.13129, 2503.04626, 2410.06462]

## Section 6: Open Challenges and Future Directions
- **Subtitle**: Remaining barriers to fully automated, trustworthy survey generation
- **Writing Requirements**: Identify the key unresolved challenges: hallucination and citation accuracy, evaluation standardization, multi-modal survey content, longitudinal updates, domain adaptation, user steering and control; propose promising future research directions.
- **Refinement Guidelines**:
  1. Address hallucination and citation fabrication: causes (retrieval miss, generation hallucination), detection methods, mitigation strategies.
  2. Discuss evaluation standardization: need for a community benchmark for survey generation, reproducibility crisis (cross-reference Section 4.6 for detailed evaluation challenges).
  3. Cover multi-modal surveys: incorporating figures, tables, equations, code; challenges in multi-modal RAG.
  4. Discuss longitudinal survey maintenance: stale citations, evolving fields, update mechanisms.
  5. Address domain adaptation: domain-specific terminology, evaluation criteria, source coverage.
  6. Discuss user steering: interactive survey generation, human-in-the-loop refinement, controllable depth/breadth.
  7. Propose future directions: self-improving survey agents, citation-aware generation, inter-survey synthesis.
- **Reference Papers**: [2502.13965, 2308.08155, 2402.14207, 2406.12178, 2408.16743, 2411.18117]

## Section 7: Conclusion
- **Subtitle**: Synthesis and outlook
- **Writing Requirements**: Summarize the key findings from each section; reiterate the three-pillar taxonomy's value; emphasize the transformative potential of automated survey generation while being honest about limitations; call for community effort on evaluation and trustworthiness.
- **Refinement Guidelines**:
  1. Recapitulate the three architectural families and their trade-offs.
  2. Summarize retrieval pipeline improvements and remaining gaps.
  3. Restate the evaluation deficit as the most critical bottleneck.
  4. End with a forward-looking statement on the role of automated surveys in accelerating scientific progress.
- **Reference Papers**: [2502.13965, 2402.05680, 2404.16130]
