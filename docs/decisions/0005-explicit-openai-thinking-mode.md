# Preserve explicit OpenAI-compatible thinking declarations

## Status
Accepted
Class: bug-fix

## Context and Problem Statement

OpenAI-compatible providers can expose thinking control through a non-standard `thinking.type` request object and can require every prior reasoning block to be replayed when tools are present. The RCM grammar accepted `thinking = "true"` or `"false"`, but compilation reduced both omission and explicit false to one Boolean default and used the value only while encoding history. A provider whose thinking mode defaults to enabled therefore could not be deterministically switched to non-thinking mode.

## Decision Drivers

- Explicit enabled and disabled declarations must produce deterministic wire behavior.
- Model declarations that omit the extension must remain compatible with OpenAI-compatible providers that reject unknown fields.
- The machine crate must not contain provider names, endpoints, or presets.
- Programmatic callers and the protobuf model contract must remain source and wire compatible.
- Thinking-history replay and provider mode selection must agree.

## Considered Options

- Send a `thinking` object for every OpenAI-compatible model based on the existing Boolean.
- Detect DeepSeek endpoints inside the machine crate and send the object only there.
- Preserve whether the RCM declaration is explicit and compile that choice into reserved model metadata.
- Move the affected workflow to an Anthropic-compatible endpoint.

## Decision Outcome

The parser preserves declaration presence, and the compiler records an explicit enabled or disabled OpenAI-compatible thinking mode in model metadata. Request construction sends `thinking.type` only when this metadata exists. The existing public `Model.thinking` field continues to control reasoning-history encoding; additive methods expose explicit mode configuration to programmatic callers.

This beats unconditional emission because omitted declarations keep the historical request shape, and it beats endpoint detection because the machine remains provider-neutral. The trade-off is a small internal metadata contract shared by the CLI compiler and machine request builder.

## Pros and Cons of the Options

### Send the extension unconditionally

- Good: the implementation is small.
- Bad: generic OpenAI-compatible providers may reject the non-standard field.
- Bad: omission can no longer preserve historical provider defaults.

### Detect provider endpoints

- Good: known DeepSeek requests get the required shape automatically.
- Bad: provider policy and hostname matching leak into the machine layer.
- Bad: gateways and alternate endpoints remain ambiguous.

### Preserve explicit declaration presence

- Good: enabled, disabled, and omitted remain distinct.
- Good: request behavior follows user intent without provider detection.
- Good: existing programmatic and protobuf callers retain their current behavior unless they opt in.
- Trade-off: the compiler and machine share one reserved metadata key behind typed methods.

### Use the Anthropic-compatible endpoint

- Good: some clients can avoid OpenAI-specific reasoning-history compatibility.
- Bad: it changes the protocol for every workflow and does not repair the false declaration contract.
- Bad: it requires separate canary and tool-history validation.

## Links

- [DeepSeek thinking-mode guide](https://api-docs.deepseek.com/guides/thinking_mode/)
- [DeepSeek Chat Completions API](https://api-docs.deepseek.com/api/create-chat-completion/)
- [Explicit thinking-mode specification](../specs/0002-explicit-openai-thinking-mode.md)
