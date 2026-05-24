"""Model configuration for the RCM gRPC SDK.

Mirrors the .rcm language `model` block:

    model deepseek-v4-flash {
        protocol = "openai"
        endpoint = "https://api.deepseek.com"
        credentials = { env = "DEEPSEEK_API_KEY" }
        limit = { context = "128000", output = "8192" }
    }

Python equivalent:
    from rcm import Model

    model = Model(
        name="deepseek-v4-flash",
        protocol="openai",
        endpoint="https://api.deepseek.com",
        credentials=Model.Credentials(env="DEEPSEEK_API_KEY"),
        limit=Model.Limit(context=128000, output=8192),
    )
"""

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class Model:
    """LLM provider configuration."""

    name: str
    protocol: str = "openai"
    endpoint: Optional[str] = None
    timeout: Optional[int] = None

    credentials: Optional["Model.Credentials"] = None
    limit: Optional["Model.Limit"] = None

    modalities_input: list[str] = field(default_factory=list)
    modalities_output: list[str] = field(default_factory=list)
    headers: dict[str, str] = field(default_factory=dict)

    @dataclass
    class Credentials:
        env: Optional[str] = None
        literal: Optional[str] = None

        def __post_init__(self):
            if self.env is not None and self.literal is not None:
                raise ValueError("env and literal are mutually exclusive")

    @dataclass
    class Limit:
        context: int
        output: int
        input: Optional[int] = None

    def _to_proto(self):
        from ._pb2 import ModelSpec, CredentialSpec, LimitSpec

        spec = ModelSpec(
            name=self.name,
            protocol=self.protocol,
            endpoint=self.endpoint,
            timeout=self.timeout,
            modalities_input=self.modalities_input,
            modalities_output=self.modalities_output,
            headers=self.headers,
        )
        if self.credentials is not None:
            if self.credentials.env:
                spec.credentials.CopyFrom(CredentialSpec(env=self.credentials.env))
            elif self.credentials.literal:
                spec.credentials.CopyFrom(
                    CredentialSpec(literal=self.credentials.literal)
                )
        if self.limit is not None:
            spec.limit.CopyFrom(
                LimitSpec(
                    context=self.limit.context,
                    output=self.limit.output,
                    input=self.limit.input,
                )
            )
        return spec
