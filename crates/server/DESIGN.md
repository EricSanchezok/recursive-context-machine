# RCM gRPC Server Design

## Overview

The RCM gRPC server exposes `Machine` as a network service. Clients create a
run (a live machine instance), drive it step-by-step by sending actions, and
destroy it when done. The server does **not** own a policy, compute rewards,
or manage task semantics — it is a pure agent runtime.

```
┌─ Python (RL Trainer) ────────────────────────────┐
│                                                    │
│  ┌──────────┐    ┌──────────────┐                  │
│  │ Policy   │───►│  Environment │  ← external     │
│  │ Network  │    │  (task-spec) │                  │
│  │          │◄───│              │                  │
│  └──────────┘    └──────────────┘                  │
│       │                                            │
│  ┌────┴──────────────────────────────┐             │
│  │          RCMClient                │             │
│  │  new() → (mid, state, actions)    │             │
│  │  step(mid, command) → (state,     │             │
│  │       actions)                    │             │
│  │  destroy(mid)                     │             │
│  └────┬──────────────────────────────┘             │
└───────┼────────────────────────────────────────────┘
        │
   protobuf over gRPC
        │
┌───────┼────────────────────────────────────────────┐
│       │   Rust RCM Server                          │
│       │                                            │
│  ┌────┴──────────────┐    ┌─────────────────────┐ │
│  │   RCMService      │    │   MachineManager     │ │
│  │                    │───►│   HashMap<Mid, Run>  │ │
│  │   New() → Mid      │    │                      │ │
│  │   Step(Mid, Cmd)   │    │   Run {              │ │
│  │   Destroy(Mid)     │    │     ctx, env,        │ │
│  └────────────────────┘    │     resources,       │ │
│                             │     inbox, step      │ │
│                             │   }                  │ │
│                             └──────────┬──────────┘ │
│                                        │             │
│                               ┌────────┴────────┐   │
│                               │ Machine::apply() │   │
│                               │ reactor::react() │   │
│                               └──────────────────┘   │
└───────────────────────────────────────────────────────┘
```

## RPCs

Three methods. No Create/Reset/Observe/Context — `State` is the single data
model, and the external environment owns the task loop.

| RPC | Purpose |
|-----|---------|
| `New` | Allocate a run. Returns `machine_id`, initial `State`, `ActionSpace`. |
| `Step` | Execute one action (including Halt → reactor). Returns new `State` and `ActionSpace`. |
| `Destroy` | Release the run and all held resources. |

## Protobuf

### Service

```protobuf
syntax = "proto3";
package rcm;

import "google/protobuf/empty.proto";

service RCM {
  rpc New(NewRequest) returns (NewResponse);
  rpc Step(StepRequest) returns (StepResponse);
  rpc Destroy(DestroyRequest) returns (google.protobuf.Empty);
}
```

### New

```protobuf
message NewRequest {
  string purpose = 1;
  repeated string models = 2;
  repeated string tools = 3;       // tool names from the built-in catalog
  map<string, string> prompts = 4; // prompt templates keyed by name
}

message NewResponse {
  string machine_id = 1;
  State state = 2;
  ActionSpace action_space = 3;
}
```

### Step

```protobuf
message StepRequest {
  string machine_id = 1;
  ActionCommand command = 2;
}

message StepResponse {
  State state = 1;
  ActionSpace action_space = 2;
}
```

`state.done == true` signals termination — no separate done field.

### Destroy

```protobuf
message DestroyRequest {
  string machine_id = 1;
}
```

### ActionCommand

The command carries the complete semantics of one machine action. The
client sends a full `ActionCommand`, not an opaque index — the server
decodes it directly to `machine::Action`.

```protobuf
message ActionCommand {
  string verb = 1;

  optional uint64 fragment_id = 2;   // Remove / Replace / Swap(a) / Insert(after)
  optional uint64 fragment_id2 = 3;  // Swap(b)
  optional FragmentContent fragment = 4; // Append / Insert / Replace payload
  optional string name = 5;          // Model / Activate / Deactivate target
}
```

Mapping to `machine::Action`:

