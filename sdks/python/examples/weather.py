"""Weather check — ask the agent for the weather at a given city."""

from rcm import RCMClient, Model
from rcm._pb2 import ActionCommand, FragmentContent
from rcm.react import ReactPolicy

CAPTAIN_PROMPT = (
    "You are a weather assistant. The user wants to know the current weather "
    "in Beijing, China.\n\n"
    'Use the shell tool: curl -s "wttr.in/Beijing?format=3"\n\n'
    "Report the result. Stop after reporting."
)


def main():
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
        tools=["shell"],
        prompts={"captain": CAPTAIN_PROMPT},
    )

    state, actions = rcm.step(
        mid,
        ActionCommand(
            verb="Append",
            fragment=FragmentContent(
                role="system",
                text=CAPTAIN_PROMPT,
                tag="agent",
            ),
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
    state, actions = rcm.step(mid, ActionCommand(verb="Activate", name="shell"))

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
