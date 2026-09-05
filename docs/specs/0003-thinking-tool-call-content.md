# Thinking tool-call assistant content

Artifact-Version: 1
Status: Implemented

## Intent

An explicitly thinking OpenAI-compatible model must replay assistant tool-call turns with every provider-required sibling field. A tool-call turn that contains no visible text must retain a present assistant `content` field together with its original `reasoning_content` and tool calls, without inventing model-visible prose or selecting behavior from a provider hostname.

## Contract

- The release artifact carrying this contract is versioned `0.2.23`; downstream pins must use its published archive checksum.
- A replayed tool-call assistant message for `thinking = "true"` always serializes a present, non-null `content` field, including when the provider emitted no visible text.
- The compatibility value is an empty text item and does not add visible prose to the model history.
- Stored reasoning is replayed unchanged; the compatibility field does not replace, truncate, or synthesize reasoning.
- Non-thinking history retains its existing request shape.
- Provider endpoints and model names do not participate in the decision.

## Plan

Keep an explicit empty text item in thinking-mode assistant tool-call messages before Rig converts the provider-neutral history into the OpenAI wire request. Cover the serialized OpenAI request, rather than only the intermediate RCM message, with a regression test.

## Verification

- The regression test demonstrates that the released adapter omitted `content` before the fix and that the corrected wire request contains it.
- Machine tests cover single, mixed-text, parallel, and failed tool-call replay.
- Workspace tests, documentation tests, formatting, Clippy, and repository gates pass.
- A live DeepSeek multi-turn tool canary completes before a downstream release adopts the new archive.

## Evidence

- [Serialized request regression test](../../crates/machine/tests/completion.rs)
- [Request-shape decision](../decisions/0006-preserve-thinking-tool-call-content.md)
- [Escaped-failure postmortem](../postmortems/0002-thinking-tool-call-content-was-omitted.md)
