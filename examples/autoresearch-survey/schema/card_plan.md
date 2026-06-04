# CardPlan Contract

`CardPlan` selects which papers the per-paper `paper_cards` stage reads in full.
It is the budget gate between the ranked pool and full-text reading.

## Constant

- `max_fulltext_papers = 100` — the maximum number of papers the plan may list.
  Reading full text is the expensive step, so this caps cost. Tune the budget
  here, in the contract — not by editing the prompt text.

## Output

The planner's final message is a JSON array (consumed by the `paper_cards` map),
one element per selected paper:

- `run_dir`: the run directory, verbatim, in every element.
- `id`: the paper's arXiv id — must exist in the ranked pool; never invented.
- `title`: the paper title.
- `why`: one line — its role + why it matters to the anchor. Tag cross-domain
  picks with "cross_domain transfer".

## Selection priority

Within `max_fulltext_papers`, prefer: core_set / method papers with strong
topic_fit; especially `abstract_only` ones (their bodies are unread — the whole
point); a few already-`full_text` anchors (re-read through the anchor lens); and
several `transfer_set` (role: cross_domain) candidates with strong
`transfer_potential`. Skip in-domain boundary / low-topic_fit / off-scope papers —
but transfer_set candidates are not off-scope; they have their own lane.
