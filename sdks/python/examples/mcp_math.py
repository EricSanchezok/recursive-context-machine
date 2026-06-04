import os
import sys
from pathlib import Path

from rcm import ActionCommand, FragmentContent, McpServer, Model, RCMClient, ReactPolicy

MODEL_NAME = "deepseek-v4-flash"
SYSTEM_PROMPT = (
    "Solve the user's arithmetic request by calling the available math MCP tools. "
    "Use tools for arithmetic instead of mental calculation. Return only the final answer."
)


def pick_label(actions, label: str):
    for action in actions.actions:
        if action.label == label:
            return action.command
    raise RuntimeError(f"action not available: {label}")


def run_loop(client, machine_id, state, actions, max_steps: int = 12):
    policy = ReactPolicy()
    for step in range(max_steps):
        command, label = policy(state, actions)
        print(f"[{step + 1}] {label}")
        state, actions = client.step(machine_id, command)
        if state.fragments:
            fragment = state.fragments[-1]
            print(
                f"  → [{fragment.role}/{fragment.kind}] {fragment.text_preview[:120]}"
            )
        if state.done:
            break
    return state, actions


def main():
    if "DEEPSEEK_API_KEY" not in os.environ:
        raise RuntimeError("DEEPSEEK_API_KEY is not set")

    question = " ".join(sys.argv[1:]) or "What is 23 multiplied by 19, plus 7?"
    server_path = Path(__file__).with_name("math_mcp_server.py")
    client = RCMClient("localhost:50051")

    machine_id, state, actions = client.open(
        purpose=question,
        model_definitions=[
            Model(
                name=MODEL_NAME,
                protocol="openai",
                endpoint="https://api.deepseek.com",
                credentials=Model.Credentials(env="DEEPSEEK_API_KEY"),
                limit=Model.Limit(context=64_000, output=2_000),
                modalities_input=["text"],
                modalities_output=["text"],
            )
        ],
        models=[MODEL_NAME],
        mcp_definitions=[
            McpServer(
                label="math",
                transport=McpServer.Stdio(
                    command=sys.executable,
                    args=[str(server_path)],
                ),
            )
        ],
        mcps=["math"],
        prompts={"math": SYSTEM_PROMPT},
    )

    try:
        state, actions = client.step(machine_id, pick_label(actions, "Append math"))
        state, actions = client.step(
            machine_id,
            ActionCommand(
                verb="Append",
                fragment=FragmentContent(role="user", text=question, tag="purpose"),
            ),
        )
        state, actions = client.step(
            machine_id, ActionCommand(verb="Model", name=MODEL_NAME)
        )
        for tool_name in state.available_tools:
            state, actions = client.step(
                machine_id, ActionCommand(verb="Activate", name=tool_name)
            )

        state, _ = run_loop(client, machine_id, state, actions)

        for fragment in state.fragments:
            if fragment.role == "assistant" and fragment.kind == "text":
                print(fragment.content_text or fragment.text_preview)
    finally:
        client.destroy(machine_id)
        client.close()


if __name__ == "__main__":
    main()
