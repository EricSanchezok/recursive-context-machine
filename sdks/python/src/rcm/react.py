"""Default reactive policy for Python RCM controllers.

Handles the main decision loop: drain inbox, Halt, Done.
Setup (system prompt, purpose, tool activation) is the
controller's responsibility — not the policy's.

Usage:
    from rcm.react import ReactPolicy

    # ... controller runs setup with explicit ActionCommands ...
    policy = ReactPolicy()
    for _ in range(max_steps):
        cmd, label = policy(state, actions)
        state, actions = rcm.step(mid, cmd)
"""

from dataclasses import dataclass


@dataclass
class ReactPolicy:
    def __call__(self, state, action_space):
        actions = action_space.actions
        verbs = {a.command.verb for a in actions}

        if state.inbox_pending and "Take" in verbs:
            return self._pick(actions, "Take")

        if "Halt" in verbs:
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
