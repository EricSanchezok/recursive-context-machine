"""
RCM Python controller — drives an RCM machine over gRPC.

Requires the RCM Python SDK (sdks/python/). Install with:
    pip install -e ../../sdks/python

Then start the server:
    cargo run -p server

And run this controller:
    python controller.py
"""

import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "../../sdks/python/src"))

from rcm import RCMClient


class Controller:
    """Drive an RCM machine, reading state at each step to decide the next action."""

    def __init__(self, client: RCMClient, machine_id: str):
        self.client = client
        self.mid = machine_id
        self.halt_count = 0
        self.max_halt = 3

    def decide(self, state, action_space):
        """Pick the next action by inspecting the current state."""
        actions = action_space.actions
        verbs = {a.command.verb for a in actions}

        # Drain inbox when it has pending fragments.
        if state.inbox_pending and "Take" in verbs:
            return self._pick(actions, "Take")

        # Append the first system prompt if context is empty.
        if not state.fragments and "Append" in verbs:
            return self._pick(actions, "Append")

        # Halt to let the LLM + tools run.
        if self.halt_count < self.max_halt and "Halt" in verbs:
            self.halt_count += 1
            return self._pick(actions, "Halt")

        # Done.
        if "Done" in verbs:
            return self._pick(actions, "Done")

        # Fallback — take the last available action.
        return actions[-1].command, actions[-1].label

    @staticmethod
    def _pick(actions, verb):
        for a in actions:
            if a.command.verb == verb:
                return a.command, a.label
        raise ValueError(f"no {verb} action available")


def main():
    rcm = RCMClient("localhost:50051")

    mid, state, actions = rcm.open(
        purpose="search for recent AI research papers on arXiv",
        models=[""],
        tools=["arxiv_search", "arxiv_download"],
        prompts={
            "captain": (
                "You are a research assistant. Search arXiv for recent papers "
                "on artificial intelligence. Use the arxiv_search tool with query "
                "'deep learning 2025' or 'large language models reasoning 2025'.\n\n"
                "After getting results, stop."
            ),
        },
    )

    controller = Controller(rcm, mid)

    print(f"Machine opened: {mid}")
    print(
        f"  state: step={state.step}, fragments={len(state.fragments)}, done={state.done}"
    )
    print(f"  actions: {[a.label for a in actions.actions]}")
    print()

    for t in range(30):
        command, label = controller.decide(state, actions)
        print(f"Step {t + 1}: {label}")

        state, actions = rcm.step(mid, command)

        print(
            f"  step={state.step} fragments={len(state.fragments)} done={state.done} inbox={state.inbox_pending}"
        )
        if state.fragments:
            frag = state.fragments[-1]
            print(f"  last: [{frag.role}/{frag.kind}] {frag.text_preview[:80]}")

        if state.done:
            break

        print()

    print(f"\nFinal: {len(state.fragments)} fragments, done={state.done}")
    for i, frag in enumerate(state.fragments):
        print(f"  [{i}] {frag.role}/{frag.kind}: {frag.text_preview[:80]}")

    rcm.destroy(mid)


if __name__ == "__main__":
    main()
