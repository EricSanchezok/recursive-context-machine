subsections:
  # ── Section 1: Introduction ──
  - id: "s1"
    section_number: "1"
    title: "Introduction and Scope"
    depends_on: []
    target_words: 350
    key_papers: []
    writing_instructions: "Write a concise introduction covering: (1) the rapid growth of ASG since 2024, (2) the four research questions: How have ASG architectures evolved? What evidence supports claimed advances? What is the state of evaluation? What blind spots remain? (3) the paper's contributions — first systematic audit of the evidence gap, the evaluation comparability crisis, and blind spots in ASG. End with a roadmap of sections. Keep the summary compelling but grounded in the evidence gap."
    depth_level: "standard"

  # ── Section 2: Evolution ──
  - id: "s2"
    section_number: "2"
    title: "The Evolution of Automated Survey Generation"
    depends_on: ["s1"]
    target_words: 200
    key_papers: []
    writing_instructions: "Brief section overview: ASG has passed through five overlapping phases. Each subsection covers one phase. End with a note that every phase used different evaluation metrics."
    depth_level: "minimal"

  - id: "s2.1"
    section_number: "2.1"
    title: "Phase 0: Pre-LLM Foundations (2012–2023)"
    depends_on: ["s2"]
    target_words: 150
    key_papers: ["1805.02262", "1806.00089", "2004.09741", "1407.5107", "2210.03629"]
    writing_instructions: "Briefly describe the pre-existing infrastructure — citation graph construction (Semantic Scholar), citation expansion (cascading, snowballing), hybrid search strategies, and agent reasoning frameworks (ReAct, LATS). Explain why none could generate text. This subsection establishes the foundation for Thread 1 by showing that graph infrastructure existed before ASG systems."
    depth_level: "minimal"

  - id: "s2.2"
    section_number: "2.2"
    title: "Phase 1: The Single-Agent Pipeline (2024–Early 2025)"
    depends_on: ["s2.1"]
    target_words: 350
    key_papers: ["2406.10252", "2502.14776", "2508.17647", "2503.04629", "2509.19370"]
    writing_instructions: "Describe the foundational AutoSurvey pipeline (Outline → Retrieve → Draft → Refine) and its successors. Explain how SurveyX improved planning with AttributeTree, SurveyGen introduced quality-aware retrieval, SurveyForge added citation chaining, and Meow added outline iteration. Conclude by noting the cognitive bottleneck of single-agent architecture (4.77/10 quality) that drove the transition. This subsection develops Thread 1 by showing the single-agent ceiling."
    depth_level: "standard"

  - id: "s2.3"
    section_number: "2.3"
    title: "Phase 2: The Multi-Agent Explosion (Late 2024–2025)"
    depends_on: ["s2.2"]
    target_words: 350
    key_papers: ["2509.18661", "2506.12689", "2508.04306", "2411.06159", "2510.26012", "2504.14822"]
    writing_instructions: "Describe the rise of multi-agent architectures — Agentic AutoSurvey (4-agent, shared task board), SciSage (4-agent, reflect-while-writing), MATC (5-agent, hierarchical with error-mitigation taskforces), KMCA (mixture-of-experts with minigraph subgraphs), AutoSurvey2 (parallel sections), and InsightAgent (human-centered). Highlight the key result: Agentic AutoSurvey's 8.18/10 vs AutoSurvey's 4.77/10. End with the limitation that drove Phase 3: no citation graph awareness. This subsection develops Thread 1 by showing how complexity increased without structural grounding."
    depth_level: "standard"

  - id: "s2.4"
    section_number: "2.4"
    title: "Phase 3: Graph Awareness and Iterative Refinement (2025)"
    depends_on: ["s2.3"]
    target_words: 250
    key_papers: ["2510.07733", "2508.14317", "2510.21900", "2409.12177"]
    writing_instructions: "Describe two parallel developments: (1) citation graph traversal as first-class retrieval — SurveyG's hierarchical graph (Foundation/Development/Frontier), LitFM's structure-aware foundation model — and (2) iterative refinement as the central pattern — SurveyGen-I's coarse-to-fine retrieval with adaptive planning, IterSurvey's self-evaluation loop. End with the limitation: without standardized evaluation, cross-system comparison is impossible. This subsection develops Thread 3 by showing how iterative systems lack convergence criteria."
    depth_level: "standard"

  - id: "s2.5"
    section_number: "2.5"
    title: "Phase 4: Evaluation Maturation (2025–2026)"
    depends_on: ["s2.4"]
    target_words: 250
    key_papers: ["2510.03120", "2512.02763", "2508.15658", "2602.11238", "2601.15307"]
    writing_instructions: "Describe the emergence of dedicated ASG benchmarks: SurveyBench (11,343 topics, quiz-driven), SurveyEval (7 subjects, 3 dimensions), SurGE (1M-paper corpus, 4 dimensions), SurveyLens (10 disciplines, discipline-aware), DeepSurvey-Bench (academic value). Highlight the irony: these benchmarks exist but no Phase 1–3 system has been evaluated on them. Conclude by foreshadowing the evaluation comparability crisis. This subsection develops Thread 2 by showing the proliferation of standards without convergence."
    depth_level: "standard"

  # ── Section 3: Core Architectures ──
  - id: "s3"
    section_number: "3"
    title: "Core Architectures — Single-Agent, Multi-Agent, and the Controlled Comparison Gap"
    depends_on: ["s2.5"]
    target_words: 150
    key_papers: []
    writing_instructions: "Section overview: This section dissects the three architectural paradigms and audits the evidence for each. The central finding is that the field's most important claim — multi-agent outperforms single-agent — rests on a single controlled comparison."
    depth_level: "minimal"

  - id: "s3.1"
    section_number: "3.1"
    title: "Single-Agent Pipelines — The Foundational Pattern"
    depends_on: ["s3"]
    target_words: 500
    key_papers: ["2406.10252", "2502.14776", "2508.17647", "2503.04629", "2509.19370"]
    writing_instructions: "Compare AutoSurvey, SurveyX, SurveyGen, SurveyForge, and Meow. Use a comparison table with dimensions: planning mechanism (outline vs. attribute_tree vs. quality_driven vs. memory_guided vs. end_to_end), retrieval method (embedding vs. hybrid vs. graph+hybrid), iteration strategy (post_hoc vs. coarse_to_fine vs. memory_driven vs. multi_round), graph awareness (yes/no/type). For each method, explain the architectural mechanism — HOW does SurveyX's AttributeTree work? HOW does SurveyGen's quality-aware retrieval function? Include pros/cons and quantitative results where available. This subsection develops Thread 1 by showing how single-agent systems reached an inherent cognitive bottleneck — one model handling planning, retrieval, writing, and quality."
    depth_level: "deep"

  - id: "s3.2"
    section_number: "3.2"
    title: "Multi-Agent Pipelines — Specialization and Coordination"
    depends_on: ["s3.1"]
    target_words: 500
    key_papers: ["2509.18661", "2506.12689", "2508.04306", "2411.06159", "2510.26012", "2504.14822"]
    writing_instructions: "Compare Agentic AutoSurvey, SciSage, MATC, KMCA, AutoSurvey2, and InsightAgent. Use a comparison table with dimensions: agent count, coordination pattern (shared task board vs. hierarchical manager vs. mixture-of-experts vs. human orchestrator), graph awareness (none vs. bfs chaining), error handling (post-hoc review vs. real-time reflection vs. dedicated error-mitigation taskforces vs. human validation), human involvement, reported metric, and quality score. For each system, explain the coordination mechanism — how does the shared task board work in Agentic AutoSurvey? How does reflect-while-writing work in SciSage? How do MATC's error-mitigation taskforces detect and correct errors? Include pros/cons. This subsection develops Thread 1 by showing how multi-agent systems distribute cognitive load but also add uncontrolled complexity — more agents, more coordination overhead, more prompt engineering."
    depth_level: "deep"

  - id: "s3.3"
    section_number: "3.3"
    title: "Iterative Refinement — Emergent Structure Through Convergence"
    depends_on: ["s3.2"]
    target_words: 400
    key_papers: ["2508.14317", "2510.21900"]
    writing_instructions: "Compare SurveyGen-I and IterSurvey. Explain the coarse-to-fine retrieval mechanism of SurveyGen-I — how the plan evolves as content is discovered. Explain the recurrent-outline mechanism of IterSurvey — how the outline adapts to generated content via self-evaluation. Include a table comparing: iteration granularity (retrieval-level vs. outline-level), memory mechanism (explicit for narrative in SurveyGen-I vs. none in IterSurvey), evaluation method (custom vs. Survey-Arena), and convergence criteria (none in either). Critically analyze: both systems emphasize iteration without specifying when to stop. This subsection develops Thread 3 by showing that iterative systems lack a well-defined objective function and stopping condition."
    depth_level: "deep"

  - id: "s3.4"
    section_number: "3.4"
    title: "The Controlled Comparison Gap"
    depends_on: ["s3.3"]
    target_words: 500
    key_papers: ["2509.18661", "2406.10252", "2505.18286", "2510.04311"]
    writing_instructions: "Systematically audit what has been compared in the literature. Present a table showing every pair of systems evaluated on the same benchmark or metric — only ONE pair exists: Agentic AutoSurvey (8.18/10) vs AutoSurvey (4.77/10) on the same custom evaluation. Show that this single comparison carries the entire evidentiary weight for the claim that multi-agent outperforms single-agent. Then analyze what this comparison CANNOT tell us: (1) does every multi-agent system outperform single-agent? (2) does agent count correlate with quality? (3) does the specific coordination pattern matter? (4) what about the 4.77/10 baseline — is AutoSurvey representative of single-agent systems? Discuss the implications: no independent third-party evaluation exists anywhere in the literature. This subsection is the core of Thread 2 — it establishes that the field's most important claim rests on a single data point."
    depth_level: "deep"

  # ── Section 4: Graph Awareness ──
  - id: "s4"
    section_number: "4"
    title: "Graph Awareness — From Retrieval Afterthought to Structural Backbone"
    depends_on: ["s3.4"]
    target_words: 100
    key_papers: []
    writing_instructions: "Section overview: Only 5 of 35 core method papers use citation graph structure. Those that do reveal a design spectrum — graph as retrieval bolt-on vs. graph as structural backbone. This section traces that spectrum."
    depth_level: "minimal"

  - id: "s4.1"
    section_number: "4.1"
    title: "Citation Chaining as a Retrieval Strategy"
    depends_on: ["s4"]
    target_words: 500
    key_papers: ["2506.12689", "2503.04629", "2508.04306"]
    writing_instructions: "Compare SciSage, SurveyForge, and MATC — three systems that use citation chaining (forward/backward bfs traversal) as a retrieval strategy within a broader pipeline. Use a comparison table with dimensions: graph type (bfs forward/backward/both), traversal depth, how graph output is used (added to retrieval pool vs. prioritized vs. used for structure), graph evaluated in isolation? (yes/no/partially), main evaluation metric. For each system, explain the specific mechanism: how SciSage's Searcher agent chains citations, how SurveyForge's Scholar Navigation Agent follows citation trails, how MATC's Exploration taskforce extends coverage. Critically assess: none of these systems evaluate the graph component in isolation — its contribution is confounded with the full pipeline. This subsection develops Thread 1 by showing graph-as-bolt-on: citation chaining adds retrieval depth but does not affect survey structure."
    depth_level: "deep"

  - id: "s4.2"
    section_number: "4.2"
    title: "SurveyG — The Hierarchical Citation Graph as Architectural Foundation"
    depends_on: ["s4.1"]
    target_words: 500
    key_papers: ["2510.07733"]
    writing_instructions: "Deep-dive into SurveyG. Explain the three-layer hierarchical citation graph construction: how papers are classified into Foundation/Development/Frontier layers, how horizontal traversal works within layers, how vertical traversal works between layers. Explain how the graph structure directly maps to the survey outline — this is the only system where citation graph determines both WHAT to retrieve and HOW to organize the output. Include a table comparing SurveyG's graph integration to the bolt-on approaches from 4.1 across: graph role (primary vs. secondary), impact on outline (direct vs. indirect), evaluation (qualitative vs. quantitative). Critically analyze the absence of an ablation study: the paper asserts that hierarchical graph improves organization but does not test the system without the graph component. This subsection develops Thread 1 by showing graph-as-backbone, contrasting with 4.1's bolt-on approaches."
    depth_level: "deep"

  - id: "s4.3"
    section_number: "4.3"
    title: "The Missed Opportunity — Learned Graph Representations"
    depends_on: ["s4.2"]
    target_words: 300
    key_papers: ["2409.12177", "2408.15371", "2305.01572", "1903.06464"]
    writing_instructions: "Contrast LitFM's structure-aware foundation model (the only paper that integrates graph structure into the retrieval model itself) with the GNN-based citation prediction methods (Temporal GNN, H2CGL, Context-Aware Citation Recommendation) that could have been integrated into ASG systems but were not. Discuss why the field chose embedding+keyword hybrid retrieval over learned graph models — is it because GNNs add complexity without demonstrated improvement for ASG? Or because the field is unaware of these methods? Include a table showing the gap: GNN citation methods achieve 0.8x+ F1/prediction accuracy but have zero adoption in ASG. This subsection develops Thread 1 by identifying what the field is collectively ignoring."
    depth_level: "standard"

  # ── Section 5: Critical Assessment ──
  - id: "s5"
    section_number: "5"
    title: "Critical Assessment — Claims, Gaps, and Blind Spots"
    depends_on: ["s4.3"]
    target_words: 150
    key_papers: []
    writing_instructions: "Section overview: This is the analytical core. Three subsections: (1) claim vs. evidence audit, (2) methodological weaknesses, (3) blind spots. Threads 2 and 3 converge here."
    depth_level: "minimal"

  - id: "s5.1"
    section_number: "5.1"
    title: "Claim vs. Evidence — Systematic Audit"
    depends_on: ["s5"]
    target_words: 500
    key_papers: ["2406.10252", "2509.18661", "2506.12689", "2504.14822", "2510.07733", "2508.17647", "2510.03120"]
    writing_instructions: "Present the Claim vs. Evidence Gap table as a formal table with columns: Claimed Advance, Supporting Evidence, Assessment. Include all 6 claims from evolution_narrative.md: (1) multi-agent outperforms single-agent, (2) +32% Citation F1 (SciSage), (3) +27.2% quality improvement (InsightAgent), (4) hierarchical graph improves organization (SurveyG), (5) SurveyBench enables comprehensive evaluation, (6) quality-aware retrieval improves surveys (SurveyGen). For each claim, explain WHY the evidence is insufficient — not just that it's weak, but what specific methodological improvement would be needed (e.g., 'an ablation study removing a single agent while keeping all else constant'). This subsection is the culmination of Thread 2 — it shows that every major claim in the field has an evidence gap."
    depth_level: "deep"

  - id: "s5.2"
    section_number: "5.2"
    title: "Methodological Weaknesses Across All Phases"
    depends_on: ["s5.1"]
    target_words: 500
    key_papers: ["2406.10252", "2509.18661", "2506.12689", "2508.14317", "2510.21900"]
    writing_instructions: "Analyze the five cross-phase weaknesses with concrete examples: (1) custom evaluation is universal — show the dispersion table of which system uses which benchmark/metric; (2) no ablation studies exist anywhere — cite specific examples (SurveyG's unablated graph claim, Agentic AutoSurvey's unablated multi-agent comparison); (3) human evaluation is unreproducible — no paper reports rubrics, annotator qualifications, or inter-annotator agreement consistently; (4) no convergence criteria — SurveyGen-I and IterSurvey iterate without stopping conditions; (5) graph evaluation in isolation — the 5 graph-aware papers each evaluate their graph component differently, making cross-system comparison impossible. This subsection develops Thread 3 by showing that the field's methodology prevents it from answering its own questions."
    depth_level: "deep"

  - id: "s5.3"
    section_number: "5.3"
    title: "The Evaluation Comparability Crisis"
    depends_on: ["s5.2"]
    target_words: 400
    key_papers: ["2510.03120", "2512.02763", "2508.15658", "2602.11238", "2508.11310"]
    writing_instructions: "Present the benchmark dispersion table showing which system uses which benchmark/metric (from evolution_narrative.md). Show that only ONE pair of systems shares evaluation (Agentic AutoSurvey vs AutoSurvey). Explain why the Phase 4 benchmarks (SurveyBench, SurveyEval, SurGE, SurveyLens, DeepSurvey-Bench, SGSimEval) were designed to solve this crisis but no Phase 1–3 system has been evaluated on any of them — the benchmarks exist but have not been adopted. Discuss the consequences: we cannot rank systems, we cannot determine which architectural choices matter, we cannot replicate any result. This subsection synthesizes Thread 2 and Thread 3 — showing that evaluation incomparability makes the field's claims unfalsifiable."
    depth_level: "deep"

  - id: "s5.4"
    section_number: "5.4"
    title: "Blind Spots — What the Field Is Not Looking At"
    depends_on: ["s5.3"]
    target_words: 500
    key_papers: ["2411.16638", "2305.14251", "2406.19276", "2510.17853", "2602.11238", "2505.18286"]
    writing_instructions: "Catalog and analyze the five blind spots: (1) Citation hallucination is unmeasured — despite CiteGuard, VERISCORE, FActScore, CiteME being available, no ASG system has been systematically audited for citation hallucination; (2) No evaluation of insight or novelty — every benchmark evaluates recall, accuracy, structure, readability; none evaluate whether a survey provides new synthesis; (3) Human ground truth is idealized — human surveys have known biases (citation network effects, self-citation, disciplinary conventions) but are treated as an unproblematic gold standard; (4) Computational cost is opaque — no paper reports token usage, API costs, or runtime; multi-agent cost-quality trade-off is unquantified; (5) No cross-lingual or cross-domain evaluation — all ASG systems generate English NLP/ML surveys. For each blind spot, explain why the field is ignoring it (difficulty? oversight? incentive structure?) and what the consequence is. This subsection is the culmination of Thread 3 — showing that the field is optimizing for the wrong things."
    depth_level: "deep"

  # ── Section 6: Future Directions ──
  - id: "s6"
    section_number: "6"
    title: "Future Directions"
    depends_on: ["s5.4"]
    target_words: 150
    key_papers: []
    writing_instructions: "Section overview: Six concrete directions grounded in the gaps identified in Sections 3–5."
    depth_level: "minimal"

  - id: "s6.1"
    section_number: "6.1"
    title: "First-Class Citation Graph Integration"
    depends_on: ["s6"]
    target_words: 300
    key_papers: ["2510.07733", "2409.12177", "2408.15371"]
    writing_instructions: "Propose building retrieval entirely around the citation graph, using GNNs or attention-over-graphs to learn which papers to retrieve and how to organize them. Reference SurveyG's hierarchical graph as a starting point and LitFM's structure-aware model as a complementary approach. Address the missed opportunity from Section 4.3: GNN-based citation prediction methods have zero adoption in ASG and should be integrated."
    depth_level: "standard"

  - id: "s6.2"
    section_number: "6.2"
    title: "Convergence-Guaranteed Iterative Refinement"
    depends_on: ["s6.1"]
    target_words: 300
    key_papers: ["2508.14317", "2510.21900"]
    writing_instructions: "Propose formal convergence criteria: maximize coverage while minimizing redundancy, maximize citation support while minimizing hallucination risk, maximize insight while minimizing verbosity. Discuss gradient-free optimization over the iterative loop. Address the gap from Section 5.2: current iterative systems loop without convergence."
    depth_level: "standard"

  - id: "s6.3"
    section_number: "6.3"
    title: "Standardized Evaluation, Cost Reporting, and Ablation Studies"
    depends_on: ["s6.2"]
    target_words: 300
    key_papers: ["2510.03120", "2508.15658", "2602.11238", "2510.17853", "2406.19276"]
    writing_instructions: "Call for community convergence on at least one shared benchmark (SurveyBench, SurGE, or SurveyLens) — ideally a community-organized leaderboard. Mandate cost-quality reporting: token usage, API costs, runtime alongside quality scores. Mandate citation hallucination reporting. Propose a research culture where every architectural claim is supported by an ablation study. Address all three threads: Thread 2 (evidence gap) requires shared benchmarks; Thread 3 (blind spots) requires citation auditing and cost reporting; Thread 1 (complexity–grounding) requires ablation studies."
    depth_level: "standard"

  # ── Section 7: Conclusion ──
  - id: "s7"
    section_number: "7"
    title: "Conclusion"
    depends_on: ["s6.3"]
    target_words: 250
    key_papers: []
    writing_instructions: "Summarize the three narrative threads and their implications. Restate the four research questions and how the survey answered them. End with: The field has achieved remarkable architectural diversity in under three years, but architectural diversity without diagnostic evidence is just engineering variety. The path forward requires a reorientation from architectural exploration to diagnostic science — shared benchmarks, systematic ablation, citation hallucination auditing, convergence criteria, and cost-quality reporting. Without this reorientation, the field will continue producing increasingly complex systems without understanding why they work."
    depth_level: "standard"
