# Survey Outline: Automated Survey Generation — From RAG Pipelines to Multi-Agent Deliberation

## Narrative Threads

### Thread 1: The Evaluation Comparability Crisis
The field has produced more evaluation benchmarks (8+) than architectural approaches (5), yet no two systems can be directly compared. Each defines its own metrics, constructs its own human-written reference set on different topics, and reports scores that are incommensurable across papers. This crisis means that claims of "SOTA," "human-competitive quality," and "superhuman performance" are unfalsifiable — not because the claims are false, but because there is no shared yardstick to verify them. This thread runs from Section 2 (where Phase 1 established ROUGE/BLEU from summarization), through Section 4 (where benchmark proliferation exploded), and culminates in Section 6 (the analytical centerpiece).

### Thread 2: The Automation–Control Tension
As systems progress from single-agent pipelines through multi-agent architectures to deliberation-first frameworks, they become more autonomous — but less transparent and less controllable by human users. Meanwhile, hybrid interactive systems preserve human control at the cost of scalability. No system has resolved this tension: the more automation, the less accountability; the more human oversight, the less efficiency. This thread connects the single-agent era (Section 3), the architectural proliferation with its multi-agent black boxes (Section 4), the interactive counterpoint (Section 4.3), and the frontier (Section 5).

### Thread 3: Citation Graph Shallowness — The Unfulfilled Promise
Despite nearly every system claiming to address citation quality, the field's engagement with citation structure remains at the level of single-hop BFS traversal. No system performs multi-hop reasoning over citation chains, temporal analysis of research trajectories, methodological lineage tracking, or claim-level provenance verification. The hierarchical tiering of SurveyG is the only structural innovation — and it too is single-hop. This thread exposes the gap between what the field claims (citation-aware generation) and what it delivers (paper-level retrieval with no graph reasoning). It weaves through the foundations (Section 2), the graph-enhanced systems (Section 4.2), and the critical assessment (Section 6).

### Thread 4: The Depth–Breadth Trade-Off
Systems face a fundamental architectural choice between breadth (retrieving hundreds of papers for comprehensive coverage) and depth (verifying individual claims with fine-grained citation attribution). ReClaim achieves per-sentence verification but cannot scale beyond short-form generation. SurveyG retrieves hierarchically but does not verify claim-citation alignment. No system simultaneously achieves broad coverage and deep citation verification. This trade-off manifests across every phase and architecture type, and resolving it is the defining challenge for next-generation systems.

---

## Section 1: Introduction and Scope
- **Narrative Arc**: Set up the problem — automated survey generation promises to accelerate scientific synthesis, but the field's rapid architectural proliferation has outpaced its evaluation infrastructure. The survey will trace the evolution from foundational RAG through single-agent pipelines to multi-agent and graph-enhanced systems, then critically assess the claims against the evidence.
- **Refinement Guidelines**:
  1. Define automated survey generation: end-to-end systems producing structured multi-section literature surveys from a topic query
  2. Scope boundaries: exclude single-document summarization, non-scientific domains, pure citation analysis without generation
  3. State the central tension: architectural innovation without standardized evaluation
  4. Present the four narrative threads that run through the survey
  5. Provide a roadmap of sections
- **Reference Papers**: [2406.10252, 2402.08565, 2501.04306]

## Section 2: The Foundations — RAG Paradigm and Dataset Infrastructure (2020–2023)
- **Narrative Arc**: This phase established the enabling technologies — retrieval-augmented generation, scientific claim verification datasets, and factuality evaluation — but addressed short-form QA and related-work generation, not full surveys. The datasets and methods from this phase remain the infrastructure on which all subsequent systems depend.
- **Refinement Guidelines**:
  1. Organize into subsections: core methods (RAG), datasets (SciFact, Multi-XScience, SciReviewGen), evaluation frameworks (FActScore)
  2. For each: describe the contribution, the genuine advance it represented, and the limitation that prevented it from scaling to full survey generation
  3. Include a brief performance summary table showing metrics from the Phase 1 papers
  4. End with a critical transition: Phase 1 established the building blocks but provided no direct pathway to multi-section survey generation — a fundamentally new decomposition strategy was required
- **Reference Papers**: [2005.11401, 2004.14974, 2010.14235, 2305.15186, 2305.14251, 2004.15011, 2010.04147]

