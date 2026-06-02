# Cook Tonight

A single-agent RCM that turns one dinner sentence into a practical cooking decision. It can run from the user's short goal alone, optionally inspecting local preference notes if present, then using AnySearch MCP for recipe discovery.

## Run

```sh
accelerate run /path/to/RCM/examples/cook-tonight/cook_tonight.rcm \
  --purpose "今晚想吃高蛋白、30分钟内、少洗碗，帮我决定吃什么"
```

When running from this repository without installing:

```sh
cargo run --manifest-path /path/to/RCM/Cargo.toml --bin accelerate -- \
  run /path/to/RCM/examples/cook-tonight/cook_tonight.rcm \
  --purpose "今晚想吃高蛋白、30分钟内、少洗碗，帮我决定吃什么"
```

## MCP Setup

This example uses:

- AnySearch MCP for recipe and technique search

Maps, weather, grocery ordering, and delivery services are intentionally not part of this example.

## What It Collects

- The runtime purpose sentence
- Optional local notes in the current folder, such as preferences, allergies, prior menus, or pantry notes
- Recipe and technique references through MCP when available

## Output

- `outputs/cook-tonight.md`

## Safety

This example does not order groceries, buy food, send messages, or claim precise nutrition. It should flag allergy and raw-food risks when relevant.
