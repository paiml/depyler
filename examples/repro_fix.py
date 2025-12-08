"""Minimal repro for double-reference bug (DEPYLER-0818)."""


def caller(s: str) -> str:
    """Pass str param to another function."""
    return callee(s)  # s is str, pass directly


def callee(s: str) -> str:
    """Receive str and return it."""
    return s
