# Explicit OpenAI-compatible thinking mode

Artifact-Version: 1
Status: Implemented

## Intent

An RCM model declaration that explicitly enables or disables thinking must send the matching OpenAI-compatible request parameter while preserving the historical request shape for declarations that omit the field. This makes a non-thinking fallback actually disable providers whose thinking mode defaults to enabled and makes canary behavior deterministic without embedding provider hostnames in the machine crate.

## Contract

- The release artifact carrying this contract is versioned `0.2.22`; downstream pins must use its published archive checksum.
- `thinking = "true"` on an OpenAI-protocol model sends `thinking.type = "enabled"` and preserves reasoning content for later tool-bearing requests.
- `thinking = "false"` on an OpenAI-protocol model sends `thinking.type = "disabled"` and does not synthesize reasoning history.
- Omitting `thinking` sends no provider-specific thinking parameter and retains the historical default behavior.
- Anthropic and Gemini requests never receive the OpenAI-compatible `thinking` object through this mechanism.
- Programmatic `Model` callers remain source compatible; the existing `thinking` field retains its history-encoding role and additive methods opt into an explicit OpenAI-compatible mode.
- No provider endpoint, model name, prompt content, tool result, or credential is used to select the request shape.

## Plan

Preserve declaration presence in the CLI AST, compile explicit OpenAI declarations into reserved model metadata, and project that metadata into `CompletionRequest.additional_params` as an enabled or disabled `thinking` object.

## Verification

- Parser tests distinguish explicit false from omission.
- Compiler tests retain enabled, disabled, and implicit modes.
- Machine tests assert the exact request parameter and its absence for implicit or non-OpenAI models.
- Workspace tests, documentation tests, formatting, Clippy, and repository gates pass.

## Evidence

- [Parser contract tests](../../crates/cli/tests/parser.rs)
- [Compiler contract tests](../../crates/cli/tests/compile.rs)
- [Request construction tests](../../crates/machine/tests/completion.rs)
- [Thinking-mode request decision](../decisions/0005-explicit-openai-thinking-mode.md)
