"""Text processing: Suffix and prefix operations.

Tests: suffix array construction primitives, longest repeated substring,
prefix tree simulation, string rotation.
"""

from typing import Dict, List, Tuple


def all_suffixes(s: str) -> List[str]:
    """Generate all suffixes of a string."""
    result: List[str] = []
    i: int = 0
    while i < len(s):
        suffix: List[str] = []
        j: int = i
        while j < len(s):
            suffix.append(s[j])
            j += 1
        result.append("".join(suffix))
        i += 1
    return result


def all_prefixes(s: str) -> List[str]:
    """Generate all prefixes of a string."""
    result: List[str] = []
    i: int = 1
    while i <= len(s):
        prefix: List[str] = []
        j: int = 0
        while j < i:
            prefix.append(s[j])
            j += 1
        result.append("".join(prefix))
        i += 1
    return result


def rotate_left(s: str, k: int) -> str:
    """Rotate string left by k positions."""
    n: int = len(s)
    if n == 0:
        return s
    shift: int = k % n
    result: List[str] = []
    i: int = shift
    while i < n:
        result.append(s[i])
        i += 1
    i = 0
    while i < shift:
        result.append(s[i])
        i += 1
    return "".join(result)


def rotate_right(s: str, k: int) -> str:
    """Rotate string right by k positions."""
    n: int = len(s)
    if n == 0:
        return s
    shift: int = k % n
    actual: int = n - shift
    return rotate_left(s, actual)


def is_rotation(a: str, b: str) -> bool:
    """Check if b is a rotation of a."""
    if len(a) != len(b):
        return False
    if len(a) == 0:
        return True
    n: int = len(a)
    i: int = 0
    while i < n:
        match: bool = True
        j: int = 0
        while j < n:
            ai: int = (i + j) % n
            if a[ai] != b[j]:
                match = False
                break
            j += 1
        if match:
            return True
        i += 1
    return False


def longest_repeated_prefix(s: str) -> str:
    """Find longest prefix that repeats in the string."""
    best: str = ""
    best_len: int = 0
    prefix_len: int = 1
    while prefix_len <= len(s) // 2:
        prefix_str: List[str] = []
        j: int = 0
        while j < prefix_len:
            prefix_str.append(s[j])
            j += 1
        pstr: str = "".join(prefix_str)
        found: bool = False
        i: int = 1
        limit: int = len(s) - prefix_len + 1
        while i < limit:
            mat: bool = True
            k: int = 0
            while k < prefix_len:
                if s[i + k] != pstr[k]:
                    mat = False
                    break
                k += 1
            if mat:
                found = True
                break
            i += 1
        if found and prefix_len > best_len:
            best = pstr
            best_len = prefix_len
        prefix_len += 1
    return best


def test_suffix_prefix() -> bool:
    """Test suffix and prefix operations."""
    ok: bool = True
    suffs: List[str] = all_suffixes("abc")
    if len(suffs) != 3:
        ok = False
    prefs: List[str] = all_prefixes("abc")
    if len(prefs) != 3:
        ok = False
    rot: str = rotate_left("abcde", 2)
    if rot != "cdeab":
        ok = False
    if not is_rotation("abcde", "cdeab"):
        ok = False
    return ok
