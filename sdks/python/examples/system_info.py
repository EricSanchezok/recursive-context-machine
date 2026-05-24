"""System info — ask the agent to inspect the local machine."""

from rcm import RCMClient
from rcm.react import run


def main():
    rcm = RCMClient("localhost:50051")

    state = run(
        rcm,
        purpose="inspect the local machine hardware and resource usage",
        prompt=(
            "You are a system diagnostics assistant. Inspect the local machine "
            "and report:\n\n"
            "1. OS and hardware model (use shell: uname -a, sysctl -n machdep.cpu.brand_string)\n"
            "2. Memory (shell: vm_stat, sysctl hw.memsize)\n"
            "3. Disk usage (shell: df -h)\n"
            "4. CPU load (shell: top -l 1 | head -10)\n\n"
            "Run each command via the shell tool and summarize the results clearly.\n"
            "Stop after reporting."
        ),
        tools=["shell"],
        max_halts=3,
        verbose=True,
    )

    print("── System Report ──")
    for frag in state.fragments:
        if frag.role == "assistant":
            print(frag.text_preview)


if __name__ == "__main__":
    main()
