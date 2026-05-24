"""System info — ask the agent to inspect the local machine."""

from rcm import RCMClient
from rcm._pb2 import ActionCommand, FragmentContent
from rcm.react import ReactPolicy

CAPTAIN_PROMPT = (
    "You are a system diagnostics assistant. Inspect the local machine "
    "and report:\n\n"
    "1. OS and hardware model (shell: uname -a, sysctl -n machdep.cpu.brand_string)\n"
    "2. Memory (shell: vm_stat, sysctl hw.memsize)\n"
    "3. Disk usage (shell: df -h)\n"
    "4. CPU load (shell: top -l 1 | head -10)\n\n"
    "Run each command via the shell tool and summarize the results clearly.\n"
    "Stop after reporting."
)


def main():
    rcm = RCMClient("localhost:50051")

    mid, state, actions = rcm.open(
        purpose="inspect the local machine hardware and resource usage",
        tools=["shell"],
        prompts={"captain": CAPTAIN_PROMPT},
    )

    # Setup — explicit action commands, not from action space.
    rcm.step(
        mid,
        ActionCommand(
            verb="Append",
            fragment=FragmentContent(
                role="system",
                text=CAPTAIN_PROMPT,
                tag="agent",
            ),
        ),
    )
    rcm.step(
        mid,
        ActionCommand(
            verb="Append",
            fragment=FragmentContent(
                role="user",
                text="inspect the local machine hardware and resource usage",
                tag="purpose",
            ),
        ),
    )

    policy = ReactPolicy()

    for step in range(30):
        cmd, label = policy(state, actions)
        print(f"[{step + 1}] {label}")

        state, actions = rcm.step(mid, cmd)

        if state.fragments:
            frag = state.fragments[-1]
            print(f"  → [{frag.role}/{frag.kind}] {frag.text_preview[:120]}")

        if state.done:
            print("done.\n")
            break

    rcm.destroy(mid)

    print("── System Report ──")
    for frag in state.fragments:
        if frag.role == "assistant":
            print(frag.text_preview)


if __name__ == "__main__":
    main()
