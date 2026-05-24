"""MCP server configuration for the RCM gRPC SDK.

Mirrors the .rcm language `mcp` block:

    mcp filesystem {
        transport = "stdio"
        command = "npx"
        args = ["-y", "@modelcontextprotocol/server-filesystem"]
        env = { HOME = env(HOME) }
    }

    mcp remote_api {
        transport = "http"
        url = "https://example.com/mcp"
        headers = { Authorization = "token123" }
    }

Python equivalent:
    from rcm import McpServer

    mcp = McpServer(
        label="filesystem",
        transport=McpServer.Stdio(
            command="npx",
            args=["-y", "@modelcontextprotocol/server-filesystem"],
            env={"HOME": McpServer.Env("HOME")},
        ),
    )
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

        def _resolve(self):
            import os

            if self.literal is not None:
                return self.literal
            if self.env is not None:
                return os.environ.get(self.env, "")
            return ""

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
            McpServerSpec,
            McpTransportSpec,
            McpStdioSpec,
            McpHttpSpec,
            McpSseSpec,
            McpValueSpec,
        )

        def _value_proto(v: "McpServer.Value"):
            return McpValueSpec(literal=v.literal, env=v.env)

        spec = McpServerSpec(label=self.label)
        transport = self.transport

        if isinstance(transport, McpServer.Stdio):
            spec.transport.CopyFrom(
                McpTransportSpec(
                    kind=McpStdioSpec(
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
                    kind=McpHttpSpec(
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
                    kind=McpSseSpec(
                        url=transport.url,
                        headers={
                            k: _value_proto(v) for k, v in transport.headers.items()
                        },
                    )
                )
            )

        return spec
