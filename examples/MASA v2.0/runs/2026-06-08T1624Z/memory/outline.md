# Survey Outline: Automated Survey Generation — Architectures, Evaluation, and the Evidence Gap

## Narrative Threads

**Thread 1 — The Architectural Complexity–Grounding Trade-off**: Systems are growing more architecturally elaborate (single-agent → multi-agent → iterative → human-in-the-loop) without becoming more structurally grounded in citation graph understanding. Complexity increases without a corresponding increase in structural awareness of the research landscape. This thread runs through Sections 2 (tracing the complexity escalation), 3 (comparing architectural patterns), 4 (contrasting bolt-on vs. first-class graph awareness), and 5 (assessing whether complexity delivers measurable value).

**Thread 2 — The Evidence Gap**: The field's central claims — "multi-agent outperforms single-agent," "graph awareness improves coverage," "iterative refinement converges to quality" — rest on a vanishingly thin empirical base. The only controlled comparison in the literature is Agentic AutoSurvey vs. AutoSurvey (8.18 vs. 4.77). Every other result is incommensurable. This thread connects Section 2's quantitative trend table (showing the metric dispersion), Section 3's controlled comparison gap analysis, Section 4's lack of ablation studies, and Section 5's systematic claim-vs-evidence audit.

**Thread 3 — The Blind Spot Cascade**: The field systematically avoids measuring what matters most — citation hallucination, insight/novelty, computational cost, and convergence behavior. Each is individually addressable but collectively ignored, producing systems optimized for coherence metrics rather than genuine scholarly value. This thread emerges in Section 4's analysis of iterative refinement (no convergence criteria), deepens in Section 5's blind spot catalog, and culminates in Section 6's call for a diagnostic-first research culture.

---

## Section 1: Introduction and Scope

- **Narrative Arc**: Automated Survey Generation (ASG) has evolved from proof-of-concept single-agent pipelines to sophisticated multi-agent, graph-aware systems in just over two years (2024–2026). Yet the field suffers from an evaluation comparability crisis, a persistent graph-awareness gap, and a systematic avoidance of diagnostic evidence. This survey traces the evolution, critically examines the evidence base, and identifies the blind spots that prevent the field from transitioning from architectural exploration to grounded scientific understanding.

- **Refinement Guidelines**:
  1. Define ASG and its scope (end-to-end survey generation, not just retrieval or summarization)
  2. State the four research questions: (a) How have ASG architectures evolved? (b) What evidence supports the claimed advances? (c) What is the state of evaluation? (d) What blind spots remain?
  3. Outline the paper's structure and contributions
  4. 300–400 words

- **Reference Papers**: None (introductory)

## Section 2: The Evolution of Automated Survey Generation — From Single-Agent Pipelines to Multi-Agent, Graph-Aware Systems

- **Narrative Arc**: ASG has passed through five overlapping phases in under three years (2022–2026): (0) Pre-LLM foundational infrastructure of citation graphs and search strategies; (1) single-agent pipelines that proved feasibility; (2) a multi-agent explosion that improved quality but added uncontrolled complexity; (3) an awakening to citation graph awareness and iterative refinement; and (4) a belated push toward standardized evaluation. Each phase genuinely advanced the state of the art, but the transitions were driven as much by architectural fashion as by measured deficiencies — no systematic diagnostic evidence forced the shifts.

- **Refinement Guidelines**:
  1. Organize chronologically by phase — Phase 0 through Phase 4, each as a subsection
  2. For each phase: describe the dominant approach, genuine contributions, and the limitation that (supposedly) drove the next phase
  3. Include a quantitative trend table showing performance metrics, coverage, and evaluation scores across phases — and highlight that every row uses a different metric
  4. End each phase subsection with a critical transition note: "what Phase N promised but did not deliver"
  5. 600–700 words total

