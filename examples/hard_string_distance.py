"""String distance and similarity metrics.

Tests: hamming distance, count differences, similarity ratio.
"""


def hamming_distance_str(s1: str, s2: str) -> int:
    """Hamming distance between equal-length strings."""
    n: int = len(s1)
    if len(s2) != n:
        return -1
    dist: int = 0
    i: int = 0
    while i < n:
        if s1[i] != s2[i]:
            dist = dist + 1
        i = i + 1
    return dist


def count_char_differences(s1: str, s2: str) -> int:
    """Count positions where characters differ (up to shorter length)."""
    n: int = len(s1)
    if len(s2) < n:
        n = len(s2)
    diff: int = 0
    i: int = 0
    while i < n:
        if s1[i] != s2[i]:
            diff = diff + 1
        i = i + 1
    len_diff: int = len(s1) - len(s2)
    if len_diff < 0:
        len_diff = -len_diff
    return diff + len_diff


def similarity_score(s1: str, s2: str) -> int:
    """Similarity as percentage (0-100) of matching chars in shorter length."""
    n: int = len(s1)
    if len(s2) < n:
        n = len(s2)
    if n == 0:
        return 100
    matches: int = 0
    i: int = 0
    while i < n:
        if s1[i] == s2[i]:
            matches = matches + 1
        i = i + 1
    return matches * 100 // n


def count_common_chars(s1: str, s2: str) -> int:
    """Count total matching character positions."""
    n: int = len(s1)
    if len(s2) < n:
        n = len(s2)
    count: int = 0
    i: int = 0
    while i < n:
        if s1[i] == s2[i]:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test string distance metrics."""
    ok: int = 0
    if hamming_distance_str("karolin", "kathrin") == 3:
        ok = ok + 1
    if hamming_distance_str("abc", "abcd") == -1:
        ok = ok + 1
    if count_char_differences("hello", "hallo") == 1:
        ok = ok + 1
    if count_char_differences("ab", "abcd") == 2:
        ok = ok + 1
    if similarity_score("hello", "hallo") == 80:
        ok = ok + 1
    if count_common_chars("abcde", "abfde") == 4:
        ok = ok + 1
    return ok
