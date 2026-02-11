# Bell numbers (set partitions count)


def bell_number(n: int) -> int:
    # Bell number B(n) using Bell triangle
    if n == 0:
        return 1
    # Build Bell triangle row by row
    # prev_row stores the previous row of the triangle
    prev_row: list[int] = [1]
    row_idx: int = 1
    while row_idx <= n:
        curr_row: list[int] = [prev_row[len(prev_row) - 1]]
        j: int = 1
        while j <= row_idx:
            curr_row.append(curr_row[j - 1] + prev_row[j - 1])
            j = j + 1
        prev_row = curr_row
        row_idx = row_idx + 1
    return prev_row[0]


def bell_triangle_row(n: int) -> list[int]:
    # Return the nth row of Bell triangle
    if n == 0:
        return [1]
    prev_row: list[int] = [1]
    row_idx: int = 1
    while row_idx <= n:
        curr_row: list[int] = [prev_row[len(prev_row) - 1]]
        j: int = 1
        while j <= row_idx:
            curr_row.append(curr_row[j - 1] + prev_row[j - 1])
            j = j + 1
        prev_row = curr_row
        row_idx = row_idx + 1
    return prev_row


def stirling_second(n: int, k: int) -> int:
    # S(n, k) = k * S(n-1, k) + S(n-1, k-1)
    if n == 0 and k == 0:
        return 1
    if n == 0 or k == 0:
        return 0
    if k > n:
        return 0
    # Build dp table as flat array
    cols: int = k + 1
    rows: int = n + 1
    dp: list[int] = []
    i: int = 0
    while i < rows * cols:
        dp.append(0)
        i = i + 1
    dp[0] = 1  # S(0,0) = 1
    r: int = 1
    while r <= n:
        c: int = 1
        while c <= k and c <= r:
            dp[r * cols + c] = c * dp[(r - 1) * cols + c] + dp[(r - 1) * cols + (c - 1)]
            c = c + 1
        r = r + 1
    return dp[n * cols + k]


def bell_via_stirling(n: int) -> int:
    # B(n) = sum of S(n, k) for k = 0..n
    total: int = 0
    k: int = 0
    while k <= n:
        total = total + stirling_second(n, k)
        k = k + 1
    return total


def test_module() -> int:
    passed: int = 0

    # Test 1: B(0) = 1
    if bell_number(0) == 1:
        passed = passed + 1

    # Test 2: B(1) = 1
    if bell_number(1) == 1:
        passed = passed + 1

    # Test 3: B(3) = 5
    if bell_number(3) == 5:
        passed = passed + 1

    # Test 4: B(5) = 52
    if bell_number(5) == 52:
        passed = passed + 1

    # Test 5: Bell triangle row 3
    row: list[int] = bell_triangle_row(3)
    if row[0] == 5 and row[1] == 7 and row[2] == 10 and row[3] == 15:
        passed = passed + 1

    # Test 6: Stirling S(4,2) = 7
    if stirling_second(4, 2) == 7:
        passed = passed + 1

    # Test 7: Bell via Stirling matches direct
    if bell_via_stirling(5) == 52:
        passed = passed + 1

    return passed
