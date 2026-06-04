import json
import sys


def write_response(request_id, result):
    sys.stdout.write(
        json.dumps({"jsonrpc": "2.0", "id": request_id, "result": result}) + "\n"
    )
    sys.stdout.flush()


def write_error(request_id, code, message):
    sys.stdout.write(
        json.dumps(
            {
                "jsonrpc": "2.0",
                "id": request_id,
                "error": {"code": code, "message": message},
            }
        )
        + "\n"
    )
    sys.stdout.flush()


def tool_definitions():
    number_schema = {"type": "number"}
    return [
        {
            "name": "add",
            "description": "Add two numbers and return the exact result.",
            "inputSchema": {
                "type": "object",
                "properties": {"left": number_schema, "right": number_schema},
                "required": ["left", "right"],
            },
        },
        {
            "name": "multiply",
            "description": "Multiply two numbers and return the exact result.",
            "inputSchema": {
                "type": "object",
                "properties": {"left": number_schema, "right": number_schema},
                "required": ["left", "right"],
            },
        },
    ]


def call_tool(name, arguments):
    left = arguments.get("left")
    right = arguments.get("right")
    if not isinstance(left, (int, float)) or not isinstance(right, (int, float)):
        raise ValueError("left and right must be numbers")

    if name == "add":
        result = left + right
    elif name == "multiply":
        result = left * right
    else:
        raise ValueError(f"unknown tool: {name}")

    return {
        "content": [{"type": "text", "text": str(result)}],
        "structuredContent": {"result": result},
    }


def handle(request):
    method = request.get("method")
    request_id = request.get("id")

    if method == "initialize":
        write_response(
            request_id,
            {
                "protocolVersion": "2025-06-18",
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "math", "version": "0.1.0"},
            },
        )
    elif method == "notifications/initialized":
        return
    elif method == "tools/list":
        write_response(request_id, {"tools": tool_definitions()})
    elif method == "tools/call":
        params = request.get("params") or {}
        try:
            write_response(
                request_id, call_tool(params.get("name"), params.get("arguments") or {})
            )
        except ValueError as error:
            write_error(request_id, -32602, str(error))
    else:
        write_error(request_id, -32601, f"unknown method: {method}")


def main():
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            handle(json.loads(line))
        except json.JSONDecodeError as error:
            write_error(None, -32700, str(error))


if __name__ == "__main__":
    main()
