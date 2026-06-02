# City Half-Day

A single-agent RCM that turns a short city outing request into a half-day micro-itinerary. It uses AnySearch MCP for current public information, while intentionally avoiding maps and weather MCP for this first version.

## Run

```sh
accelerate run /path/to/RCM/examples/city-halfday/city_halfday.rcm \
  --purpose "明天下午在上海安排一个轻松半日游，少走路，适合聊天"
```

When running from this repository without installing:

```sh
cargo run --manifest-path /path/to/RCM/Cargo.toml --bin accelerate -- \
  run /path/to/RCM/examples/city-halfday/city_halfday.rcm \
  --purpose "明天下午在上海安排一个轻松半日游，少走路，适合聊天"
```

## MCP Setup

This example uses:

- AnySearch MCP for current public information

Maps and weather MCP are intentionally deferred. The itinerary should be honest about route, travel-time, and weather uncertainty.

## What It Collects

- The runtime purpose sentence
- Current public information about places, hours, tickets, closures, and events via MCP
- Optional local preference notes if present in the current folder

## Output

- `outputs/city-halfday.md`

## Safety

This example does not book tickets, make reservations, or message companions. It writes a plan and a message draft only.
