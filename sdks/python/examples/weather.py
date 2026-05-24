"""Weather check — ask the agent for the weather at a given city."""

from rcm import RCMClient
from rcm.react import ReactPolicy


def main():
    rcm = RCMClient("localhost:50051")

    mid, state, actions = rcm.open(
        purpose="check the weather in Beijing",
        tools=["shell"],
        prompts={
            "captain": (
                "You are a weather assistant. The user wants to know the current weather "
                "in Beijing, China.\n\n"
                'Use the shell tool: curl -s "wttr.in/Beijing?format=3"\n\n'
                "Report the result. Stop after reporting."
            ),
        },
    )

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
