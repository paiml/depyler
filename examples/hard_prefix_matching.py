"""Prefix matching and checking operations.

Tests: common prefix, prefix match count, longest common prefix length.
"""


def common_prefix_len(s1: str, s2: str) -> int:
    """Length of common prefix between s1 and s2."""
    n: int = len(s1)
    if len(s2) < n:
        n = len(s2)
    length: int = 0
    i: int = 0
    while i < n:
        if s1[i] != s2[i]:
            return length
        length = length + 1
        i = i + 1
    return length


def count_prefix_matches(words: list[str], prefix: str) -> int:
    """Count how many words start with the given prefix."""
    count: int = 0
    plen: int = len(prefix)
    for w in words:
        if len(w) >= plen:
            match: int = 1
            j: int = 0
            while j < plen:
                if w[j] != prefix[j]:
                    match = 0
                    j = plen
                else:
                    j = j + 1
            if match == 1:
                count = count + 1
    return count


def longest_common_prefix_of_list(words: list[str]) -> int:
    """Length of longest common prefix among all words."""
    n: int = len(words)
    if n == 0:
        return 0
    if n == 1:
        return len(words[0])
    best: int = len(words[0])
    i: int = 1
    while i < n:
        cp: int = common_prefix_len(words[0], words[i])
        if cp < best:
            best = cp
        i = i + 1
    return best


def has_prefix(s: str, prefix: str) -> int:
    """Returns 1 if s starts with prefix, else 0."""
    if len(prefix) > len(s):
        return 0
    i: int = 0
    while i < len(prefix):
        if s[i] != prefix[i]:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test prefix matching."""
    ok: int = 0
    if common_prefix_len("flower", "flow") == 4:
        ok = ok + 1
    if common_prefix_len("abc", "xyz") == 0:
        ok = ok + 1
    if count_prefix_matches(["apple", "ape", "banana"], "ap") == 2:
        ok = ok + 1
    if longest_common_prefix_of_list(["flower", "flow", "flight"]) == 2:
        ok = ok + 1
    if has_prefix("hello", "hel") == 1:
        ok = ok + 1
    if has_prefix("hello", "xyz") == 0:
        ok = ok + 1
    return ok
