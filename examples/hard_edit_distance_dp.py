"""Edit distance (Levenshtein) using dynamic programming table."""


def min_of_three(a: int, b: int, c: int) -> int:
    """Return minimum of three integers."""
    result: int = a
    if b < result:
        result = b
    if c < result:
        result = c
    return result


def edit_distance(s: list[int], t: list[int]) -> int:
    """Compute edit distance between two sequences encoded as int lists."""
    m: int = len(s)
    n: int = len(t)
    dp: list[int] = []
    i: int = 0
    while i < (m + 1) * (n + 1):
        dp.append(0)
        i = i + 1
    i = 0
    while i <= m:
        dp[i * (n + 1) + 0] = i
        i = i + 1
    j: int = 0
    while j <= n:
        dp[0 * (n + 1) + j] = j
        j = j + 1
    i = 1
    while i <= m:
        j = 1
        while j <= n:
            cost: int = 0
            if s[i - 1] != t[j - 1]:
                cost = 1
            insert_cost: int = dp[(i) * (n + 1) + (j - 1)] + 1
            delete_cost: int = dp[(i - 1) * (n + 1) + j] + 1
            replace_cost: int = dp[(i - 1) * (n + 1) + (j - 1)] + cost
            dp[i * (n + 1) + j] = min_of_three(insert_cost, delete_cost, replace_cost)
            j = j + 1
        i = i + 1
    return dp[m * (n + 1) + n]


def strings_to_ints(chars: str) -> list[int]:
    """Convert a string to a list of character codes."""
    result: list[int] = []
    i: int = 0
    n: int = len(chars)
    while i < n:
        val: int = ord(chars[i])
        result.append(val)
        i = i + 1
    return result


def edit_distance_str(a: str, b: str) -> int:
    """Edit distance for strings via int encoding."""
    sa: list[int] = strings_to_ints(a)
    sb: list[int] = strings_to_ints(b)
    return edit_distance(sa, sb)


def test_module() -> int:
    """Test edit distance."""
    passed: int = 0

    s1: list[int] = [1, 2, 3]
    t1: list[int] = [1, 3, 3]
    if edit_distance(s1, t1) == 1:
        passed = passed + 1

    s2: list[int] = [1, 2, 3]
    t2: list[int] = [4, 5, 6]
    if edit_distance(s2, t2) == 3:
        passed = passed + 1

    s3: list[int] = []
    t3: list[int] = [1, 2, 3]
    if edit_distance(s3, t3) == 3:
        passed = passed + 1

    s4: list[int] = [1, 2, 3]
    t4: list[int] = [1, 2, 3]
    if edit_distance(s4, t4) == 0:
        passed = passed + 1

    s5: list[int] = [1]
    t5: list[int] = []
    if edit_distance(s5, t5) == 1:
        passed = passed + 1

    if min_of_three(5, 3, 7) == 3:
        passed = passed + 1

    return passed
