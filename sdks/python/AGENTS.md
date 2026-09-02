# Python SDK

This directory is the Python client for the gRPC contract in `../../proto/rcm.proto`.

- Keep generated `_pb2.py` and `_pb2_grpc.py` files synchronized with the proto.
- Regenerate with `bash generate.sh` after a proto change and review the generated
  diff rather than hand-editing generated modules.
- Keep public client behavior and model types in `src/rcm/`; put behavior tests in
  `tests/` and examples in `examples/`.
- Use the SDK's own `pyproject.toml`; the repository root is not a Python package.

Run `pytest` from this directory for the SDK test suite and run the Rust server
integration tests when the wire contract changes.
