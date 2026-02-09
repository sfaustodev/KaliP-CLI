#!/usr/bin/env python3
"""
Environment Bridge MCP Server

Provides shared access to a central .env file for all AI agents.
This allows multiple agents (OpenCode, Claude, Codex, KLP) to share
API keys, configuration, and context.

Location: ~/.ai-agents/mcp-servers/env-bridge/server.py
"""

import os
import sys
from pathlib import Path
from dotenv import load_dotenv, set_key

# Load shared environment
SHARED_ENV_PATH = os.getenv("SHARED_ENV_PATH", "/Users/peluche/.ai-agents/.env")
load_dotenv(SHARED_ENV_PATH)


def handle_list_tools():
    """List available tools."""
    return {
        "tools": [
            {
                "name": "env_get",
                "description": "Get an environment variable value",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "key": {
                            "type": "string",
                            "description": "Environment variable name",
                        }
                    },
                    "required": ["key"],
                },
            },
            {
                "name": "env_set",
                "description": "Set an environment variable value",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "key": {
                            "type": "string",
                            "description": "Environment variable name",
                        },
                        "value": {"type": "string", "description": "Value to set"},
                    },
                    "required": ["key", "value"],
                },
            },
            {
                "name": "env_list",
                "description": "List all shared environment variables (names only, no values)",
                "inputSchema": {"type": "object", "properties": {}},
            },
            {
                "name": "context_save",
                "description": "Save context for later retrieval by any agent",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "key": {"type": "string", "description": "Context key"},
                        "value": {
                            "type": "string",
                            "description": "Context value to save",
                        },
                    },
                    "required": ["key", "value"],
                },
            },
            {
                "name": "context_get",
                "description": "Retrieve saved context",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "key": {"type": "string", "description": "Context key"}
                    },
                    "required": ["key"],
                },
            },
        ]
    }


def handle_call_tool(name, arguments):
    """Handle tool calls."""
    if name == "env_get":
        key = arguments.get("key", "")
        value = os.getenv(key, "")
        return {
            "content": [
                {
                    "type": "text",
                    "text": f"{key}={value}" if value else f"{key} not set",
                }
            ]
        }

    elif name == "env_set":
        key = arguments.get("key", "")
        value = arguments.get("value", "")
        set_key(SHARED_ENV_PATH, key, value)
        return {"content": [{"type": "text", "text": f"Set {key}={value}"}]}

    elif name == "env_list":
        # List only keys, not values (for security)
        keys = [k for k in os.environ.keys() if not k.startswith("_")]
        return {
            "content": [
                {
                    "type": "text",
                    "text": "Available environment variables:\n"
                    + "\n".join(sorted(keys)),
                }
            ]
        }

    elif name == "context_save":
        key = arguments.get("key", "")
        value = arguments.get("value", "")
        context_key = f"CONTEXT_{key}"
        set_key(SHARED_ENV_PATH, context_key, value)
        return {"content": [{"type": "text", "text": f"Saved context: {key}"}]}

    elif name == "context_get":
        key = arguments.get("key", "")
        context_key = f"CONTEXT_{key}"
        value = os.getenv(context_key, "")
        return {
            "content": [
                {
                    "type": "text",
                    "text": value if value else f"Context '{key}' not found",
                }
            ]
        }

    else:
        return {
            "content": [{"type": "text", "text": f"Unknown tool: {name}"}],
            "isError": True,
        }


def main():
    """Main entry point for MCP server."""
    import json

    # Ensure shared env file exists
    Path(SHARED_ENV_PATH).parent.mkdir(parents=True, exist_ok=True)
    if not Path(SHARED_ENV_PATH).exists():
        Path(SHARED_ENV_PATH).touch()

    for line in sys.stdin:
        try:
            request = json.loads(line)
            method = request.get("method")

            if method == "initialize":
                response = {
                    "jsonrpc": "2.0",
                    "id": request.get("id"),
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {},
                        "serverInfo": {"name": "env-bridge", "version": "1.0.0"},
                    },
                }
                print(json.dumps(response), flush=True)

            elif method == "tools/list":
                response = {
                    "jsonrpc": "2.0",
                    "id": request.get("id"),
                    "result": handle_list_tools(),
                }
                print(json.dumps(response), flush=True)

            elif method == "tools/call":
                tool_name = request.get("params", {}).get("name", "")
                arguments = request.get("params", {}).get("arguments", {})
                result = handle_call_tool(tool_name, arguments)
                response = {"jsonrpc": "2.0", "id": request.get("id"), "result": result}
                print(json.dumps(response), flush=True)

        except json.JSONDecodeError:
            continue
        except Exception as e:
            error_response = {
                "jsonrpc": "2.0",
                "id": request.get("id"),
                "error": {"code": -32603, "message": str(e)},
            }
            print(json.dumps(error_response), flush=True)


if __name__ == "__main__":
    main()
