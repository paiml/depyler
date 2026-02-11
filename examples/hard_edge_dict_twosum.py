"""Two-sum and variants using dict lookup."""


def two_sum_indices(arr: list[int], target: int) -> list[int]:
    """Find two indices whose elements sum to target."""
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        complement: int = target - arr[i]
        if complement in seen:
            return [seen[complement], i]
        seen[arr[i]] = i
        i = i + 1
    return []


def two_sum_count(arr: list[int], target: int) -> int:
    """Count pairs that sum to target."""
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val in seen:
            seen[val] = seen[val] + 1
        else:
            seen[val] = 1
        i = i + 1
    count: int = 0
    processed: dict[int, int] = {}
    for sv in seen:
        complement: int = target - sv
        if complement in seen:
            if sv == complement:
                c: int = seen[sv]
                count = count + (c * (c - 1)) // 2
            elif complement not in processed:
                count = count + seen[sv] * seen[complement]
        processed[sv] = 1
    return count


def three_sum_zero(arr: list[int]) -> int:
    """Count triplets that sum to zero. Brute force."""
    n: int = len(arr)
    count: int = 0
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            needed: int = 0 - arr[i] - arr[j]
            k: int = j + 1
            while k < n:
                if arr[k] == needed:
                    count = count + 1
                k = k + 1
            j = j + 1
        i = i + 1
    return count


def pair_with_difference(arr: list[int], diff: int) -> int:
    """Count pairs with given absolute difference."""
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        seen[val] = 1
        i = i + 1
    count: int = 0
    processed: dict[int, int] = {}
    for sv in seen:
        target1: int = sv + diff
        if target1 in seen and target1 != sv:
            pair_id: int = sv * 100000 + target1
            if pair_id not in processed:
                count = count + 1
                processed[pair_id] = 1
                reverse_id: int = target1 * 100000 + sv
                processed[reverse_id] = 1
        if diff == 0:
            count = 0
            break
        i = i + 1
    return count


def subarray_sum_equals(arr: list[int], target: int) -> int:
    """Count subarrays with sum equal to target using prefix sum dict."""
    count: int = 0
    current_sum: int = 0
    prefix_counts: dict[int, int] = {}
    prefix_counts[0] = 1
    i: int = 0
    while i < len(arr):
        current_sum = current_sum + arr[i]
        needed: int = current_sum - target
        if needed in prefix_counts:
            count = count + prefix_counts[needed]
        if current_sum in prefix_counts:
            prefix_counts[current_sum] = prefix_counts[current_sum] + 1
        else:
            prefix_counts[current_sum] = 1
        i = i + 1
    return count


def longest_subarray_sum_zero(arr: list[int]) -> int:
    """Find length of longest subarray with sum zero."""
    prefix_first: dict[int, int] = {}
    prefix_first[0] = -1
    current_sum: int = 0
    best: int = 0
    i: int = 0
    while i < len(arr):
        current_sum = current_sum + arr[i]
        if current_sum in prefix_first:
            length: int = i - prefix_first[current_sum]
            if length > best:
                best = length
        else:
            prefix_first[current_sum] = i
        i = i + 1
    return best


def test_module() -> int:
    """Test all two-sum variant functions."""
    passed: int = 0
    r1: list[int] = two_sum_indices([2, 7, 11, 15], 9)
    if len(r1) == 2:
        passed = passed + 1
    if r1[0] == 0:
        passed = passed + 1
    if r1[1] == 1:
        passed = passed + 1
    r2: list[int] = two_sum_indices([1, 2, 3], 10)
    if len(r2) == 0:
        passed = passed + 1
    tc: int = two_sum_count([1, 1, 1], 2)
    if tc == 3:
        passed = passed + 1
    t3: int = three_sum_zero([0 - 1, 0, 1, 2, 0 - 1, 0 - 4])
    if t3 >= 1:
        passed = passed + 1
    sc: int = subarray_sum_equals([1, 1, 1], 2)
    if sc == 2:
        passed = passed + 1
    lz: int = longest_subarray_sum_zero([1, 0 - 1, 3, 2, 0 - 2, 0 - 3])
    if lz >= 4:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
