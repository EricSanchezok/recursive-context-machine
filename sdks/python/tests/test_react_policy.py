"""Tests for ReactPolicy termination logic."""

from rcm._pb2 import Fragment, ActionSpace, ActionItem, ActionCommand
from rcm.react import ReactPolicy


def _make_state(fragments: list, inbox_pending: bool = False):
    return type("FakeState", (), {
        "fragments": fragments,
        "inbox_pending": inbox_pending,
    })()


def _make_action_space(*verbs: str) -> ActionSpace:
    return ActionSpace(actions=[
        ActionItem(command=ActionCommand(verb=verb), label=verb)
        for verb in verbs
    ])


def _make_fragment(role: str, kind: str = "text") -> Fragment:
    return Fragment(id=1, role=role, kind=kind, text_preview="content", tag=kind)


# ── First call ──

def test_first_call_halts():
    policy = ReactPolicy()
    state = _make_state([])
    actions = _make_action_space("Halt", "Done")
    cmd, _ = policy(state, actions)
    assert cmd.verb == "Halt"
    assert not policy.first


def test_first_call_done_fallback():
    policy = ReactPolicy()
    state = _make_state([])
    actions = _make_action_space("Done")
    cmd, _ = policy(state, actions)
    assert cmd.verb == "Done"


# ── Inbox ──

def test_inbox_pending_takes():
    policy = ReactPolicy()
    policy.first = False
    state = _make_state([], inbox_pending=True)
    actions = _make_action_space("Take", "Halt", "Done")
    cmd, _ = policy(state, actions)
    assert cmd.verb == "Take"


# ── Last was tool_result → Halt ──

def test_last_was_tool_result_halts():
    policy = ReactPolicy()
    policy.first = False
    state = _make_state([_make_fragment("tool")])
    actions = _make_action_space("Halt", "Done")
    cmd, _ = policy(state, actions)
    assert cmd.verb == "Halt"


# ── Inbox empty + idle → Done ──

def test_idle_dones():
    policy = ReactPolicy()
    policy.first = False
    state = _make_state([_make_fragment("assistant")])
    actions = _make_action_space("Halt", "Done")
    cmd, _ = policy(state, actions)
    assert cmd.verb == "Done"


def test_idle_with_empty_fragments_dones():
    policy = ReactPolicy()
    policy.first = False
    state = _make_state([], inbox_pending=False)
    actions = _make_action_space("Halt", "Done")
    cmd, _ = policy(state, actions)
    assert cmd.verb == "Done"


# ── No Done available → Halt fallback ──

def test_no_done_fallback_to_halt():
    policy = ReactPolicy()
    policy.first = False
    state = _make_state([_make_fragment("assistant")])
    actions = _make_action_space("Halt")
    cmd, _ = policy(state, actions)
    assert cmd.verb == "Halt"
