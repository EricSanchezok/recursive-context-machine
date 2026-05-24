"""Weather check — ask the agent for the weather at a given city."""

from rcm import RCMClient
from rcm.react import run


def main():
    rcm = RCMClient("localhost:50051")

    state = run(
        rcm,
        purpose="check the weather in Beijing",
        prompt=(
            "You are a weather assistant. The user wants to know the current weather "
            "in Beijing, China.\n\n"
            "Use the shell tool to fetch weather data. A good approach:\n"
            '  curl -s "wttr.in/Beijing?format=3"\n\n'
            "Report the result. Stop after reporting."
        ),
        tools=["shell"],
        max_halts=2,
        verbose=True,
    )

    print("── Weather ──")
    for frag in state.fragments:
        if frag.role == "assistant":
            print(frag.text_preview)


if __name__ == "__main__":
    main()
