"""Find the period of a string (represented as int array)."""


def compute_failure(pattern: list[int]) -> list[int]:
    """Compute KMP failure function."""
    n: int = len(pattern)
    if n == 0:
        return []
    fail: list[int] = []
    i: int = 0
    while i < n:
        fail.append(0)
        i = i + 1
    fail[0] = 0
    k: int = 0
    i = 1
    while i < n:
        while k > 0 and pattern[k] != pattern[i]:
            k = fail[k - 1]
        if pattern[k] == pattern[i]:
            k = k + 1
        fail[i] = k
        i = i + 1
    return fail


def string_period(pattern: list[int]) -> int:
    """Find smallest period of string. Period p means s[i] == s[i+p] for all valid i."""
    n: int = len(pattern)
    if n == 0:
        return 0
    fail: list[int] = compute_failure(pattern)
    period: int = n - fail[n - 1]
    return period


def is_periodic(pattern: list[int]) -> int:
    """Returns 1 if pattern has period less than its length."""
    n: int = len(pattern)
    if n <= 1:
        return 0
    p: int = string_period(pattern)
    if p < n and n % p == 0:
        return 1
    return 0


def count_distinct_prefix_periods(pattern: list[int]) -> int:
    """Count distinct periods among all prefixes."""
    n: int = len(pattern)
    if n == 0:
        return 0
    fail: list[int] = compute_failure(pattern)
    seen: list[int] = []
    i: int = 0
    while i < n:
        period: int = (i + 1) - fail[i]
        found: int = 0
        j: int = 0
        while j < len(seen):
            if seen[j] == period:
                found = 1
            j = j + 1
        if found == 0:
            seen.append(period)
        i = i + 1
    return len(seen)


def test_module() -> int:
    """Test string period detection."""
    ok: int = 0
    s1: list[int] = [1, 2, 1, 2, 1, 2]
    if string_period(s1) == 2:
        ok = ok + 1
    if is_periodic(s1) == 1:
        ok = ok + 1
    s2: list[int] = [1, 2, 3, 4]
    if string_period(s2) == 4:
        ok = ok + 1
    if is_periodic(s2) == 0:
        ok = ok + 1
    s3: list[int] = [1, 1, 1, 1]
    if string_period(s3) == 1:
        ok = ok + 1
    if is_periodic(s3) == 1:
        ok = ok + 1
    s4: list[int] = [1, 2, 3, 1, 2, 3]
    if string_period(s4) == 3:
        ok = ok + 1
    return ok
