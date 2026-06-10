# Gate Contract

Every stage ends with a **gate** node. The gate is the pipeline's hard constraint:
it does not produce new research content, it *checks* that the stage actually met
its contract, and it emits a verdict the next stage (or the human) can trust.

A gate must be honest. It checks what is **actually on disk**, not what the
upstream handoff claims. If a required artifact is missing or a rule fails, the
gate says so — a truthful `insufficient` is the gate doing its job, not a failure.

One file per stage at `project_dir/<NN>_<stage>_gate.md`.

## Required sections

- `stage`: which stage this gate closes.
- `inputs_checked`: the artifact paths the gate read (so the check is auditable).
- `checklist`: one row per hard rule for this stage — `rule`, `result`
  (`pass` | `fail`), and a one-line `evidence` pointing at the artifact/field that
  satisfies or violates it. Every rule the stage's gate prompt lists must appear.
- `verdict`: one of
  - `strong` — every rule passes and the stage's output is clearly above bar.
  - `acceptable` — every *required* rule passes; some quality rule is weak but the
    stage may proceed (note the weakness).
  - `insufficient` — at least one required rule fails; the stage must iterate (a
    re-run) before proceeding. Name what to fix.
  - `blocked` — a structural dead-end the stage cannot fix by iterating; routes a
    *pivot* to an earlier stage (e.g. ground finds the idea fully scooped). Name
    the target stage and why.
- `checkpoint`: the human checkpoint this stage surfaces, or `none`.
- `next`: the recommended action — `promote` (proceed), `iterate` (re-run this
  stage), or `pivot:<stage>` (re-run an earlier stage), matching the verdict.

## State update

After writing the gate file, the gate node updates `project_dir/state.md`: set the
stage's line to `status: ok|blocked` + `verdict: …` + today's date, refresh the
relevant `refs`, and append one `timeline` line. The gate is the only place a
stage's `state.md` line is written, so `state.md` always reflects real verdicts.

## Honesty rules

- Never write `pass` for a rule whose evidence you did not find on disk.
- Never upgrade a verdict to keep the chain moving. `insufficient`/`blocked` with a
  concrete reason is the correct, valuable output when the work is not there.
- A `strong`/`acceptable` verdict means you opened every `inputs_checked` file and
  the `checklist` reflects their real contents.

## Example

```
stage: explore
inputs_checked:
  - runs/20260609T120600Z/06_idea_pool.md
  - runs/20260609T120600Z/papers/
checklist:
  - rule: every top-K idea has a complete StorySpine
    result: pass
    evidence: 06_idea_pool.md — 5/5 ideas have all StorySpine fields
  - rule: every top-K idea has >=2 candidate paper angles
    result: pass
    evidence: 06_idea_pool.md — angles listed per idea
  - rule: every idea traces to a wonder signal
    result: pass
    evidence: 06_idea_pool.md — wonder_signal field non-empty
  - rule: every idea has an anchor-alignment sentence
    result: pass
    evidence: 06_idea_pool.md — anchor_alignment present
  - rule: ranked by batch ranking, not hand-picked to one
    result: pass
    evidence: 5 ranked candidates retained, not pre-filtered to 1
verdict: acceptable
checkpoint: taste_selection
next: promote
```
