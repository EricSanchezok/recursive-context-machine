"""Default reactive policy for Python RCM controllers.

Replicates the Captain loop without phases (phases are Rust-internal
and not exposed over gRPC).

Usage:
    from rcm.react import run

    state = run(rcm, purpose="...", prompt="You are...", tools=["shell"])
    print(state.fragments[-1].text_preview)
"""

from dataclasses import dataclass, field
from .client import RCMClient


@dataclass
class ReactPolicy:
    max_halts: int = 5

    _halt_count: int = field(default=0, init=False)
    _prompt_appended: bool = field(default=False, init=False)

    def __call__(self, state, action_space):
        actions = action_space.actions
        verbs = {a.command.verb for a in actions}

        if state.inbox_pending and "Take" in verbs:
            return self._pick(actions, "Take")

        if not self._prompt_appended and "Append" in verbs:
            self._prompt_appended = True
            return self._pick(actions, "Append")

        if self._halt_count < self.max_halts and "Halt" in verbs:
            self._halt_count += 1
            return self._pick(actions, "Halt")

        if "Done" in verbs:
            return self._pick(actions, "Done")

        return actions[-1].command, actions[-1].label

    @staticmethod
    def _pick(actions, verb):
        for a in actions:
            if a.command.verb == verb:
                return a.command, a.label
        raise ValueError(f"no {verb} action available")


def run(
    rcm: RCMClient,
    purpose: str,
    prompt: str,
    *,
    models: list[str] | None = None,
    tools: list[str] | None = None,
    max_steps: int = 30,
    max_halts: int = 5,
    verbose: bool = False,
):
    """Run a single episode with the reactive policy.

    Returns the final state. Inspect state.fragments for output.
    """
    mid, state, actions = rcm.open(
        purpose=purpose,
        models=models or [""],
        tools=tools or [],
        prompts={"captain": prompt},
    )

    react = ReactPolicy(max_halts=max_halts)

    for step in range(max_steps):
        command, label = react(state, actions)

        if verbose:
            print(f"[{step + 1}] {label}")

        state, actions = rcm.step(mid, command)

        if verbose and state.fragments:
            frag = state.fragments[-1]
            print(f"  → [{frag.role}/{frag.kind}] {frag.text_preview[:120]}")

        if state.done:
            if verbose:
                print("done.\n")
            break

    rcm.destroy(mid)
    return state
