# Evolution Narrative — Automated Literature Survey Agents with Citation Graph Expansion

## Critical Arc

The field of automated literature survey generation has traced a curious arc: it began with citation graph infrastructure and structure-aware embeddings (2015–2020), was disrupted by LLMs that largely ignored citation structure in favor of semantic retrieval (2023–2024), scaled up through massive datastores and multi-agent coordination (2024–2025), and is now circling back toward graph-aware architectures — but with LLMs in the driver's seat rather than GNNs. The central tension throughout this evolution is between **semantic content** (what papers say) and **structural context** (where papers sit in the citation network). After a multi-year detour into purely semantic systems, the field is rediscovering that citation graphs encode complementary signals that embedding similarity alone cannot capture. Yet the current "re-integration" is nascent: most multi-agent survey systems still operate with zero graph awareness, and the few graph-aware systems (SurveyG, HiGTL) have not been integrated into the dominant multi-agent architectures. The field remains fragmented, with evaluation methodologies proliferating faster than consensus can form, and no system today combines the three ingredients that a mature solution would require: citation-graph-aware retrieval, multi-agent coordination with error mitigation, and RL-optimized search policies.

---

## Phase 1: Citation Graph Foundations (2015–2020)

### Technical Approach

This phase predates the LLM revolution. The dominant paradigm is graph-based: constructing large-scale citation graphs, learning paper embeddings from graph structure, and using graph topology for retrieval, clustering, and taxonomy induction. The computational engine is GNNs (GCNs, graph transformers) and contrastive learning on citation edges. No paper in this phase generates survey text — they produce embeddings, clusters, taxonomies, or ranked lists.

### Key Contributions

- **Semantic Scholar Literature Graph (1805.02262, 2018)**: Built the foundational infrastructure — a 280M+ node heterogeneous citation graph that would later power nearly every citation-aware survey system. Without this, citation-based retrieval at scale would not exist.

- **SPECTER (2004.07180, 2020)**: Introduced citation-informed document embeddings via contrastive learning on citation pairs. Became the standard building block for citation-aware retrieval, used as a drop-in replacement for text-only embeddings across the field.

- **HiGTL (2410.03761, 2024, transitional)**: End-to-end taxonomy tree generation from citation graph structure using GNN + hierarchical clustering + LLM verbalization. Bridges Phase 1 methodology (graph-driven taxonomy) with Phase 5's LLM integration.

### Reported Performance

| Paper | Metric | Value | Dataset |
|-------|--------|-------|---------|
| LitFM (2409.12177, 2024) | Retrieval precision | +28.1% over text-only | Citation retrieval benchmarks |
| SPECTER (2004.07180, 2020) | Zero-shot embedding quality | SOTA | Multiple citation prediction benchmarks |
| GrapAL (1902.05170, 2019) | Query capability | Neo4j graph DB | Scientific literature graph |
| Context-Aware Citation Rec (1903.06464, 2019) | MAP | +28% | Citation recommendation benchmarks |

### What This Phase Genuinely Achieved

It established that citation graph structure carries information orthogonal to text content — a paper's position in the citation network reveals its role (foundational, developmental, frontier), its community, and its intellectual lineage. SPECTER proved that citation signals can be distilled into reusable embeddings. Semantic Scholar's infrastructure made large-scale graph operations practical. These remain uncontested achievements.

### Unfulfilled Claims

- **"Citation clustering yields accurate taxonomies"** — The claim that direct citation replication outperforms NLP-based clustering (2004.05904, 2020) is true for flat clustering but breaks down for hierarchical taxonomies where semantic relationships become necessary. HiGTL (2024) implicitly acknowledges this by adding an LLM verbalization layer.
- **"GNN embeddings capture everything"** — GNN-based methods claimed their learned embeddings subsume both text and graph information, but subsequent LLM-based systems achieved competitive or superior performance using text-only semantic retrieval, suggesting the graph signal is complementary rather than dominant.

### Limitation That Drove Transition to Next Phase

