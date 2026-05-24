"""
RCM Python controller — hand-written policy over gRPC.

This controller drives an RCM machine step by step, deciding each
action explicitly. It's the same interface an RL agent would use:
observe state + action space → pick action → step.

Task: search for recent AI papers on arXiv.
"""

import grpc
import sys
import os

# ── Generated stubs (run generate.sh first) ──────────────────────

sys.path.insert(0, os.path.join(os.path.dirname(__file__)))
import rcm_pb2
import rcm_pb2_grpc
from google.protobuf import empty_pb2


class SimplePolicy:
    """
    A hard-coded policy that replicates a minimal Captain loop:
    pre → decide loop (Append prompt → Halt → Take → ...) → Done.

    In real use this would be an RL network (GRPO/PPO policy).
    """

    def __init__(self):
        self.prompt_appended = False
        self.take_done = False
        self.take_count = 0

    def pick(self, state, action_space):
        """Choose the next action from the action space."""
        actions = action_space.actions
        verbs = {a.command.verb for a in actions}

        # Phase 1: append the captain prompt to context
        if not self.prompt_appended:
            for a in actions:
                if a.command.verb == "Append":
                    self.prompt_appended = True
                    return a.command, a.label

        # Phase 2: halt → let the LLM + tools run
        if "Halt" in verbs and not self.take_done:
            return next(a.command for a in actions if a.command.verb == "Halt"), "Halt"

        # Phase 3: drain inbox (Take) after reactor
        if "Take" in verbs:
            self.take_count += 1
            if self.take_count > 20:
                # Safety net — if LLM doesn't respond, bail
                return next(
                    a.command for a in actions if a.command.verb == "Done"
                ), "Done"
            return next(
                a.command for a in actions if a.command.verb == "Take"
            ), f"Take #{self.take_count}"

        # Phase 4: done
        if "Done" in verbs:
            return next(a.command for a in actions if a.command.verb == "Done"), "Done"

        # Fallback
        return actions[0].command, actions[0].label


def main():
    channel = grpc.insecure_channel("localhost:50051")
    stub = rcm_pb2_grpc.RCMStub(channel)

    # ── Open a machine run ────────────────────────────────────

    resp = stub.Open(
        rcm_pb2.OpenRequest(
            purpose="search for recent AI research papers on arXiv",
            models=[""],  # server will use the default model from kit
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
    )

    machine_id = resp.machine_id
    state = resp.state
    action_space = resp.action_space

    print(f"Machine opened: {machine_id}")
    print(
        f"Initial state: step={state.step}, fragments={len(state.fragments)}, done={state.done}"
    )
    print(f"Actions available: {[a.label for a in action_space.actions]}")
    print()

    # ── Run the policy loop ───────────────────────────────────

    policy = SimplePolicy()
    max_steps = 30

    for t in range(max_steps):
        command, label = policy.pick(state, action_space)
        print(f"Step {t + 1}: {label} (verb={command.verb})")

        resp = stub.Step(
            rcm_pb2.StepRequest(
                machine_id=machine_id,
                command=command,
            )
        )

        state = resp.state
        action_space = resp.action_space

        print(
            f"  → state: step={state.step}, fragments={len(state.fragments)}, done={state.done}"
        )
        print(f"  → inbox_pending={state.inbox_pending}")
        if state.fragments:
            preview = state.fragments[-1].text_preview[:100]
            print(f"  → last fragment preview: {preview}...")

        if state.done:
            print("\nEpisode complete.")
            break
        print()

    print(f"\nFinal state: {len(state.fragments)} fragments")
    for i, frag in enumerate(state.fragments):
        print(f"  [{i}] {frag.role}/{frag.kind}: {frag.text_preview[:80]}")

    # ── Clean up ─────────────────────────────────────────────

    stub.Destroy(rcm_pb2.DestroyRequest(machine_id=machine_id))
    channel.close()


if __name__ == "__main__":
    main()