- **Reference Papers**:
  - Phase 0: 1805.02262, 1806.00089, 2004.09741, 1407.5107, 2210.03629
  - Phase 1: 2406.10252, 2502.14776, 2508.17647, 2503.04629, 2509.19370
  - Phase 2: 2509.18661, 2506.12689, 2508.04306, 2411.06159, 2510.26012, 2504.14822
  - Phase 3: 2510.07733, 2508.14317, 2510.21900, 2409.12177
  - Phase 4: 2510.03120, 2512.02763, 2508.15658, 2602.11238, 2601.15307

### 2.1 Phase 0: Pre-LLM Foundations (2012–2023)
- **depth_level**: minimal
- **target_words**: 150
- **key_papers**: 1805.02262, 1806.00089, 2004.09741, 1407.5107, 2210.03629

### 2.2 Phase 1: The Single-Agent Pipeline (2024 — Early 2025)
- **depth_level**: standard
- **target_words**: 400
- **key_papers**: 2406.10252, 2502.14776, 2508.17647, 2503.04629, 2509.19370

### 2.3 Phase 2: The Multi-Agent Explosion (Late 2024 — 2025)
- **depth_level**: standard
- **target_words**: 400
- **key_papers**: 2509.18661, 2506.12689, 2508.04306, 2411.06159, 2510.26012, 2504.14822

### 2.4 Phase 3: Graph Awareness and Iterative Refinement (2025)
- **depth_level**: standard
- **target_words**: 300
- **key_papers**: 2510.07733, 2508.14317, 2510.21900, 2409.12177

### 2.5 Phase 4: Evaluation Maturation (2025–2026)
- **depth_level**: standard
- **target_words**: 300
- **key_papers**: 2510.03120, 2512.02763, 2508.15658, 2602.11238, 2601.15307

## Section 3: Core Architectures — Single-Agent, Multi-Agent, and the Controlled Comparison Gap

- **Narrative Arc**: The field's central architectural question — how many agents should a survey generation system use? — has a remarkably thin evidence base. This section dissects the three dominant architectural paradigms (single-agent pipeline, multi-agent pipeline, iterative refinement) and shows that the only controlled comparison in the literature (Agentic AutoSurvey's 8.18 vs. AutoSurvey's 4.77) carries an outsized evidentiary burden. Thread 2 (the evidence gap) is developed here through a systematic audit of what has been compared, what has not, and why the field cannot answer its own core question.

- **Refinement Guidelines**:
  1. For each architecture, explain the mechanism (how it works, not just what it does) — the key design decisions, coordination patterns, iteration strategies
  2. Include a comparison table across: agent count, coordination pattern, graph awareness, iteration strategy, human involvement, key metric, dataset, quality score
  3. Dedicate a subsection to the Controlled Comparison Gap — the only direct comparison and what it implies
  4. Include an analysis of the computational cost opacity (no system reports token usage or runtime)
  5. 600–700 words total

- **Reference Papers**:
  - Single-agent: 2406.10252, 2502.14776, 2508.17647, 2503.04629, 2509.19370
  - Multi-agent: 2509.18661, 2506.12689, 2508.04306, 2411.06159, 2510.26012, 2504.14822
  - Iterative: 2508.14317, 2510.21900
  - Critique of MAS vs SAS: 2505.18286, 2510.04311

### 3.1 Single-Agent Pipelines — The Foundational Pattern
- **depth_level**: deep
- **target_words**: 500
- **key_papers**: 2406.10252, 2502.14776, 2508.17647, 2503.04629, 2509.19370
- **writing_instructions**: "Compare AutoSurvey, SurveyX, SurveyGen, SurveyForge, and Meow in a table across dimensions: planning mechanism (outline vs. attribute_tree vs. quality_driven vs. memory_guided vs. end_to_end), retrieval method, iteration strategy, graph awareness. For each method, explain the architectural mechanism — not just the purpose. Include pros/cons and any quantitative results. This subsection develops Thread 1 by showing how single-agent systems reached an inherent cognitive bottleneck."

