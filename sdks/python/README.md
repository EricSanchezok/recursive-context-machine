# RCM Python SDK

Python gRPC client for RCM (Recursive Context Machine).

## Setup

```bash
cd sdks/python
pip install -e . grpcio-tools
bash generate.sh
```

Start the RCM gRPC server in another terminal:

```bash
cargo run --release -p server --bin rcm-server
```

Set an OpenAI-compatible model key. The examples below use DeepSeek:

```bash
export DEEPSEEK_API_KEY="REDACTED"
```

## Examples

### Echo Agent

A minimal external Python controller: define a model, select it by name, append a prompt, and let the model repeat the user message.

```bash
PYTHONPATH=src python examples/echo_agent.py "hello from python"
```

### MCP Math

A local stdio MCP server exposes `add` and `multiply`. The SDK registers the MCP server definition, selects it with `mcps=["math"]`, activates the discovered tools, and drives the loop from Python.

```bash
PYTHONPATH=src python examples/mcp_math.py "What is 23 multiplied by 19, plus 7?"
```

## API Shape

Definitions and selections are separate:

```python
from rcm import McpServer, Model, RCMClient

client = RCMClient("localhost:50051")
model = Model(name="deepseek-v4-flash", ...)
math_server = McpServer(label="math", transport=McpServer.Stdio(...))

machine_id, state, actions = client.open(
    purpose="solve arithmetic",
    model_definitions=[model],
    models=["deepseek-v4-flash"],
    mcp_definitions=[math_server],
    mcps=["math"],
    tools=[],
    prompts={"math": "Use math tools for arithmetic."},
)
```

- `model_definitions` and `mcp_definitions` register external resources for this machine.
- `models`, `tools`, and `mcps` select resources by name.
- `prompts` registers prompt texts that appear as append actions in the returned action space.
- `ReactPolicy` is a small external controller for the running loop; setup remains explicit in Python.
