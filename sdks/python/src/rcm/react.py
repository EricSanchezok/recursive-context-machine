"""Default reactive policy for Python RCM controllers.

Mirrors the Rust Captain's decision logic:
  - Inbox not empty         → Take
  - First call ever         → Halt
  - Last was tool_result    → Halt (LLM needs to read the tool output)
  - Inbox empty + idle      → Done (natural termination)

Setup (system prompt, purpose, tool activation) is the
controller's responsibility.
"""

from dataclasses import dataclass, field


@dataclass
class ReactPolicy:
    first: bool = True

    def __call__(self, state, action_space):
        actions = action_space.actions
        verbs = {action.command.verb for action in actions}

        if state.inbox_pending and "Take" in verbs:
            return self._pick(actions, "Take")

        if self.first:
            self.first = False
            if "Halt" in verbs:
                return self._pick(actions, "Halt")
            if "Done" in verbs:
                return self._pick(actions, "Done")
            raise ValueError("no Halt or Done action available")

        if self._last_was_tool_result(state):
            if "Halt" in verbs:
                return self._pick(actions, "Halt")
            raise ValueError("no Halt action available after tool result")

        if "Done" in verbs:
            return self._pick(actions, "Done")

        if "Halt" in verbs:
            return self._pick(actions, "Halt")
        return actions[-1].command, actions[-1].label

    @staticmethod
    def _last_was_tool_result(state) -> bool:
        fragments = state.fragments
        return len(fragments) > 0 and fragments[-1].role == "tool"

    @staticmethod
    def _pick(actions, verb):
        for action in actions:
            if action.command.verb == verb:
                return action.command, action.label
        raise ValueError(f"no {verb} action available")
