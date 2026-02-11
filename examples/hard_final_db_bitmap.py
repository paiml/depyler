"""Bitmap index for efficient set operations on columns.

Each distinct value gets a bitmap (list[int] where each int is 0 or 1).
Supports AND, OR, NOT, and cardinality estimation.
"""


def create_bitmap(num_rows: int) -> list[int]:
    """Create a zero bitmap of given size."""
    result: list[int] = []
    i: int = 0
    while i < num_rows:
        result.append(0)
        i = i + 1
    return result


def build_bitmap(column: list[int], target_val: int) -> list[int]:
    """Build bitmap for rows where column == target_val."""
    result: list[int] = []
    i: int = 0
    while i < len(column):
        cv: int = column[i]
        if cv == target_val:
            result.append(1)
        else:
            result.append(0)
        i = i + 1
    return result


def bitmap_and(bm_a: list[int], bm_b: list[int]) -> list[int]:
    """Bitwise AND of two bitmaps."""
    result: list[int] = []
    i: int = 0
    while i < len(bm_a):
        va: int = bm_a[i]
        vb: int = bm_b[i]
        if va == 1:
            if vb == 1:
                result.append(1)
            else:
                result.append(0)
        else:
            result.append(0)
        i = i + 1
    return result


def bitmap_or(bm_a: list[int], bm_b: list[int]) -> list[int]:
    """Bitwise OR of two bitmaps."""
    result: list[int] = []
    i: int = 0
    while i < len(bm_a):
        va: int = bm_a[i]
        vb: int = bm_b[i]
        if va == 1:
            result.append(1)
        else:
            if vb == 1:
                result.append(1)
            else:
                result.append(0)
        i = i + 1
    return result


def bitmap_not(bm: list[int]) -> list[int]:
    """Bitwise NOT of a bitmap."""
    result: list[int] = []
    i: int = 0
    while i < len(bm):
        bv: int = bm[i]
        if bv == 1:
            result.append(0)
        else:
            result.append(1)
        i = i + 1
    return result


def bitmap_count(bm: list[int]) -> int:
    """Count 1-bits in bitmap (cardinality)."""
    cnt: int = 0
    i: int = 0
    while i < len(bm):
        bv: int = bm[i]
        if bv == 1:
            cnt = cnt + 1
        i = i + 1
    return cnt


def bitmap_to_rows(bm: list[int]) -> list[int]:
    """Convert bitmap to list of matching row indices."""
    result: list[int] = []
    i: int = 0
    while i < len(bm):
        bv: int = bm[i]
        if bv == 1:
            result.append(i)
        i = i + 1
    return result


def multi_value_bitmap(column: list[int], values: list[int]) -> list[int]:
    """Build bitmap for rows where column value is in values list."""
    result: list[int] = create_bitmap(len(column))
    vi: int = 0
    while vi < len(values):
        target: int = values[vi]
        bm: list[int] = build_bitmap(column, target)
        result = bitmap_or(result, bm)
        vi = vi + 1
    return result


def test_module() -> int:
    """Test bitmap index."""
    ok: int = 0
    col: list[int] = [1, 2, 1, 3, 2, 1, 3, 2]
    bm1: list[int] = build_bitmap(col, 1)
    if bitmap_count(bm1) == 3:
        ok = ok + 1
    bm2: list[int] = build_bitmap(col, 2)
    if bitmap_count(bm2) == 3:
        ok = ok + 1
    bm_and: list[int] = bitmap_and(bm1, bm2)
    if bitmap_count(bm_and) == 0:
        ok = ok + 1
    bm_or: list[int] = bitmap_or(bm1, bm2)
    if bitmap_count(bm_or) == 6:
        ok = ok + 1
    bm_not: list[int] = bitmap_not(bm1)
    if bitmap_count(bm_not) == 5:
        ok = ok + 1
    return ok
