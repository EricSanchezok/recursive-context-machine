"""MCP server configuration for the RCM gRPC SDK.

Mirrors the .rcm language `mcp` block.
"""

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class McpServer:
    label: str
    transport: "McpServer.Stdio | McpServer.Http | McpServer.Sse"

    @staticmethod
    def Env(name: str):
        return McpServer.Value(env=name)

    @dataclass
    class Value:
        literal: Optional[str] = None
        env: Optional[str] = None

    @dataclass
    class Stdio:
        command: str
        args: list[str] = field(default_factory=list)
        env: dict[str, "McpServer.Value"] = field(default_factory=dict)
        cwd: Optional[str] = None

    @dataclass
    class Http:
        url: str
        headers: dict[str, "McpServer.Value"] = field(default_factory=dict)

    @dataclass
    class Sse:
        url: str
        headers: dict[str, "McpServer.Value"] = field(default_factory=dict)

    def _to_proto(self):
        from ._pb2 import (
            McpHttpSpec,
            McpServerSpec,
            McpSseSpec,
            McpStdioSpec,
            McpTransportSpec,
            McpValueSpec,
        )

        def _value_proto(value: "McpServer.Value"):
            if value.env is not None:
                return McpValueSpec(env=value.env)
            if value.literal is not None:
                return McpValueSpec(literal=value.literal)
            return McpValueSpec()

        spec = McpServerSpec(label=self.label)
        transport = self.transport

        if isinstance(transport, McpServer.Stdio):
            spec.transport.CopyFrom(
                McpTransportSpec(
                    stdio=McpStdioSpec(
                        command=transport.command,
                        args=transport.args,
                        env={k: _value_proto(v) for k, v in transport.env.items()},
                        cwd=transport.cwd,
                    )
                )
            )
        elif isinstance(transport, McpServer.Http):
            spec.transport.CopyFrom(
                McpTransportSpec(
                    http=McpHttpSpec(
                        url=transport.url,
                        headers={
                            k: _value_proto(v) for k, v in transport.headers.items()
                        },
                    )
                )
            )
        elif isinstance(transport, McpServer.Sse):
            spec.transport.CopyFrom(
                McpTransportSpec(
                    sse=McpSseSpec(
                        url=transport.url,
                        headers={
                            k: _value_proto(v) for k, v in transport.headers.items()
                        },
                    )
                )
            )

        return spec
