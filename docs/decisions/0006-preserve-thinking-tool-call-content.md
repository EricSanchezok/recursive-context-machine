# Preserve content in thinking tool-call history

## Status
Accepted
Class: bug-fix

## Context and Problem Statement

DeepSeek thinking-mode requests with tools require prior assistant messages to retain `content`, `reasoning_content`, and `tool_calls`. RCM preserved the reasoning and calls, but Rig 0.36 omitted the OpenAI `content` field when an assistant tool-call turn had no visible text. The first mixed-text tool turn could therefore succeed while a later pure tool turn failed with HTTP 400.

## Decision Drivers

- The actual OpenAI wire request must satisfy the provider's multi-turn tool-history contract.
- The fix must not fabricate visible assistant prose or alter captured reasoning.
- Non-thinking and non-OpenAI request shapes must remain stable.
- RCM must not branch on provider hostnames or model names.
- The downstream production canary must exercise the same shipped entry path.

## Considered Options

- Preserve an empty text item for explicitly thinking tool-call messages before OpenAI conversion.
- Upgrade Rig and rely on a newer provider-conversion contract.
- Route the affected workflows through DeepSeek's Anthropic-compatible endpoint.
- Detect DeepSeek hosts inside the machine request builder.

## Decision Outcome

RCM preserves an explicit empty text item only while encoding tool-call history with the existing thinking-history flag. Rig therefore serializes a present `content` field alongside the original reasoning and tool calls. The item carries no visible prose, and the existing explicit model declaration remains the capability signal.

This beats an immediate Rig upgrade because the narrow compatibility correction is independently testable without coupling an incident fix to a multi-version dependency migration. It beats a protocol switch because the existing OpenAI workflows and fallback remain valid. It gives up relying exclusively on the provider adapter to supply required empty fields.

## Pros and Cons of the Options

### Preserve an empty text item

- Good: fixes the exact omitted wire field at the smallest owned seam.
- Good: keeps model names and endpoints out of the machine layer.
- Good: permits a serialized-request regression test against the pinned Rig version.
- Trade-off: RCM carries one compatibility item that a future Rig upgrade may make redundant.

### Upgrade Rig

- Good: a provider adapter can centralize wire compatibility.
- Bad: the available upgrade spans several Rig releases and expands the regression surface during a production incident.
- Bad: adoption still requires proof that the new adapter retains empty assistant content.

### Use the Anthropic-compatible endpoint

- Good: avoids the affected OpenAI conversion path.
- Bad: changes protocol behavior across every affected workflow and requires a separate compatibility rollout.
- Bad: does not repair existing OpenAI-compatible users.

### Detect DeepSeek endpoints

- Good: scopes the workaround to one known provider.
- Bad: gateways and alternate endpoints make hostname policy incomplete.
- Bad: provider identity would leak into the provider-neutral machine layer.

## Links

- [DeepSeek thinking-mode tool-call contract](https://api-docs.deepseek.com/guides/thinking_mode/)
- [DeepSeek Chat Completions assistant message](https://api-docs.deepseek.com/api/create-chat-completion/)
- [Thinking tool-call content specification](../specs/0003-thinking-tool-call-content.md)
