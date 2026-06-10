# AnchorSpec Contract

The `AnchorSpec` freezes the research direction so the rest of the pipeline cannot
drift. It is written once by the explore stage's `anchor` node into
`project_dir/00_anchor.md`, and every later stage reads it as the fixed reference
for "are we still solving the right problem?".

The topic comes from the node's **purpose** (the CLI `--purpose` flag, surfaced in
context as a `purpose`-tagged message). Use that text as the research direction;
do not look for an environment variable or a topic file.

## Required fields

- `anchor`: the research direction in one or two sentences — the user's problem,
  in their framing.
- `bottleneck`: the specific weakness/obstacle that makes this problem worth
  attacking now.
- `non_goals`: what this project is explicitly **not** trying to do (keeps scope
  honest).
- `constraints`: the practical envelope — compute budget, data available, target
  venue/deadline if known. Mark unknowns as unknown.
- `success_condition`: what evidence would convince a skeptical reader the problem
  is meaningfully addressed.
- `concept_seeds`: 5–10 keyword/phrase seeds (mix of methods, problems,
  benchmarks, adjacent fields) for the landscape scouts to search from.

## Rules

- Do not generate ideas, score anything, or survey literature here — only freeze
  the anchor and create the `project_dir`.
- If the purpose is too broad to anchor honestly, still write the best anchor you
  can and record the breadth risk under `non_goals`/`constraints`; do not invent a
  narrow problem the user did not ask for.
