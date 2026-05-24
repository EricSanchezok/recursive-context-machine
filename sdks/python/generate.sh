#!/bin/bash
# Regenerate Python gRPC stubs from proto/rcm.proto.
# Run from: sdks/python/
set -euo pipefail

PROTO="../../proto/rcm.proto"

PROTOBUF_INCLUDE=$(python -c '
import grpc_tools.protoc, inspect, os
base = os.path.dirname(inspect.getfile(grpc_tools.protoc))
for d in [os.path.join(base, "_proto"), os.path.join(base, "google")]:
    if os.path.isdir(d):
        print(os.path.dirname(d) if d.endswith("google") else d)
        exit(0)
# fallback
import google.protobuf
print(os.path.dirname(os.path.dirname(google.protobuf.__file__)))
')

python -m grpc_tools.protoc \
    -I "$(dirname "$PROTO")" \
    -I "$PROTOBUF_INCLUDE" \
    --python_out=src/rcm \
    --grpc_python_out=src/rcm \
    "$PROTO"

# Rename to private modules.
mv src/rcm/rcm_pb2.py src/rcm/_pb2.py
mv src/rcm/rcm_pb2_grpc.py src/rcm/_pb2_grpc.py

# Fix the auto-generated import to use relative import.
sed -i '' 's/^import rcm_pb2 as rcm__pb2$/from . import _pb2 as rcm__pb2/' src/rcm/_pb2_grpc.py

echo "Generated src/rcm/_pb2.py and src/rcm/_pb2_grpc.py"