## Section 3: The Single-Agent Pipeline Emerges — Task Decomposition (2024)
- **Narrative Arc**: 2024's defining innovation was task decomposition — breaking survey writing into retrieval → outline → drafting → integration → evaluation, each executed by a single LLM via prompt engineering. AutoSurvey established the baseline architecture; PaperQA2 and OpenScholar demonstrated the power of domain-specific retrieval. But the single-pass, single-LLM design hit a ceiling: one model could not simultaneously achieve deep retrieval, coherent organization, accurate citation, and critical synthesis.
- **Refinement Guidelines**:
  1. Describe the single-agent architecture template in detail (retrieval → outline → drafting → integration → evaluation)
  2. Compare AutoSurvey, SurveyX, LitLLM, Instruct LLMs Step by Step, DimInd — include a comparison table across dimensions: retrieval method, preprocessing strategy, generation scope, iteration strategy, evaluation approach
  3. Highlight the methodological variation: fine-tuned (ScholarCopilot, AcademicGPT, OpenScholar) vs zero-shot (AutoSurvey, SurveyX)
  4. Discuss what Phase 2 achieved: first demonstration of coherent multi-section surveys, the stage-decomposition template, PaperQA2's superhuman QA result
  5. End with what it did not achieve: no iterative refinement, no citation graph awareness, no critical synthesis — these limitations drove the architectural proliferation of 2025
- **Reference Papers**: [2406.10252, 2502.14776, 2402.01788, 2408.07884, 2504.18496, 2409.13740, 2411.14199, 2504.00824, 2407.01796, 2402.16063]

## Section 4: Architectural Proliferation — Multi-Agent, Graph, and Human-in-the-Loop (2025)
- **Narrative Arc**: 2025 saw an explosion of architectural diversity as the field fragmented into five distinct approaches. Multi-agent systems specialized agent roles; graph-enhanced systems used citation structure for paper discovery; hybrid interactive systems put humans in the loop; iterative refinement systems added multi-round generation. Simultaneously, benchmark papers proliferated as the field recognized the evaluation crisis — but each benchmark defined different metrics, deepening the problem rather than solving it.
- **Refinement Guidelines**:
  1. This is the longest, most complex section — organize into 4 subsections
  2. Each subsection should follow the same structure: describe the architectural pattern, present the key systems with a comparison table, discuss genuine advances, then end with limitations and unfulfilled claims
  3. Include a cross-subsection comparison at the end
  4. Tie each subsection to narrative threads

### Subsection 4.1: Multi-Agent Architectures — Dividing the Labour
- **Refinement Guidelines**:
  1. Define the multi-agent pattern: specialized agent roles (Planner, Retriever, Writer, Evaluator, Reviewer) with coordination mechanisms
  2. Compare ARISE (rubric-guided, 5 agents), SciSage (reflect-when-you-write, 4 agents), Agentic AutoSurvey (orchestrator-driven, 4 agents), **AutoSurvey2 (parallel section generation with real-time re-retrieval, multiple parallel LLM instances)**, MARCO (chat-based coordination), Federation of Agents (knowledge graph coordination), OrchMAS (domain-expert agents), DOVA (deliberation-first)
  3. Include a comparison table: agent count, coordination pattern, iteration strategy, citation graph awareness, key innovation
  4. Genuine advances: role specialization improves output quality; rubric-guided evaluation (ARISE, SciSage) moves quality assessment into the generation loop; deliberation-first (DOVA) inverts the retrieve-then-deliberate pattern; AutoSurvey2's parallel drafting and real-time re-retrieval solves the single-pass retrieval gap of its predecessor
  5. Unfulfilled claims: ARISE's 92.48 score is a system-defined number with no human calibration; "SOTA" claims are based on self-created benchmarks; all agents typically use the same underlying LLM — specialization is prompt-deep only (discuss what evidence would confirm or falsify this: do agents with the same base model but different prompts produce measurably different retrieval choices?); AutoSurvey2's homogeneous agents lack the role specialization that distinguishes genuine multi-agent systems from mere parallel dispatch
- **Reference Papers**: [2511.17689, 2506.12689, 2509.18661, **2510.26012**, 2410.21784, 2509.20175, 2603.03005, 2603.13327, 2404.07738, 2403.08399, 2510.15624]

### Subsection 4.2: Graph-Enhanced Retrieval — Beyond Keyword Search
- **Refinement Guidelines**:
  1. Define the graph-enhanced pattern: citation graph traversal (forward/backward chaining) is a core retrieval component
  2. Compare SurveyForge (BFS chaining + memory-driven generation), SurveyG (hierarchical three-tier graph), SurveyGen (quality-aware RAG with graph traversal), ProfOlaf (explicit snowballing), GEAR-Up (KG-based query expansion)
  3. Include a comparison table: graph traversal strategy (BFS vs hierarchical), graph role (retrieval expansion vs outline organization), iteration strategy, agent count
  4. Genuine advances: citation chaining discovers papers that keyword/embedding search misses; SurveyG's hierarchical tiering provides principled paper organization by research lineage role
  5. Unfulfilled claims: all graph traversal is single-hop BFS — no multi-hop reasoning; hierarchical tiering is citation-position-based, not intellectual lineage-based; claims of "improved coverage" lack comparison against strong baselines
