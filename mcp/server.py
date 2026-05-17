#!/usr/bin/env python3
"""MCP server wrapping gramadoir-server Irish grammar checker."""
import os
import httpx
from mcp.server.fastmcp import FastMCP

BASE_URL = os.environ.get("GRAMADOIR_URL", "http://localhost:5050").rstrip("/")

mcp = FastMCP("gramadoir")


@mcp.tool()
def check_grammar(text: str) -> list:
    """Check Irish-language text for grammatical errors using An Gramadóir.

    Returns a list of error objects. Each object may contain:
      - fromy / fromx: line/column of error start
      - toy / tox:     line/column of error end
      - ruleId:        rule identifier (e.g. SÉIMHIÚ, NOGENITIVE)
      - msg:           human-readable error message in Irish
      - context:       surrounding text snippet

    An empty list means no errors were found.
    Note: An Gramadóir does not know every valid word — unknown words
    are flagged as ANAITHNID and may be safely waived if the word is
    confirmed valid Irish.
    """
    r = httpx.post(
        f"{BASE_URL}/api/gramadoir/1.0",
        json={"teacs": text},
        timeout=30,
    )
    r.raise_for_status()
    return r.json()


if __name__ == "__main__":
    mcp.run()
