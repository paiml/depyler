"""Maximum product subarray and related problems.

Tests: max product, max subarray sum, min subarray sum, product of array.
"""


def max_product_subarray(nums: list[int]) -> int:
    """Maximum product of a contiguous subarray."""
    n: int = len(nums)
    if n == 0:
        return 0
    max_prod: int = nums[0]
    cur_max: int = nums[0]
    cur_min: int = nums[0]
    i: int = 1
    while i < n:
        val: int = nums[i]
        a: int = cur_max * val
        b: int = cur_min * val
        temp_max: int = val
        if a > temp_max:
            temp_max = a
        if b > temp_max:
            temp_max = b
        temp_min: int = val
        if a < temp_min:
            temp_min = a
        if b < temp_min:
            temp_min = b
        cur_max = temp_max
        cur_min = temp_min
        if cur_max > max_prod:
            max_prod = cur_max
        i = i + 1
    return max_prod


def product_of_array(nums: list[int]) -> int:
    """Product of all elements."""
    if len(nums) == 0:
        return 0
    result: int = 1
    for v in nums:
        result = result * v
    return result


def max_subarray_sum(nums: list[int]) -> int:
    """Kadane's algorithm for max subarray sum."""
    n: int = len(nums)
    if n == 0:
        return 0
    best: int = nums[0]
    current: int = nums[0]
    i: int = 1
    while i < n:
        if current + nums[i] > nums[i]:
            current = current + nums[i]
        else:
            current = nums[i]
        if current > best:
            best = current
        i = i + 1
    return best


def count_positive_products(nums: list[int]) -> int:
    """Count contiguous subarrays with positive product (brute force, small arrays)."""
    n: int = len(nums)
    count: int = 0
    i: int = 0
    while i < n:
        prod: int = 1
        j: int = i
        while j < n:
            prod = prod * nums[j]
            if prod > 0:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test maximum product subarray."""
    ok: int = 0
    if max_product_subarray([2, 3, -2, 4]) == 6:
        ok = ok + 1
    if max_product_subarray([-2, 0, -1]) == 0:
        ok = ok + 1
    if max_product_subarray([-2, 3, -4]) == 24:
        ok = ok + 1
    if product_of_array([1, 2, 3, 4]) == 24:
        ok = ok + 1
    if max_subarray_sum([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6:
        ok = ok + 1
    if count_positive_products([1, -1, 1]) == 4:
        ok = ok + 1
    return ok