- **Reference Papers**: [2503.04629, 2510.07733, 2508.17647, 2510.26750, 2312.09948]

### Subsection 4.3: Human-in-the-Loop and Iterative Refinement
- **Refinement Guidelines**:
  1. Define the hybrid interactive pattern (human validates/guides intermediate outputs) and the iterative refinement pattern (multi-round generate-evaluate-revise)
  2. Compare InteractiveSurvey, PROMPTHEUS, CRUISE-Screening (hybrid interactive) with ReClaim, IterSurvey (iterative refinement)
  3. Include a comparison table: interaction type (human decisions vs quality-driven iteration), granularity (per-section vs per-sentence), scope (full survey vs QA/summarization)
  4. Genuine advances: human-in-the-loop provides transparency and control missing from automated systems; paper cards (IterSurvey) ensure faithful citation attribution; rubric-driven iteration (ARISE) converges to quality targets
  5. **Quantify ReClaim's computational cost**: per-sentence verification requires ~10 API calls per sentence (retrieve → generate → verify); scaling to a 100-sentence survey would require 1,000+ API calls, making full-survey deployment prohibitive at current pricing. This concrete estimate anchors the Depth–Breadth Trade-Off (Thread 4).
  6. Unfulfilled claims: sentence-level verification (ReClaim) doesn't scale to full surveys; iterating per-draft (IterSurvey) is computationally expensive; human-in-the-loop trades automation for scalability
- **Reference Papers**: [2504.08762, 2410.15978, 2309.01684, 2407.01796, 2510.21900, 2508.14317, 2411.18583, 2504.14822]

### Subsection 4.4: The Evaluation Benchmark Explosion — More Metrics, Less Clarity
- **Refinement Guidelines**:
  1. Chronological overview of benchmark introductions: SurveyBench (2510.03120), SurveyEval, SurveyScope (SciSage companion), Survey-Arena (IterSurvey companion), SurGE, SGSimEval, SurveyLens, DeepSurvey-Bench, CiteRAG
  2. Compare across dimensions: scale (number of topics/paper sets), metric types (ROUGE, Citation F1, quiz answerability, rubric scores, win-rate), discipline coverage, reference survey availability
  3. The crisis: each benchmark measures something different, on different topic sets, with different reference standards
  4. No benchmark has achieved consensus adoption — every generation system reports on a different combination
  5. This subsection develops Thread 1 (Evaluation Comparability Crisis) centrally
- **Reference Papers**: [2510.03120, 2508.15658, 2508.11310, 2512.02763, 2602.11238, 2601.15307, 2601.14949, 2412.15249, 2310.04480, 2308.10410]

## Section 5: Current Frontier — Deliberation, Domain Expertise, and Local Deployment (2026)
- **Narrative Arc**: The frontier phase inverts the traditional retrieve-then-generate pipeline. DOVA's deliberation-first architecture has agents discuss the topic before retrieval; OrchMAS deploys domain-specialized expert agents; ResearchPilot demonstrates that multi-agent survey generation can work on consumer hardware. These are genuine innovations — but they inherit all the evaluation problems of the previous phases and introduce new ones: deliberation quality is unmeasured, domain expertise is hard to validate, and citation graph awareness remains shallow.
- **Refinement Guidelines**:
  1. Describe each frontier system with concrete mechanism detail:
     - **DOVA (deliberation-first)**: Configurable number of perspective agents (at least 4: empiricist, theoretician, methodologist, domain specialist) deliberate on the research topic to identify key questions and information needs before any retrieval occurs. Deliberation output guides targeted keyword queries. Single-pass (deliberation → retrieval → synthesis; no iterative refinement after retrieval). Deliberation quality is unmeasured — no metric exists to determine whether productive deliberation has occurred.
     - **OrchMAS (domain-expert agents)**: Coordinator Agent analyzes the research question and creates a work plan. N Expert Agents, each pre-configured with domain-specific knowledge profiles (e.g., ML, statistics, neuroscience), are assigned tasks matching their expertise. Experts retrieve papers with domain-specific configurations, analyze findings, produce structured summaries. Coordinator integrates outputs, resolving conflicts and identifying consensus. Requires upfront domain expertise configuration — how this scales to new domains is not addressed.
     - **ResearchPilot (local-first)**: 4 local agents (Retriever, Analyst, Writer, Reviewer) run sequentially on consumer hardware using quantized local LLMs (e.g., Llama, Mistral). Local embedding search over a manually curated paper index. Quality is acknowledged to be lower than cloud-based frontier models; the trade-off is near-zero cost and full privacy. No citation graph awareness.
  2. Compare them against each other and against the strongest Phase 3 systems (ARISE, SciSage, SurveyG)
  3. Include a comparison table: approach, agent count, retrieval method, citation graph awareness, iteration strategy, key innovation, deployment profile (cloud vs local), reported quality (with caveats)
  4. Discuss open problems: deliberation quality cannot be measured; domain expertise = pre-configured knowledge vs LLM knowledge; local models trade quality for privacy
  5. Cross-reference: the DOVA deliberation quality critique echoes the rubric-quality problem from §4.1 — both systems make unvalidated claims about intermediate process quality
  6. This section develops Thread 2 (Automation–Control Tension) by showing how deliberation-first increases autonomy but reduces transparency, and Thread 3 (Citation Shallowness) by showing the frontier does not address it
