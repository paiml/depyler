"""Pattern matching in strings: wildcard matching and simple glob patterns."""


def match_wildcard(text: str, pattern: str) -> int:
    """Match text against pattern with ? (single char) and * (any chars).
    Returns 1 if match, 0 otherwise. Uses iterative approach."""
    t_len: int = len(text)
    p_len: int = len(pattern)
    ti: int = 0
    pi: int = 0
    star_pi: int = -1
    star_ti: int = -1
    while ti < t_len:
        if pi < p_len and (pattern[pi] == text[ti] or pattern[pi] == "?"):
            ti = ti + 1
            pi = pi + 1
        elif pi < p_len and pattern[pi] == "*":
            star_pi = pi
            star_ti = ti
            pi = pi + 1
        elif star_pi >= 0:
            pi = star_pi + 1
            star_ti = star_ti + 1
            ti = star_ti
        else:
            return 0
    while pi < p_len and pattern[pi] == "*":
        pi = pi + 1
    if pi == p_len:
        return 1
    return 0


def count_pattern_occurrences(text: str, pattern: str) -> int:
    """Count non-overlapping occurrences of pattern in text."""
    t_len: int = len(text)
    p_len: int = len(pattern)
    if p_len == 0 or p_len > t_len:
        return 0
    count: int = 0
    i: int = 0
    while i <= t_len - p_len:
        match: int = 1
        j: int = 0
        while j < p_len:
            if text[i + j] != pattern[j]:
                match = 0
                j = p_len
            j = j + 1
        if match == 1:
            count = count + 1
            i = i + p_len
        else:
            i = i + 1
    return count


def longest_common_prefix_pair(a: str, b: str) -> int:
    """Find length of longest common prefix between two strings."""
    limit: int = len(a)
    if len(b) < limit:
        limit = len(b)
    i: int = 0
    while i < limit:
        if a[i] != b[i]:
            return i
        i = i + 1
    return limit


def hamming_distance_str(a: str, b: str) -> int:
    """Calculate Hamming distance between two equal-length strings.
    Returns -1 if lengths differ."""
    if len(a) != len(b):
        return -1
    dist: int = 0
    i: int = 0
    while i < len(a):
        if a[i] != b[i]:
            dist = dist + 1
        i = i + 1
    return dist


def test_module() -> int:
    """Test string matching functions."""
    ok: int = 0

    if match_wildcard("hello", "h*o") == 1:
        ok = ok + 1

    if match_wildcard("hello", "h?llo") == 1:
        ok = ok + 1

    if match_wildcard("hello", "h?lo") == 0:
        ok = ok + 1

    if count_pattern_occurrences("abcabcabc", "abc") == 3:
        ok = ok + 1

    if longest_common_prefix_pair("hello", "help") == 3:
        ok = ok + 1

    if hamming_distance_str("karolin", "kathrin") == 3:
        ok = ok + 1

    if hamming_distance_str("ab", "abc") == -1:
        ok = ok + 1

    return ok
