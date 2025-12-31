"""MCP server tests for angreal."""
import subprocess
import time

import angreal

from task_test import test

# Nested group under test for MCP tests
mcp_tests = angreal.command_group(name="mcp", about="MCP server tests")


def start_mcp_server():
    """Helper to start MCP server as subprocess."""
    proc = subprocess.Popen(
        ["angreal", "mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    return proc


def send_mcp_request(proc, request: dict):
    """Send a JSON-RPC request to MCP server."""
    import json
    proc.stdin.write(json.dumps(request) + "\n")
    proc.stdin.flush()


def read_mcp_response(proc, timeout: float = 5.0):
    """Read JSON-RPC response from MCP server."""
    import json
    import select
    import sys

    # Use select on Unix, simple readline on Windows
    if sys.platform != "win32":
        ready, _, _ = select.select([proc.stdout], [], [], timeout)
        if not ready:
            raise TimeoutError("No response from MCP server")

    # Read lines until we find valid JSON (skip debug output from rust-mcp-sdk)
    max_attempts = 10
    for _ in range(max_attempts):
        line = proc.stdout.readline()
        if not line:
            stderr = proc.stderr.read()
            raise RuntimeError(f"MCP server closed connection: {stderr}")
        line = line.strip()
        if not line:
            continue
        # Try to parse as JSON
        if line.startswith('{'):
            try:
                return json.loads(line)
            except json.JSONDecodeError:
                continue
        # Skip debug lines (e.g., ">>>  Ok(..." from rust-mcp-sdk)

    raise RuntimeError(f"No valid JSON response after {max_attempts} lines")


@test()
@mcp_tests()
@angreal.command(
    name="startup",
    about="Test MCP server startup and initialization",
    tool=angreal.ToolDescription("""
Test that the MCP server starts correctly and responds to initialize request.

## When to use
- After MCP server changes
- To verify server starts without panic
- Before releases

## When NOT to use
- When MCP code unchanged

## Examples
```
angreal test mcp startup
```
""", risk_level="safe")
)
def test_mcp_startup():
    """
    Test MCP server starts correctly and responds to initialize request.
    Catches regressions like the tracing subscriber panic in v2.7.1.
    """
    print("Starting MCP server...", flush=True)
    proc = start_mcp_server()

    try:
        # Give it time to start (would panic immediately if broken)
        time.sleep(1)

        # Check it's still running
        if proc.poll() is not None:
            stderr = proc.stderr.read()
            print(f"FAIL: MCP server exited early: {stderr}", flush=True)
            return 1

        print("OK: Server started without immediate crash", flush=True)

        # Send initialize request
        print("Sending initialize request...", flush=True)
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "angreal-test", "version": "1.0"}
            }
        }
        send_mcp_request(proc, init_request)

        # Read response
        response = read_mcp_response(proc)

        # Validate response structure
        assert response.get("jsonrpc") == "2.0", f"Bad jsonrpc version: {response}"
        assert response.get("id") == 1, f"Bad response id: {response}"
        assert "result" in response, f"No result in response: {response}"

        result = response["result"]
        assert "serverInfo" in result, f"No serverInfo in result: {result}"
        assert "capabilities" in result, f"No capabilities in result: {result}"

        print(f"OK: Server initialized: {result.get('serverInfo', {})}", flush=True)
        print("PASS: MCP startup test", flush=True)

    except TimeoutError as e:
        print(f"FAIL: Timeout waiting for response: {e}", flush=True)
        return 1
    except Exception as e:
        print(f"FAIL: Test failed: {e}", flush=True)
        return 1
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()


@test()
@mcp_tests()
@angreal.command(
    name="tools",
    about="Test MCP tool discovery",
    tool=angreal.ToolDescription("""
Test that the MCP server discovers and exposes angreal tasks as tools.

## When to use
- After task definition changes
- To verify tools are exposed correctly
- Before releases

## When NOT to use
- When task definitions unchanged

## Examples
```
angreal test mcp tools
```
""", risk_level="safe")
)
def test_mcp_tools():
    """
    Test MCP server discovers and exposes angreal tasks as tools.
    """
    print("Starting MCP server...")
    proc = start_mcp_server()

    try:
        time.sleep(1)
        if proc.poll() is not None:
            stderr = proc.stderr.read()
            print(f"FAIL: MCP server exited early: {stderr}")
            return 1

        # Initialize first
        print("Initializing server...")
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "angreal-test", "version": "1.0"}
            }
        }
        send_mcp_request(proc, init_request)
        init_response = read_mcp_response(proc)
        assert "result" in init_response, f"Initialize failed: {init_response}"

        # Send initialized notification
        send_mcp_request(proc, {
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        })

        # Request tool list
        print("Requesting tool list...")
        send_mcp_request(proc, {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        })

        response = read_mcp_response(proc)
        assert "result" in response, f"tools/list failed: {response}"

        tools = response["result"].get("tools", [])
        print(f"Found {len(tools)} tools")

        if len(tools) == 0:
            print("WARN: No tools discovered (may be expected outside angreal project)")
        else:
            # Verify tool structure
            tool_names = [t.get("name") for t in tools]
            suffix = "..." if len(tool_names) > 10 else ""
            print(f"Tool names: {tool_names[:10]}{suffix}")

            # Check for expected tools (test group should exist in this project)
            has_test_tools = any("test" in name for name in tool_names)
            if has_test_tools:
                print("OK: Found test-related tools")
            else:
                print("WARN: No test tools found (may be expected)")

            # Verify each tool has required fields
            for tool in tools:
                assert "name" in tool, f"Tool missing name: {tool}"
                assert "description" in tool, f"Tool missing description: {tool}"

                # Verify inputSchema is valid if present
                if "inputSchema" in tool:
                    schema = tool["inputSchema"]
                    assert schema.get("type") == "object", \
                        f"Invalid inputSchema type for {tool['name']}: {schema}"

            print("OK: All tools have valid structure")

        print("PASS: MCP tool discovery test")

    except TimeoutError as e:
        print(f"FAIL: Timeout waiting for response: {e}")
        return 1
    except Exception as e:
        print(f"FAIL: Test failed: {e}")
        return 1
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()


@test()
@mcp_tests()
@angreal.command(
    name="all",
    about="Run all MCP server tests",
    tool=angreal.ToolDescription("""
Run all MCP server tests (startup and tool discovery).

## When to use
- Before releases
- After MCP-related changes
- For comprehensive MCP validation

## When NOT to use
- When MCP code unchanged

## Examples
```
angreal test mcp all
```
""", risk_level="safe")
)
def test_mcp_all():
    """Run all MCP server tests."""
    print("=== Running All MCP Tests ===\n")
    failures = []

    print("1. Testing MCP startup...")
    result = test_mcp_startup()
    if result:
        failures.append("MCP startup")

    print("\n2. Testing MCP tool discovery...")
    result = test_mcp_tools()
    if result:
        failures.append("MCP tool discovery")

    if failures:
        print(f"\nFAIL: The following MCP tests failed: {', '.join(failures)}")
        return 1

    print("\nPASS: All MCP tests")
