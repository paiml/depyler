"""Catalan numbers computation using dynamic programming."""


def catalan_dp(n: int) -> int:
    """Compute nth Catalan number using dynamic programming."""
    if n <= 1:
        return 1
    table: list[int] = [0]
    i: int = 1
    while i <= n:
        table.append(0)
        i = i + 1
    table[0] = 1
    table[1] = 1
    i = 2
    while i <= n:
        j: int = 0
        while j < i:
            complement: int = i - 1 - j
            table[i] = table[i] + table[j] * table[complement]
            j = j + 1
        i = i + 1
    return table[n]


def catalan_sequence(count: int) -> list[int]:
    """Generate first count Catalan numbers."""
    result: list[int] = []
    i: int = 0
    while i < count:
        val: int = catalan_dp(i)
        result.append(val)
        i = i + 1
    return result


def ballot_problem(n: int) -> int:
    """Number of valid ballot sequences of length 2n (Catalan application)."""
    return catalan_dp(n)


def count_bst_structures(n: int) -> int:
    """Count structurally unique BSTs with n nodes (equals Catalan(n))."""
    return catalan_dp(n)


def test_module() -> int:
    """Test Catalan number computations."""
    passed: int = 0

    if catalan_dp(0) == 1:
        passed = passed + 1

    if catalan_dp(1) == 1:
        passed = passed + 1

    if catalan_dp(2) == 2:
        passed = passed + 1

    if catalan_dp(3) == 5:
        passed = passed + 1

    if catalan_dp(5) == 42:
        passed = passed + 1

    seq: list[int] = catalan_sequence(5)
    if seq[0] == 1 and seq[1] == 1 and seq[2] == 2 and seq[3] == 5 and seq[4] == 14:
        passed = passed + 1

    if ballot_problem(3) == 5:
        passed = passed + 1

    if count_bst_structures(4) == 14:
        passed = passed + 1

    return passed
