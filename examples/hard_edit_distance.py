"""Edit distance (Levenshtein distance) variants.

Tests: standard edit distance, only insertions/deletions, hamming distance.
"""


def edit_distance(s1: str, s2: str) -> int:
    """Compute minimum edit distance between two strings."""
    m: int = len(s1)
    n: int = len(s2)
    prev: list[int] = []
    j: int = 0
    while j <= n:
        prev.append(j)
        j = j + 1
    i: int = 1
    while i <= m:
        curr: list[int] = [i]
        j = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                curr.append(prev[j - 1])
            else:
                insert_cost: int = curr[j - 1] + 1
                delete_cost: int = prev[j] + 1
                replace_cost: int = prev[j - 1] + 1
                best: int = insert_cost
                if delete_cost < best:
                    best = delete_cost
                if replace_cost < best:
                    best = replace_cost
                curr.append(best)
            j = j + 1
        prev = curr
        i = i + 1
    return prev[n]


def hamming_distance(s1: str, s2: str) -> int:
    """Count positions where characters differ (strings must be same length)."""
    n: int = len(s1)
    if len(s2) < n:
        n = len(s2)
    count: int = 0
    i: int = 0
    while i < n:
        if s1[i] != s2[i]:
            count = count + 1
        i = i + 1
    return count


def insertion_deletion_distance(s1: str, s2: str) -> int:
    """Minimum insertions + deletions to transform s1 to s2 (no replacement)."""
    m: int = len(s1)
    n: int = len(s2)
    prev: list[int] = []
    j: int = 0
    while j <= n:
        prev.append(j)
        j = j + 1
    i: int = 1
    while i <= m:
        curr: list[int] = [i]
        j = 1
        while j <= n:
            if s1[i - 1] == s2[j - 1]:
                curr.append(prev[j - 1])
            else:
                ins: int = curr[j - 1] + 1
                delt: int = prev[j] + 1
                best: int = ins
                if delt < best:
                    best = delt
                curr.append(best)
            j = j + 1
        prev = curr
        i = i + 1
    return prev[n]


def is_one_edit_away_val(s1: str, s2: str) -> int:
    """Check if two strings are at most one edit apart. Returns 1 if yes, 0 if no."""
    dist: int = edit_distance(s1, s2)
    if dist <= 1:
        return 1
    return 0


def test_module() -> None:
    assert edit_distance("kitten", "sitting") == 3
    assert edit_distance("", "abc") == 3
    assert edit_distance("abc", "") == 3
    assert edit_distance("abc", "abc") == 0
    assert hamming_distance("karolin", "kathrin") == 3
    assert hamming_distance("abc", "abc") == 0
    assert insertion_deletion_distance("abc", "adc") == 2
    assert is_one_edit_away_val("abc", "ab") == 1
    assert is_one_edit_away_val("abc", "abc") == 1
    assert is_one_edit_away_val("abc", "xyz") == 0
