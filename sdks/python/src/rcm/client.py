"""Thin Python wrapper over the RCM gRPC service."""

import grpc

from ._pb2 import (
    ActionCommand,
    DestroyRequest,
    OpenRequest,
    StepRequest,
)
from ._pb2_grpc import RCMStub
from .mcp import McpServer
from .model import Model


class RCMClient:
    def __init__(self, endpoint: str = "localhost:50051"):
        self._channel = grpc.insecure_channel(endpoint)
        self._stub = RCMStub(self._channel)

    def open(
        self,
        purpose: str,
        models: list[Model] | None = None,
        mcps: list[McpServer] | None = None,
        tools: list[str] | None = None,
        prompts: dict[str, str] | None = None,
    ):
        resp = self._stub.Open(
            OpenRequest(
                purpose=purpose,
                models=[m._to_proto() for m in (models or [])],
                mcps=[s._to_proto() for s in (mcps or [])],
                tools=tools or [],
                prompts=prompts or {},
            )
        )
        return resp.machine_id, resp.state, resp.action_space

    def step(self, machine_id: str, command):
        resp = self._stub.Step(StepRequest(machine_id=machine_id, command=command))
        return resp.state, resp.action_space

    def destroy(self, machine_id: str):
        self._stub.Destroy(DestroyRequest(machine_id=machine_id))

    def close(self):
        self._channel.close()
