subsections:
  - id: "s1"
    section_number: "1"
    title: "Introduction and Scope"
    depends_on: []
    target_words: 450
    key_papers:
      - "2309.09727"
      - "2002.06961"
      - "2409.04600"
      - "2004.05904"
      - "1501.05462"
    writing_instructions: >
      Write a concise introduction covering: (a) the explosion of scientific
      publishing and the resulting need for automated survey generation —
      cite STM Global Brief 2023 or UNESCO Science Report for the "2M+ papers
      annually" figure, (b) the dual challenge of retrieval and synthesis,
      (c) positioning relative to existing surveys — distinguish this survey
      from "When LLMs Meet Citation" (2309.09727), the citation recommendation
      survey (2002.06961), and "Emergence of LLM as a Tool in Literature
      Reviews" (2409.04600) — by noting that this survey uniquely focuses on
      citation graph expansion as a specific retrieval strategy, provides
      a critical assessment of unsubstantiated claims, and traces a five-phase
      evolution narrative, (d) cross-domain context: connect to PRISMA
      systematic review methodology (2004.05904), SummEval multi-dimensional
      summarization evaluation, and scientometric citation analysis (1501.05462,
      2203.17239) — note that automated survey systems have not adopted
      PRISMA's screening rigor, SummEval's multi-dimensional evaluation, or
      scientometric bias corrections, (e) the three contributions of this
      survey — taxonomy of 135+ papers (NOT 140+), five-phase narrative,
      critical assessment, (f) a roadmap enumerating each section's purpose.
      Use "135+ papers" throughout, not "140+". Do not report results or
      pre-empt the critical analysis. Establish the topic's timeliness.
      CRITICAL: Cite the "2M+ papers annually" claim.
    depth_level: "standard"

  - id: "s2.1"
    section_number: "2.1"
    title: "Citation Graph Foundations (2015–2020)"
    depends_on: ["s1"]
    target_words: 600
    key_papers:
      - "1805.02262"
      - "2004.07180"
      - "2409.12177"
      - "2410.03761"
      - "1903.06464"
    writing_instructions: >
      Describe Phase 1: pre-LLM graph-based methods. Explain the Semantic
      Scholar Literature Graph (1805.02262) as infrastructure — 280M+ node
      heterogeneous citation graph. Explain SPECTER (2004.07180) citation-informed
      embeddings via triplet contrastive learning — how citation pairs create
      structure-aware representations. Include LitFM (2409.12177) graph transformer
      as a late-phase extension showing the graph+text joint modeling approach.
      Include a performance table across papers. End with the genuine achievement
      (graph structure carries orthogonal signals — intellectual lineage, community
      boundaries, role differentiation) and the limitation that drove Phase 2
      (graphs could retrieve but not synthesize). This subsection develops Thread 1
      by establishing what structural signals Phase 1 captured and why they could
      not generate narrative text. FIRST MENTION of LitFM +28.1% precision
      metric here — this is the canonical location.
    depth_level: "deep"

  - id: "s2.2"
    section_number: "2.2"
    title: "Single-Agent Survey Pipelines (2023–Early 2024)"
    depends_on: ["s2.1"]
    target_words: 650
    key_papers:
      - "2406.10252"
      - "2312.07559"
      - "2409.13740"
      - "2402.14207"
      - "2411.14199"
    writing_instructions: >
      Describe Phase 2: the LLM revolution. Explain AutoSurvey's
      canonical pipeline (Outline→Retrieve→Draft→Refine). Explain
      PaperQA's citation chaining as BFS expansion (the exception
      proving the graph-blind rule). Explain PaperQA2's contradiction
      detection and "superhuman" claim. Include a comparison table
      across all five papers: pipeline stages, retrieval method, graph
      awareness, iteration strategy, claimed metric, evaluation
      benchmark, computational scale (datastore size, model size).
      Note OpenScholar's 45M-paper datastore as a significant
      infrastructure advantage — the cost dimension of scale. End with
      what Phase 2 achieved (proved feasibility) and the unfulfilled
      claim analysis: "competitive with human" conflates structural
      coherence with critical depth. FIX VAGUE ATTRIBUTION: Replace
      phrases like "claims that would later attract scrutiny" with
      specific citations — e.g., "AutoSurvey's 'competitive with
      human-written' claim [2406.10252] was critiqued by [2508.15658]
      for lacking critical depth measurement." This subsection develops
      Threads 1 (abandoning graphs for semantics) and 2 (nearly every
      system uses a different benchmark — PaperQA/PaperQA2 share
      LitQA/LitQA2, so use "nearly every" not "every").
    depth_level: "deep"

  - id: "s2.3"
    section_number: "2.3"
    title: "Multi-Agent Architectures (Late 2024–2025)"
    depends_on: ["s2.2"]
    target_words: 600
    key_papers:
      - "2506.12689"
      - "2509.18661"
      - "2508.04306"
      - "2504.14822"
    writing_instructions: >
      Describe Phase 3: multi-agent systems. Explain SciSage's
      reflect-while-writing mechanism — how the Reflector agent
      evaluates in real-time rather than post-hoc, and why this
      prevents error accumulation. Explain Agentic AutoSurvey's
      4-agent framework (Planner, Researcher, Writer, Reviewer) with
      shared task board. Explain MATC's error-mitigation taskforces
      (Exploitation, Exploration, Experience, Self-Correction). Include
      a comparison table: agent count, roles, coordination pattern,
      citation graph awareness, quality metric, evaluation benchmark,
      computational overhead (N agents × API calls). End with
      unfulfilled claims: the near-doubling in quality (8.18 vs 4.77)
      lacks controlled ablation — confounds with prompt engineering,
      base model, and retrieval are not isolated. METRIC HANDLING:
      This is the FIRST MENTION of SciSage +32% citation F1 and
      Agentic AutoSurvey 8.18/10 vs 4.77/10. These are the canonical
      locations. Later sections should cross-reference here (e.g.,
      "SciSage (§2.3) reports +32%"). This subsection develops Thread 3
      (multi-agent coordination cannot fix incomplete retrieval) and
      Thread 2 (baselines are not standardized).
    depth_level: "deep"

  - id: "s2.4"
    section_number: "2.4"
    title: "Iterative and RL-Guided Systems (2025)"
    depends_on: ["s2.3"]
    target_words: 550
    key_papers:
      - "2501.10120"
      - "2510.21900"
      - "2510.27126"
      - "2508.14317"
    writing_instructions: >
      Describe Phase 4: learning from feedback. Explain PaSa's
      RL-optimized search policy — how the agent learns when to follow
      citations vs. refine keywords vs. search by author, using
      epsilon-greedy exploration with synthetic trajectory generation.
      Explain IterSurvey's recurrent outline generation as procedural
      (non-RL) refinement. Explain AURA's adaptive questioning with
      the LSDE metric. Include a comparison table: learning approach
      (RL vs procedural), objective optimized (recall, information
      gain, self-eval criteria), citation graph awareness, training
      data requirement, computational cost (training compute vs.
      inference-only). METRIC HANDLING: This is the FIRST MENTION of
      PaSa +37.78% recall@20. Include baseline: GPT-4o recall@20 for
      context. This is the canonical location for this metric. End with
      unfulfilled claims: PaSa optimizes for recall, not survey
      quality — the transfer is asserted, not demonstrated. This
      subsection develops Thread 3 (the bottleneck transfer problem:
      optimizing retrieval does not guarantee better surveys).
    depth_level: "deep"

  - id: "s2.5"
    section_number: "2.5"
    title: "Citation Graph Re-integration (Current Frontier, 2025–2026)"
    depends_on: ["s2.4"]
    target_words: 550
    key_papers:
      - "2510.07733"
      - "2605.14790"
      - "2504.13834"
      - "2409.12177"
    writing_instructions: >
      Describe Phase 5: the return to structure. Explain SurveyG's
      three-layer hierarchical graph (Foundation/Development/Frontier)
      with horizontal and vertical traversal — how the graph
      organization becomes the survey outline. Explain Graphs of
      Research's 2-hop citation DAG as SFT training data for idea
      generation. Explain Science Hierarchography's hybrid
      embedding+LLM clustering. Include a performance table. End
      with open problems: graph-LLM integration remains shallow
      (SurveyG uses graphs for outline but not retrieval), temporal
      dynamics are ignored, hierarchy granularity is arbitrary, and
      no graph-aware multi-agent system exists. This subsection
      develops Thread 1 (rediscovering structure) and sets up the
      gaps that Section 5 analyzes.
    depth_level: "deep"

  - id: "s2.cross"
    section_number: "2 (end)"
    title: "Cross-Phase Comparison Table"
    depends_on: ["s2.5"]
    target_words: 300
    key_papers: []
    writing_instructions: >
      REQUIRED TABLE: Place this AFTER subsection 2.5 and BEFORE
      Section 3. Create a comprehensive cross-phase comparison table
      with columns: phase name, time period, representative systems,
      graph awareness level (gnn/hierarchical/bfs/embedding/none),
      iteration strategy (single_pass/multi_round/interactive),
      claimed performance metric + value, evaluation benchmark used,
      computational cost profile (datastore size, agent count, API
      calls per survey if available), and number of papers in the
      pool from that phase. This table is the single most important
      comparative artifact in the survey. Add a brief analysis
      paragraph after the table: note that cross-phase comparison is
      impossible due to benchmark proliferation (Thread 2), that
      graph awareness was lost and then regained (Thread 1), and that
      computational cost reporting is almost entirely absent.
    depth_level: "standard"

  - id: "s3.1"
    section_number: "3.1"
    title: "Graph-Enhanced Retrieval — Structure as Signal"
    depends_on: ["s2.cross"]
    target_words: 700
    key_papers:
      - "2409.12177"
      - "2510.07733"
      - "2004.07180"
      - "2410.03761"
      - "2605.14790"
    writing_instructions: >
      Deep-dive into graph-enhanced retrieval mechanisms. Explain
      LitFM's graph transformer architecture: how it processes
      attention over both text tokens and citation graph neighbors
      simultaneously, and why joint text+graph pretraining achieves
      +28.1% precision (first mentioned in §2.1 — cross-reference
      rather than re-stating in full). Explain SurveyG's hierarchical
      traversal algorithm — how horizontal traversal (breadth within
      a layer) differs from vertical traversal (depth across layers),
      and why 3 layers may oversimplify. Explain HiGTL's end-to-end
      taxonomy tree learning from citation structure + LLM
      verbalization. Include a mechanism comparison table: graph
      modeling approach, traversal strategy, pretraining objective,
      downstream task, performance metric, computational cost (GNN
      training vs. inference-only). Key question: What does graph
      structure encode that text cannot? Answer: intellectual
      lineage (forward/backward citation chains), role
      differentiation (foundational vs. frontier), community
      boundaries, and temporal evolution patterns. This subsection
      develops Thread 1 by establishing the structural side of the
      semantic–structural tension. METRIC DISCIPLINE: Cross-reference
      LitFM +28.1% to its first mention in §2.1 — do not re-state
      full benchmark context.
    depth_level: "deep"

  - id: "s3.2"
    section_number: "3.2"
    title: "Single-Agent Pipelines — Semantic Retrieval at Scale"
    depends_on: ["s3.1"]
    target_words: 700
    key_papers:
      - "2406.10252"
      - "2409.13740"
      - "2411.14199"
      - "2402.14207"
      - "2502.14776"
    writing_instructions: >
      Deep-dive into single-agent pipeline mechanisms. Explain
      AutoSurvey's outline-driven staging — how the structured outline
      guides section-level retrieval and generation. Explain
      PaperQA2's contradiction detection algorithm — how it identifies
      conflicting claims across citing-cited paper pairs and either
      resolves or presents both sides. Explain OpenScholar's 45M
      paper datastore and ScholarQABench — how retrieval infrastructure
      compensates for model size (8B beats GPT-4o by 5%). Include a
      comparison table: pipeline stages, retrieval mechanism, graph
      awareness level (most: none), iteration strategy, quality
      metric, computational cost (datastore size, embedding
      dimension, indexing cost). Key question: Does massive datastore
      scale compensate for graph blindness? The evidence is
      mixed — OpenScholar shows scale helps, but it cannot discover
      papers that use different terminology for the same concept
      (where citation graphs would help). This subsection develops
      Thread 1 from the semantic side and Thread 2 (evaluation
      diversity prevents direct comparison). Avoid re-stating metrics
      already in §2.2 — use cross-references.
    depth_level: "deep"

  - id: "s3.3"
    section_number: "3.3"
    title: "Multi-Agent Architectures — Division of Labor, Amplification of Gaps"
    depends_on: ["s3.2"]
    target_words: 700
    key_papers:
      - "2506.12689"
      - "2509.18661"
      - "2508.04306"
      - "2504.14822"
    writing_instructions: >
      Deep-dive into multi-agent mechanisms. Explain SciSage's
      reflect-while-writing in detail: the Writer pauses every N
      sentences, the Reflector evaluates against retrieved papers,
      and the Writer adjusts before continuing. Why this prevents
      error accumulation better than post-hoc revision. Explain
      Agentic AutoSurvey's shared task board protocol: how the
      Planner, Researcher, Writer, and Reviewer pass section drafts
      and feedback, enabling parallel work. Explain MATC's
      error-mitigation taskforces: Exploitation (search), Exploration
      (expand scope), Experience (historical patterns),
      Self-Correction (internal review). Include a comparison table:
      agent roles, coordination protocol, graph awareness level,
      citation F1/quality metric, number of agents, computational
      overhead (N agents × per-agent API calls × iteration depth).
      Critical analysis: The performance improvements are genuine but
      confounded — controlled ablation studies (same base LLM, same
      retrieval, same evaluation) are needed to attribute gains to
      multi-agent architecture rather than better prompts or larger
      models. This subsection develops Thread 3 (multi-agent
      coordination amplifies retrieval gaps) and Thread 4 (even
      the best multi-agent systems optimize surface quality, not
      critical depth). METRIC DISCIPLINE: Cross-reference SciSage
      +32% and Agentic AutoSurvey 8.18/10 to their first mention
      in §2.3 — do not re-state full context.
    depth_level: "deep"

  - id: "s3.4"
    section_number: "3.4"
    title: "The Bottleneck Transfer Problem — Retrieval Gains ≠ Survey Gains"
    depends_on: ["s3.3"]
    target_words: 600
    key_papers:
      - "2501.10120"
      - "2409.12177"
      - "2506.12689"
      - "2509.18661"
    writing_instructions: >
      Present the bottleneck transfer argument directly. Build a
      table showing for each system: (a) system name, (b) claimed
      metric with value, (c) baseline absolute value, (d) evaluation
      benchmark, (e) whether transfer to survey quality is tested.
      Include: PaSa +37.78% recall@20 (baseline: GPT-4o recall@20
      — specify absolute value), LitFM +28.1% precision (baseline:
      text-only embedding precision — specify absolute value),
      SciSage +32% citation F1 (baseline: single-agent citation F1
      — specify absolute value), Agentic AutoSurvey 8.18/10 vs
      4.77/10 (both are absolute values on a 10-point scale). Then
      ask: is there a monotonic relationship between retrieval metrics
      and survey quality? Answer: No study tests this. The field
      operates on an untested assumption. This gap is the bottleneck
      transfer problem. Explain why recall gains may not transfer:
      finding every relevant paper does not guarantee selecting the
      right papers for a coherent narrative, and survey quality
      depends on exclusion as well as inclusion. Add a cost–efficiency
      argument: expensive retrieval gains (massive datastores, RL
      training) may not be worth marginal survey quality improvements.
      This subsection crystallizes Thread 3 as a falsifiable hypothesis
      and a call for end-to-end evaluation linking retrieval to
      survey quality. METRIC HANDLING: This subsection may SYNTHESIZE
      metrics from multiple sources (§2.1, §2.3, §2.4) in a dedicated
      table. This is the EXCEPTION to the "first-mention-only" rule.
    depth_level: "deep"

  - id: "s4.1"
    section_number: "4.1"
    title: "Human-in-the-Loop Systems — Quality Through Oversight"
    depends_on: ["s3.4"]
    target_words: 550
    key_papers:
      - "2504.14822"
      - "2006.12166"
      - "2505.23789"
      - "1705.05420"
    writing_instructions: >
      Describe HITL mechanisms. Explain InsightAgent's human
      orchestrator interacting with 6 agents — how the human provides
      strategic direction while agents execute retrieval, writing,
      and review. Explain ASReview's active learning stopping
      criterion (when to stop screening based on estimated recall).
      Explain LitChat's conversational KG construction. Include a
      comparison table: human role, automation level, time
      reduction, quality improvement, human effort hours. Critical
      analysis: HITL achieves the highest quality (InsightAgent:
      +27.2%) but at the cost of human effort — the scalability
      question is whether the human-in-the-loop bottleneck can be
      automated without quality loss. This subsection develops
      Thread 3 (human oversight addresses the bottleneck but creates
      a new one) and Thread 4 (even human-guided systems do not
      measure critical-analytic depth). Set up data for §4.4 cross-
      approach synthesis — note scalability limits, quality ceiling,
      and cost profile.
    depth_level: "deep"

  - id: "s4.2"
    section_number: "4.2"
    title: "Procedural Iterative Refinement — Self-Evaluation Loops"
    depends_on: ["s4.1"]
    target_words: 450
    key_papers:
      - "2510.21900"
      - "2508.14317"
      - "2508.17647"
    writing_instructions: >
      Describe procedural self-evaluation. Explain IterSurvey's
      recurrent outline generation — how the outline is updated as
      content is generated, using the LLM's own evaluation of
      coverage gaps. Explain SurveyGen-I's adaptive planning and
      memory mechanism. Include a comparison table: refinement
      strategy, self-evaluation method, number of iterations,
      convergence criteria, computational cost (token overhead per
      iteration). Critical analysis: Self-evaluation relies on the
      LLM to detect its own errors — known biases (overconfidence,
      failure to detect subtle errors) are not addressed. The
      refinement loop may converge to a locally optimal but globally
      flawed survey. This subsection develops Thread 4
      (self-evaluation measures surface quality, not critical depth).
      Set up data for §4.4 cross-approach synthesis.
    depth_level: "deep"

  - id: "s4.3"
    section_number: "4.3"
    title: "Reinforcement Learning for Search and Generation Policies"
    depends_on: ["s4.2"]
    target_words: 550
    key_papers:
      - "2501.10120"
      - "2510.27126"
      - "2505.22338"
      - "2305.08844"
    writing_instructions: >
      Describe RL-based approaches. Explain PaSa's RL training
      setup: action space (citation-follow, keyword-refine,
      author-search), reward function (recall@k), epsilon-greedy
      exploration, synthetic trajectory generation. Explain AURA's
      epsilon-greedy adaptation for conversational surveys and the
      LSDE metric. Include a comparison table: RL algorithm,
      action space, reward function, training data source, task
      (search vs. generation vs. revision), training compute cost.
      Critical analysis: RL systems optimize narrowly defined
      objectives (recall, information gain, self-evaluation
      criteria) that may not correlate with holistic survey quality.
      The field needs a training signal that captures higher-order
      qualities (critical insight, research gap identification).
      This subsection develops Thread 3 (RL optimizes the wrong
      objective for survey quality) and Thread 4 (no RL objective
      captures critical-analytic depth). Set up data for §4.4
      cross-approach synthesis.
    depth_level: "deep"

  - id: "s4.4"
    section_number: "4.4"
    title: "Cross-Approach Synthesis — Scalability, Quality Ceiling, and Cost"
    depends_on: ["s4.3"]
    target_words: 500
    key_papers:
      - "2504.14822"
      - "2510.21900"
      - "2501.10120"
      - "2510.27126"
      - "2504.04193"
    writing_instructions: >
      NEW subsection. Provide a cross-approach comparison table
      comparing HITL (§4.1), procedural iteration (§4.2), and RL
      (§4.3) on: scalability (papers per unit time — HITL lowest,
      RL highest), quality ceiling (maximum achievable score —
      HITL highest, procedural middle, RL narrows based on
      objective), cost per survey (human hours, token budget, API
      calls), citation accuracy, graph awareness. Key finding: HITL
      achieves the highest quality (InsightAgent: +27.2%) but does
      not scale beyond expert-curated reviews; RL achieves the best
      scalability (PaSa's RL policy can be deployed at scale) but
      optimizes narrow recall/information-gain objectives;
      procedural iteration sits between. Critical gap: No approach
      measures or optimizes critical-analytic depth — all three
      optimize for surface quality dimensions (coherence, coverage,
      citation accuracy) that existing benchmarks capture. This
      subsection develops Thread 3 (each approach addresses a
      different part of the bottleneck, but none bridges the
      retrieval→survey quality gap) and Thread 4 (surface quality
      optimization is universal regardless of approach). End with
      a table summarizing the comparison and a recommendation for
      which deployment scenarios each approach suits.
    depth_level: "deep"

  - id: "s5.1"
    section_number: "5.1"
    title: "Claim vs. Evidence Gap Analysis"
    depends_on: ["s4.4"]
    target_words: 600
    key_papers:
      - "2406.10252"
      - "2409.13740"
      - "2411.14199"
      - "2506.12689"
      - "2509.18661"
      - "2501.10120"
      - "2510.07733"
    writing_instructions: >
      Present the Claim vs. Evidence Gap table from the evolution
      narrative in extended form. For each of 7 major claims:
      (1) AutoSurvey "human-competitive", (2) PaperQA2 "superhuman",
      (3) OpenScholar "8B beats GPT-4o", (4) multi-agent
      "dramatically better", (5) graph-aware "28.1% precision",
      (6) PaSa "37.78% recall", (7) SurveyG "better organization",
      provide: the exact claim, the supporting evidence, and a
      critical assessment that identifies the gap. Include a
      summary table with columns: Claim, Paper, Evidence Type,
      Gap. METRIC DISCIPLINE: Cross-reference metrics to their
      first-introduction section. E.g., "SciSage's +32% citation
      F1 improvement (§2.3)" rather than re-stating full benchmark
      context. Use "nearly every system" language (PaperQA/PaperQA2
      share LitQA/LitQA2). This subsection is the evidence
      foundation for the entire critical assessment.
    depth_level: "deep"

  - id: "s5.2"
    section_number: "5.2"
    title: "Methodological Weaknesses Across All Phases"
    depends_on: ["s5.1"]
    target_words: 550
    key_papers:
      - "2602.11238"
      - "2301.13298"
      - "2508.11310"
      - "2510.17853"
    writing_instructions: >
      Diagnose 6 cross-cutting weaknesses: (1) no shared evaluation
      benchmark, (2) non-standardized human evaluation, (3)
      unvalidated LLM-as-judge, (4) missing ablation studies, (5)
      unaudited citation hallucination rates, and (6) incomparable
      computational cost reporting (no standardized way to compare
      token budgets, API costs, or total compute across systems,
      making practical deployment comparisons impossible). For each
      weakness, provide specific examples from papers, explain the
      consequences for the field, and cite the most relevant
      reference. This subsection establishes that the field's
      methodological infrastructure is insufficient for reliable
      progress tracking.
    depth_level: "standard"

  - id: "s5.3"
    section_number: "5.3"
    title: "The Evaluation Comparability Crisis"
    depends_on: ["s5.2"]
    target_words: 600
    key_papers:
      - "2510.03120"
      - "2508.15658"
      - "2601.15307"
      - "2602.11238"
      - "2512.02763"
      - "2508.11310"
    writing_instructions: >
      Enumerate and map all 11+ evaluation benchmarks (LitQA,
      LitQA2, ScholarQABench, SurveyScope, Survey-Arena,
      SurveyBench, SurGE, ReportBench, DeepSurvey-Bench,
      SurveyLens, SGSimEval, SurveyEval). For each: task type
      (factual QA vs. survey generation vs. citation accuracy),
      metric type (F1, quality score, recall, precision), dataset
      size, evaluation protocol (human vs. automatic). Show that
      they are incommensurable — no mathematical framework exists
      to compare results across benchmarks. Include a visual
      summary table. Mention that PaperQA/PaperQA2 share
      LitQA/LitQA2, making them the exception. Propose a
      resolution path: shared benchmark across disciplines,
      standardized human evaluation rubric with validated
      inter-annotator agreement, core automatic metrics benchmarked
      against human judgment, and a leaderboard. This subsection
      is the analytical core of Thread 2.
    depth_level: "deep"

  - id: "s5.4"
    section_number: "5.4"
    title: "Blind Spots — What the Field Is Not Looking At"
    depends_on: ["s5.3"]
    target_words: 600
    key_papers:
      - "2305.18554"
      - "2402.12046"
      - "2401.03545"
      - "2411.05584"
      - "2508.12735"
      - "2509.04190"
    writing_instructions: >
      Identify 5 blind spots: (1) critical-analytic depth — no system
      measures whether surveys provide original analysis, identify
      contradictions, or propose new taxonomies; (2) citation
      hallucination rates — no paper reports a systematic audit of
      hallucinated citations; (3) domain transferability — almost
      all systems evaluated on CS/AI papers, untested on medicine,
      physics, humanities; (4) temporal recency bias — automated
      systems retrieve primarily recent papers, potentially
      amplifying the 62% <5-year citation pattern; (5) prestige/status
      bias (Matthew Effect) — graph traversal preferentially
      discovers well-cited papers, potentially missing high-quality
      work from less-established venues. For each blind spot,
      provide evidence, explain the consequence, and note any
      paper that partially addresses it. This subsection develops
      Thread 4 as the field's most consequential omission.
    depth_level: "deep"

  - id: "s5.5"
    section_number: "5.5"
    title: "The Root Cause — You Cannot Optimize for What You Do Not Measure"
    depends_on: ["s5.4"]
    target_words: 400
    key_papers:
      - "2601.15307"
      - "2508.11310"
    writing_instructions: >
      Argue that the evaluation comparability crisis (Thread 2)
      and the critical-analytic blind spot (Thread 4) are causally
      linked. Because no benchmark measures critical-analytic
      depth, the field optimizes for what it can measure (coherence,
      coverage, citation accuracy) and has no incentive to address
      what it cannot measure. This is not a coordination problem
      but an epistemic gap: the field has not defined what "good"
      means for automated surveys. Propose that defining and
      measuring critical-analytic depth is the single most
      important missing capability. Reference DeepSurvey-Bench's
      "academic value" dimension and SGSimEval's three-dimension
      evaluation as partial steps. Add that standardized cost
      reporting is a second critical missing capability — without it,
      practical deployment comparisons are impossible. Conclude that
      without these definitions, architectural innovation will
      continue to produce incommensurable results that may improve
      surface quality without advancing scholarly value.
    depth_level: "standard"

  - id: "s6.1"
    section_number: "6.1"
    title: "Deep Graph-LLM Integration — Towards a Unified Architecture"
    depends_on: ["s5.5"]
    target_words: 600
    key_papers:
      - "2510.07733"
      - "2409.12177"
      - "2501.10120"
      - "2506.12689"
      - "2510.17853"
      - "2605.14790"
    writing_instructions: >
      Sketch the architecture of a unified system where: (a) LitFM
      serves as the structure-aware retrieval backbone (joint
      text+graph embeddings for paper retrieval), (b) SurveyG
      provides the organizational scaffold (hierarchical graph
      traversal to structure the survey outline), (c) PaSa's RL
      policy handles learned traversal decisions (when to follow
      citations vs. search semantically, how deep to traverse),
      (d) SciSage's reflect-while-writing multi-agent framework
      handles generation (Searcher uses LitFM + PaSa policy, Writer
      generates sections structured by SurveyG's hierarchy,
      Reflector validates claims against graph structure), and
      (e) CiteGuard validates each citation against the source
      paper. Explain how each component would interact and what
      research challenges remain (e.g., how to train a unified
      RL policy that optimizes for survey quality, not just
      retrieval recall). Discuss cost–quality trade-offs: what
      does adding each component cost in tokens, API calls, and
      latency? Include a table showing estimated computational
      cost of each component. This subsection develops Thread 1 by
      proposing a resolution to the semantic–structural tension.
    depth_level: "deep"

  - id: "s6.2"
    section_number: "6.2"
    title: "Learned Traversal Policies for Hierarchical Graphs"
    depends_on: ["s6.1"]
    target_words: 450
    key_papers:
      - "2501.10120"
      - "2408.15371"
      - "2605.14790"
    writing_instructions: >
      Propose extending PaSa's RL policy to hierarchical graph
      traversal. Current traversal is fixed (SurveyG's 3 layers,
      fixed depth). The next step is learned policies that
      determine: how many hops in each direction (forward vs.
      backward citations), when to switch from graph traversal to
      semantic search, how to prioritize breadth vs. depth per
      section type (Introduction requires broader coverage than
      Methods), and when to stop traversing. Reference PaSa's
      action space as a template and Temporal GNN (2408.15371)
      for dynamic graph updating. This subsection develops Thread 1
      (making graph traversal adaptive) and Thread 3 (aligning
      traversal objectives with survey quality).
    depth_level: "standard"

  - id: "s6.3"
    section_number: "6.3"
    title: "A Reimagined Evaluation Framework"
    depends_on: ["s6.2"]
    target_words: 500
    key_papers:
      - "2601.15307"
      - "2508.11310"
      - "2602.11238"
      - "2508.15658"
      - "2510.17853"
    writing_instructions: >
      Propose 5 evaluation dimensions beyond surface quality:
      (a) critical-analytic depth (does the survey identify
      contradictions, gaps, opportunities? — measured by human
      evaluation with a standardized rubric), (b) bias awareness
      (does the survey acknowledge limitations of its retrieval
      and selection process? — measured by temporal coverage,
      citation concentration, venue/author diversity), (c) field-
      situatedness (does the survey correctly identify which
      questions are settled and which are contested? — measured by
      expert annotation), (d) citation hallucination audit
      (systematic verification of each citation against source
      paper — measured by CiteGuard-style validation), and (e)
      standardized cost reporting (token budget, API calls, total
      compute per survey). For each dimension: definition,
      measurement protocol, validation strategy. This subsection
      develops Thread 4 by proposing concrete ways to measure what
      currently cannot be measured.
    depth_level: "deep"

  - id: "s6.prio"
    section_number: "6 (intro)"
    title: "Prioritization of Proposals"
    depends_on: ["s5.5"]
    target_words: 150
    key_papers: []
    writing_instructions: >
      After introducing Section 6 but before subsection 6.1,
      include a brief prioritization comparison table. Columns:
      proposal, impact (H/M/L), feasibility (H/M/L), key reference.
      Rows: deep graph-LLM integration (§6.1), learned traversal
      (§6.2), reimagined evaluation (§6.3), community benchmarking
      (§6.4). Note: Reimagined evaluation has the highest impact
      (solves the measurement crisis) but lowest feasibility
      (requires community consensus). Graph-LLM integration has
      medium impact and medium feasibility (building blocks exist
      but integration challenges remain). Learned traversal has
      medium impact and high feasibility (PaSa already
      demonstrates the approach). Community benchmarking has the
      lowest feasibility (requires coordination) but highest
      long-term impact. This helps readers prioritize which
      direction to invest in.
    depth_level: "minimal"

  - id: "s6.4"
    section_number: "6.4"
    title: "The Path to Community-Wide Benchmarking"
    depends_on: ["s6.3"]
    target_words: 350
    key_papers:
      - "2602.11238"
      - "2601.15307"
      - "2508.11310"
    writing_instructions: >
      Call for: shared benchmark across disciplines (modeled on
      SurveyLens's 10-discipline design), standardized human
      evaluation rubric (modeled on SurGE's 4-dimension protocol
      and DeepSurvey-Bench's academic value dimension), leaderboard
      where all systems evaluated under identical conditions,
      standardized cost reporting as a required submission field.
      Reference how other fields resolved similar crises (e.g.,
      GLUE/SuperGLUE for NLP). This is the final synthesis of
      Threads 2 and 4: the community must build the measurement
      infrastructure before architectural innovation can be
      reliably evaluated.
    depth_level: "standard"

  - id: "s7"
    section_number: "7"
    title: "Conclusion"
    depends_on: ["s6.4"]
    target_words: 400
    key_papers: []
    writing_instructions: >
      Write an expanded conclusion (400 words, up from ~7 lines
      in the draft). Structure: (a) 2-3 sentences recapping the
      evolution arc — the field traced a curious arc from graphs
      to semantics back to graphs, but integration remains shallow.
      (b) 1-2 sentences per narrative thread, synthesizing how
      they interact: the semantic–structural tension (Thread 1)
      drives architectural design; the evaluation comparability
      crisis (Thread 2) makes progress unmeasurable; the bottleneck
      transfer problem (Thread 3) means component improvements may
      not yield survey gains; the critical-analytic blind spot
      (Thread 4) means the field optimizes surface quality while
      ignoring scholarly value. Show that Threads 2 and 4 are
      causally linked. (c) Concrete vision: the field in 2 years
      would have a shared leaderboard with standardized cost
      reporting, a graph-aware multi-agent system generating
      surveys that identify research gaps rather than restructure
      known content, and evaluation rubrics that distinguish
      surface quality from critical-analytic depth. (d) Closing
      statement: the next breakthrough will come not from a better
      pipeline but from the evaluation infrastructure that makes
      pipeline comparison meaningful, and from genuine integration
      of graph-aware retrieval with multi-agent coordination.
    depth_level: "standard"
