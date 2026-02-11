"""Sequence alignment operations.

Implements basic sequence alignment scoring using
flat arrays to simulate matrices for dynamic programming.
"""


def match_score(a: int, b: int) -> int:
    """Score for matching two elements: +2 for match, -1 for mismatch."""
    if a == b:
        return 2
    return -1


def compute_alignment_score(seq_a: list[int], len_a: int, seq_b: list[int], len_b: int) -> int:
    """Compute simple alignment score between two integer sequences.

    Uses a flat array for the DP matrix with gap penalty of -1.
    """
    rows: int = len_a + 1
    cols: int = len_b + 1
    total: int = rows * cols
    dp: list[int] = []
    i: int = 0
    while i < total:
        dp.append(0)
        i = i + 1

    r: int = 0
    while r < rows:
        idx: int = r * cols
        dp[idx] = r * (-1)
        r = r + 1

    c: int = 0
    while c < cols:
        dp[c] = c * (-1)
        c = c + 1

    r2: int = 1
    while r2 < rows:
        c2: int = 1
        while c2 < cols:
            score: int = match_score(seq_a[r2 - 1], seq_b[c2 - 1])
            diag_idx: int = (r2 - 1) * cols + (c2 - 1)
            up_idx: int = (r2 - 1) * cols + c2
            left_idx: int = r2 * cols + (c2 - 1)
            diag: int = dp[diag_idx] + score
            up: int = dp[up_idx] - 1
            left: int = dp[left_idx] - 1
            best: int = diag
            if up > best:
                best = up
            if left > best:
                best = left
            curr_idx: int = r2 * cols + c2
            dp[curr_idx] = best
            c2 = c2 + 1
        r2 = r2 + 1

    final_idx: int = len_a * cols + len_b
    return dp[final_idx]


def count_matches(seq_a: list[int], seq_b: list[int], length: int) -> int:
    """Count exact position matches between two equal-length sequences."""
    matches: int = 0
    i: int = 0
    while i < length:
        if seq_a[i] == seq_b[i]:
            matches = matches + 1
        i = i + 1
    return matches


def hamming_like_distance(seq_a: list[int], seq_b: list[int], length: int) -> int:
    """Count positional differences between two sequences."""
    diffs: int = 0
    i: int = 0
    while i < length:
        if seq_a[i] != seq_b[i]:
            diffs = diffs + 1
        i = i + 1
    return diffs


def test_module() -> int:
    """Test sequence alignment operations."""
    ok: int = 0

    sc: int = match_score(5, 5)
    if sc == 2:
        ok = ok + 1

    sa: list[int] = [1, 2, 3]
    sb: list[int] = [1, 2, 3]
    score: int = compute_alignment_score(sa, 3, sb, 3)
    if score == 6:
        ok = ok + 1

    matches: int = count_matches(sa, sb, 3)
    if matches == 3:
        ok = ok + 1

    sc2: list[int] = [1, 3, 2]
    dist: int = hamming_like_distance(sa, sc2, 3)
    if dist == 2:
        ok = ok + 1

    return ok
