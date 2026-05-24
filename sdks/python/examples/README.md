# Python-Controlled Policy Example

Drives an RCM machine from Python over gRPC using the RCM Python SDK.

## Flow

```
Python controller                    Rust RCM server
┌─────────────────┐                  ┌──────────────────┐
│                 │    gRPC          │                  │
│  1. open()      │ ───────────────► │  Create Run      │
│     ← state     │ ◄─────────────── │                  │
│     ← actions   │                  │                  │
│                 │                  │                  │
│  2. step(Append │ ───────────────► │  Machine::apply  │
│     "captain")  │                  │  → ctx.append    │
│     ← state     │ ◄─────────────── │                  │
│                 │                  │                  │
│  3. step(Halt)  │ ───────────────► │  reactor::react  │
│     ← state     │ ◄─────────────── │  → LLM call      │
│     ← actions   │                  │  → tool execute  │
│                 │                  │  → fragments →   │
│  4. step(Take)  │ ───────────────► │    inbox         │
│     × N          │                  │  → ctx.append    │
│                 │                  │                  │
│  5. step(Done)  │ ───────────────► │  done = true     │
│                 │                  │                  │
│  6. destroy()   │ ───────────────► │  free resources  │
└─────────────────┘                  └──────────────────┘
```

## Prerequisites

```bash
pip install grpcio protobuf

pip install -e ..
```

## Running

**Terminal 1** — start the server:
```bash
cargo run -p server
```

**Terminal 2** — run the controller:
```bash
python controller.py
```
