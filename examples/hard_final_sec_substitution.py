"""Substitution cipher with arbitrary permutation mapping.

Uses a permutation table (list[int] of length 26) to map each letter.
Includes inverse computation and frequency analysis.
"""


def apply_substitution(text: list[int], perm: list[int]) -> list[int]:
    """Apply substitution cipher using permutation table."""
    result: list[int] = []
    i: int = 0
    while i < len(text):
        tv: int = text[i]
        mapped: int = perm[tv]
        result.append(mapped)
        i = i + 1
    return result


def invert_permutation(perm: list[int]) -> list[int]:
    """Compute inverse of a permutation."""
    inv: list[int] = []
    j: int = 0
    while j < 26:
        inv.append(0)
        j = j + 1
    i: int = 0
    while i < 26:
        pv: int = perm[i]
        inv[pv] = i
        i = i + 1
    return inv


def is_valid_permutation(perm: list[int]) -> int:
    """Check if list is a valid permutation of 0-25."""
    if len(perm) != 26:
        return 0
    seen: list[int] = []
    j: int = 0
    while j < 26:
        seen.append(0)
        j = j + 1
    i: int = 0
    while i < 26:
        pv: int = perm[i]
        if pv < 0:
            return 0
        if pv > 25:
            return 0
        old: int = seen[pv]
        if old == 1:
            return 0
        seen[pv] = 1
        i = i + 1
    return 1


def compose_permutations(p1: list[int], p2: list[int]) -> list[int]:
    """Compose two permutations: result[i] = p2[p1[i]]."""
    result: list[int] = []
    i: int = 0
    while i < 26:
        v1: int = p1[i]
        v2: int = p2[v1]
        result.append(v2)
        i = i + 1
    return result


def frequency_histogram(text: list[int]) -> list[int]:
    """Count frequency of each letter (0-25) in text."""
    counts: list[int] = []
    j: int = 0
    while j < 26:
        counts.append(0)
        j = j + 1
    i: int = 0
    while i < len(text):
        tv: int = text[i]
        old: int = counts[tv]
        counts[tv] = old + 1
        i = i + 1
    return counts


def lists_same(a: list[int], b: list[int]) -> int:
    """Check list equality."""
    if len(a) != len(b):
        return 0
    i: int = 0
    while i < len(a):
        va: int = a[i]
        vb: int = b[i]
        if va != vb:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test substitution cipher."""
    ok: int = 0
    perm: list[int] = [25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    plain: list[int] = [0, 1, 2, 3, 4]
    enc: list[int] = apply_substitution(plain, perm)
    v0: int = enc[0]
    if v0 == 25:
        ok = ok + 1
    inv: list[int] = invert_permutation(perm)
    dec: list[int] = apply_substitution(enc, inv)
    if lists_same(dec, plain) == 1:
        ok = ok + 1
    if is_valid_permutation(perm) == 1:
        ok = ok + 1
    bad_perm: list[int] = [0, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]
    if is_valid_permutation(bad_perm) == 0:
        ok = ok + 1
    composed: list[int] = compose_permutations(perm, perm)
    identity: list[int] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]
    if lists_same(composed, identity) == 1:
        ok = ok + 1
    return ok
