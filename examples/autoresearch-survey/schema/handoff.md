# Handoff Contract

Every node ends its turn with a **handoff**: the final assistant message that the
graph carries on the `context` channel to the next node. The handoff is *not* the
artifact — full data always lives in `run_dir` files. The handoff only tells the
next node where to look and how the run is doing.

Keep it short: at most ~15 lines, one `key: value` per line.

## Required keys

- `run_dir`: the run directory, verbatim. **Must be the first line.** The whole
  chain depends on this to find upstream artifacts.
- `artifact`: the primary file this node wrote, relative to repo root. Producer
  nodes always set this; a pure gate may omit it.
- `status`: `ok` | `partial` | `blocked`.

## Optional keys (include only what applies; omit empty ones)

- `counts`: small integers as `k=v` pairs, e.g. `total=42, core=8, method=12`.
- `ids`: up to 5 representative arXiv IDs / titles / names.
- `verdict`: judge or gate nodes only — `strong` | `acceptable` | `insufficient` | `blocked`.
- `risks`: up to 3 short caveats (scope drift, benchmark mismatch, missing evidence…).
- `next`: up to 3 follow-up queries or actions for another retrieval loop.

## run_dir invariant

`run_dir` is the single key the whole chain shares. Rules:

1. Prefer `run_dir` from incoming context.
2. If incoming context has no `run_dir` (e.g. running this unit standalone), you
   may fall back to the newest run directory: `fs list` the `runs` directory
   **directly** (listing its parent hides it — `runs/` is gitignored) and pick
   the last entry, since the UTC timestamp names sort chronologically. You
   **must** then add a `risks:` line saying the run_dir was recovered from disk,
   not from context. Never switch run_dir silently.
3. `anchor` is the only node that creates a new `run_dir`.

## Example

```
run_dir: runs/20260530T144227Z
artifact: runs/20260530T144227Z/02a_method_candidates.md
status: ok
counts: candidates=14
ids: 2306.14048, 2401.18079, 2404.06654, 2503.24000, 2308.14508
risks: 2 titles lacked arXiv IDs; kept by title
```

Do not paste full search results, PDF text, tables, or artifact bodies into the handoff.
