# Content presence did not prove DeepSeek shape

Artifact-Version: 1

## Executive summary

RCM v0.2.23 made a missing thinking tool-call `content` field present, but the generic OpenAI adapter serialized the compatibility value as a text-parts array while DeepSeek documents a string-or-null field. The regression checked only presence, and the live canary was nondeterministic because some tool turns contained visible text; the permanent correction is an explicit native DeepSeek transport plus an exact local wire-body test.

## Summary

The downstream production deployment canary performed sequential filesystem tool calls. Earlier runs passed when the model included visible assistant text beside the call. A later run produced a pure tool call, replayed it as `content: [{"type":"text","text":""}]`, and received HTTP 400 classified by content-free diagnostics as `thinking_tool_history`.

## Timeline

- RCM v0.2.23 shipped an explicit empty provider-neutral text item so Rig's generic OpenAI serializer would not omit assistant `content`.
- Its regression asserted only that serialized `content` was non-null.
- A downstream image pinned v0.2.23 and passed an initial live multi-turn DeepSeek canary.
- A later immutable-image deployment hit a pure tool-call response and failed safely before any production stack update.
- The captured content-free request shape and the provider's published schema narrowed the defect to JSON type rather than field presence.
- An exact assertion reproduced the parts-array/string mismatch locally without credentials or user content.

## Root cause

The previous fix treated OpenAI compatibility as equivalent to the DeepSeek wire schema. Rig's generic OpenAI request accepts multimodal assistant content parts, so inserting an empty text item produced a present but wrongly typed DeepSeek field. The test encoded the same assumption by checking presence rather than the exact JSON type and sibling fields. The live canary could not deterministically force a pure tool-call response, so successful runs did not prove the edge shape.

## Guardrails

- [DeepSeek wire regression](../../crates/machine/tests/completion.rs) captures the actual native-adapter HTTP request and asserts exact string content, reasoning, tool calls, thinking control, and token limit.
- [DeepSeek native transport specification](../specs/0004-deepseek-native-transport.md) owns the compatibility and N-1 contract.
- [Provider-native transport decision](../decisions/0007-use-provider-native-deepseek-transport.md) prevents provider identity from returning to hostname inference or generic JSON rewriting.
- The downstream release workflow continues to run the live multi-turn provider canary before stack mutation, as a complement to the deterministic local wire contract.
