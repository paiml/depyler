"""Pascal's triangle row computation."""


def pascal_row(n: int) -> list[int]:
    """Compute the nth row of Pascal's triangle (0-indexed)."""
    row: list[int] = [1]
    i: int = 0
    while i < n:
        new_row: list[int] = [1]
        j: int = 0
        row_len: int = len(row) - 1
        while j < row_len:
            next_j: int = j + 1
            new_row.append(row[j] + row[next_j])
            j = j + 1
        new_row.append(1)
        row = new_row
        i = i + 1
    return row


def binomial_coefficient(n: int, r: int) -> int:
    """Compute C(n, r) using Pascal's triangle row."""
    if r < 0 or r > n:
        return 0
    row: list[int] = pascal_row(n)
    return row[r]


def pascal_sum(n: int) -> int:
    """Compute the sum of nth row (should be 2^n)."""
    row: list[int] = pascal_row(n)
    total: int = 0
    i: int = 0
    length: int = len(row)
    while i < length:
        total = total + row[i]
        i = i + 1
    return total


def pascal_alternating_sum(n: int) -> int:
    """Compute alternating sum of nth row (should be 0 for n>0)."""
    row: list[int] = pascal_row(n)
    total: int = 0
    i: int = 0
    length: int = len(row)
    while i < length:
        if i % 2 == 0:
            total = total + row[i]
        else:
            total = total - row[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test Pascal's triangle computations."""
    passed: int = 0

    r0: list[int] = pascal_row(0)
    if len(r0) == 1 and r0[0] == 1:
        passed = passed + 1

    r4: list[int] = pascal_row(4)
    if r4[0] == 1 and r4[1] == 4 and r4[2] == 6 and r4[3] == 4 and r4[4] == 1:
        passed = passed + 1

    if binomial_coefficient(5, 2) == 10:
        passed = passed + 1

    if binomial_coefficient(6, 3) == 20:
        passed = passed + 1

    if pascal_sum(4) == 16:
        passed = passed + 1

    if pascal_sum(0) == 1:
        passed = passed + 1

    if pascal_alternating_sum(5) == 0:
        passed = passed + 1

    if binomial_coefficient(5, 0) == 1:
        passed = passed + 1

    return passed
