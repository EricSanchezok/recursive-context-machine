focus_sections:
  - section: "§2.5: Citation Graph Re-integration (Current Frontier)"
    issue: "Covers only 4 papers (SurveyG, Graphs of Research, Science Hierarchography, LitFM) with shallow integration depth. The section would benefit from DeepSurvey (2605.29522), a May 2026 paper combining citation-graph expansion with multi-granularity agentic refinement that achieves 8.644/10 content score and 83.3% expert preference over human-written surveys. This would strengthen the 'open problems' analysis by showing a first attempt at deeper integration."
    knowledge_gap: "More 2025-2026 papers demonstrating deep graph-aware survey generation with quantitative quality metrics on standardized benchmarks"
    search_keywords: ["graph-aware survey generation 2025 2026", "citation graph expansion survey generation", "deep graph LLM integration literature survey"]
    retrieved_papers:
      - arxiv_id: "2605.29522"
        title: "DeepSurvey: Enhancing Analytical Depth and Citation Reliability in Automated Survey Generation"
        relevance: "Directly demonstrates citation-graph expansion combined with multi-granularity refinement — a deeper level of graph-LLM integration than any existing system in §2.5. Provides quantitative evidence (8.644/10 content score, 83.3% expert preference over human) that deep integration improves both content quality and citation reliability."

  - section: "§3.4: The Bottleneck Transfer Problem"
    issue: "The bottleneck transfer argument is conceptually strong but lacks direct empirical evidence linking retrieval quality measures to generation outcomes. The section makes a theoretical argument with structural reasons but cannot cite a study that directly measures the correlation between retrieval metrics and survey quality."
    knowledge_gap: "Empirical evidence on whether retrieval quality metrics (precision, recall, NDCG) actually correlate with downstream generation quality (coherence, citation accuracy, coverage)"
    search_keywords: ["retrieval quality generation quality correlation", "bottleneck transfer RAG survey generation", "retrieval metric generation outcome relationship"]
    retrieved_papers:
      - arxiv_id: "2512.20854"
        title: "How important is Recall for Measuring Retrieval Quality?"
        relevance: "Directly addresses the bottleneck transfer gap: measures the correlation between retrieval quality metrics and LLM-judged response quality (where responses are generated from retrieved documents). Provides empirical evidence that the correlation is metric-dependent and not guaranteed — supporting the argument that the field cannot assume retrieval improvements translate to survey quality gains."

  - section: "§6.2: Learned Traversal Policies for Hierarchical Graphs"
    issue: "Relatively thin compared to §6.1 and §6.3. Currently references only PaSa and Temporal GNN (2408.15371). Would benefit from RL-based reference selection paper (2509.05874) which uses deep RL for sparse reference selection — demonstrating learned stopping criteria and selection policies."
    knowledge_gap: "Existing work on reinforcement learning for hierarchical graph traversal, learned stopping criteria in knowledge-intensive retrieval, and RL-based reference selection for literature analysis"
    search_keywords: ["reinforcement learning graph traversal retrieval", "learned stopping criteria knowledge retrieval", "RL reference selection sparse relevant papers"]
    retrieved_papers: []
criteria: "These sections were identified as the weakest in terms of analytical depth and supporting evidence. §2.5 has the fewest papers and shallowest analysis among evolution sections. §3.4 makes a strong theoretical case but lacks empirical evidence for the bottleneck transfer claim. §6.2 has the fewest concrete references among future direction proposals."
