"""RCM Paper Digest — gRPC Full Lifecycle Demo.

Demonstrates the complete Open → Setup → Policy loop → Destroy lifecycle
through Python gRPC, mirroring the paper digest pipeline from
examples/research-assistant/rcm/arxiv_pipeline.rcm.

Usage:
    # Terminal 1: start server
    DEEPSEEK_API_KEY="REDACTED" ./target/release/rcm-server

    # Terminal 2: run demo
    DEEPSEEK_API_KEY="REDACTED" python examples/research-assistant/grpc_demo.py
"""

import os, sys, time, shutil

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "../../sdks/python/src"))
from rcm import RCMClient, Model
from rcm._pb2 import ActionCommand, FragmentContent
from rcm.react import ReactPolicy

API_KEY = os.environ.get("DEEPSEEK_API_KEY", "")
ENDPOINT = os.environ.get("RCM_ENDPOINT", "localhost:50051")
TERM_WIDTH = shutil.get_terminal_size().columns if shutil.get_terminal_size().columns else 100


# ── ANSI helpers ──
C = lambda s, c: f"\033[{c}m{s}\033[0m"
BOLD = lambda s: C(s, "1")
DIM = lambda s: C(s, "2")
GREEN = lambda s: C(s, "32")
YELLOW = lambda s: C(s, "33")
CYAN = lambda s: C(s, "36")
MAGENTA = lambda s: C(s, "35")
RED = lambda s: C(s, "31")


def section(title):
    w = TERM_WIDTH
    print()
    print(DIM("─" * w))
    print(f"  {BOLD(title)}")
    print(DIM("─" * w))


def state_vars(state):
    return (
        f"done={state.done}  "
        f"frags={len(state.fragments)}  "
        f"model={state.active_model or '(none)'}  "
        f"tools={','.join(state.active_tools) or '(none)'}  "
        f"inbox={'Y' if state.inbox_pending else 'N'}"
    )


def fragment_label(f):
    role_colors = {
        "system": DIM, "user": CYAN, "assistant": GREEN, "tool": YELLOW,
    }
    rc = role_colors.get(f.role, DIM)
    kind_icon = {
        "hitch": RED("\u26a1"),
        "tool_call": MAGENTA("\U0001f527"),
        "tool_result": YELLOW("\U0001f4e6"),
        "text": DIM("\U0001f4ac"),
    }.get(f.kind, DIM("\U0001f4c4"))
    preview = f.text_preview[:100].replace("\n", " ")
    return f"  #{f.id} {rc(f.role):>10} {kind_icon} {rc(f.kind)} {DIM('|')} {preview}"


def main():
    print()
    print(BOLD(f"{' RCM Paper Digest — gRPC Demo ':=^{min(80, TERM_WIDTH)}}"))
    print(DIM(f"  endpoint={ENDPOINT}"))
    print(DIM(f"  api_key={'set' if API_KEY else 'NOT SET (LLM calls will fail)'}"))
    print()

    rcm = RCMClient(ENDPOINT)

    # ── Open ──
    section("1\ufe0f\u20e3 Open — Create Machine")
    model = Model(
        name="deepseek-v4-flash",
        protocol="openai",
        endpoint="https://api.deepseek.com",
        credentials=Model.Credentials(env="DEEPSEEK_API_KEY"),
        limit=Model.Limit(context=1_000_000, output=393_216),
    )
    mid, state, actions = rcm.open(
        purpose="Search arXiv for recent papers on agent dataflow, summarize in Chinese",
        models=[model],
        tools=["shell", "arxiv_search"],
        prompts={"captain": "You are a research assistant. Use shell and arxiv_search."},
    )
    print(f"  machine_id:  {BOLD(mid)}")
    print(f"  purpose:     {DIM(state.purpose[:70])}")
    print(f"  platform:    {DIM(state.platform)}")
    print(f"  tool_profiles: {len(state.tool_profiles)} tools registered")
    print(f"  model_profiles: {len(state.model_profiles)} models registered")
    for tp in state.tool_profiles:
        print(f"    {tp.name:>20}  {DIM(tp.description[:60])}")

    # ── Setup ──
    section("2\ufe0f\u20e3 Setup — Inject Prompts + Activate Model + Tools")
    for label, tag, text in [
        ("Append system prompt", "agent", "You are a research assistant."),
        ("Append user purpose", "purpose", state.purpose),
    ]:
        state, actions = rcm.step(mid, ActionCommand(
            verb="Append",
            fragment=FragmentContent(role="user" if tag == "purpose" else "system", text=text, tag=tag),
        ))
        print(f"  {GREEN('\u2713')} {label}")

    state, actions = rcm.step(mid, ActionCommand(verb="Model", name="deepseek-v4-flash"))
    print(f"  {GREEN('\u2713')} Model activated: {state.active_model}")

    for t in ("shell", "arxiv_search"):
        state, actions = rcm.step(mid, ActionCommand(verb="Activate", name=t))
    print(f"  {GREEN('\u2713')} Tools activated: {', '.join(state.active_tools)}")
    print(f"  State: {DIM(state_vars(state))}")

    # ── Policy Loop ──
    section("3\ufe0f\u20e3 Agent Loop")
    policy = ReactPolicy()

    for step_idx in range(20):
        cmd, label = policy(state, actions)
        print(f"  {BOLD(f'Step {step_idx+1:>2}')} {YELLOW(label):>8}  {DIM(state_vars(state))}")

        t0 = time.time()
        state, actions = rcm.step(mid, cmd)
        elapsed = time.time() - t0

        if cmd.verb == "Halt":
            print(f"           LLM: {elapsed:.1f}s")

        if cmd.verb == "Take" and state.fragments:
            print(fragment_label(state.fragments[-1]))

        if state.done:
            print(f"\n  {GREEN('\u2713')} Machine done")
            break

    # ── Report ──
    section("4\ufe0f\u20e3 Report")
    print(f"\n  {BOLD('Fragment Timeline')} (last 15):")
    for f in state.fragments[-15:]:
        print(fragment_label(f))

    print(f"\n  {BOLD('Counts')}:")
    for k, v in sorted(state.counts.items()):
        print(f"    {k:>10}: {v}")

    # ── Destroy ──
    section("5\ufe0f\u20e3 Cleanup")
    rcm.destroy(mid)
    print(f"  {GREEN('\u2713')} Machine destroyed")
    print()
    print(BOLD(f"{' Done ':=^{min(80, TERM_WIDTH)}}"))
    print(f"  Fragments: {len(state.fragments)}")
    print(f"  LLM calls: {state.counts.get('halt', 0)}")
    print(f"  Tools:     {state.counts.get('take', 0)} fragments consumed")


if __name__ == "__main__":
    main()
