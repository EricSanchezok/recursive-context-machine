focus_sections:
  - section: "Section 4.3: Missed Opportunity — Learned Graph Representations"
    issue: "Ends speculatively with 'practical concerns or disciplinary isolation' instead of concrete technical barrier analysis. The subsection lists 4 GNN methods but does not explain why each has not been adopted by ASG systems."
    knowledge_gap: "Specific technical barriers preventing GNN-based citation graph integration into ASG pipelines — training data requirements, task-specific retraining cost, architectural mismatch between recommendation/ranking GNNs and generative organization tasks."
    search_keywords: ["GNN citation graph survey generation integration barrier", "learned graph representations generative text organization"]
    retrieved_papers:
      - arxiv_id: "2605.14790"
        title: "Graphs of Research: Citation Evolution Graphs as Supervision for Research Idea Generation"
        relevance: "Demonstrates that learned citation evolution DAG representations can be used for LLM-based generation (idea generation), providing evidence that the barrier is not insurmountable but requires domain-specific fine-tuning (498 seed papers, 5 ML/NLP venues). Directly addresses the 'GNN integration is impossible' vs. 'just hasn't been tried' question."

  - section: "Section 5.4: Blind Spots — What the Field Is Not Looking At (esp. #1 Citation Hallucination and #4 Cost Opacity)"
    issue: "Citation hallucination and cost opacity blind spots are correctly identified but lack quantitative evidence tying them to ASG specifically. The draft mentions available tools but does not provide real-world hallucination prevalence data or cost multiplier estimates."
    knowledge_gap: "Quantitative evidence of citation hallucination prevalence in LLM-generated text and token cost multiplier for agentic vs. single-agent pipelines."
    search_keywords: ["citation hallucination prevalence LLM generated text audit 2025", "token consumption cost multi-agent LLM task benchmark"]
    retrieved_papers:
      - arxiv_id: "2605.07723"
        title: "LLM hallucinations in the wild: Large-scale evidence from non-existent citations"
        relevance: "Large-scale audit of 111M references across 2.5M papers finding ~147K hallucinated citations in 2025 alone. Provides the quantitative real-world evidence needed to move the citation hallucination blind spot from 'this is a risk' to 'this is happening at scale.'"
      - arxiv_id: "2604.22750"
        title: "How Do AI Agents Spend Your Money? Analyzing and Predicting Token Consumption in Agentic Coding Tasks"
        relevance: "First systematic token consumption analysis for agentic tasks — shows 1000× token multiplier vs. non-agentic tasks, 30× variability on same task, and accuracy peaking at intermediate cost. Provides the quantitative frame of reference needed for the cost opacity blind spot."

  - section: "Section 6: Future Directions (esp. restoring 6.4)"
    issue: "Section 6.4 (Ablation Studies and Citation Hallucination Auditing) was merged into 6.3, diluting the importance of both topics. The section overall needs stronger grounding in the new evidence retrieved for Round 2."
    knowledge_gap: "Best practices for ablation study design in NLP/AI systems and operationalized citation hallucination auditing workflows."
    search_keywords: ["ablation study methodology NLP multi-agent system", "citation hallucination detection benchmark LLM 2025"]
    retrieved_papers:
      - arxiv_id: "2605.07723"
        title: "LLM hallucinations in the wild"
        relevance: "The 147K hallucinated citations finding provides the empirical motivation for making citation hallucination auditing a primary evaluation metric, which is the core argument of subsection 6.4."

criteria: "These sections were identified as the weakest in terms of analytical depth: Section 4.3 ends speculatively without technical barrier analysis; Section 5.4 correctly identifies blind spots but lacks quantitative evidence; Section 6 needs structural restoration and stronger empirical grounding."