### 3.2 Multi-Agent Pipelines — Specialization and Coordination
- **depth_level**: deep
- **target_words**: 500
- **key_papers**: 2509.18661, 2506.12689, 2508.04306, 2411.06159, 2510.26012, 2504.14822
- **writing_instructions**: "Compare Agentic AutoSurvey, SciSage, MATC, KMCA, AutoSurvey2, and InsightAgent in a table across dimensions: agent count, coordination pattern (shared task board vs. hierarchical vs. mixture-of-experts vs. human-centered), graph awareness, iteration strategy. For each, explain the coordination mechanism and how error is handled. This subsection develops Thread 1 by showing how multi-agent systems distribute cognitive load but also add uncontrolled complexity."

### 3.3 Iterative Refinement — Emergent Structure Through Convergence
- **depth_level**: deep
- **target_words**: 400
- **key_papers**: 2508.14317, 2510.21900
- **writing_instructions**: "Compare SurveyGen-I and IterSurvey in a table. Explain the coarse-to-fine vs. recurrent-outline mechanisms. Critically analyze the absence of convergence criteria in both systems. This subsection develops Thread 3 by showing how iterative systems lack a well-defined stopping condition, making the iteration loop unbounded."

### 3.4 The Controlled Comparison Gap
- **depth_level**: deep
- **target_words**: 500
- **key_papers**: 2509.18661, 2406.10252, 2505.18286, 2510.04311
- **writing_instructions**: "Systematically audit what has been compared in the literature. Present a table showing every pair of systems that have been evaluated on the same benchmark — only one pair exists (Agentic AutoSurvey vs AutoSurvey). Analyze why this single comparison (8.18 vs 4.77) carries so much weight and what it cannot tell us. Discuss the implications of the evaluation comparability crisis. This subsection is the core of Thread 2 — it establishes that the field's most important claim rests on a single data point."

## Section 4: Graph Awareness — From Retrieval Afterthought to Structural Backbone

- **Narrative Arc**: Only 5 of 35 core method papers use citation graph structure for anything beyond keyword search. Those that do (SurveyG, SciSage, SurveyForge, MATC, LitFM) represent a minority, but their approaches reveal a fundamental design spectrum: graph as a retrieval bolt-on vs. graph as the structural backbone of the entire generation process. This section traces that spectrum, showing how only SurveyG treats the citation graph as a first-class organizational primitive. The rest use graph traversal as one retrieval strategy among many. Thread 1 (the grounding–complexity trade-off) is developed here by contrasting SurveyG's architectural ambition with the bolt-on approaches.

- **Refinement Guidelines**:
  1. Organize along the spectrum from minimal to maximal graph integration
  2. Include a comparison table across: graph type (bfs chaining vs. hierarchical), traversal strategy, how graph informs organization, graph evaluation method
  3. Dedicate a subsection to what the field is missing: GNN-based retrieval, learned citation dynamics, graph→outline mapping
  4. 500–600 words total

- **Reference Papers**:
  - Graph-aware systems: 2510.07733, 2506.12689, 2503.04629, 2508.04306, 2409.12177
  - Foundational graph mechanisms: 1806.00089, 1904.07579, 2408.15371, 2305.01572, 2402.08339
  - Cross-domain: 2104.02562, 1903.06464

### 4.1 Citation Chaining as a Retrieval Strategy — SciSage, SurveyForge, MATC
- **depth_level**: deep
- **target_words**: 500
- **key_papers**: 2506.12689, 2503.04629, 2508.04306
- **writing_instructions**: "Compare three systems that use citation chaining as a retrieval strategy within a broader pipeline. Show how each integrates bfs traversal (forward, backward, or both) into its retrieval module. For each, assess whether the graph component is evaluated in isolation or only as part of the full system. This subsection develops Thread 1 by showing graph-as-bolt-on: citation chaining adds retrieval depth but does not affect survey structure."

### 4.2 SurveyG — The Hierarchical Citation Graph as Architectural Foundation
- **depth_level**: deep
- **target_words**: 500
- **key_papers**: 2510.07733
- **writing_instructions**: "Deep-dive into SurveyG's three-layer hierarchical citation graph (Foundation/Development/Frontier) and its horizontal + vertical traversal. Explain how the graph structure directly maps to survey outline — a qualitatively different approach from all other systems. Critically analyze the absence of an ablation study: the paper asserts that hierarchical graph improves organization but does not demonstrate it. This subsection develops Thread 1 by showing graph-as-backbone, contrasting with 4.1's bolt-on approaches."

