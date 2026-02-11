"""Pigeonhole principle applications in integer arithmetic."""


def min_pigeons_for_guarantee(holes: int, per_hole: int) -> int:
    """Minimum pigeons to guarantee at least per_hole in some hole.
    Answer: (per_hole - 1) * holes + 1."""
    if holes <= 0:
        return 0
    return (per_hole - 1) * holes + 1


def max_in_some_hole(pigeons: int, holes: int) -> int:
    """At least one hole has ceil(pigeons/holes) pigeons."""
    if holes <= 0:
        return pigeons
    result: int = pigeons // holes
    if pigeons % holes != 0:
        result = result + 1
    return result


def min_draws_pair(categories: int) -> int:
    """Minimum draws to guarantee a pair (same category).
    By pigeonhole: categories + 1."""
    return categories + 1


def birthday_bound(slots: int) -> int:
    """Minimum people to guarantee a shared birthday.
    slots + 1 by pigeonhole."""
    return slots + 1


def distribute_evenly(items: int, bins: int) -> list[int]:
    """Distribute items into bins as evenly as possible.
    Returns [min_per_bin, max_per_bin]."""
    if bins <= 0:
        return [0, 0]
    lo: int = items // bins
    hi: int = lo
    if items % bins != 0:
        hi = lo + 1
    return [lo, hi]


def has_duplicate_mod(vals: list[int], m: int) -> int:
    """Check if any two values have same remainder mod m (pigeonhole if len > m)."""
    n: int = len(vals)
    if n > m:
        return 1
    seen: list[int] = []
    i: int = 0
    while i < m:
        seen.append(0)
        i = i + 1
    j: int = 0
    while j < n:
        r: int = vals[j] % m
        if seen[r] == 1:
            return 1
        seen[r] = 1
        j = j + 1
    return 0


def test_module() -> int:
    """Test pigeonhole principle functions."""
    ok: int = 0
    if min_pigeons_for_guarantee(5, 3) == 11:
        ok = ok + 1
    if max_in_some_hole(10, 3) == 4:
        ok = ok + 1
    if min_draws_pair(7) == 8:
        ok = ok + 1
    d: list[int] = distribute_evenly(10, 3)
    if d[0] == 3 and d[1] == 4:
        ok = ok + 1
    vals: list[int] = [1, 5, 3, 8]
    if has_duplicate_mod(vals, 4) == 1:
        ok = ok + 1
    return ok
