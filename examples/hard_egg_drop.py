"""Egg drop problem: find minimum number of trials to find critical floor.

Tests: base cases, two eggs, one egg, many floors, known results.
"""


def egg_drop(eggs: int, floors: int) -> int:
    """Return minimum number of trials to find critical floor in worst case."""
    dp: list[list[int]] = []
    i: int = 0
    while i <= eggs:
        row: list[int] = []
        j: int = 0
        while j <= floors:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    j: int = 1
    while j <= floors:
        dp[1][j] = j
        j = j + 1
    i = 2
    while i <= eggs:
        j = 1
        while j <= floors:
            dp[i][j] = 999999999
            x: int = 1
            while x <= j:
                breaks: int = dp[i - 1][x - 1]
                survives: int = dp[i][j - x]
                worst: int = breaks
                if survives > worst:
                    worst = survives
                trial: int = 1 + worst
                if trial < dp[i][j]:
                    dp[i][j] = trial
                x = x + 1
            j = j + 1
        i = i + 1
    return dp[eggs][floors]


def max_floors_with_trials(eggs: int, trials: int) -> int:
    """Return maximum number of floors that can be checked with given eggs and trials."""
    dp: list[list[int]] = []
    i: int = 0
    while i <= eggs:
        row: list[int] = []
        j: int = 0
        while j <= trials:
            row.append(0)
            j = j + 1
        dp.append(row)
        i = i + 1
    t: int = 1
    while t <= trials:
        e: int = 1
        while e <= eggs:
            dp[e][t] = dp[e - 1][t - 1] + dp[e][t - 1] + 1
            e = e + 1
        t = t + 1
    return dp[eggs][trials]


def one_egg_trials(floors: int) -> int:
    """Return trials needed with one egg (must be linear)."""
    return floors


def test_module() -> int:
    """Test egg drop problem."""
    ok: int = 0

    if egg_drop(1, 10) == 10:
        ok = ok + 1
    if egg_drop(2, 10) == 4:
        ok = ok + 1
    if egg_drop(2, 6) == 3:
        ok = ok + 1
    if egg_drop(2, 1) == 1:
        ok = ok + 1
    if egg_drop(1, 1) == 1:
        ok = ok + 1
    if egg_drop(3, 14) == 4:
        ok = ok + 1

    if max_floors_with_trials(2, 4) == 10:
        ok = ok + 1
    if max_floors_with_trials(1, 5) == 5:
        ok = ok + 1

    if one_egg_trials(100) == 100:
        ok = ok + 1

    if egg_drop(2, 36) == 8:
        ok = ok + 1

    return ok
