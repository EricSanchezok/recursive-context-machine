# Landscape & Wonder Contract

Explore builds its ideas from **friction**, not decoration. Two artifacts capture
that: the landscape (what exists) and the wonder log (what doesn't fit). Novel
ideas come from the mismatches.

## Landscape — `project_dir/01_landscape.md`

Written by the `landscape` merger from the four scouts' artifacts.

- `landscape_overview`: the terrain in a few sentences — the dominant approaches
  and the through-line of the field around the anchor.
- `method_families`: the main approach families, each with 1–2 representative
  papers (arXiv id + one-line method). Grouped, with rough dates.
- `industry_signals`: what practitioners actually build/complain about (from the
  industry scout), where it diverges from what papers claim.
- `wonder_signal_candidates`: the raw friction the scouts surfaced — anomalies,
  contradictions, missing mechanisms, brittleness, missing measurements — each as
  one line with its source. These are *candidates*; `wonder` sharpens them.
- `core_paper_worklist`: 5–10 papers worth reading in full to ground ideation —
  arXiv id + title + one line on why it is core. This drives the `read_core`
  paper-card fan-out.
- `failed_approaches`: a banlist — approaches already tried and shown not to work,
  so ideation does not re-propose them.

## Wonder log — `project_dir/02_wonder.md`

Written by `wonder` from the landscape + the core paper cards. Each surviving
signal is one row:

| field | meaning |
|---|---|
| `signal_type` | anomaly \| contradiction \| missing_mechanism \| brittleness \| missing_measurement \| cross_domain_resonance |
| `source` | the paper(s)/observation it comes from (arXiv id or card) |
| `what_doesnt_fit` | the specific mismatch with the prevailing story |
| `why_it_matters` | which assumption would be wrong if this is real |
| `sharpened_hypothesis` | the hidden variable / broken assumption / missing control that would explain it |

A signal survives into the wonder log only if it passes sharpening: there is a
hidden variable, a probably-false assumption, or a missing measurement behind it —
not just "paper X didn't test setting Y".

## Rules

- Ground every signal in a real paper/observation; cite the source. No invented
  anomalies.
- Prefer fewer, sharper signals over a long list of weak ones.
