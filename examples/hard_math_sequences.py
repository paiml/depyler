"""Mathematical sequence patterns.

Tests: prime sieve, Pascal's triangle, Catalan numbers,
tribonacci sequence, and collatz sequence length.
"""


def sieve_of_eratosthenes(limit: int) -> list[int]:
    """Return all primes up to limit using sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime: list[bool] = [True] * (limit + 1)
    is_prime[0] = False
    is_prime[1] = False
    i: int = 2
    while i * i <= limit:
        if is_prime[i]:
            j: int = i * i
            while j <= limit:
                is_prime[j] = False
                j = j + i
        i = i + 1
    primes: list[int] = []
    k: int = 2
    while k <= limit:
        if is_prime[k]:
            primes.append(k)
        k = k + 1
    return primes


def pascals_triangle(n: int) -> list[list[int]]:
    """Generate first n rows of Pascal's triangle."""
    if n <= 0:
        return []
    triangle: list[list[int]] = [[1]]
    row: int = 1
    while row < n:
        prev: list[int] = triangle[row - 1]
        new_row: list[int] = [1]
        j: int = 1
        while j < row:
            new_row.append(prev[j - 1] + prev[j])
            j = j + 1
        new_row.append(1)
        triangle.append(new_row)
        row = row + 1
    return triangle


def catalan_numbers(n: int) -> list[int]:
    """Generate first n+1 Catalan numbers C(0) through C(n)."""
    dp: list[int] = [0] * (n + 1)
    dp[0] = 1
    i: int = 1
    while i <= n:
        j: int = 0
        while j < i:
            dp[i] = dp[i] + dp[j] * dp[i - 1 - j]
            j = j + 1
        i = i + 1
    return dp


def tribonacci(n: int) -> list[int]:
    """Generate first n tribonacci numbers: T(0)=0, T(1)=0, T(2)=1."""
    if n <= 0:
        return []
    if n == 1:
        return [0]
    if n == 2:
        return [0, 0]
    result: list[int] = [0, 0, 1]
    i: int = 3
    while i < n:
        result.append(result[i - 1] + result[i - 2] + result[i - 3])
        i = i + 1
    return result


def collatz_length(n: int) -> int:
    """Length of Collatz sequence starting from n until reaching 1."""
    if n <= 0:
        return 0
    steps: int = 0
    val: int = n
    while val != 1:
        if val % 2 == 0:
            val = val // 2
        else:
            val = 3 * val + 1
        steps = steps + 1
    return steps


def test_module() -> bool:
    """Test all math sequence functions."""
    ok: bool = True

    primes: list[int] = sieve_of_eratosthenes(30)
    if primes != [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]:
        ok = False
    if len(sieve_of_eratosthenes(1)) != 0:
        ok = False

    tri: list[list[int]] = pascals_triangle(5)
    if len(tri) != 5:
        ok = False
    if tri[4] != [1, 4, 6, 4, 1]:
        ok = False

    cat: list[int] = catalan_numbers(5)
    if cat != [1, 1, 2, 5, 14, 42]:
        ok = False

    trib: list[int] = tribonacci(7)
    if trib != [0, 0, 1, 1, 2, 4, 7]:
        ok = False

    if collatz_length(6) != 8:
        ok = False
    if collatz_length(1) != 0:
        ok = False

    return ok
