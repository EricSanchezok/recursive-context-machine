"""
RCM (Recursive Context Machine) Python SDK.

Usage:
    from rcm import RCMClient

    rcm = RCMClient("localhost:50051")
    mid, state, actions = rcm.open(
        purpose="fix the bug in auth.rs",
        models=["fast"],
        tools=["read", "edit"],
        prompts={"captain": "You are..."},
    )

    for _ in range(max_steps):
        command = policy(state, actions)
        state, actions = rcm.step(mid, command)
        if state.done:
            break

    rcm.destroy(mid)
"""

from .client import RCMClient
from .model import Model

__all__ = ["RCMClient", "Model"]
