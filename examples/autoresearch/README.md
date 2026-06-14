# RCM Autoresearch Pipeline

This example re-expresses the **full autoresearch lifecycle** as a system
assembled from RCMs. Where [`autoresearch-survey`](../autoresearch-survey) builds
one *survey paper for a human reader*, this example treats autoresearch itself as
the system: the seven stages a real research project moves through —

```
explore → ground → design → realize → experiment → compose → review
```

— each become a **separate, independently runnable `.rcm` stage graph**, wired
from small single-job units, and assembled into one end-to-end graph.

It is a from-scratch RCM reimplementation of a plugin-based autoresearch SOP, made
**more intuitive, cleaner, more standard, and harder-constrained**: stages are
explicit graphs instead of prose DAGs, and every stage ends with a typed **gate**
that emits a machine-checkable verdict instead of relying on prompt discipline
alone.

## Design principles

- **Each stage is one `.rcm`, each stage runs alone.** `explore.rcm`,
  `ground.rcm`, … are runnable on their own; `autoresearch.rcm` chains them.
- **Build the graph, then judge it.** Within a stage: parallel attempts → merge →
  independent review → a gate. No node both produces and rubber-stamps its own work.
- **Files are the source of truth; context is just a handoff.** Everything lands
  under `project_dir` = `runs/<timestamp>/`; graph context carries ≤15-line
  handoffs. See [STATE.md](STATE.md).
- **Depth from full text, not abstracts.** Papers that matter are read in full,
  *through the research anchor's lens*, into a shared paper-card knowledge base
  (`papers/`), grown by citation-graph expansion. (Both ideas are carried over
  from the survey example.)
- **Hard gates, not vibes.** Each stage's `gate` checks required artifacts against
  that stage's contract and emits `strong | acceptable | insufficient | blocked`.
- **Honesty over completion.** No invented papers, results, or numbers; a truthful
  "insufficient" beats a fabricated "pass". See [AGENTS.md](AGENTS.md).

## Status

Built: **explore**, **ground**, **compose** (two phases — technical report, then
claim-driven paper), and **review** — together with all shared conventions (state
model, handoff/gate/paper-card schemas, the shared paper-card and section-writer
workers).

**`design`, `realize`, and `experiment` are intentionally done outside this
pipeline** (by you / your tooling). Compose picks up from their hand-in — code,
results, and notes left under `project_dir` — per
[`schema/external_inputs.md`](schema/external_inputs.md).

## Running

The topic is the project's **purpose** — the research anchor. Inject it with the
CLI `--purpose` flag on the explore stage. Run from this example's directory so
that `schema/`, `prompts/`, and this `AGENTS.md` resolve correctly and `runs/`
lands here.

When this pipeline is launched from Research Portal through holos-research, the
runner must pass `RDC_URL` and `RDC_RESEARCH_ID` in the process environment and
run with `--stream` so portal-gateway can mirror RCM node events through the
AgentBay MCP bridge. Use `--run-dir <path>` for queued or concurrent runs so two
projects never recover the same newest `runs/*` directory.

```sh
cd examples/autoresearch
export DEEPSEEK_API_KEY=sk-...

# Stage 1 — explore: topic → ranked idea pool with grounded StorySpines
../../target/release/accelerate run rcm/explore.rcm \
  --purpose "test-time scaling for small language models under a fixed latency budget" \
  --speed 0 --context

# Stage 2 — ground: position the selected idea against the closest prior work
../../target/release/accelerate run rcm/ground.rcm --speed 0 --context

# (design / realize / experiment happen OUTSIDE this pipeline — drop their
#  outputs under runs/<ts>/ as code/, results/, method.md, experiments.md;
#  see schema/external_inputs.md)

# Compose Phase 1 — technical report: rigorous, code- and results-aligned record
../../target/release/accelerate run rcm/compose_report.rcm --speed 0 --context
# Compose Phase 2 — paper: claim-driven, de-engineered rewrite
../../target/release/accelerate run rcm/compose_paper.rcm --speed 0 --context
# (or both phases in one shot)
../../target/release/accelerate run rcm/compose.rcm --speed 0 --context

# Review — independent panel + audit + routed findings
../../target/release/accelerate run rcm/review.rcm --speed 0 --context
```

A later stage run with no incoming context recovers the newest `runs/*` as its
`project_dir` (and says so under `risks`). Between explore and ground there is a
**taste-selection** checkpoint: the explore gate surfaces the top-K ideas; record
your pick in `state.md` (or let ground default to rank-1 and note it).

Parse / inspect without an API key:

```sh
../../target/release/accelerate parse rcm/explore.rcm        # AST as JSON
../../target/release/accelerate inventory .                  # list all units/tools/prompts
```

## Stages

| # | Stage | Question | Main output | Gate checks |
|---|---|---|---|---|
| 1 | `explore` | Which directions are worth pursuing? | ranked idea pool + StorySpines | every top-K idea has a complete StorySpine, ≥2 paper angles, a wonder-signal origin, anchor alignment |
| 2 | `ground` | What contribution can this honestly be? | grounded StorySpine + closest-work positioning | grounded angle + 2–3 closest works with concrete differences + a coherent ~3-facet contribution set (scope / artifact / results) + minimum evidence |
| 3–5 | `design` · `realize` · `experiment` | *(external)* mechanism, code, and experiments | code + results + method/experiment notes under `project_dir` | done outside the pipeline; hand-in per [`external_inputs.md`](schema/external_inputs.md) |
| 6a | `compose_report` | What was actually built and measured? | rigorous, code-aligned **technical report** | every result number traces to a `results/` file; method aligns to `code/` |
| 6b | `compose_paper` | How is this told honestly, as a paper? | claim-driven, **de-engineered** paper | a focused ~3-facet claim set, all numbers trace to the report, intro arc, de-engineered |
| 7 | `review` | Is it ready, and what must change? | independent reviews + audit + routing | reviewer-independence protocol; integrity dims pass; findings routed to their owner stage |

## Run artifacts

See the [artifact map in STATE.md](STATE.md#artifact-map). In short, each stage
writes numerically-prefixed markdown under `project_dir`, plus the shared
`papers/` knowledge base, and updates `state.md`. These are runtime artifacts and
are gitignored.
