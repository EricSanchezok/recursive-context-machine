# PaperCard Contract

A `PaperCard` is a compact, **full-text-grounded** summary of one paper, read
*with the survey's research anchor in mind*. Cards replace abstract-only notes for
the papers that matter most, and are the evidence the survey is written from.

One file per paper at `run_dir/cards/<arxiv_id>.md`.

## Required sections

- `header`: arXiv id, title, authors (if available), year/venue.
- `problem`: the problem the paper addresses (1–2 sentences).
- `method`: the core mechanism in enough detail to *write about* it — the key
  idea, how it works, what is novel. Grounded in the body, not just the abstract.
- `results`: the concrete empirical claims (datasets, metrics, numbers) **with
  their caveats** (which baseline, which setting, which architecture). Note which
  numbers are from the full text vs only the abstract.
- `anchor_relevance`: why this paper matters to the survey's anchor/topic and
  where it sits in the field's structure.
- `transfer`: the cross-domain insight — *could this method or idea transfer to
  the anchor's direction? how, and what would block it?* This is the payoff of
  reading the full text through the anchor lens. If there is no clear transfer,
  say so in one line.
- `evidence`: `full_text` | `partial` | `abstract_only` — be honest about how
  much of the body you actually read and parsed.

## Rules

- Ground every claim in the paper. Never fabricate numbers, methods, or results.
- If the PDF could not be parsed, set `evidence: abstract_only` and say so under
  `results` — do not pretend to have read the body.
- Keep it compact (≈ a long abstract plus the transfer note), not a reproduction
  of the paper.
