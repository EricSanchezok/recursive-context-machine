"""ArXiv search — ask the agent to research recent AI papers."""

from rcm import RCMClient
from rcm.react import run


def main():
    rcm = RCMClient("localhost:50051")

    state = run(
        rcm,
        purpose="search for recent AI research papers on arXiv",
        prompt=(
            "You are a research assistant. Search arXiv for recent papers "
            "using the arxiv_search tool.\n\n"
            "Run two searches:\n"
            "1. Query: 'large language model reasoning chain of thought 2025'\n"
            "2. Query: 'RLHF reinforcement learning human feedback alignment 2025'\n\n"
            "For each search, pick 2-3 interesting papers and report:\n"
            "- Title and authors\n"
            "- One-sentence summary of the abstract\n\n"
            "Stop after reporting."
        ),
        tools=["arxiv_search"],
        max_halts=4,
        verbose=True,
    )

    print("── ArXiv Research ──")
    for frag in state.fragments:
        if frag.role == "assistant":
            print(frag.text_preview)


if __name__ == "__main__":
    main()
