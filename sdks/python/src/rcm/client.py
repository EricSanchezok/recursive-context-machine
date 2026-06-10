"""Thin Python wrapper over the RCM gRPC service."""

import grpc

from ._pb2 import DestroyRequest, OpenRequest, StepRequest
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
        *,
        model_definitions: list[Model] | None = None,
        models: list[str] | None = None,
        mcp_definitions: list[McpServer] | None = None,
        mcps: list[str] | None = None,
        tools: list[str] | None = None,
        prompts: dict[str, str] | None = None,
        environment: str | None = None,
    ):
        request = OpenRequest(
            purpose=purpose,
            model_definitions=[
                model._to_proto() for model in (model_definitions or [])
            ],
            mcp_definitions=[server._to_proto() for server in (mcp_definitions or [])],
            models=models or [],
            mcps=mcps or [],
            tools=tools or [],
            prompts=prompts or {},
        )
        if environment is not None:
            request.environment = environment
        response = self._stub.Open(request)
        return response.machine_id, response.state, response.action_space

    def step(self, machine_id: str, command):
        response = self._stub.Step(StepRequest(machine_id=machine_id, command=command))
        return response.state, response.action_space

    def destroy(self, machine_id: str):
        self._stub.Destroy(DestroyRequest(machine_id=machine_id))

    def close(self):
        self._channel.close()
