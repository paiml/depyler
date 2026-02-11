"""String rotation checking and operations.

Tests: rotation check, rotation count, min rotation, max rotation.
"""


def is_rotation(s1: str, s2: str) -> int:
    """Check if s2 is a rotation of s1. Returns 1 if yes, 0 otherwise."""
    if len(s1) != len(s2):
        return 0
    if len(s1) == 0:
        return 1
    doubled: str = s1 + s1
    n: int = len(s2)
    m: int = len(doubled)
    i: int = 0
    while i <= m - n:
        match: int = 1
        j: int = 0
        while j < n:
            if doubled[i + j] != s2[j]:
                match = 0
                j = n
            else:
                j = j + 1
        if match == 1:
            return 1
        i = i + 1
    return 0


def rotation_distance(s1: str, s2: str) -> int:
    """Find minimum left rotations to transform s1 to s2. Returns -1 if impossible."""
    if len(s1) != len(s2):
        return -1
    n: int = len(s1)
    if n == 0:
        return 0
    k: int = 0
    while k < n:
        match: int = 1
        j: int = 0
        while j < n:
            idx: int = (j + k) % n
            if s1[idx] != s2[j]:
                match = 0
                j = n
            else:
                j = j + 1
        if match == 1:
            return k
        k = k + 1
    return -1


def count_distinct_rotations(s: str) -> int:
    """Count number of distinct rotations of s."""
    n: int = len(s)
    if n == 0:
        return 0
    count: int = 0
    k: int = 0
    while k < n:
        is_dup: int = 0
        j: int = 0
        while j < k:
            same: int = 1
            m: int = 0
            while m < n:
                idx_k: int = (m + k) % n
                idx_j: int = (m + j) % n
                if s[idx_k] != s[idx_j]:
                    same = 0
                    m = n
                else:
                    m = m + 1
            if same == 1:
                is_dup = 1
                j = k
            else:
                j = j + 1
        if is_dup == 0:
            count = count + 1
        k = k + 1
    return count


def test_module() -> int:
    """Test string rotation operations."""
    ok: int = 0
    if is_rotation("abcd", "cdab") == 1:
        ok = ok + 1
    if is_rotation("abcd", "abdc") == 0:
        ok = ok + 1
    if is_rotation("", "") == 1:
        ok = ok + 1
    if rotation_distance("abcd", "cdab") == 2:
        ok = ok + 1
    if rotation_distance("abc", "xyz") == -1:
        ok = ok + 1
    if count_distinct_rotations("abc") == 3:
        ok = ok + 1
    if count_distinct_rotations("aa") == 1:
        ok = ok + 1
    return ok
