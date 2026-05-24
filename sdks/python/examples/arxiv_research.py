"""ArXiv search — ask the agent to research recent AI papers."""

from rcm import RCMClient, Model
from rcm._pb2 import ActionCommand, FragmentContent
from rcm.react import ReactPolicy

CAPTAIN_PROMPT = (
    "You are a research assistant. Search arXiv for recent papers "
    "using the arxiv_search tool.\n\n"
    "Run these queries:\n"
    "1. 'large language model reasoning chain of thought 2025'\n"
    "2. 'RLHF reinforcement learning human feedback alignment 2025'\n\n"
    "For each search, pick 2-3 interesting papers and report:\n"
    "- Title and authors\n"
    "- One-sentence summary of the abstract\n\n"
    "Stop after reporting."
)


def main():
    rcm = RCMClient("localhost:50051")

    mid, state, actions = rcm.open(
        purpose="search for recent AI research papers on arXiv",
        models=[
            Model(
                name="deepseek-v4-flash",
                protocol="openai",
                endpoint="https://api.deepseek.com",
                credentials=Model.Credentials(env="DEEPSEEK_API_KEY"),
                limit=Model.Limit(context=1000000, output=393216),
            ),
        ],
        tools=["arxiv_search"],
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
                text="search for recent AI research papers on arXiv",
                tag="purpose",
            ),
        ),
    )

    state, actions = rcm.step(
        mid, ActionCommand(verb="Model", name="deepseek-v4-flash")
    )
    state, actions = rcm.step(mid, ActionCommand(verb="Activate", name="arxiv_search"))

    policy = ReactPolicy()

    for step in range(40):
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

    print("── ArXiv Research ──")
    for frag in state.fragments:
        if frag.role == "assistant":
            print(frag.text_preview)


if __name__ == "__main__":
    main()