| verb | fragment_id | fragment_id2 | fragment | name | → Action |
|------|-------------|--------------|----------|------|----------|
| `Halt` | — | — | — | — | `Action::Halt` |
| `Done` | — | — | — | — | `Action::Done` |
| `Take` | — | — | — | — | `Action::Take` |
| `Append` | — | — | ✓ | — | `Action::Append(f)` |
| `Remove` | id | — | — | — | `Action::Remove(id)` |
| `Swap` | a | b | — | — | `Action::Swap(a, b)` |
| `Insert` | after | — | ✓ | — | `Action::Insert { after, fragment: f }` |
| `Replace` | id | — | ✓ | — | `Action::Replace { id, fragment: f }` |
| `Model` | — | — | — | ✓ | `Action::Model(name)` |
| `Activate` | — | — | — | ✓ | `Action::Activate(name)` |
| `Deactivate` | — | — | — | ✓ | `Action::Deactivate(name)` |

### FragmentContent

```protobuf
message FragmentContent {
  string role = 1;  // "system" | "user"
  string text = 2;  // full text content
}
```

Server constructs `machine::Fragment` from this:
- `role: "user"` → `Role::User`, otherwise `Role::System`
- `text` → `Content::Text(text.into())`

### ActionSpace

```protobuf
message ActionSpace {
  repeated ActionItem actions = 1;
}

message ActionItem {
  ActionCommand command = 1;        // the action to send back on Step
  string label = 2;                 // human-readable label
  FragmentContent sink = 3;         // preview of the fragment (for Append-type)
}
```

The action space is built from the current `RunState`:

- **Free mode** (inbox empty): Halt, Append (one per prompt resource), Remove (one
  per context fragment), Replace (one per fragment × one per prompt),
  Insert, Swap, Model, Activate, Deactivate, Done.
- **Consumption mode** (inbox pending): only Take and Remove.

### State

```protobuf
message State {
  string purpose = 1;

  repeated Fragment fragments = 2;

  string workdir = 3;
  map<string, string> env_vars = 4;

  string active_model = 5;
  repeated string active_tools = 6;
  repeated string available_models = 7;
  repeated string available_tools = 8;

  bool done = 9;
  uint64 step = 10;

  bool inbox_pending = 11;
  optional Fragment inbox_peek = 12;
}

message Fragment {
  uint64 id = 1;
  string role = 2;         // system / user / assistant / tool
  string kind = 3;         // text / tool_call / tool_result / hitch / image
  string text_preview = 4; // first ~200 chars
  optional string tag = 5;
}
```

`State` corresponds to the four mutable inputs of `Machine::apply()`:
`ctx` → `fragments`, `env` → `workdir` + `env_vars`, `resources` →
`active_model` + `active_tools` + `available_*`, `inbox` →
`inbox_pending` + `inbox_peek`.

## Rust Implementation

### Crate layout

```
proto/                        # shared proto source of truth
└── rcm.proto

crates/server/
├── Cargo.toml
├── build.rs                  # tonic-build from ../../proto/rcm.proto
└── src/
    ├── lib.rs
    ├── service.rs            # handlers + State/ActionSpace serialization
    └── manager.rs            # MachineManager — HashMap<MachineId, Run>

sdks/python/                  # Python SDK (pip install -e .)
├── pyproject.toml
├── generate.sh               # regenerate stubs from proto/rcm.proto
└── src/rcm/
    ├── __init__.py           # from rcm import RCMClient
    ├── client.py             # thin wrapper over gRPC stub
    ├── _pb2.py               # generated message classes
    └── _pb2_grpc.py          # generated client stub

### Dependencies

```toml
[dependencies]
tonic = "0.13"
prost = "0.14"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
uuid = { version = "1", features = ["v4"] }
machine = { path = "../machine" }
accelerator = { path = "../accelerator" }

[build-dependencies]
tonic-build = "0.13"
```

### MachineManager

```rust
use std::collections::HashMap;
use machine::{Context, Environment, Inbox, Resources};
use uuid::Uuid;

pub struct MachineManager {
    machines: HashMap<MachineId, Run>,
}

