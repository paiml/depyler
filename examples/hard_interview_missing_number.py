def find_missing_number(nums: list[int], n: int) -> int:
    expected: int = n * (n + 1) // 2
    actual: int = 0
    i: int = 0
    sz: int = len(nums)
    while i < sz:
        actual = actual + nums[i]
        i = i + 1
    return expected - actual

def find_missing_xor(nums: list[int], n: int) -> int:
    xor_all: int = 0
    i: int = 0
    while i <= n:
        xor_all = xor_all ^ i
        i = i + 1
    j: int = 0
    sz: int = len(nums)
    while j < sz:
        xor_all = xor_all ^ nums[j]
        j = j + 1
    return xor_all

def find_duplicate(nums: list[int]) -> int:
    n: int = len(nums)
    count: list[int] = []
    i: int = 0
    while i < n:
        count.append(0)
        i = i + 1
    j: int = 0
    while j < n:
        v: int = nums[j]
        if v < n:
            count[v] = count[v] + 1
        j = j + 1
    k: int = 0
    while k < n:
        if count[k] > 1:
            return k
        k = k + 1
    return 0 - 1

def first_missing_positive(nums: list[int]) -> int:
    n: int = len(nums)
    present: list[int] = []
    i: int = 0
    while i <= n:
        present.append(0)
        i = i + 1
    j: int = 0
    while j < n:
        v: int = nums[j]
        if v > 0:
            if v <= n:
                present[v] = 1
        j = j + 1
    k: int = 1
    while k <= n:
        if present[k] == 0:
            return k
        k = k + 1
    return n + 1

def test_module() -> int:
    passed: int = 0
    r1: int = find_missing_number([0, 1, 3], 3)
    if r1 == 2:
        passed = passed + 1
    r2: int = find_missing_xor([0, 1, 3], 3)
    if r2 == 2:
        passed = passed + 1
    r3: int = find_duplicate([1, 3, 4, 2, 2])
    if r3 == 2:
        passed = passed + 1
    r4: int = first_missing_positive([3, 4, 0 - 1, 1])
    if r4 == 2:
        passed = passed + 1
    r5: int = first_missing_positive([1, 2, 3])
    if r5 == 4:
        passed = passed + 1
    r6: int = find_missing_number([1, 2, 3, 4], 4)
    if r6 == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
