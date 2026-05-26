# RCM Python SDK

Python gRPC client for RCM (Recursive Context Machine).

## Setup

```bash
cd sdks/python
pip install grpcio protobuf grpcio-tools
bash generate.sh
```

## Usage — Paper Digest Demo

```bash
# Terminal 1: start the gRPC server
DEEPSEEK_API_KEY="REDACTED" cargo run --release -p server --bin rcm-server

# Terminal 2: run the Paper Digest demo
DEEPSEEK_API_KEY="REDACTED" python examples/research-assistant/grpc_demo.py
```

## Usage — Programmatic

```python
from rcm import RCMClient, Model

rcm = RCMClient("localhost:50051")

# Open a new machine
mid, state, actions = rcm.open(
    purpose="search for papers",
    models=[Model(name="deepseek-v4-flash", protocol="openai",
                  endpoint="https://api.deepseek.com",
                  credentials=Model.Credentials(env="DEEPSEEK_API_KEY"),
                  limit=Model.Limit(context=1_000_000, output=393_216))],
    tools=["shell", "arxiv_search"],
    prompts={"captain": "You are a research assistant."},
)

# Setup: append prompts + activate model + tools
state, _ = rcm.step(mid, ActionCommand(verb="Append", fragment=FragmentContent(role="system", text="...", tag="agent")))
state, _ = rcm.step(mid, ActionCommand(verb="Append", fragment=FragmentContent(role="user", text="...", tag="purpose")))
state, _ = rcm.step(mid, ActionCommand(verb="Model", name="deepseek-v4-flash"))
state, _ = rcm.step(mid, ActionCommand(verb="Activate", name="shell"))

# Policy loop
for _ in range(max_steps):
    cmd, label = my_policy(state, actions)
    state, actions = rcm.step(mid, cmd)
    if state.done:
        break

rcm.destroy(mid)
```

## API

| Method | Description |
|--------|-------------|
| `open(purpose, models, tools, prompts)` | Create machine, returns (machine_id, state, action_space) |
| `step(machine_id, ActionCommand)` | Execute one action, returns (state, action_space) |
| `destroy(machine_id)` | Release machine resources |
| `close()` | Close gRPC channel (call when done using this client) |
