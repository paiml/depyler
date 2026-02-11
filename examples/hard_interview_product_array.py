def product_except_self(nums: list[int]) -> list[int]:
    n: int = len(nums)
    result: list[int] = []
    i: int = 0
    while i < n:
        result.append(1)
        i = i + 1
    left: int = 1
    j: int = 0
    while j < n:
        result[j] = left
        left = left * nums[j]
        j = j + 1
    right: int = 1
    k: int = n - 1
    while k >= 0:
        result[k] = result[k] * right
        right = right * nums[k]
        k = k - 1
    return result

def product_with_zeros(nums: list[int]) -> list[int]:
    n: int = len(nums)
    zero_count: int = 0
    total_product: int = 1
    i: int = 0
    while i < n:
        if nums[i] == 0:
            zero_count = zero_count + 1
        else:
            total_product = total_product * nums[i]
        i = i + 1
    result: list[int] = []
    j: int = 0
    while j < n:
        if zero_count > 1:
            result.append(0)
        elif zero_count == 1:
            if nums[j] == 0:
                result.append(total_product)
            else:
                result.append(0)
        else:
            val: int = total_product // nums[j]
            result.append(val)
        j = j + 1
    return result

def running_product(nums: list[int]) -> list[int]:
    result: list[int] = []
    prod: int = 1
    i: int = 0
    n: int = len(nums)
    while i < n:
        prod = prod * nums[i]
        result.append(prod)
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    r1: list[int] = product_except_self([1, 2, 3, 4])
    if r1 == [24, 12, 8, 6]:
        passed = passed + 1
    r2: list[int] = product_except_self([2, 3, 4, 5])
    if r2 == [60, 40, 30, 24]:
        passed = passed + 1
    r3: list[int] = product_with_zeros([1, 2, 0, 4])
    if r3 == [0, 0, 8, 0]:
        passed = passed + 1
    r4: list[int] = product_with_zeros([0, 2, 0, 4])
    if r4 == [0, 0, 0, 0]:
        passed = passed + 1
    r5: list[int] = running_product([1, 2, 3, 4])
    if r5 == [1, 2, 6, 24]:
        passed = passed + 1
    r6: list[int] = product_except_self([1, 1, 1, 1])
    if r6 == [1, 1, 1, 1]:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
