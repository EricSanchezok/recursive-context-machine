# Section Contract

The survey is built **section by section**: an outline node emits a JSON list of
section specs, a `map` expands each section into its own file, and an assembler
stitches them into the final survey. This keeps each writing step's context small
while letting every section go deep.

## Section spec (outline → expansion)

The outline's **final message is a JSON array**, one element per section, in the
order they should appear. Each element:

- `run_dir`: the run directory, repeated verbatim in every element.
- `n`: two-digit order string, e.g. `"01"` — controls file name and final order.
- `slug`: short kebab id, e.g. `"feedback-alignment"`.
- `title`: the section heading.
- `thesis`: one-line claim the section argues (its part of the through-line).
- `card_ids`: arXiv ids (present in `run_dir/cards/`) the section draws on.
- `transfer_angle`: a one-line cross-domain / anchor-transfer idea to weave in
  (may be empty).

A typical list: introduction, an evolution / research-arc section, one section
per method family, a synthesizing comparison, open problems, conclusion.

## Section file (expansion → assembler)

- Path: `run_dir/sections/<n>_<slug>.md`, starting with `## <title>`.
- Detailed, flowing prose grounded in the cards; cites inline by `(Author, year)`
  or `[arxiv_id]`. The assembler builds the single reference list.
- Self-contained: no internal stage/object names (see `schema/survey.md`).