- **Reference Papers**: [2603.13327, 2603.03005, 2603.14629]

## Section 6: Critical Assessment — Claims, Gaps, and Blind Spots
- **Narrative Arc**: This is the analytical core of the survey. After reviewing four phases of architectural evolution, this section systematically examines what the field claims vs. what the evidence actually shows, identifies methodological weaknesses that persist across all phases, diagnoses the evaluation comparability crisis, and exposes blind spots the field is collectively ignoring.
- **Refinement Guidelines**:
  1. Present the Claim vs. Evidence Gap table from evolution_narrative.md with 7 key claims and their assessment — **include a "Claim Source" column** citing the specific paper and section for each claim to improve traceability
  2. Analyze 6 methodological weaknesses: self-evaluation epidemic, **aggregate the field-wide picture (compute mean/median/max evaluation scope across surveyed systems: e.g., mean ~20 topics, median ~15 topics, max ~100 topics across 12 representative systems)** to strengthen the "small topical samples" weakness beyond anecdotal examples, single-discipline bias, no standardized benchmark, superficial citation verification, no measure of analytical depth
  3. Diagnose the evaluation comparability crisis in detail: metric fragmentation, reference survey fragmentation, human evaluation inconsistency, benchmark proliferation without consolidation
  4. Expose 7 blind spots: multi-hop citation reasoning, temporal analysis, methodological quality assessment, figure/table generation, cross-lingual surveys, longitudinal trustworthiness, user trust and calibration
  5. Note: the "most surveyed papers" figure in §6.4 is drawn from the cumulative corpus — soften from "77" to "most surveyed papers" to avoid an unverifiable precise number
  6. This section is the convergence point for all four narrative threads
- **Reference Papers**: [cross-phase references — all key method and benchmark papers]

## Section 7: Future Directions — Toward Next-Generation Survey Generation
- **Narrative Arc**: Based on the critical assessment, outline what a genuinely next-generation system would need: a unified evaluation protocol, multi-hop citation reasoning, analytical synthesis capability, domain-adaptive expertise, temporal grounding and maintenance, and calibrated uncertainty communication. Non-textual content generation is folded into analytical synthesis as a structured output extension. Each direction is grounded in a specific gap identified in Section 6. Directions are not independent — tensions between them are explored (e.g., multi-hop reasoning may exacerbate the Depth–Breadth Trade-Off; domain adaptation complicates the evaluation comparability crisis).
- **Refinement Guidelines**:
  1. For each future direction, state the gap it addresses (from Section 6), describe what a solution would look like, and identify preliminary work (if any)
  2. **Merge non-textual content (former direction 6) into analytical synthesis (direction 3)** — comparison tables, methodology taxonomies, and evidence maps are structured outputs that serve analytical synthesis, not an independent capability
  3. **Add cross-cutting trade-off analysis**: explore tensions between directions — does multi-hop reasoning exacerbate the Depth–Breadth Trade-Off? Does domain adaptation complicate the evaluation comparability crisis? Does local-first deployment (ResearchPilot) constrain the evaluation protocol it can support?
  4. Prioritize the directions by impact: unified evaluation protocol is the most urgent; multi-hop reasoning is the most technically challenging; analytical synthesis is the most transformative
  5. End with a call to action: the field must consolidate evaluation before it can meaningfully measure progress
- **Reference Papers**: [all Phase 4 papers for frontier context, evaluation benchmarks for consolidation proposals]
