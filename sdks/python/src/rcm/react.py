"""Default reactive policy for Python RCM controllers.

A simple Captain-style loop without phases:

    Append prompt → Halt → Take (drain inbox) → repeat → Done.

Usage:
    from rcm.react import ReactPolicy

    policy = ReactPolicy()
    for _ in range(max_steps):
        cmd, label = policy(state, actions)
        state, actions = rcm.step(mid, cmd)
        if state.done:
            break
"""

from dataclasses import dataclass, field


@dataclass
class ReactPolicy:
    """Reactive policy that observes state at each step."""

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

        if "Halt" in verbs:
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