### 4.3 LitFM and the Missed Opportunity of Learned Graph Representations
- **depth_level**: standard
- **target_words**: 300
- **key_papers**: 2409.12177, 2408.15371, 2305.01572, 1903.06464
- **writing_instructions**: "Contrast LitFM's structure-aware foundation model with the GNN-based citation prediction methods (Temporal GNN, H2CGL) that could have been integrated into ASG systems but were not. Discuss why the field chose embedding+keyword hybrid over learned graph models. This subsection develops Thread 1 by showing what the field is collectively ignoring."

## Section 5: Critical Assessment — Claims, Gaps, and Blind Spots

- **Narrative Arc**: This is the analytical core of the survey. The evidence gap (Thread 2) and blind spot cascade (Thread 3) converge here. The field's claims about multi-agent superiority, graph-awareness benefits, and iterative refinement are systematically audited against the evidence that supports them — or fails to. Four methodological weaknesses that cut across all phases are examined. The evaluation comparability crisis is quantified by showing that no two systems (except Agentic AutoSurvey and AutoSurvey) share a benchmark. Finally, five blind spots — citation hallucination, insight/novelty, human ground truth idealization, computational cost opacity, and cross-lingual/domain vacuum — are cataloged and their consequences assessed.

- **Refinement Guidelines**:
  1. Present the Claim vs. Evidence Gap table from the evolution narrative, expanded with paper-specific evidence
  2. Analyze the four cross-phase methodological weaknesses: custom evaluation, no ablation studies, unreproducible human evaluation, no convergence criteria, isolated graph evaluation
  3. Quantify the evaluation comparability crisis — show the benchmark dispersion table
  4. Catalog and analyze the five blind spots
  5. Synthesize: what the field would need to become genuinely scientific
  6. 600–700 words

- **Reference Papers**: All cross-phase references

### 5.1 Claim vs. Evidence — Systematic Audit
- **depth_level**: deep
- **target_words**: 500
- **key_papers**: 2406.10252, 2509.18661, 2506.12689, 2504.14822, 2510.07733, 2508.17647, 2510.03120
- **writing_instructions**: "Present the Claim vs. Evidence Gap table from evolution_narrative.md as a formal table with columns: Claim, Supporting Evidence, Assessment. For each claim, explain why the evidence is insufficient — not just that it's weak, but what specific methodological improvement would strengthen it. This subsection is the culmination of Thread 2."

### 5.2 Methodological Weaknesses Across All Phases
- **depth_level**: deep
- **target_words**: 500
- **key_papers**: 2406.10252, 2509.18661, 2506.12689, 2508.14317, 2510.21900
- **writing_instructions**: "Analyze the five cross-phase weaknesses with concrete examples from specific papers: (1) custom evaluation is universal, (2) no ablation studies anywhere, (3) human evaluation unreproducible, (4) no convergence criteria, (5) graph evaluation in isolation. This subsection develops Thread 3 by showing that the field's methodology prevents it from answering its own questions."

### 5.3 The Evaluation Comparability Crisis
- **depth_level**: deep
- **target_words**: 500
- **key_papers**: 2510.03120, 2512.02763, 2508.15658, 2602.11238, 2508.11310
- **writing_instructions**: "Present the benchmark dispersion table showing which system uses which benchmark/metric. Show that only one pair (Agentic AutoSurvey vs. AutoSurvey) shares evaluation. Explain why the Phase 4 benchmarks (SurveyBench, SurveyEval, SurGE, SurveyLens) exist but no Phase 1–3 system has been evaluated on any of them. Discuss the consequences for the field. This subsection is the synthesis of Thread 2 and Thread 3."

