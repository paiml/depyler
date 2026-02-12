"""Text processing: URL and path parsing.

Tests: component extraction, normalization, query string parsing,
path manipulation, protocol detection.
"""

from typing import Dict, List, Tuple


def split_path(path: str) -> List[str]:
    """Split file path by / separator."""
    parts: List[str] = []
    current: List[str] = []
    for ch in path:
        if ch == "/":
            if len(current) > 0:
                parts.append("".join(current))
                current = []
        else:
            current.append(ch)
    if len(current) > 0:
        parts.append("".join(current))
    return parts


def get_extension(filename: str) -> str:
    """Extract file extension from filename."""
    last_dot: int = -1
    i: int = 0
    while i < len(filename):
        if filename[i] == ".":
            last_dot = i
        i += 1
    if last_dot < 0:
        return ""
    result: List[str] = []
    i = last_dot + 1
    while i < len(filename):
        result.append(filename[i])
        i += 1
    return "".join(result)


def get_basename(path: str) -> str:
    """Get base filename from full path."""
    parts: List[str] = split_path(path)
    if len(parts) == 0:
        return ""
    return parts[len(parts) - 1]


def normalize_path(path: str) -> str:
    """Normalize path by removing . and resolving .."""
    parts: List[str] = split_path(path)
    stack: List[str] = []
    for part in parts:
        if part == ".":
            continue
        elif part == "..":
            if len(stack) > 0:
                stack.pop()
        else:
            stack.append(part)
    return "/" + "/".join(stack)


def parse_query_string(qs: str) -> Dict[str, str]:
    """Parse URL query string into key-value pairs."""
    result: Dict[str, str] = {}
    pairs: List[str] = []
    current: List[str] = []
    for ch in qs:
        if ch == "&":
            pairs.append("".join(current))
            current = []
        else:
            current.append(ch)
    if len(current) > 0:
        pairs.append("".join(current))
    for pair in pairs:
        eq_pos: int = -1
        idx: int = 0
        while idx < len(pair):
            if pair[idx] == "=":
                eq_pos = idx
                break
            idx += 1
        if eq_pos >= 0:
            k_parts: List[str] = []
            j: int = 0
            while j < eq_pos:
                k_parts.append(pair[j])
                j += 1
            v_parts: List[str] = []
            j = eq_pos + 1
            while j < len(pair):
                v_parts.append(pair[j])
                j += 1
            result["".join(k_parts)] = "".join(v_parts)
    return result


def build_query_string(params: Dict[str, str]) -> str:
    """Build query string from key-value pairs."""
    parts: List[str] = []
    first: bool = True
    for k in params:
        if not first:
            parts.append("&")
        parts.append(k)
        parts.append("=")
        parts.append(params[k])
        first = False
    return "".join(parts)


def join_paths(a: str, b: str) -> str:
    """Join two path segments."""
    result: List[str] = []
    for ch in a:
        result.append(ch)
    if len(a) > 0 and a[len(a) - 1] != "/":
        result.append("/")
    for ch in b:
        result.append(ch)
    return "".join(result)


def test_paths() -> bool:
    """Test path and URL parsing."""
    ok: bool = True
    parts: List[str] = split_path("/usr/local/bin")
    if len(parts) != 3:
        ok = False
    ext: str = get_extension("file.txt")
    if ext != "txt":
        ok = False
    base: str = get_basename("/home/user/doc.py")
    if base != "doc.py":
        ok = False
    norm: str = normalize_path("/a/b/../c/./d")
    if norm != "/a/c/d":
        ok = False
    qs: Dict[str, str] = parse_query_string("a=1&b=2")
    if "a" not in qs:
        ok = False
    return ok