Phase 1 systems could retrieve and organize papers but could not synthesize them into a coherent narrative — they produced embeddings, clusters, and ranked lists, not survey text. The arrival of instruction-tuned LLMs with long-context windows (GPT-4, Llama 2, 2023) made narrative synthesis suddenly tractable, but the LLM generation pipeline replaced graph methods rather than integrating with them.

---

## Phase 2: Single-Agent Survey Pipelines (2023–Early 2024)

### Technical Approach

A single LLM orchestrates all stages — planning, retrieval, drafting, refinement — through prompted pipelines. Retrieval is almost exclusively embedding-based similarity search, with no citation graph awareness. The innovation is in pipeline design: structured outlines (AutoSurvey), multi-perspective questioning (STORM), iterative citation chaining (PaperQA). The unifying assumption is that semantic content alone, boosted by LLM reasoning, is sufficient for survey-quality synthesis.

### Key Contributions

- **AutoSurvey (2406.10252, 2024)**: Established the canonical pipeline architecture (Outline → Retrieve → Draft → Refine) that nearly all subsequent systems would adopt or extend. First systematic demonstration that LLMs can produce coherent surveys from seed topics.

- **PaperQA (2312.07559, 2023)**: Introduced citation chaining into the RAG pipeline — following forward/backward citations to expand literature coverage. The only Phase 2 system with explicit graph traversal, but implemented as a BFS expansion strategy rather than a graph-aware architecture.

- **STORM (2402.14207, 2024)**: Multi-perspective question-asking for comprehensive coverage. Demonstrated that deliberate perspective decomposition can match human-written article quality without any citation graph awareness.

### Reported Performance

| Paper | Metric | Value | Dataset | Notes |
|-------|--------|-------|---------|-------|
| AutoSurvey | Survey quality | "Competitive with human-written" | Custom evaluation | No quantitative metric specified |
| PaperQA | QA accuracy | SOTA at time | LitQA benchmark | Citation chaining improved coverage |
| STORM | Article quality | "Competitive with human-written Wikipedia" | Wikipedia articles | |
| PaperQA2 (2409.13740) | QA accuracy | Superhuman | LitQA2 benchmark | Exceeds human expert performance |
| OpenScholar (2411.14199) | QA accuracy | 8B beats GPT-4o by 5% | ScholarQABench | Smaller model + better retrieval |

### What This Phase Genuinely Achieved

It proved that LLM-based survey generation is feasible and useful. AutoSurvey-like pipelines produce coherent, well-structured surveys that human evaluators rate as comparable to human-written ones. PaperQA2's "superhuman" claim on LitQA2, while narrowly scoped to factual QA, demonstrated that automated synthesis can exceed human recall in evidence-grounded question answering. OpenScholar showed that a well-designed RAG system with a massive datastore enables smaller models to outperform larger ones — the retrieval infrastructure matters more than raw model capability.

### Unfulfilled Claims

- **"Competitive with human-written surveys"** — This claim, made by AutoSurvey and repeated by STORM, rests on evaluation rubrics that measure coherence and structure but not depth of analysis, novelty of synthesis, or critical evaluation. Human-written surveys provide original taxonomies, identify open problems, and offer methodological critiques. LLM-generated surveys largely summarize and restructure existing knowledge. The claim conflates "looks like a survey" with "provides the scholarly value of a survey."

- **"Superhuman synthesis" (PaperQA2)** — The claim is benchmark-specific (LitQA2) and task-specific (factual QA). The benchmark questions test whether the system can find and synthesize factual claims across papers. This is qualitatively different from writing a survey that identifies research gaps, compares methodologies critically, or proposes future directions. The "superhuman" framing implies general superiority, whereas the evidence supports only superior factual recall.

- **"8B beats GPT-4o by 5%" (OpenScholar)** — This was a controlled comparison on ScholarQABench, but OpenScholar's 45M paper datastore is a significant infrastructure advantage. The claim suggests architectural superiority, when the real story is that retrieval infrastructure compensates for model size — a valuable but different finding.

### Limitation That Drove Transition to Next Phase

