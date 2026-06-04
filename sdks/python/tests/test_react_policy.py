from rcm._pb2 import ActionCommand, ActionItem, ActionSpace, Fragment
from rcm.react import ReactPolicy


def _make_state(fragments: list, inbox_pending: bool = False):
    return type(
        "FakeState",
        (),
        {
            "fragments": fragments,
            "inbox_pending": inbox_pending,
        },
    )()


def _make_action_space(*verbs: str) -> ActionSpace:
    return ActionSpace(
        actions=[
            ActionItem(command=ActionCommand(verb=verb), label=verb) for verb in verbs
        ]
    )


def _make_fragment(role: str, kind: str = "text") -> Fragment:
    return Fragment(id=1, role=role, kind=kind, text_preview="content", tag=kind)


def test_inbox_pending_takes():
    policy = ReactPolicy()
    state = _make_state([], inbox_pending=True)
    actions = _make_action_space("Take", "Halt", "Done")
    command, _ = policy(state, actions)
    assert command.verb == "Take"


def test_user_fragment_halts_for_model_response():
    policy = ReactPolicy()
    state = _make_state([_make_fragment("user")])
    actions = _make_action_space("Halt", "Done")
    command, _ = policy(state, actions)
    assert command.verb == "Halt"


def test_last_tool_result_halts_for_model_response():
    policy = ReactPolicy()
    state = _make_state([_make_fragment("tool", "tool_result")])
    actions = _make_action_space("Halt", "Done")
    command, _ = policy(state, actions)
    assert command.verb == "Halt"


def test_idle_after_assistant_dones():
    policy = ReactPolicy()
    state = _make_state([_make_fragment("assistant")])
    actions = _make_action_space("Halt", "Done")
    command, _ = policy(state, actions)
    assert command.verb == "Done"


def test_idle_with_empty_fragments_dones():
    policy = ReactPolicy()
    state = _make_state([], inbox_pending=False)
    actions = _make_action_space("Halt", "Done")
    command, _ = policy(state, actions)
    assert command.verb == "Done"


def test_hitch_retries_then_dones():
    policy = ReactPolicy(max_hitch_retries=1)
    state = _make_state([_make_fragment("system", "hitch")])
    actions = _make_action_space("Halt", "Done")

    first_command, _ = policy(state, actions)
    second_command, _ = policy(state, actions)

    assert first_command.verb == "Halt"
    assert second_command.verb == "Done"


def test_no_done_fallback_to_halt():
    policy = ReactPolicy()
    state = _make_state([_make_fragment("assistant")])
    actions = _make_action_space("Halt")
    command, _ = policy(state, actions)
    assert command.verb == "Halt"
