"""Thin Python wrapper over the RCM gRPC service."""

import grpc

from ._pb2 import (
    ActionCommand,
    DestroyRequest,
    OpenRequest,
    StepRequest,
)
from ._pb2_grpc import RCMStub


class RCMClient:
    """Client for the RCM gRPC agent runtime.

    Connects to an RCM server and drives a machine step-by-step:

        rcm = RCMClient("localhost:50051")

        mid, state, actions = rcm.open(purpose="...", models=["fast"])
        state, actions = rcm.step(mid, command)
        rcm.destroy(mid)
    """

    def __init__(self, endpoint: str = "localhost:50051"):
        self._channel = grpc.insecure_channel(endpoint)
        self._stub = RCMStub(self._channel)

    def open(
        self,
        purpose: str,
        models: list[str] | None = None,
        tools: list[str] | None = None,
        prompts: dict[str, str] | None = None,
    ):
        """Create a new machine run.

        Returns (machine_id, state, action_space) — the triple that
        drives the episode loop.
        """
        resp = self._stub.Open(
            OpenRequest(
                purpose=purpose,
                models=models or [],
                tools=tools or [],
                prompts=prompts or {},
            )
        )
        return resp.machine_id, resp.state, resp.action_space

    def step(self, machine_id: str, command):
        """Execute one action. Returns (state, action_space).

        command is an ActionCommand message — typically taken from
        the action_space returned by open() or the previous step().
        """
        resp = self._stub.Step(StepRequest(machine_id=machine_id, command=command))
        return resp.state, resp.action_space

    def destroy(self, machine_id: str):
        """Release the machine run and all held resources."""
        self._stub.Destroy(DestroyRequest(machine_id=machine_id))
        self._channel.close()
