# Generate combinations, sum of subsets


def binomial(n: int, k: int) -> int:
    if k > n:
        return 0
    if k == 0 or k == n:
        return 1
    # Use min(k, n-k) for efficiency
    effective_k: int = k
    if k > n - k:
        effective_k = n - k
    result: int = 1
    i: int = 0
    while i < effective_k:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result


def nth_combination(n: int, k: int, idx: int) -> list[int]:
    # Generate idx-th combination of k elements from [0..n-1]
    result: list[int] = []
    start: int = 0
    remaining: int = idx
    chosen: int = 0
    while chosen < k:
        candidate: int = start
        while candidate <= n - k + chosen:
            count: int = binomial(n - candidate - 1, k - chosen - 1)
            if remaining < count:
                result.append(candidate)
                start = candidate + 1
                chosen = chosen + 1
                break
            remaining = remaining - count
            candidate = candidate + 1
    return result


def subset_sum_count(arr: list[int], target: int) -> int:
    # Count subsets that sum to target using bitmask enumeration
    n: int = len(arr)
    total_masks: int = 1
    i: int = 0
    while i < n:
        total_masks = total_masks * 2
        i = i + 1
    count: int = 0
    mask: int = 0
    while mask < total_masks:
        s: int = 0
        bit: int = 0
        while bit < n:
            if (mask >> bit) & 1 == 1:
                s = s + arr[bit]
            bit = bit + 1
        if s == target:
            count = count + 1
        mask = mask + 1
    return count


def combination_sum(arr: list[int], k: int) -> int:
    # Sum of all k-element combinations' sums
    n: int = len(arr)
    if k > n:
        return 0
    # Each element appears in C(n-1, k-1) combinations
    multiplier: int = binomial(n - 1, k - 1)
    total: int = 0
    i: int = 0
    while i < n:
        total = total + arr[i] * multiplier
        i = i + 1
    return total


def test_module() -> int:
    passed: int = 0

    # Test 1: binomial coefficients
    if binomial(5, 2) == 10:
        passed = passed + 1

    # Test 2: binomial edge cases
    if binomial(5, 0) == 1 and binomial(5, 5) == 1:
        passed = passed + 1

    # Test 3: 0th combination of C(4,2) = [0,1]
    c: list[int] = nth_combination(4, 2, 0)
    if c[0] == 0 and c[1] == 1:
        passed = passed + 1

    # Test 4: last combination of C(4,2) = [2,3]
    c = nth_combination(4, 2, 5)
    if c[0] == 2 and c[1] == 3:
        passed = passed + 1

    # Test 5: subset sum count
    arr: list[int] = [1, 2, 3, 4]
    if subset_sum_count(arr, 5) == 3:
        passed = passed + 1

    # Test 6: subset sum with target 0
    if subset_sum_count(arr, 0) == 1:
        passed = passed + 1

    # Test 7: combination sum
    arr2: list[int] = [1, 2, 3]
    # C(2,1)=2, each appears 2 times: 2*(1+2+3) = 12
    if combination_sum(arr2, 2) == 12:
        passed = passed + 1

    return passed
