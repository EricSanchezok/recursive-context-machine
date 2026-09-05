# Thinking tool-call content was omitted

Artifact-Version: 1

## Executive summary

A production release canary received HTTP 400 on a later DeepSeek thinking-mode tool turn because the replayed assistant message retained its reasoning and tool call but omitted the required `content` field when no visible text existed. Intermediate-message tests missed the provider conversion step; the permanent correction retains an empty content item and asserts the serialized OpenAI request.

## Summary

The fixed canary performs two sequential filesystem tool calls before returning a final response. Its first tool turn included visible assistant text and succeeded. A later pure tool-call turn was replayed through Rig 0.36 with an empty assistant content collection, which Serde omitted from the OpenAI request even though the reasoning and tool-call fields remained present.

## Timeline

- Reasoning replay and explicit thinking-mode selection were released with unit coverage.
- The downstream immutable Survey image passed hermetic tests and entered the production candidate gate.
- The multi-turn provider canary failed with HTTP 400 and content-free diagnostics classified the request as `thinking_tool_history`.
- A serialized-request regression reproduced the absent field without provider credentials or user content.
- The history encoder retained an explicit empty content item, and the provider canary became the adoption gate for the corrected release.

## Root cause

RCM tests asserted that the provider-neutral assistant message contained its tool call and reasoning. They did not inspect the request after Rig's OpenAI conversion. Rig represented assistant content as a collection and skipped the field when that collection was empty, while the provider requires the field to remain present beside thinking-mode tool calls.

## Guardrails

- [Serialized OpenAI request regression](../../crates/machine/tests/completion.rs) asserts that a pure thinking tool-call turn retains `content` after provider conversion.
- [Thinking tool-call content specification](../specs/0003-thinking-tool-call-content.md) owns the observable compatibility contract.
- [Request-shape decision](../decisions/0006-preserve-thinking-tool-call-content.md) records why the compatibility item lives at the RCM history boundary.
- The downstream release workflow runs a live two-tool-turn provider canary before any production stack update.
