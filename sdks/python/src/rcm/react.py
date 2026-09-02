"""Small ReAct controller for external Python RCM loops."""

from dataclasses import dataclass


@dataclass
class ReactPolicy:
    max_hitch_retries: int = 3
    hitch_retries: int = 0

    def __call__(self, state, action_space):
        actions = list(action_space.actions)

        if state.inbox_pending:
            return self._pick(actions, "Take")

        last_fragment = state.fragments[-1] if state.fragments else None

        if last_fragment is not None and last_fragment.kind == "hitch":
            if self.hitch_retries < self.max_hitch_retries:
                self.hitch_retries += 1
                return self._pick(actions, "Halt")
            return self._pick(actions, "Done")

        self.hitch_retries = 0

        if self._should_call_model(last_fragment):
            return self._pick(actions, "Halt")

        if last_fragment is not None and self._last_was_tool_result(last_fragment):
            return self._pick(actions, "Halt")

        if self._has(actions, "Done"):
            return self._pick(actions, "Done")

        return self._pick(actions, "Halt")

    @staticmethod
    def _should_call_model(fragment) -> bool:
        if fragment is None:
            return False
        return fragment.role in {"user", "system"} and fragment.kind == "text"

    @staticmethod
    def _last_was_tool_result(fragment) -> bool:
        return fragment.role == "tool" or fragment.kind == "tool_result"

    @staticmethod
    def _has(actions, verb: str) -> bool:
        return any(action.command and action.command.verb == verb for action in actions)

    @staticmethod
    def _pick(actions, verb: str):
        for action in actions:
            if action.command and action.command.verb == verb:
                return action.command, action.label
        raise ValueError(f"no {verb} action available")
