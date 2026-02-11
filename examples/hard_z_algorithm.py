"""Z-algorithm for string matching (using int arrays)."""


def z_function(s: list[int]) -> list[int]:
    """Compute Z-array for string s (as int array)."""
    n: int = len(s)
    if n == 0:
        return []
    z: list[int] = []
    i: int = 0
    while i < n:
        z.append(0)
        i = i + 1
    z[0] = n
    left: int = 0
    right: int = 0
    i = 1
    while i < n:
        if i < right:
            if z[i - left] < right - i:
                z[i] = z[i - left]
            else:
                z[i] = right - i
        k: int = z[i]
        while i + k < n and s[k] == s[i + k]:
            k = k + 1
        z[i] = k
        if i + k > right:
            left = i
            right = i + k
        i = i + 1
    return z


def z_search(text: list[int], pat: list[int]) -> list[int]:
    """Find all occurrences of pat in text using Z-algorithm.
    Returns list of starting indices."""
    pn: int = len(pat)
    tn: int = len(text)
    if pn == 0:
        return []
    if tn == 0:
        return []
    if pn > tn:
        return []
    combined: list[int] = []
    i: int = 0
    while i < pn:
        combined.append(pat[i])
        i = i + 1
    combined.append(0 - 1)
    i = 0
    while i < tn:
        combined.append(text[i])
        i = i + 1
    z: list[int] = z_function(combined)
    result: list[int] = []
    i = pn + 1
    while i < len(z):
        if z[i] == pn:
            pos: int = i - pn - 1
            result.append(pos)
        i = i + 1
    return result


def count_occurrences(text: list[int], pat: list[int]) -> int:
    """Count occurrences of pat in text."""
    matches: list[int] = z_search(text, pat)
    return len(matches)


def has_match(text: list[int], pat: list[int]) -> int:
    """Returns 1 if pat found in text."""
    matches: list[int] = z_search(text, pat)
    if len(matches) > 0:
        return 1
    return 0


def test_module() -> int:
    """Test Z-algorithm."""
    ok: int = 0
    s1: list[int] = [1, 2, 1, 2, 1]
    z1: list[int] = z_function(s1)
    if z1[0] == 5:
        ok = ok + 1
    if z1[2] == 3:
        ok = ok + 1
    text: list[int] = [1, 2, 3, 1, 2, 3, 1, 2]
    pat: list[int] = [1, 2, 3]
    matches: list[int] = z_search(text, pat)
    if len(matches) == 2:
        ok = ok + 1
    if matches[0] == 0:
        ok = ok + 1
    if matches[1] == 3:
        ok = ok + 1
    if count_occurrences(text, pat) == 2:
        ok = ok + 1
    if has_match(text, pat) == 1:
        ok = ok + 1
    return ok
