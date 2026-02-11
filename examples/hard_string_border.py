"""String border and period operations.

Implements algorithms for finding string borders (prefixes that
are also suffixes) and string periods.
"""


def is_border(s: str, length: int) -> int:
    """Check if prefix of given length is also a suffix. Returns 1 if yes."""
    s_len: int = len(s)
    if length <= 0 or length >= s_len:
        return 0
    i: int = 0
    offset: int = s_len - length
    while i < length:
        j: int = offset + i
        if s[i] != s[j]:
            return 0
        i = i + 1
    return 1


def longest_border(s: str) -> int:
    """Find the length of the longest proper border of s."""
    s_len: int = len(s)
    best: int = 0
    length: int = 1
    limit: int = s_len
    while length < limit:
        check: int = is_border(s, length)
        if check == 1:
            best = length
        length = length + 1
    return best


def compute_failure_function(s: str) -> list[int]:
    """Compute KMP failure function (longest proper prefix-suffix at each position)."""
    s_len: int = len(s)
    fail: list[int] = []
    i: int = 0
    while i < s_len:
        fail.append(0)
        i = i + 1
    if s_len == 0:
        return fail
    fail[0] = 0
    k: int = 0
    i2: int = 1
    while i2 < s_len:
        while k > 0 and s[i2] != s[k]:
            prev_k: int = k - 1
            k = fail[prev_k]
        if s[i2] == s[k]:
            k = k + 1
        fail[i2] = k
        i2 = i2 + 1
    return fail


def string_period(s: str) -> int:
    """Find the length of the shortest period of s using failure function."""
    s_len: int = len(s)
    if s_len == 0:
        return 0
    tmp_fail: list[int] = compute_failure_function(s)
    last_idx: int = s_len - 1
    last_fail: int = tmp_fail[last_idx]
    period: int = s_len - last_fail
    return period


def test_module() -> int:
    """Test string border operations."""
    ok: int = 0

    chk: int = is_border("abcab", 2)
    if chk == 1:
        ok = ok + 1

    lb: int = longest_border("abcabc")
    if lb == 3:
        ok = ok + 1

    tmp_fail: list[int] = compute_failure_function("aabaa")
    if tmp_fail[0] == 0 and tmp_fail[1] == 1 and tmp_fail[4] == 2:
        ok = ok + 1

    per: int = string_period("abcabc")
    if per == 3:
        ok = ok + 1

    return ok
