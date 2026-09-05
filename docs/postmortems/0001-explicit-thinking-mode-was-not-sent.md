# Explicit thinking mode was not sent

Artifact-Version: 1

## Executive summary

A production release canary received HTTP 400 from a thinking-capable OpenAI-compatible provider because RCM treated an explicit `thinking` declaration only as a history-encoding hint and did not send the corresponding request parameter; omission and explicit false were also indistinguishable. The permanent correction preserves declaration presence, emits deterministic enabled or disabled wire control only for explicit OpenAI declarations, and tests all three states.

## Summary

The host application defined a thinking-mode protocol canary and a non-thinking fallback. The RCM grammar accepted both declarations, but the compiled model retained only a Boolean whose default was false. Request construction never emitted the provider's `thinking.type` object, so the fallback could not override a provider default and the canary could not prove the declared mode was active.

## Timeline

- A release containing provider request-shape diagnostics passed unit and hermetic host tests.
- The immutable host image was published and its additive database migration succeeded.
- The production candidate canary received HTTP 400 before any service or CloudFormation update.
- Inspection found that explicit mode intent disappeared between parsing and request construction.
- The grammar/compiler boundary and completion request tests were extended to distinguish enabled, disabled, and omitted modes.

## Root cause

The `thinking` grammar field served two responsibilities: selecting provider mode and deciding whether missing reasoning should be synthesized while replaying tool history. The AST stored a plain Boolean, so explicit false collapsed into the same value as omission, and the request builder used neither state to construct provider parameters. Tests covered reasoning replay but did not assert the serialized mode-selection request.

## Guardrails

- [Explicit thinking-mode specification](../specs/0002-explicit-openai-thinking-mode.md) owns the observable contract.
- [Thinking-mode decision](../decisions/0005-explicit-openai-thinking-mode.md) prevents provider hostname logic from entering the machine layer.
- [Parser and compiler tests](../../crates/cli/tests/parser.rs) distinguish explicit declarations from omission.
- [Completion request tests](../../crates/machine/tests/completion.rs) assert enabled, disabled, implicit, and non-OpenAI behavior.
