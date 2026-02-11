# Generate nth permutation, count inversions


def factorial(n: int) -> int:
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result


def nth_permutation(n: int, k: int) -> list[int]:
    # Generate k-th (0-indexed) permutation of [0..n-1]
    available: list[int] = []
    i: int = 0
    while i < n:
        available.append(i)
        i = i + 1
    result: list[int] = []
    remaining: int = k
    pos: int = 0
    while pos < n:
        fact: int = factorial(n - 1 - pos)
        idx: int = remaining // fact
        remaining = remaining % fact
        result.append(available[idx])
        # Remove element at idx
        new_avail: list[int] = []
        j: int = 0
        while j < len(available):
            if j != idx:
                new_avail.append(available[j])
            j = j + 1
        available = new_avail
        pos = pos + 1
    return result


def count_inversions(arr: list[int]) -> int:
    count: int = 0
    i: int = 0
    while i < len(arr):
        j: int = i + 1
        while j < len(arr):
            if arr[i] > arr[j]:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def is_even_permutation(arr: list[int]) -> int:
    inv: int = count_inversions(arr)
    if inv % 2 == 0:
        return 1
    return 0


def permutation_rank(perm: list[int]) -> int:
    n: int = len(perm)
    rank: int = 0
    i: int = 0
    while i < n:
        smaller: int = 0
        j: int = i + 1
        while j < n:
            if perm[j] < perm[i]:
                smaller = smaller + 1
            j = j + 1
        rank = rank + smaller * factorial(n - 1 - i)
        i = i + 1
    return rank


def test_module() -> int:
    passed: int = 0

    # Test 1: factorial
    if factorial(5) == 120:
        passed = passed + 1

    # Test 2: 0th permutation of 3 = [0,1,2]
    p: list[int] = nth_permutation(3, 0)
    if p[0] == 0 and p[1] == 1 and p[2] == 2:
        passed = passed + 1

    # Test 3: last permutation of 3 = [2,1,0]
    p = nth_permutation(3, 5)
    if p[0] == 2 and p[1] == 1 and p[2] == 0:
        passed = passed + 1

    # Test 4: count inversions of sorted = 0
    sorted_arr: list[int] = [0, 1, 2, 3]
    if count_inversions(sorted_arr) == 0:
        passed = passed + 1

    # Test 5: count inversions of reversed
    rev: list[int] = [3, 2, 1, 0]
    if count_inversions(rev) == 6:
        passed = passed + 1

    # Test 6: even permutation
    if is_even_permutation(sorted_arr) == 1:
        passed = passed + 1

    # Test 7: rank of identity
    identity: list[int] = [0, 1, 2]
    if permutation_rank(identity) == 0:
        passed = passed + 1

    # Test 8: rank round-trip
    p = nth_permutation(4, 10)
    if permutation_rank(p) == 10:
        passed = passed + 1

    return passed
