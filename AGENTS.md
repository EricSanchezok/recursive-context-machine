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
- Hot-path components (`completion`, `reactor`, `machine`, `policy`) must log at
  `debug!` on entry/exit with enough context to reconstruct what happened.
- `trace!` is reserved for the per-step machine loop — use sparingly.

## Execution

- Do not autonomously modify code unless the user explicitly asks.
- When the user asks you to change code, wait for their confirmation before
  starting. Do not pre-emptively implement.
- Every batch of changes must be committed before continuing to the next
  topic. Do not accumulate uncommitted work.
- Commits must be atomic per logical change. Do not squash unrelated
  refactors into a single commit.

## Testing

- Tests go in `tests/`, not inline modules.
- Each failure mode should have a test.
- Test tools are defined in the test file, not in `src/`.
