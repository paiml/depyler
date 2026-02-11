def power_set(nums: list[int]) -> list[list[int]]:
    n: int = len(nums)
    total: int = 1 << n
    result: list[list[int]] = []
    mask: int = 0
    while mask < total:
        subset: list[int] = []
        bit: int = 0
        while bit < n:
            if (mask >> bit) & 1 == 1:
                subset.append(nums[bit])
            bit = bit + 1
        result.append(subset)
        mask = mask + 1
    return result

def power_set_size(n: int) -> int:
    return 1 << n

def subset_sum_exists(nums: list[int], target: int) -> int:
    n: int = len(nums)
    total: int = 1 << n
    mask: int = 0
    while mask < total:
        s: int = 0
        bit: int = 0
        while bit < n:
            if (mask >> bit) & 1 == 1:
                s = s + nums[bit]
            bit = bit + 1
        if s == target:
            return 1
        mask = mask + 1
    return 0

def count_subsets_with_sum(nums: list[int], target: int) -> int:
    n: int = len(nums)
    total: int = 1 << n
    count: int = 0
    mask: int = 0
    while mask < total:
        s: int = 0
        bit: int = 0
        while bit < n:
            if (mask >> bit) & 1 == 1:
                s = s + nums[bit]
            bit = bit + 1
        if s == target:
            count = count + 1
        mask = mask + 1
    return count

def max_subset_sum(nums: list[int]) -> int:
    n: int = len(nums)
    total: int = 1 << n
    best: int = 0
    mask: int = 0
    while mask < total:
        s: int = 0
        bit: int = 0
        while bit < n:
            if (mask >> bit) & 1 == 1:
                s = s + nums[bit]
            bit = bit + 1
        if s > best:
            best = s
        mask = mask + 1
    return best

def test_module() -> int:
    passed: int = 0
    ps: list[list[int]] = power_set([1, 2, 3])
    nps: int = len(ps)
    if nps == 8:
        passed = passed + 1
    r2: int = power_set_size(4)
    if r2 == 16:
        passed = passed + 1
    r3: int = subset_sum_exists([3, 7, 1, 8], 11)
    if r3 == 1:
        passed = passed + 1
    r4: int = count_subsets_with_sum([1, 2, 3, 4], 5)
    if r4 == 2:
        passed = passed + 1
    r5: int = max_subset_sum([1, 0 - 2, 3, 0 - 4, 5])
    if r5 == 9:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