### 5.4 Blind Spots — What the Field Is Not Looking At
- **depth_level**: deep
- **target_words**: 500
- **key_papers**: 2411.16638, 2305.14251, 2406.19276, 2510.17853, 2602.11238, 2505.18286
- **writing_instructions**: "Catalog and analyze the five blind spots: (1) citation hallucination is unmeasured despite available tools (CiteGuard, VERISCORE, FActScore), (2) no evaluation of insight or novelty, (3) human ground truth idealized, (4) computational cost opaque, (5) no cross-lingual or cross-domain evaluation. For each, explain why it matters and how the field could address it. This subsection is the culmination of Thread 3."

## Section 6: Future Directions — Toward a Diagnostic-First Science of Survey Generation

- **Narrative Arc**: The path forward requires a fundamental reorientation from architectural exploration to diagnostic evidence. Six directions are proposed: (1) citation graph traversal as a first-class retrieval primitive, (2) convergence-guaranteed iterative refinement with explicit stopping criteria, (3) standardized evaluation on community benchmarks, (4) systematic citation hallucination auditing as a primary metric, (5) controlled cost-quality reporting, and (6) ablation studies for every architectural choice. These directions directly address the blind spots and evidence gaps identified in Section 5.

- **Refinement Guidelines**:
  1. Each direction should specify: what to do, why it matters, and how to evaluate success
  2. Ground each direction in the specific gaps identified in Sections 3–5
  3. Avoid vague calls for "better evaluation" — specify concrete next steps
  4. End with a vision of what a diagnostic-first field would look like
  5. 400–500 words

- **Reference Papers**: 2510.07733, 2409.12177, 2510.03120, 2305.14251, 2406.19276, 2510.17853, 2408.15371, 2505.18286

### 6.1 First-Class Citation Graph Integration
- **depth_level**: standard
- **target_words**: 300
- **key_papers**: 2510.07733, 2409.12177, 2408.15371
- **writing_instructions**: "Propose how GNN-based citation graph models could be integrated into ASG systems as a primary retrieval and organization primitive, not a bolt-on. Reference SurveyG's hierarchical graph and LitFM's structure-aware model as starting points."

### 6.2 Convergence-Guaranteed Iterative Refinement
- **depth_level**: standard
- **target_words**: 300
- **key_papers**: 2508.14317, 2510.21900
- **writing_instructions**: "Propose formal convergence criteria for iterative systems — maximizing coverage while minimizing redundancy, maximizing citation support while minimizing hallucination risk. Discuss gradient-free optimization over the iterative loop."

### 6.3 Standardized Evaluation and Cost-Quality Reporting
- **depth_level**: standard
- **target_words**: 300
- **key_papers**: 2510.03120, 2508.15658, 2602.11238
- **writing_instructions**: "Call for community convergence on at least one shared benchmark (SurveyBench, SurGE, or SurveyLens) and mandatory cost-quality reporting (token usage, API costs, runtime alongside quality scores)."

### 6.4 Ablation Studies and Citation Hallucination Auditing
- **depth_level**: standard
- **target_words**: 300
- **key_papers**: 2510.17853, 2406.19276, 2305.14251
- **writing_instructions**: "Propose a research culture where every architectural claim is supported by an ablation study and every system reports citation hallucination rate using CiteGuard or VERISCORE. These should be primary metrics, not optional extras."

## Section 7: Conclusion

- **Narrative Arc**: The field of automated survey generation has achieved remarkable architectural diversity in under three years — from single-agent pipelines to multi-agent, graph-aware, and iterative refinement systems. But architectural diversity without diagnostic evidence is just engineering variety. The central finding of this survey is that the field's progress claims outrun its evidence base. The evaluation comparability crisis means we cannot rank systems. The absence of ablation studies means we cannot attribute improvements to specific design choices. The blind spots — citation hallucination, insight, cost, convergence — mean we may be optimizing for the wrong things. A genuinely next-generation system will require not just better architecture but better science.

- **Refinement Guidelines**:
  1. Summarize the three narrative threads and their implications
  2. Restate the four research questions and how the survey answered them
  3. End with a forward-looking call to action
  4. 200–300 words

- **Reference Papers**: None (concluding)