Phase 2 systems were citation-graph-blind. Without graph traversal, they could not discover papers that use different terminology for the same concepts, trace the evolution of ideas through citation chains, or ground surveys in the structural context of the research field. PaperQA's citation chaining was the exception that proved the rule — it showed that BFS expansion helped, but the approach was shallow (one or two hops) and lacked hierarchical structure. The field recognized that scaling up retrieval datastores (OpenScholar: 45M papers) was one solution, but this still couldn't capture the directional, temporal, and hierarchical signals encoded in citation graphs.

---

## Phase 3: Multi-Agent Architectures (Late 2024–2025)

### Technical Approach

Multiple specialized LLM agents — Planner/Researcher/Writer/Reviewer/Reflector — coordinate through shared task boards or structured messaging to produce surveys. The driving insight is that division of labor improves quality: dedicated retrieval agents search more thoroughly, dedicated review agents catch errors, and dedicated writing agents maintain coherent narrative voice. Agent counts range from 2 (RL4F) to 6 (InsightAgent).

### Key Contributions

- **SciSage (2506.12689, 2025)**: "Reflect-when-you-write" design where the Reflector agent evaluates draft in real-time rather than post-hoc. The +32% citation F1 improvement over single-agent baselines is the strongest quantitative evidence for multi-agent architectures in this space.

- **Agentic AutoSurvey (2509.18661, 2025)**: Extended AutoSurvey to a 4-agent framework (Planner, Researcher, Writer, Reviewer). Reported 8.18/10 survey quality vs AutoSurvey's 4.77/10 — a near-doubling that represents the largest reported quality improvement from architectural change alone.

- **MATC (2508.04306, 2025)**: Introduced explicit error-mitigation taskforces (Exploitation, Exploration, Experience, Self-Correction). The only architecture designed specifically to address compounding errors in multi-step generation — a problem that single-agent and naive multi-agent systems silently suffer from.

### Reported Performance

| Paper | Metric | Value | Dataset | Notes |
|-------|--------|-------|---------|-------|
| SciSage | Citation F1 | +32% over single-agent | SurveyScope benchmark | Reflect-while-writing design |
| Agentic AutoSurvey | Survey quality | 8.18/10 vs 4.77/10 | Custom evaluation | Near-doubling over AutoSurvey baseline |
| InsightAgent | Time reduction | Months → 1.5 hours | Systematic review tasks | Hybrid interactive with human orchestrator |
| InsightAgent | Quality improvement | +27.2% | Systematic review tasks | Compared to manual systematic review |

### What This Phase Genuinely Achieved

Multi-agent architectures produce measurably better surveys than single-agent pipelines, with consistent improvements across citation accuracy (SciSage: +32% F1), overall quality (Agentic AutoSurvey: 8.18 vs 4.77), and error reduction (MATC). The agent specialization pattern — separate retrieval, writing, review, and reflection — is now the dominant paradigm for state-of-the-art systems. SciSage's reflect-while-writing approach is a genuine architectural insight: error correction is more effective when applied during generation rather than after.

### Unfulfilled Claims

- **"Multi-agent outperforms single-agent"** — The claim is true but undersupported by controlled comparisons. SciSage and Agentic AutoSurvey compare against different single-agent baselines (likely different base LLMs, retrieval methods, and evaluation protocols). Without standardized benchmarking, the magnitude of improvement attributable to multi-agent architecture vs. other variables is unclear. The improvement could partly reflect better prompt engineering, better retrieval, or larger base models rather than the multi-agent design itself.

- **"Error mitigation" (MATC)** — MATC's error-tracking architecture is well-motivated, but the paper does not report quantitative error reduction metrics in the available metadata. The claim that "self-correcting taskforces catch compounding errors" remains an architectural aspiration without empirical verification.

- **Citation graph awareness** — SciSage uses BFS citation chaining, Agentic AutoSurvey uses none, MATC uses BFS in its Exploration taskforce. The multi-agent paradigm has not systematically integrated citation graph structure. The "awareness" is shallow (one-hop BFS) and used by only a subset of agents.

### Limitation That Drove Transition to Next Phase

