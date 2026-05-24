#!/bin/bash
# Generate Python gRPC stubs from rcm.proto.
# Run this from: examples/python-control/
set -euo pipefail

PROTO_DIR="../../crates/server/proto"

# Find the Google proto include path (comes with grpcio-tools)
PROTOBUF_PATH=$(python -c '
import grpc_tools.protoc
import inspect, os
src = inspect.getfile(grpc_tools.protoc)
# Look inside the installed package
base = os.path.dirname(src)
candidate = os.path.join(base, "_proto")
if os.path.isdir(candidate):
    print(candidate)
    exit(0)
# Fallback: search the protobuf package
import google.protobuf
pb = os.path.dirname(google.protobuf.__file__)
print(os.path.dirname(pb))
')

echo "Proto path: $PROTOBUF_PATH"

python -m grpc_tools.protoc \
  -I "$PROTO_DIR" \
  -I "$PROTOBUF_PATH" \
  --python_out=src \
  --grpc_python_out=src \
  "$PROTO_DIR/rcm.proto"

echo "Generated src/rcm_pb2.py and src/rcm_pb2_grpc.py"
