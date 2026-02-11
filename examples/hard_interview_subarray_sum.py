def subarray_with_sum(arr: list[int], target: int) -> list[int]:
    n: int = len(arr)
    i: int = 0
    while i < n:
        total: int = 0
        j: int = i
        while j < n:
            total = total + arr[j]
            if total == target:
                result: list[int] = []
                k: int = i
                while k <= j:
                    result.append(arr[k])
                    k = k + 1
                return result
            j = j + 1
        i = i + 1
    return []

def max_subarray_sum(arr: list[int]) -> int:
    n: int = len(arr)
    if n == 0:
        return 0
    best: int = arr[0]
    current: int = arr[0]
    i: int = 1
    while i < n:
        v: int = arr[i]
        if current + v > v:
            current = current + v
        else:
            current = v
        if current > best:
            best = current
        i = i + 1
    return best

def max_sum_no_adjacent(arr: list[int]) -> int:
    n: int = len(arr)
    if n == 0:
        return 0
    if n == 1:
        return arr[0]
    prev2: int = arr[0]
    first_two: int = arr[1]
    if arr[0] > arr[1]:
        first_two = arr[0]
    prev1: int = first_two
    i: int = 2
    while i < n:
        v: int = arr[i]
        pick: int = prev2 + v
        skip: int = prev1
        curr: int = pick
        if skip > pick:
            curr = skip
        prev2 = prev1
        prev1 = curr
        i = i + 1
    return prev1

def test_module() -> int:
    passed: int = 0
    r1: list[int] = subarray_with_sum([1, 4, 20, 3, 10, 5], 33)
    if r1 == [20, 3, 10]:
        passed = passed + 1
    r2: int = max_subarray_sum([0 - 2, 1, 0 - 3, 4, 0 - 1, 2, 1, 0 - 5, 4])
    if r2 == 6:
        passed = passed + 1
    r3: int = max_sum_no_adjacent([3, 2, 7, 10])
    if r3 == 13:
        passed = passed + 1
    r4: int = max_sum_no_adjacent([5, 5, 10, 100, 10, 5])
    if r4 == 110:
        passed = passed + 1
    r5: list[int] = subarray_with_sum([1, 2, 3], 10)
    if r5 == []:
        passed = passed + 1
    r6: int = max_subarray_sum([0 - 1, 0 - 2, 0 - 3])
    if r6 == 0 - 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