Multi-agent systems scale well but amplify a critical weakness: if the retrieval agent returns an incomplete or biased set of papers, all downstream agents (writer, reviewer, reflector) operate on an impoverished knowledge base. The field began to realize that better retrieval — particularly retrieval that leverages citation graph structure — is a prerequisite for better generation. Purely semantic retrieval misses entire lineages of work, and multi-agent coordination cannot fix a missing citation chain.

---

## Phase 4: Iterative and Reinforcement-Learning-Guided Systems (2025)

### Technical Approach

Systems that learn from feedback — either through RL training of search/generation policies (PaSa, AURA) or through procedural self-evaluation loops (IterSurvey, SurveyGen-I). The shift is from "build a better pipeline" to "train a better search/generation policy." RL-based approaches formalize what manual prompt engineering attempts: finding optimal strategies for literature discovery and survey composition.

### Key Contributions

- **PaSa (2501.10120, 2025)**: RL-optimized search policy for academic paper discovery, achieving +37.78% recall@20 over GPT-4o. Learned when to follow citations vs. refine keywords vs. search by author — the first system where the search strategy itself is optimized rather than hand-designed.

- **IterSurvey (2510.21900, 2025)**: Recurrent outline generation with iterative refinement — the outline adapts as content is generated. Introduced Survey-Arena benchmark. Procedural (non-RL) refinement that demonstrates the value of self-evaluation loops.

- **AURA (2510.27126, 2025)**: RL (epsilon-greedy) for adaptive questioning in conversational surveys. While designed for questionnaire-type surveys, the LSDE metric and adaptive policy framework are directly transferable to literature survey generation.

### Reported Performance

| Paper | Metric | Value | Dataset | Notes |
|-------|--------|-------|---------|-------|
| PaSa | Recall@20 | +37.78% over GPT-4o | Academic search tasks | RL-optimized search policy |
| AURA | Survey quality | Improved over static baselines | Conversational survey datasets | RL adaptation improves information gain |

### What This Phase Genuinely Achieved

PaSa represents a genuine paradigm shift: instead of designing retrieval heuristics (how many hops, which direction, what similarity threshold), the RL agent learns the optimal strategy from search trajectories. The +37.78% recall improvement is achieved not by building a better index but by learning to _choose_ the right search action at each step. This opens the door to learned citation traversal policies — the agent could learn when deeper citation chaining is warranted vs. when to switch to keyword search.

### Unfulfilled Claims

- **Transferability to survey generation** — PaSa optimizes for recall (finding papers), not for survey quality. The relationship between search recall and survey quality is not linear: finding every relevant paper does not guarantee a coherent survey, and missing some papers may not harm survey quality. The claim that better search implies better surveys is plausible but untested.

- **Self-evaluation quality** — IterSurvey and similar procedural systems rely on the LLM to self-evaluate its own output. The known biases of LLM self-evaluation (overconfidence, failure to detect subtle errors) are not addressed. The refinement loop may converge to a locally optimal but globally flawed survey if the self-evaluation is systematically wrong.

- **No citation graph integration** — PaSa includes citation-based search as one strategy type but does not model the citation graph explicitly. AURA has no graph awareness at all. The RL policy learns from search trajectories but does not learn from graph structure.

### Limitation That Drove Transition to Next Phase

RL-based systems optimize for narrowly defined objectives — recall (PaSa), information gain (AURA), self-evaluation criteria (IterSurvey). But these objectives may not correlate with the holistic quality dimensions that human readers value: critical insight, research gap identification, methodological comparison, and future direction proposal. The field lacks a training signal that captures these higher-order qualities. Concurrently, the RL systems share the same limitation as multi-agent systems: they optimize search or generation strategies without leveraging citation graph structure.

---

## Phase 5: Citation Graph Re-integration (Current Frontier, 2025–2026)

### Technical Approach

