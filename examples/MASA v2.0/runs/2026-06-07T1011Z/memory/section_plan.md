subsections:
  - id: "s1"
    section_number: "1"
    title: "Introduction"
    depends_on: []
    target_words: 400
    key_papers: ["2406.10252", "2402.08565", "2501.04306"]
    writing_instructions: >-
      Write a concise introduction covering: (a) the promise of automated survey generation for accelerating scientific synthesis; (b) the field's rapid evolution from 2020–2026; (c) the central tension — architectural innovation has outpaced evaluation infrastructure; (d) the four narrative threads: evaluation comparability crisis, automation–control tension, citation graph shallowness, depth–breadth trade-off; (e) the paper roadmap. Define "automated survey generation" as end-to-end systems producing structured multi-section literature surveys from a topic query. Scope out single-document summarization, non-scientific domains, pure citation analysis without generation. This subsection develops Thread 1 and Thread 2 by setting up the evaluation crisis and the automation–control framing.
    depth_level: "standard"

  - id: "s2.1"
    section_number: "2.1"
    title: "Core Enabling Methods — RAG and Retrieval Paradigm"
    depends_on: ["s1"]
    target_words: 400
    key_papers: ["2005.11401", "2004.14974", "2005.11401"]
    writing_instructions: >-
      Describe the foundational RAG paradigm (Lewis et al., 2005.11401): the retrieve-then-generate architecture combining parametric and non-parametric memory. Explain the mechanism: a retriever finds relevant passages from a large corpus, then a generator produces text grounded in those passages. Highlight that this was designed for short-form QA, not survey generation, and the paradigm's assumption — that single-pass retrieval provides sufficient context — does not hold for multi-topic surveys. Include RAG's reported performance on QA benchmarks. This subsection lays groundwork for Thread 3 (citation shallowness) by noting that RAG retrieves at the passage level, not the citation graph level.
    depth_level: "standard"

  - id: "s2.2"
    section_number: "2.2"
    title: "Datasets and Evaluation Infrastructure"
    depends_on: ["s2.1"]
    target_words: 400
    key_papers: ["2004.14974", "2010.14235", "2305.15186", "2305.14251", "2004.15011", "2104.06486"]
    writing_instructions: >-
      Survey the key datasets that established evaluation infrastructure: SciFact (claim verification paradigm, SUPPORT/REFUTE/NEI), Multi-XScience (multi-document related-work generation), SciReviewGen (10K reviews, 690K cited papers), FActScore (atomic fact decomposition), SciTLDR (extreme summarization), MS² (contradiction-aware medical summarization). For each, describe: (a) what it measures; (b) its scale; (c) how subsequent survey systems use it. Note the limitation: all these datasets address short-form tasks (claim verification, related-work, one-sentence summarization) — none provide evaluation infrastructure for full multi-section surveys. This subsection develops Thread 1 by showing that the evaluation crisis has deep roots in the foundation phase.
    depth_level: "standard"

  - id: "s2.3"
    section_number: "2.3"
    title: "Limitations That Drove the Transition"
    depends_on: ["s2.2"]
    target_words: 250
    key_papers: ["2406.10252", "2005.11401"]
    writing_instructions: >-
      Synthesize the critical transition from Phase 1 to Phase 2. Phase 1 established the building blocks (RAG, datasets, factuality metrics) but addressed short-form tasks only. No system could produce a coherent multi-section survey. The transition was driven by the recognition that LLMs had sufficient reasoning capacity to attempt this larger task — but Phase 1 methods provided no direct pathway. A fundamentally new decomposition strategy (stage-based pipeline) was required. Emphasize that this gap is not a failure of Phase 1 but a phase boundary: short-form methods do not trivially scale to long-form surveys.
    depth_level: "minimal"

  - id: "s3.1"
    section_number: "3.1"
    title: "The Stage-Decomposition Template — AutoSurvey and Its Contemporaries"
    depends_on: ["s2.3"]
    target_words: 600
    key_papers: ["2406.10252", "2502.14776", "2402.01788", "2408.07884", "2504.18496"]
    writing_instructions: >-
      Deep dive into the single-agent pipeline template. Start with AutoSurvey (2406.10252) as the archetype: describe the five-stage pipeline (retrieval → outline → subsection drafting → integration → evaluation), emphasizing that a single LLM (GPT-4) drives all stages via careful prompt engineering. Explain HOW each stage works — e.g., retrieval is performed once upfront using keyword + embedding search on arXiv, outline is generated based on retrieved paper set, each subsection is drafted independently. Then contrast with SurveyX (AttributeTree preprocessing — multi-level decomposition before retrieval), LitLLM (modular RAG toolkit for related-work), Instruct LLMs Step by Step (prompt decomposition), DimInd (multi-level compression). Include a comparison table with columns: preprocessing strategy, retrieval method, generation scope, iteration strategy, evaluation approach. Explain the mechanisms — e.g., how AttributeTree enables per-attribute retrieval vs AutoSurvey's flat retrieval. This subsection develops Thread 2 (automation–control) by showing the single-agent pipeline as the highest-automation, lowest-control point in the design space.
    depth_level: "deep"

  - id: "s3.2"
    section_number: "3.2"
    title: "Training Paradigms — Fine-Tuned vs. Zero-Shot"
    depends_on: ["s3.1"]
    target_words: 500
    key_papers: ["2411.14199", "2504.00824", "2409.13740", "2311.12315"]
    writing_instructions: >-
      Compare the two training paradigms in Phase 2. OpenScholar (2411.14199): specialized retriever trained on 45M papers with citation-supervised fine-tuning; ScholarCopilot (2504.00824): fine-tuned model with retrieval-token-gated architecture; AcademicGPT (2311.12315): domain-specific fine-tuning. Contrast with zero-shot approaches (AutoSurvey, SurveyX) that rely entirely on prompt engineering without fine-tuning. Include a comparison table: training data, parameter count (if known), retrieval approach, citation accuracy improvement over baselines. Explain the mechanism of citation-supervised fine-tuning (how OpenScholar uses citation relationships as training signals). Note the tension: fine-tuning improves domain performance but requires significant compute and data; zero-shot approaches are more portable but less accurate. PaperQA2 (2409.13740) as a bridge — superhuman QA without fine-tuning but with multi-step verification. This subsection develops Thread 3 (citation shallowness) by noting that even fine-tuned systems do not perform citation graph reasoning.
    depth_level: "deep"

  - id: "s3.3"
    section_number: "3.3"
    title: "Phase 2 Assessment — Genuine Advances and the Ceiling"
    depends_on: ["s3.2"]
    target_words: 300
    key_papers: ["2406.10252", "2409.13740", "2407.01796"]
    writing_instructions: >-
      Assess what Phase 2 genuinely achieved: first demonstration of coherent multi-section surveys (>5,000 words), the stage-decomposition template that all later systems inherit, PaperQA2's superhuman QA proving LLMs can exceed human expert performance on literature synthesis tasks. Then establish the ceiling: no iterative refinement (retrieval gaps discovered during writing cannot be addressed), no citation graph awareness (papers discovered via keyword/embedding search only), single LLM for all stages (cannot simultaneously optimize retrieval, organization, citation accuracy, and critical synthesis). Transition to Phase 3: these limitations drove two distinct responses — multi-agent architectures (specializing different LLMs) and graph-enhanced retrieval (using citation structure for paper discovery).
    depth_level: "standard"

  - id: "s4.1"
    section_number: "4.1"
    title: "Multi-Agent Architectures — Dividing the Labour"
    depends_on: ["s3.3"]
    target_words: 700
    key_papers: ["2511.17689", "2506.12689", "2509.18661", "2510.26012", "2410.21784", "2509.20175", "2603.03005", "2603.13327"]
    writing_instructions: >-
      Deep dive into multi-agent systems. Define the pattern: specialized LLM agents (Planner, Retriever, Writer, Evaluator, Reviewer) with distinct prompts and coordination mechanisms. Compare ARISE (5 agents: Planning, Retrieval, Drafting, Evaluation, Revision — rubric-guided iterative loop), SciSage (4 agents: Planner, Retriever, Writer, hierarchical Reflector — reflect-when-you-write), Agentic AutoSurvey (4 agents: Orchestrator, Search, Writing, Review — 12-dimensional rubric), AutoSurvey2 (multiple parallel LLM instances with centralized integration and real-time re-retrieval during refinement — parallel section generation solves the single-pass retrieval gap of its predecessor), MARCO (5+ agents with chat-based Conversation Manager), Federation of Agents (semantics-aware federated collaboration), OrchMAS (domain-specialized experts with Coordinator), DOVA (deliberation-first with perspective agents). Include a comparison table with columns: agent count, coordination pattern, iteration strategy, citation graph awareness, key innovation, evaluation approach. For each system, explain the mechanism — e.g., HOW the rubric-guided loop works in ARISE (each agent references rubric dimensions, Evaluation scores, Revision revises to improve scores, loop continues until threshold met). Critically assess: ARISE's 92.48 score is a system-defined number with no human calibration (Thread 1); all agents typically use the same underlying LLM — specialization is prompt-deep only (discuss what evidence would confirm or falsify this: do agents with the same base model but different prompts produce measurably different retrieval choices? A controlled experiment comparing agent outputs under identical base-model conditions with varying prompts would settle the question); DOVA's deliberation quality is unmeasured (Thread 2 — more autonomy, less transparency); AutoSurvey2's homogeneous agents lack the role specialization that distinguishes genuine multi-agent systems — it is conceptually multi-agent but practically a parallel dispatch architecture.
    depth_level: "deep"

  - id: "s4.2"
    section_number: "4.2"
    title: "Graph-Enhanced Retrieval — Beyond Keyword Search"
    depends_on: ["s4.1"]
    target_words: 600
    key_papers: ["2503.04629", "2510.07733", "2508.17647", "2510.26750", "2312.09948"]
    writing_instructions: >-
      Deep dive into graph-enhanced retrieval systems. Define the pattern: citation graph traversal is a core retrieval component, not an add-on. Compare SurveyForge (bidirectional BFS chaining + cross-section memory buffer), SurveyG (three-tier hierarchical graph: Foundation/Development/Frontier), SurveyGen (quality-aware RAG with citation graph traversal + quality estimation), ProfOlaf (explicit snowballing following SLR protocols), GEAR-Up (knowledge graph for query expansion). Include a comparison table: graph traversal strategy (BFS vs hierarchical vs KG), graph role (retrieval expansion vs outline organization), agent count, iteration strategy, evaluation approach. Explain the mechanisms — e.g., HOW SurveyG's three-tier graph works (backward chaining → Foundation + Development tiers, forward chaining → Frontier tier; tiers directly inform survey outline). Critically assess: all graph traversal is single-hop BFS — no multi-hop reasoning (Thread 3); hierarchical tiering is citation-position-based, not intellectual-lineage-based; claims of "improved coverage" lack comparison against strong baselines like ARISE or SciSage (Thread 1).
    depth_level: "deep"

  - id: "s4.3"
    section_number: "4.3"
    title: "Human-in-the-Loop and Iterative Refinement"
    depends_on: ["s4.2"]
    target_words: 500
    key_papers: ["2504.08762", "2410.15978", "2309.01684", "2407.01796", "2510.21900", "2508.14317"]
    writing_instructions: >-
      Cover the two non-fully-automated patterns. (1) Hybrid interactive: InteractiveSurvey (three-stage user interaction: reference categorization, outline refinement, draft review), PROMPTHEUS (PRISMA-compliant with human validation at each stage), CRUISE-Screening (living review with NLP screening). (2) Iterative refinement: ReClaim (per-sentence interleaved retrieve-generate-verify), IterSurvey (recurrent outline refinement with paper cards). Include a comparison table: interaction type (human decisions vs quality-driven iteration), granularity (per-section vs per-sentence), scope (full survey vs QA/summarization), citation graph awareness. Explain mechanisms — e.g., HOW InteractiveSurvey's reference categorization works (user assigns categories like "foundational" / "methodological" / "related but not central" → system adjusts emphasis), HOW IterSurvey's paper cards work (each citation gets a structured card with title, claims, method, findings → cards stored and reused for consistent attribution). Quantify ReClaim's computational cost to anchor Thread 4 (Depth–Breadth Trade-Off): per-sentence verification requires ~10 API calls per sentence (retrieve → generate → verify cycle); scaling to a 100-sentence survey would require ~1,000+ API calls, making full-survey deployment prohibitive at current pricing and latency. Develop Thread 2: these systems resolve the automation–control tension by preserving human oversight, but sacrifice scalability. Note that ReClaim's sentence-level verification does not scale to full surveys (Thread 4 depth–breadth trade-off).
    depth_level: "deep"

  - id: "s4.4"
    section_number: "4.4"
    title: "The Evaluation Benchmark Explosion"
    depends_on: ["s4.3"]
    target_words: 500
    key_papers: ["2510.03120", "2508.15658", "2508.11310", "2512.02763", "2602.11238", "2601.15307", "2601.14949", "2412.15249"]
    writing_instructions: >-
      Survey the 8+ evaluation benchmarks introduced in 2025–2026. Compare SurveyBench (11K+ papers, 4.9K+ surveys, quiz-driven evaluation), SurveyEval (cross-subject evaluation), SurveyScope (46 papers, 11 CS domains, Citation F1), Survey-Arena (pairwise comparison), SurGE (survey generation evaluation), SGSimEval (similarity evaluation), SurveyLens (discipline-aware adaptation), DeepSurvey-Bench (academic depth), CiteRAG (citation prediction). Include a comparison table: scale, metric types, discipline coverage, reference survey availability, quiz-based vs rubric-based vs pairwise. Diagnose the crisis: metric fragmentation (ROUGE, BLEU, FActScore, Citation F1, rubric scores, win-rate, quiz answerability), reference survey fragmentation (no shared corpus across evaluations), human evaluation inconsistency (no standardized guidelines). This subsection IS the central development of Thread 1 — it shows that benchmark proliferation has deepened rather than solved the evaluation crisis. Conclude: no benchmark has achieved consensus adoption; every system reports on a different combination; cross-comparison is impossible.
    depth_level: "deep"

  - id: "s5"
    section_number: "5"
    title: "Current Frontier — Deliberation, Domain Expertise, and Local Deployment (2026)"
    depends_on: ["s4.4"]
    target_words: 700
    key_papers: ["2603.13327", "2603.03005", "2603.14629"]
    writing_instructions: >-
      Deep dive into the 2026 frontier. Describe each system with concrete mechanism detail and available quantitative results:
    
    (1) DOVA (deliberation-first): Configurable number of perspective agents (at least 4: empiricist, theoretician, methodologist, domain specialist) deliberate on the research topic before any retrieval occurs. Mechanism: deliberation phase involves structured discussion to identify key questions, potential approaches, and information needs → deliberation output guides targeted keyword queries → agents collaborate to synthesize based on retrieved papers. Single-pass — no iterative refinement after retrieval. Critical caveat: deliberation quality is entirely unmeasured — there is no metric to determine whether productive deliberation has occurred. Cross-reference: this echoes the rubric-quality problem from §4.1 — both systems make unvalidated claims about intermediate process quality.
    
    (2) OrchMAS (domain-expert agents): Coordinator Agent analyzes the research question and creates a work plan. N Expert Agents, each pre-configured with domain-specific knowledge profiles (e.g., ML, statistics, neuroscience), are assigned tasks matching their expertise. Mechanism: experts retrieve papers using domain-specific configurations → analyze findings → produce structured summaries → Coordinator integrates outputs, resolving conflicts and identifying consensus. Critical caveat: requires upfront domain expertise configuration — how this scales to new domains, and whether the expertise is genuinely different from what a general-purpose LLM already knows, is not addressed.
    
    (3) ResearchPilot (local-first): 4 agents (Retriever, Analyst, Writer, Reviewer) run on consumer hardware using quantized local LLMs (e.g., Llama, Mistral — typical 4-bit or 8-bit quantization). Mechanism: local embedding search over a manually curated paper index → Analyst extracts structured info → Writer synthesizes → Reviewer evaluates. Reported results: quality acknowledged lower than cloud-based frontier models; near-zero cost (no API fees). Critical caveat: no citation graph awareness; local paper index requires manual curation.
    
    Include a comparison table against strongest Phase 3 baselines (ARISE, SciSage): approach, agent count, retrieval method, citation graph awareness, iteration strategy, deployment profile (cloud vs local), key innovation, reported quality (with caveats). Critically assess: NONE of the frontier systems address citation graph reasoning beyond what Phase 3 did (Thread 3); DOVA's deliberation quality is unmeasured (Thread 2 — more autonomy, less transparency); OrchMAS's domain expertise may not differ from general-purpose LLM knowledge; ResearchPilot trades quality for privacy. This section develops Thread 2 by showing how deliberation-first increases autonomy without increasing transparency.
    depth_level: "deep"

  - id: "s6"
    section_number: "6"
    title: "Critical Assessment — Claims, Gaps, and Blind Spots"
    depends_on: ["s5"]
    target_words: 900
    key_papers: ["2406.10252", "2511.17689", "2409.13740", "2506.12689", "2510.07733", "2503.04629", "2510.03120"]
    writing_instructions: >-
      This is the analytical core. Structure as four sub-parts. (a) Claim vs. Evidence Gap — Present the table from evolution_narrative.md with 7 key claims (AutoSurvey human-competitive, ARISE 92.48 score, PaperQA2 superhuman, SciSage SOTA, multi-agent > single-agent, graph traversal improves coverage, field maturity for practical use) with supporting evidence and critical assessment. Include a "Claim Source" column citing the specific paper and section for each claim to improve traceability. For each claim, explain WHY the evidence is insufficient — e.g., ROUGE-L measures n-gram overlap not analytical depth; self-evaluation numbers have no human calibration. (b) Methodological Weaknesses — 6 persistent weaknesses: self-evaluation epidemic, small topical samples (provide aggregated field-wide statistics: compute mean/median/max evaluation scope across the 12 representative systems — e.g., mean ~20 topics, median ~15 topics, max ~100 topics — to move from anecdotal examples to a field-wide claim), single-discipline bias (CS dominance), no standardized benchmark, superficial citation verification (post-hoc NLI, paper-level not claim-level), no measure of analytical depth. (c) Evaluation Comparability Crisis — Metric fragmentation, reference survey fragmentation, human evaluation inconsistency, benchmark proliferation. (d) Blind Spots — 7 blind spots: multi-hop citation reasoning, temporal analysis of research trajectories, methodological quality assessment, figure/table generation, cross-lingual surveys, longitudinal trustworthiness, user trust and calibration. Note: soften "77 surveyed papers" to "most surveyed papers" to avoid an unverifiable precise number. This section converges all four narrative threads. Thread 1 is the organizing framework. Thread 3 gets its full exposition. Thread 4 is exposed in the depth–breadth trade-off. Thread 2 is developed in the blind spots (user trust).
    depth_level: "deep"

  - id: "s7"
    section_number: "7"
    title: "Future Directions — Toward Next-Generation Survey Generation"
    depends_on: ["s6"]
    target_words: 600
    key_papers: ["2603.13327", "2603.03005", "2510.03120", "2602.11238", "2601.15307"]
    writing_instructions: >-
      For each of 6 future directions (non-textual content merged into analytical synthesis), state the gap it addresses (from Section 6), describe what a solution would look like in concrete architectural terms, and identify preliminary work: (1) Unified evaluation protocol — shared corpus of 500+ topics across disciplines, multi-dimensional metric suite, independent third-party evaluation; (2) Multi-hop citation reasoning — citation graph traversal beyond BFS with relevance filtering per hop, claim-level provenance tracking; (3) Analytical synthesis (incorporating non-textual content) — claim extraction and cross-paper comparison, methodological feature matching, confidence tracking, PLUS automated generation of comparison tables, methodology taxonomies, and evidence maps as structured outputs serving analytical synthesis; (4) Domain-adaptive expertise — discipline-specific retrieval configurations, prompt templates, quality rubrics; (5) Temporal grounding — change detection, selective re-generation, version tracking; (6) Calibrated uncertainty — confidence expressions, weak-evidence flagging, contested-finding signaling.
    Add cross-cutting trade-off analysis: explore tensions between directions — does multi-hop reasoning (direction 2) exacerbate the Depth–Breadth Trade-Off (Thread 4) by requiring more retrieval per claim? Does domain adaptation (direction 4) complicate the evaluation comparability crisis (Thread 1) by multiplying the number of discipline-specific benchmarks? Does local-first deployment constrain the evaluation protocol a system can support?
    Prioritize: unified evaluation protocol is most urgent (Thread 1 resolution); multi-hop reasoning is most technically challenging (Thread 3 resolution); analytical synthesis is most transformative (Thread 4 resolution). End with a call to action: the field must consolidate evaluation before it can meaningfully measure progress — architectural innovation without evaluation standards is exploration without a compass.
    depth_level: "standard"
