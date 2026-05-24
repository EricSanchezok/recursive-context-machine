"""Weather check — ask the agent for the weather using MCP anysearch."""

import os

from rcm import RCMClient, Model, McpServer
from rcm._pb2 import ActionCommand, FragmentContent
from rcm.react import ReactPolicy

CAPTAIN_PROMPT = (
    "You are a weather assistant. The user wants to know the current weather "
    "in Beijing, China.\n\n"
    "Use the anysearch tool to find weather info. Search for 'weather in Beijing China'.\n"
    "Report the result. Stop after reporting."
)


def main():
    api_key = os.environ.get("ANYSEARCH_API_KEY")
    if not api_key:
        print("ANYSEARCH_API_KEY not set")
        return

    rcm = RCMClient("localhost:50051")

    mid, state, actions = rcm.open(
        purpose="check the weather in Beijing",
        models=[
            Model(
                name="deepseek-v4-flash",
                protocol="openai",
                endpoint="https://api.deepseek.com",
                credentials=Model.Credentials(env="DEEPSEEK_API_KEY"),
                limit=Model.Limit(context=1000000, output=393216),
            ),
        ],
        mcps=[
            McpServer(
                label="anysearch",
                transport=McpServer.Http(
                    url="https://api.anysearch.com/mcp",
                    headers={
                        "Authorization": McpServer.Value(literal=f"Bearer {api_key}")
                    },
                ),
            ),
        ],
        prompts={"captain": CAPTAIN_PROMPT},
    )

    state, actions = rcm.step(
        mid,
        ActionCommand(
            verb="Append",
            fragment=FragmentContent(role="system", text=CAPTAIN_PROMPT, tag="agent"),
        ),
    )
    state, actions = rcm.step(
        mid,
        ActionCommand(
            verb="Append",
            fragment=FragmentContent(
                role="user",
                text="check the weather in Beijing",
                tag="purpose",
            ),
        ),
    )
    state, actions = rcm.step(
        mid, ActionCommand(verb="Model", name="deepseek-v4-flash")
    )

    print(f"Available tools: {list(state.available_tools)}")
    for tool_name in state.available_tools:
        state, actions = rcm.step(mid, ActionCommand(verb="Activate", name=tool_name))

    policy = ReactPolicy()

    for step in range(20):
        cmd, label = policy(state, actions)
        print(f"[{step + 1}] {label}")

        state, actions = rcm.step(mid, cmd)

        if state.fragments:
            frag = state.fragments[-1]
            print(f"  → [{frag.role}/{frag.kind}] {frag.text_preview[:120]}")

        if state.done:
            print("done.\n")
            break

    rcm.destroy(mid)

    print("── Weather ──")
    for frag in state.fragments:
        if frag.role == "assistant":
            print(frag.text_preview)


if __name__ == "__main__":
    main()