Systems that explicitly model citation graph structure alongside LLM-based generation. The graph is no longer just a retrieval expansion strategy (Phase 2's shallow BFS) but a structural backbone that organizes surveys: hierarchical graphs (SurveyG), taxonomies induced from graph topology (HiGTL), and graph transformers that jointly model text and structure (LitFM). These are the first systems since Phase 1 to place graph structure at the center, but now with LLMs handling the synthesis that Phase 1 could not.

### Key Contributions

- **SurveyG (2510.07733, 2025)**: Three-layer hierarchical citation graph (Foundation/Development/Frontier) with horizontal and vertical traversal. The most direct integration of citation graph structure into survey generation — the graph organization becomes the survey outline.

- **LitFM (2409.12177, 2024)**: Structure-aware graph transformer achieving +28.1% precision improvement. Demonstrates that joint text+graph pretraining outperforms text-only embeddings for citation-related tasks, validating Phase 1's insight with modern architectures.

- **Graphs of Research (2605.14790, 2025)**: 2-hop citation DAG as supervision signal for research idea generation. Uses SFT on citation evolution patterns — the first system to treat citation graph trajectories as training data for generation tasks.

### Reported Performance

| Paper | Metric | Value | Dataset | Notes |
|-------|--------|-------|---------|-------|
| LitFM | Retrieval precision | +28.1% | Citation retrieval benchmarks | Structure-aware pretraining |
| SurveyG | Survey structure quality | Not specified | Custom evaluation | Hierarchical graph improves organization |
| HiGTL | Taxonomy quality | Not specified | Custom dataset | GNN + hierarchical clustering for taxonomy |

### What This Phase Genuinely Achieved

This phase re-establishes that citation graph structure provides a qualitatively different signal from text content. LitFM's +28.1% precision improvement is the strongest quantitative evidence yet that structure-aware representations matter. SurveyG's three-layer hierarchy — while simplifying real research landscapes — provides a principled organizational framework that "flat" RAG systems cannot match. The phase has successfully re-integrated graph awareness into LLM-era systems.

### Open Problems This Phase Exposes (but does not solve)

1. **Graph-LLM integration remains shallow**: SurveyG uses the graph for outline structure but not for retrieval; LitFM produces better embeddings but does not generate surveys. No system has yet achieved tight integration where the citation graph simultaneously guides retrieval, organizes the outline, informs the narrative, and validates citation accuracy.

2. **Temporal graph dynamics are ignored**: Most graph-aware systems treat the citation graph as a static snapshot. Citation patterns evolve, older papers accumulate citations, and "sleeping beauties" (papers that gain citations long after publication) are invisible to static graphs. Temporal GNNs exist (2408.15371) but have not been integrated into survey generation.

3. **Hierarchy granularity is arbitrary**: SurveyG's three layers and HiGTL's tree depth are design choices, not learned from data. Different research fields may require different hierarchical granularities — a deep taxonomy for well-established fields, a shallow one for emerging areas.

4. **No graph-aware multi-agent system exists**: The most promising architectures — multi-agent (SciSage, Agentic AutoSurvey) and graph-aware (SurveyG, HiGTL) — operate in isolation. A system where the Planner agent uses SurveyG's hierarchical graph, the Researcher agent uses LitFM's structure-aware retrieval, and the Reflector agent validates citations against graph structure does not yet exist.

---

## Critical Assessment

### Claim vs. Evidence Gap

| Claimed Advance | Supporting Evidence | Assessment |
|---|---|---|
| "AutoSurvey produces human-competitive surveys" | Qualitative "competitive with human-written" rating | No standardized rubric; human evaluation on a small sample. The claim conflates structure/completeness with critical-analytic depth. Human surveys identify gaps and propose frameworks; AutoSurvey restructures known content. |
| "PaperQA2 achieves superhuman synthesis" | SOTA on LitQA2 factual QA benchmark | LitQA2 measures factual retrieval and summarization, not survey-quality synthesis. "Superhuman" applies narrowly to factual question answering, not to the broader task of survey generation. |
| "OpenScholar's 8B beats GPT-4o by 5%" | ScholarQABench comparison | Valid comparison on that benchmark, but OpenScholar's 45M paper datastore represents a massive infrastructure advantage. The result shows retrieval scale matters more than model scale — important but not an architectural breakthrough. |
| "Multi-agent systems produce dramatically better surveys" | SciSage: +32% citation F1; Agentic AutoSurvey: 8.18 vs 4.77 | Genuine improvement, but baselines are not standardized. The 8.18 vs 4.77 gap may partly reflect better prompts, better retrieval, or different evaluators rather than multi-agent architecture alone. Controlled ablation studies (same base LLM, same retrieval, same evaluation) are rarely reported. |
| "Graph-aware retrieval improves precision by 28.1%" | LitFM benchmark comparison | The strongest quantitative claim in the pool. Validated on retrieval benchmarks. However, improved retrieval precision has not been shown to translate to improved survey quality. |
| "PaSa achieves +37.78% recall through RL" | Academic search task comparison | Well-supported claim for the search task. But the link to survey generation is asserted, not demonstrated. Higher recall does not automatically yield better surveys. |
| "Citation graphs drive better survey organization" | SurveyG: "improves organization" | No quantitative survey quality metrics reported. The claim is logical but empirically unverified on standard benchmarks. |

### Methodological Weaknesses Across All Phases

1. **No shared evaluation benchmark**: The field is fractured across at least 7 benchmarks (LitQA, LitQA2, ScholarQABench, SurveyScope, Survey-Arena, SurveyBench, SurGE, ReportBench, DeepSurvey-Bench, SurveyLens, SGSimEval), each with different metrics, topics, and evaluation protocols. A system that performs well on LitQA2 may not perform well on SurveyBench. This fragmentation makes it impossible to rank systems meaningfully or track progress over time.

2. **Human evaluation is non-standardized**: Nearly every system uses custom human evaluation with different rubrics, scales, and annotator pools. Agentic AutoSurvey's 8.18/10 is not comparable to SciSage's +32% F1. The field needs a standard human evaluation rubric (like the one SurGE proposes) with inter-annotator agreement metrics.

3. **LLM-as-judge is unvalidated**: Several recent evaluations (SurveyBench's quiz-driven approach, some quality ratings) use LLMs as evaluators. The correlation between LLM-judged quality and human-judged quality is rarely reported. LLM self-evaluation is known to overestimate quality and miss subtle errors.

4. **Ablation studies are rare**: Most papers report end-to-end performance but do not ablate their key architectural components. For example: SciSage reports +32% citation F1, but is this from the reflect-while-writing design, the multi-agent architecture, the specific prompts, or the citation chaining? Without ablations, the source of improvement is unclear.

5. **Citation accuracy is under-evaluated**: Despite citation accuracy being a claimed focus (SciSage, CiteGuard), no system uses a rigorous hallucination audit — systematically verifying each citation claim against the source paper. ReportBench comes closest but focuses on citation quality rather than factual fidelity. The rate of hallucinated citations in auto-generated surveys is unknown.

### Evaluation Comparability Crisis

The proliferation of evaluation benchmarks and metrics has created a crisis of comparability:

- **Task mismatch**: LitQA/LitQA2 evaluate factual QA; ScholarQABench evaluates literature QA; SurveyScope evaluates citation F1; SurveyBench evaluates quiz-enabling quality; SurGE evaluates 4 dimensions of survey quality. These measure different constructs with unknown correlations.

- **Metric proliferation**: Systems report quality scores (8.18/10), F1 scores (+32%), recall improvements (+37.78%), precision improvements (+28.1%), and qualitative ratings ("competitive with human"). No mathematical framework exists to compare these across papers.

- **Dataset overlap**: It is unclear whether benchmarks share topics, papers, or evaluation rubrics. A system trained or tuned on one benchmark may have leaked information about another.

- **Resolution path**: The field needs a unified evaluation framework with (a) a shared benchmark of survey topics across disciplines, (b) standardized human evaluation rubrics with validated inter-annotator agreement, (c) a core set of automatic metrics (citation accuracy, coverage, coherence) benchmarked against human judgment, and (d) a leaderboard approach where all systems are evaluated under identical conditions. SurveyLens (discipline-aware benchmark, 2602.11238) and DeepSurvey-Bench (academic value dimension, 2601.15307) represent steps in this direction but have not yet achieved unified adoption.

### Blind Spots

1. **Critical-analytic depth**: No system evaluates whether the generated survey provides original analysis — identifying contradictions in the literature, proposing new taxonomies, or suggesting future research directions. Every evaluation measures surface quality (coherence, coverage, citation accuracy). The most valuable function of a human-written survey — critical synthesis — is entirely unmeasured.

2. **Citation hallucination rates across the field**: Despite citation accuracy being a central concern, no paper in the pool reports a systematic audit of hallucinated or misattributed citations in generated surveys. The most relevant work (CiteGuard, 2510.17853) achieves 65.4% on CiteME (vs. human 69.7%), but CiteME is a specific benchmark for citation attribution, not a survey-level hallucination audit. The field operates with unknown error rates.

3. **Domain transferability**: Almost all systems are evaluated on CS/AI papers (arXiv categories, NLP conferences). Whether these methods transfer to medicine (clinical trial literature), physics (preprint + journal culture), or humanities (monograph-heavy citation patterns) is untested. SurveyLens begins to address this with 10 disciplines, but the field remains overwhelmingly CS-focused.

4. **Temporal recency bias**: Analysis papers (2305.18554, 2402.12046) demonstrate that NLP citation practices exhibit strong recency bias (62% of citations to papers <5 years old). Automated survey systems, which retrieve primarily from recent arXiv papers, may amplify this bias. No survey-generation system explicitly measures or mitigates temporal coverage gaps.

5. **Prestige/status bias (Matthew Effect)**: Citation networks exhibit the Matthew Effect (famous papers get cited more). Systems that traverse citation graphs preferentially discover well-cited papers, potentially missing high-quality work from less-established authors, venues, or institutions. The one paper that explicitly addresses this (2411.05584) is an analytical study, not a mitigation strategy.

### What a Truly Next-Generation System Would Need

1. **Deep graph-LLM integration, not bolted-on awareness**: The citation graph should simultaneously serve as (a) the retrieval backbone (structure-aware embeddings like LitFM), (b) the organizational scaffold (hierarchical traversal like SurveyG), (c) the citation validation substrate (graph-traversal-based fact-checking), and (d) the narrative guide (tracing idea evolution through citation chains). No current system integrates even two of these functions.

2. **Learned traversal policies, not fixed heuristics**: The number of citation hops, direction (forward/backward), and branching factor should be learned per-topic and per-section — some topics require deep backward chaining (tracing foundations), others require broad forward chaining (mapping recent developments). PaSa's RL policy demonstrates the feasibility of learned strategies; this should be extended to hierarchical graph traversal with stopping criteria.

3. **Multi-agent architecture with graph-aware agents**: A system where the Planner uses a learned taxonomy (HiGTL-style) to structure the outline, the Researcher uses a learned traversal policy (PaSa-style) to search the citation graph, the Writer uses structure-aware embeddings (LitFM-style) to situate each paper in context, the Reflector validates citations against the graph structure, and the Refiner optimizes for survey-level coherence. No system today combines graph awareness with multi-agent coordination.

4. **Rigorous citation hallucination audit**: Every generated survey should be accompanied by a systematic verification of each citation against the source paper's content, with confidence scores and flagged claims. This is not an optional quality metric — it is a prerequisite for practical deployment. No system currently provides this.

5. **Multi-dimensional quality evaluation beyond surface metrics**: The evaluation framework should measure not just coherence, coverage, and citation accuracy but also (a) critical-analytic depth (does the survey identify contradictions, gaps, or opportunities?), (b) novelty of synthesis (does it provide an organization that differs from existing surveys?), (c) field-situatedness (does it correctly identify which questions are settled and which are contested?), and (d) bias awareness (does it acknowledge the limitations of its own retrieval and selection process?). These dimensions require new benchmarks and evaluation methodologies.

6. **Temporal and prestige bias mitigation**: The system should explicitly measure and report temporal coverage distribution (to detect recency bias), citation concentration (to detect Matthew Effect), and venue/author diversity. It should adjust its retrieval strategy to ensure coverage of foundational work, non-English sources, and less-visible contributors. No system currently reports any bias metric.
