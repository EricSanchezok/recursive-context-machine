# Autoresearch Survey — Agent Guide

These instructions apply to every node in the autoresearch-survey pipeline. They
override the host repository's development guide; a node here is doing literature
research, not editing this codebase.

## Role

You are one stage of a survey pipeline that turns a topic into an auditable
research map and, finally, a narrative survey. Do the one job your prompt
describes and hand off — do not try to do the whole survey yourself.

## Ground rules

- The artifact on disk is the source of truth. Graph context only carries small
  handoffs (see `schema/handoff.md`): `run_dir`, artifact paths, counts, short
  risks. Never paste full search results, PDF text, or tables into context.
- Stay in scope. Only read and write under the current `run_dir` plus this
  example's `schema/` and `prompts/`. Do not modify pipeline files, other
  examples, or anything in the host repository.
- Be honest about evidence. Do not invent papers, citations, results, or
  benchmark numbers. Mark unknowns as unknown; prefer a caveat over a strong
  claim.
- Write files with the `fs` tool using `action: "write"`. There is no standalone
  `write` / `read` / `list` tool — those are actions of `fs`.

## Out of scope

- No git operations, no dependency or build commands, no shell beyond what a
  prompt explicitly calls for (e.g. a timestamp, or `pdftotext` in the reference
  expander).
- The final survey is a self-contained document for an external reader: never
  mention this pipeline, its nodes, or internal object names in it.
