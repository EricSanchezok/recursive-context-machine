# RICA Development Guide

This document encodes the development rules for RICA. All agents editing this codebase
must follow these conventions. Do not override unless explicitly instructed by the user.

## Comments

- Comments explain **why**, not **what**. The code already says what it does.
- Doc comments on public APIs are acceptable only when the purpose is not obvious from
  the name and signature alone.
- No inline comments that restate the next line of code.
- No "defaults to ..." or "(... minutes)" decay comments that specify values — they go
  stale on every config change. Types and const names are the source of truth.
- Section dividers like `// ── Foo ──` are not allowed in files under 200 lines.
- No TODO comments in published docs — use real issue tracking.

## Naming

- Single-letter variable names (`a`, `b`, `c`, `e`, `r`, `s`, `v`, etc.) are not
  allowed. Every variable must describe what it holds. The only exceptions are:
  `i` / `j` for loop indices, and `f` in closures like `|f| f.id == id`.
- Abbreviated names are acceptable when they are domain-standard and unambiguous
  across the codebase: `frag` (fragment), `ctx` (context), `env` (environment),
  `msg` (message), `args` (arguments), `req` (request). If in doubt, spell it out.
- Single-letter generic parameters are not allowed. Use `impl Trait` in argument
  position, or a descriptive name.
- Enum variants and function names must be self-explanatory without a doc comment.
  If a variant needs a comment to explain its purpose, rename it.
- `Catch`/`Drop` style verb confusion is not allowed. "Catch" is error handling,
  "Drop" is deallocation. Use `Activate`/`Deactivate` for tool toggling.
- Avoid `XxxInfo`, `XxxDetail`, `XxxData` — find a real name.
- The same concept must be named consistently across all files. If `completion.rs`
  calls it `endpoint`, `machine.rs` must not call it `base_url` for the same thing.

## Architecture

- `reactor` is internal to `machine`. It is a plain async function, not a trait,
  not injected, not visible to `accelerator`.
- `machine` does not know about provider presets (DeepSeek, Groq, etc.).
  Protocol + endpoint is sufficient.
- Policy receives `&self` — use atomics for internal state, not `Mutex`.
- All execution (LLM calls, tool calls) uses `tokio::time::timeout`.
  Defaults are defined once as module-level constants.

## Logging

- Use `tracing` macros (`debug!`, `info!`, `warn!`, `trace!`) — never `println!`
  or `eprintln!` for operational output.
- Every failed outcome must log at `warn!` with structured fields (`?error` or named
  fields), not a bare string.

## Execution

- Do not autonomously modify code unless the user explicitly asks.
- When the user asks you to change code, wait for their confirmation before
  starting. Do not pre-emptively implement.
- Every batch of changes must be committed before continuing to the next
  topic. Do not accumulate uncommitted work.
- Commits must be atomic per logical change. Do not squash unrelated
  refactors into a single commit.
- Only commit changes you authored. Do not include, revert, or modify
  other people's work (e.g., Cargo.lock updates, dependency bumps,
  files created by other agents or users) unless explicitly instructed.

## Testing

- Tests go in `tests/`, not inline modules.
- Test tools and test policies are defined in the test file, not in `src/`.
- Never test mock infrastructure (helpers, builders, replay policies). Tests must
  exercise the real crate API — `Machine::run`, `Context::append`, `Fragment::system`,
  etc. If a test only asserts the behaviour of a `SeqPolicy` helper, delete it.
- Never test getters (`tool_name_and_description`, `new_context_is_empty`). Test
  **behaviour**: "does this produce the right side-effect?" not "does this struct
  field match what I just set it to?"
- Prefer edge cases over CRUD enumeration. One test for "remove and verify length"
  is enough — don't write three variations.
- Every `#[should_panic]` test must verify the panic message (`expected = "..."`).
- Tests must be fast. No test may depend on external services (LLM APIs) or long
  timeouts. Tests that currently depend on the reactor loop use SeqPolicy without
  `Action::Halt` to avoid HTTP calls.
