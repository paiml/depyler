"""Integer partition operations.

Implements algorithms for partitioning integers into sums
and counting partition possibilities.
"""


def count_partitions_two(n: int) -> int:
    """Count ways to partition n into exactly two positive parts."""
    count: int = 0
    a: int = 1
    while a <= n // 2:
        b: int = n - a
        if b >= a:
            count = count + 1
        a = a + 1
    return count


def partition_into_equal(n: int, parts: int) -> int:
    """Check if n can be evenly divided into given number of parts.

    Returns the part size if divisible, otherwise -1.
    """
    if parts <= 0:
        return -1
    if n % parts == 0:
        return n // parts
    return -1


def max_product_partition(n: int) -> int:
    """Find partition of n into positive integers with maximum product.

    Uses the mathematical insight that 3s give optimal product.
    """
    if n <= 1:
        return 1
    if n == 2:
        return 1
    if n == 3:
        return 2
    product: int = 1
    remaining: int = n
    while remaining > 4:
        product = product * 3
        remaining = remaining - 3
    product = product * remaining
    return product


def generate_partitions_sum(target: int, max_part: int) -> int:
    """Count the number of partitions of target using parts up to max_part.

    Uses a flat DP array.
    """
    size: int = target + 1
    dp: list[int] = []
    i: int = 0
    while i < size:
        dp.append(0)
        i = i + 1
    dp[0] = 1

    part: int = 1
    while part <= max_part:
        j: int = part
        while j <= target:
            prev_idx: int = j - part
            dp[j] = dp[j] + dp[prev_idx]
            j = j + 1
        part = part + 1
    return dp[target]


def test_module() -> int:
    """Test integer partition operations."""
    ok: int = 0

    two_parts: int = count_partitions_two(7)
    if two_parts == 3:
        ok = ok + 1

    equal: int = partition_into_equal(12, 3)
    if equal == 4:
        ok = ok + 1

    no_equal: int = partition_into_equal(7, 3)
    if no_equal == -1:
        ok = ok + 1

    prod: int = max_product_partition(10)
    if prod == 36:
        ok = ok + 1

    partitions: int = generate_partitions_sum(5, 5)
    if partitions == 7:
        ok = ok + 1

    return ok
