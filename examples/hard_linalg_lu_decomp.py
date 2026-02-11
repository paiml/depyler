"""LU decomposition using Doolittle method (integer-scaled)."""


def mat_get(m: list[int], cols: int, r: int, c: int) -> int:
    """Get element at row r, col c from flat matrix."""
    idx: int = r * cols + c
    return m[idx]


def lu_decompose(a: list[int], n: int) -> list[int]:
    """LU decomposition. Returns flat [L..., U...] each n*n."""
    ln: int = n * n
    lower: list[int] = []
    upper: list[int] = []
    i: int = 0
    while i < ln:
        lower.append(0)
        upper.append(0)
        i = i + 1
    ii: int = 0
    while ii < n:
        j: int = ii
        while j < n:
            s: int = 0
            k: int = 0
            while k < ii:
                s = s + lower[ii * n + k] * upper[k * n + j]
                k = k + 1
            upper[ii * n + j] = a[ii * n + j] - s
            j = j + 1
        j2: int = ii
        while j2 < n:
            if ii == j2:
                lower[ii * n + ii] = 1
            else:
                s2: int = 0
                k2: int = 0
                while k2 < ii:
                    s2 = s2 + lower[j2 * n + k2] * upper[k2 * n + ii]
                    k2 = k2 + 1
                pivot: int = upper[ii * n + ii]
                if pivot != 0:
                    lower[j2 * n + ii] = (a[j2 * n + ii] - s2) // pivot
            j2 = j2 + 1
        ii = ii + 1
    result: list[int] = []
    idx2: int = 0
    while idx2 < ln:
        result.append(lower[idx2])
        idx2 = idx2 + 1
    idx3: int = 0
    while idx3 < ln:
        result.append(upper[idx3])
        idx3 = idx3 + 1
    return result


def test_module() -> int:
    """Test LU decomposition."""
    ok: int = 0
    a: list[int] = [2, 4, 6, 8]
    lu: list[int] = lu_decompose(a, 2)
    if mat_get(lu, 2, 0, 0) == 1:
        ok = ok + 1
    if mat_get(lu, 2, 1, 1) == 1:
        ok = ok + 1
    u00: int = mat_get(lu, 2, 2, 0)
    if u00 == 2:
        ok = ok + 1
    u01: int = mat_get(lu, 2, 2, 1)
    if u01 == 4:
        ok = ok + 1
    l10: int = mat_get(lu, 2, 1, 0)
    if l10 * u00 == a[2]:
        ok = ok + 1
    return ok