pub struct Run {
    pub ctx: Context,
    pub env: Environment,
    pub resources: Resources,
    pub inbox: Inbox,
    pub step: u64,
    pub done: bool,
}
```

A simple key-value store. `Run` holds the four mutable state components
and two counters. No policy, no phases — those are Rust-internal accelerator
concepts.

### RCMService

Three public handlers plus private helpers:

```
RCMService
├── new(request) → (machine_id, state, action_space)
│   ├── Build resources from request (models, tools, prompts)
│   ├── Create Run with empty ctx, local env, inbox
│   ├── build_state() + build_action_space()
│
├── step(request) → (state, action_space)
│   ├── decode_command() → machine::Action
│   ├── Machine::apply()
│   ├── build_state() + build_action_space()
│
├── destroy(request)
│   └── manager.destroy(mid)
│
├── build_state(Run) → proto::State
├── build_action_space(Run) → proto::ActionSpace
└── decode_command(ActionCommand) → Result<machine::Action>
```

### Action decode

`decode_command` maps `ActionCommand` fields to `machine::Action` variants:

```rust
fn decode_command(cmd: &ActionCommand) -> Result<Action, Status> {
    let build_frag = |fc: &FragmentContent| {
        let role = match fc.role.as_str() {
            "user" => Role::User,
            _ => Role::System,
        };
        Fragment::new(role, Content::Text(fc.text.clone().into()))
    };

    match cmd.verb.as_str() {
        "Halt" => Ok(Action::Halt),
        "Done" => Ok(Action::Done),
        "Take" => Ok(Action::Take),
        "Append" => {
            let fc = cmd.fragment.as_ref().ok_or(Status::invalid_argument("fragment required"))?;
            Ok(Action::Append(build_frag(fc)))
        }
        "Replace" => {
            let id = cmd.fragment_id.ok_or(Status::invalid_argument("fragment_id required"))?;
            let fc = cmd.fragment.as_ref().ok_or(Status::invalid_argument("fragment required"))?;
            Ok(Action::Replace { id, fragment: build_frag(fc) })
        }
        // ... etc
    }
}
```

## Python Client

```python
import grpc
from rcm.proto import rcm_pb2, rcm_pb2_grpc

class RCMClient:
    def __init__(self, endpoint: str = "localhost:50051"):
        self.channel = grpc.insecure_channel(endpoint)
        self.stub = rcm_pb2_grpc.RCMStub(self.channel)

    def new(
        self, purpose: str, models: list[str], tools: list[str] | None = None,
        prompts: dict[str, str] | None = None,
    ) -> tuple[str, rcm_pb2.State, rcm_pb2.ActionSpace]:
        ...

    def step(
        self, machine_id: str, command: rcm_pb2.ActionCommand
    ) -> tuple[rcm_pb2.State, rcm_pb2.ActionSpace]:
        ...

    def destroy(self, machine_id: str):
        ...
```

Training loop:

```python
rcm = RCMClient()

machine_id, state, actions = rcm.new(
    purpose="fix the bug in auth.rs",
    models=["fast"],
    tools=["read", "edit", "shell"],
    prompts={"captain": "You are a senior engineer..."},
)

for t in range(max_steps):
    command = policy.pick(state, actions.actions)
    state, actions = rcm.step(machine_id, command)

    if state.done:
        reward = evaluate_task(state)
        break

rcm.destroy(machine_id)
```

The external trainer owns the RL loop: policy → action → Step → reward.
RCM is only the agent runtime.

## Design decisions

1. **No index-based actions.** `ActionCommand` carries verb + parameters
   directly. The client sends back the same message it received from
   `ActionSpace`. No mapping table between client and server.

2. **No Reward / Reset / Context RPCs.** RCM is an agent runtime, not an RL
   environment. Task semantics belong to the external benchmark/trainer.
   `State` is the single data model — the trainer derives reward and reset
   logic from it.

3. **No phases in the gRPC layer.** Phases are a Rust-internal accelerator
   concept (pre/post/pre_halt/post_halt). The gRPC server does not expose
   them. If a client wants phase-like behavior (e.g. inject environment info
   before each Halt), it issues the corresponding actions explicitly.

4. **`Machine::apply()` as the single primitive.** Every `Step` calls
   `apply(action, step, ctx, env, resources, inbox)`. Halt triggers
   `reactor::react()` internally. Done sets `done = true`.
