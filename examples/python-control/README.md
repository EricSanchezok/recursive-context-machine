# Python-Controlled Policy Example

This example shows how to drive an RCM machine from Python over gRPC.
The controller is a hand-written "policy" that decides each action —
the same interface an RL agent would use.

## Flow

```
Python controller                    Rust RCM server
┌─────────────────┐                  ┌──────────────────┐
│                 │    gRPC          │                  │
│  1. Open()      │ ───────────────► │  Create Run      │
│     ← state     │ ◄─────────────── │                  │
│     ← actions   │                  │                  │
│                 │                  │                  │
│  2. Step(Append │ ───────────────► │  Machine::apply  │
│     "captain")  │                  │  → ctx.append    │
│     ← state     │ ◄─────────────── │                  │
│                 │                  │                  │
│  3. Step(Halt)  │ ───────────────► │  reactor::react  │
│     ← state     │ ◄─────────────── │  → LLM call      │
│     ← actions   │                  │  → tool execute  │
│                 │                  │  → fragments →   │
│  4. Step(Take)  │ ───────────────► │    inbox         │
│     × N         │                  │  → ctx.append    │
│                 │                  │                  │
│  5. Step(Done)  │ ───────────────► │  done = true     │
│                 │                  │                  │
│  6. Destroy()   │ ───────────────► │  free resources  │
└─────────────────┘                  └──────────────────┘
```

## Prerequisites

```bash
# 1. Generate Python stubs from the proto definition
pip install grpcio grpcio-tools protobuf

cd examples/python-control
python -m grpc_tools.protoc \
  -I ../../crates/server/proto \
  --python_out=src \
  --grpc_python_out=src \
  --proto_path=$(python -c 'import grpc_tools; print(grpc_tools.PATH)')/protobuf \
  ../../crates/server/proto/rcm.proto \
  google/protobuf/empty.proto
```

## Running

Terminal 1 — start the server:
```bash
cargo build -p server
cargo run -p server
# RCM server listening on 127.0.0.1:50051
```

Terminal 2 — run the controller:
```bash
cd examples/python-control
python src/controller.py
```
