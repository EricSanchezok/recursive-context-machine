# Review Contract

The review stage stress-tests the finished paper the way a top venue would, and audits
it for integrity. Its output is a verdict plus **routed findings**: each problem is
tagged with the stage that owns it, so fixing it is unambiguous.

## Reviewer Independence Protocol (hard requirement)

Score inflation comes from reviewers who see each other or see prior rounds. So:

- Each simulated reviewer is a **fresh, independent** node. It reads **only the paper**
  (`68_paper.md`) and forms its own conclusion — it does not see the other reviewers,
  the technical report, the claims file, or any "we fixed X" framing.
- The **auditor** is the one exception: to verify numbers and citations it reads the
  paper **and** the technical report (`53_tech_report.md`) as the evidence base.
- No leading language: reviewers are never told what changed or what to conclude.
- (Cross-model review would further reduce self-review bias; with a single configured
  model the independence here is structural — fresh, zero-context, no shared findings.)

## Dimensions

| # | Dimension | Reads | Question |
|---|---|---|---|
| 1 | data truthfulness | paper + report | Does every number in the paper trace to the technical report (→ `results/`)? |
| 2 | citation accuracy | paper (+ report) | Do cited works exist and say what the paper claims? Any hallucinated/mischaracterised? |
| 3 | claim–evidence | paper | Is every claim supported, honestly scoped, traceable; caveats present? |
| 4 | soundness | paper | Is the methodology rigorous; are comparisons fair? |
| 5 | novelty & significance | paper | Does the contribution survive the reduction test; is the "why it matters" honest and specific? |
| 6 | clarity & reproducibility | paper | Is it well-written; could someone reproduce it from the text? |
| 7 | simulated venue review | paper | Would Reviewer 2 accept it? Rating 1–10 with strengths/weaknesses. |

Dimensions 1–2 are the auditor's; 3–7 are split across the independent reviewers.

## Outputs

- `78_review.md`: per-dimension verdict, each reviewer's scores/strengths/weaknesses, and
  the **synthesised concern list** — take the *intersection* of valid concerns (raised by
  ≥2 reviewers) and the *union* of CRITICAL issues (any reviewer). The simulated rating.
- `79_review_gate.md` (per `schema/gate.md`): the verdict + routing.

## Verdict and routing

- `verdict`: `strong` (ready; integrity dims pass, rating high) | `acceptable` (ready
  after minor PAPER-FIX) | `insufficient` (revise — fixable in compose) | `blocked`
  (a rollback is required).
- Every finding is routed to exactly one owner:
  - `PAPER-FIX` → fix in `compose_paper` (prose, structure, missing caveat).
  - `ROLLBACK:compose_paper` → claims overclaimed/unsupported relative to the report.
  - `ROLLBACK:compose_report` → the report itself has an evidence/alignment gap.
  - `ROLLBACK:external` → genuinely missing evidence (needs more experiments) — back to
    the external design/realize/experiment work.

## Hard rules

- Any data-truthfulness `INCONSISTENT` or any `HALLUCINATED`/`MISCHARACTERISED` citation
  is a blocker — never `strong`/`acceptable` while one stands.
- The gate reports the honest synthesised verdict; it never upgrades to "ready" to move
  on. A low simulated rating with concrete weaknesses is the review working.
