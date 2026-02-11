"""Rod cutting problem: maximize revenue by cutting a rod into pieces.

Tests: optimal revenue, no cuts needed, single length, various rod lengths.
"""


def rod_cutting(prices: list[int], length: int) -> int:
    """Return maximum revenue from cutting a rod of given length.

    prices[i] is the price for a piece of length i+1.
    """
    dp: list[int] = []
    i: int = 0
    while i <= length:
        dp.append(0)
        i = i + 1
    i = 1
    while i <= length:
        best: int = -1
        j: int = 0
        while j < i:
            candidate: int = prices[j] + dp[i - j - 1]
            if candidate > best:
                best = candidate
            j = j + 1
        dp[i] = best
        i = i + 1
    return dp[length]


def rod_cutting_with_cuts(prices: list[int], length: int) -> list[int]:
    """Return list of cut lengths that maximize revenue."""
    dp: list[int] = []
    cuts: list[int] = []
    i: int = 0
    while i <= length:
        dp.append(0)
        cuts.append(0)
        i = i + 1
    i = 1
    while i <= length:
        best: int = -1
        best_cut: int = 0
        j: int = 0
        while j < i:
            candidate: int = prices[j] + dp[i - j - 1]
            if candidate > best:
                best = candidate
                best_cut = j + 1
            j = j + 1
        dp[i] = best
        cuts[i] = best_cut
        i = i + 1
    result: list[int] = []
    remaining: int = length
    while remaining > 0:
        result.append(cuts[remaining])
        remaining = remaining - cuts[remaining]
    return result


def max_pieces(length: int, min_piece: int) -> int:
    """Return maximum number of pieces of at least min_piece length."""
    return length // min_piece


def test_module() -> int:
    """Test rod cutting problem."""
    ok: int = 0

    prices1: list[int] = [1, 5, 8, 9, 10, 17, 17, 20]
    if rod_cutting(prices1, 8) == 22:
        ok = ok + 1
    if rod_cutting(prices1, 4) == 10:
        ok = ok + 1

    prices2: list[int] = [3, 5, 8, 9, 10, 17, 17, 20]
    if rod_cutting(prices2, 1) == 3:
        ok = ok + 1

    if rod_cutting(prices1, 0) == 0:
        ok = ok + 1

    prices3: list[int] = [2, 5, 7, 8]
    if rod_cutting(prices3, 4) == 10:
        ok = ok + 1

    cut_result: list[int] = rod_cutting_with_cuts(prices1, 4)
    total: int = 0
    idx: int = 0
    while idx < len(cut_result):
        total = total + cut_result[idx]
        idx = idx + 1
    if total == 4:
        ok = ok + 1

    if max_pieces(10, 3) == 3:
        ok = ok + 1

    prices4: list[int] = [1, 5, 8, 9, 10]
    if rod_cutting(prices4, 5) == 13:
        ok = ok + 1

    prices5: list[int] = [10]
    if rod_cutting(prices5, 1) == 10:
        ok = ok + 1

    if rod_cutting(prices1, 2) == 5:
        ok = ok + 1

    return ok
