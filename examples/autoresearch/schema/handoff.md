# Handoff Contract

Every node ends its turn with a **handoff**: the final assistant message that the
graph carries on the `context` channel to the next node. The handoff is *not* the
artifact — full data always lives in `project_dir` files. The handoff only tells
the next node where to look and how the stage is doing.

Keep it short: at most ~15 lines, one `key: value` per line.

## This must be your LAST message — and it carries project_dir downstream

The graph forwards only your **last message** to the next node. If your last
message is a tool result (e.g. a file-write receipt) or any text without
`project_dir`, the next node receives no `project_dir` and has to guess it. So:

- Do all your file writes and tool calls **first**.
- Then send the handoff as a **plain text message with no tool call**, so it is
  the final fragment in your context.
- `project_dir` must be the **first line**, verbatim, in the exact form you
  received it (e.g. `runs/20260609T120600Z`) — do not add a prefix, do not make
  it absolute. The next node uses it as-is to build paths.

## Required keys

- `project_dir`: the project directory, verbatim. **Must be the first line.** The
  whole chain depends on this to find upstream artifacts.
- `artifact`: the primary file this node wrote, relative to the example root.
  Producer nodes always set this; a pure gate may point at its gate file.
- `status`: `ok` | `partial` | `blocked`.

## Optional keys (include only what applies; omit empty ones)

- `verdict`: gate nodes only — `strong` | `acceptable` | `insufficient` | `blocked`.
- `counts`: small integers as `k=v` pairs, e.g. `ideas=12, top_k=5, cards=8`.
- `ids`: up to 5 representative ids/titles (arXiv ids, idea ids like `idea_003`).
- `checkpoint`: the human checkpoint this stage surfaces, e.g. `taste_selection`.
- `risks`: up to 3 short caveats (scope drift, scooped claim, missing evidence…).
- `next`: up to 3 follow-up queries or actions.

## project_dir invariant

`project_dir` is the single key the whole chain shares. Rules:

1. Prefer `project_dir` from incoming context.
2. If incoming context has none (e.g. running a later stage standalone), you may
   fall back to the newest run directory: `fs list` the `runs` directory
   **directly** (listing its parent hides it — `runs/` is gitignored) and pick the
   last entry, since the UTC timestamp names sort chronologically. You **must**
   then add a `risks:` line saying `project_dir` was recovered from disk, not from
   context. Never switch it silently.
3. `anchor` (in the explore stage) is the only node that creates a new
   `project_dir`.

## Example

```
project_dir: runs/20260609T120600Z
artifact: runs/20260609T120600Z/07_explore_gate.md
status: ok
verdict: acceptable
counts: ideas=14, top_k=5, cards=8
ids: idea_003, idea_007, idea_011
checkpoint: taste_selection
```

Do not paste full search results, paper text, tables, or artifact bodies into the
handoff.
