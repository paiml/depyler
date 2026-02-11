def rotate_right(arr: list[int], k: int) -> list[int]:
    n: int = len(arr)
    if n == 0:
        return arr
    k = k % n
    if k == 0:
        return arr
    result: list[int] = []
    i: int = n - k
    while i < n:
        result.append(arr[i])
        i = i + 1
    j: int = 0
    while j < n - k:
        result.append(arr[j])
        j = j + 1
    return result

def rotate_left(arr: list[int], k: int) -> list[int]:
    n: int = len(arr)
    if n == 0:
        return arr
    k = k % n
    if k == 0:
        return arr
    result: list[int] = []
    i: int = k
    while i < n:
        result.append(arr[i])
        i = i + 1
    j: int = 0
    while j < k:
        result.append(arr[j])
        j = j + 1
    return result

def rotate_by_reversal(arr: list[int], k: int) -> list[int]:
    n: int = len(arr)
    if n == 0:
        return arr
    k = k % n
    if k == 0:
        return arr
    rev: list[int] = []
    i: int = n - 1
    while i >= 0:
        rev.append(arr[i])
        i = i - 1
    left: list[int] = []
    j: int = 0
    while j < k:
        left.append(rev[j])
        j = j + 1
    left_rev: list[int] = []
    m: int = k - 1
    while m >= 0:
        left_rev.append(left[m])
        m = m - 1
    right: list[int] = []
    r: int = k
    while r < n:
        right.append(rev[r])
        r = r + 1
    right_rev: list[int] = []
    s: int = len(right) - 1
    while s >= 0:
        right_rev.append(right[s])
        s = s - 1
    result: list[int] = left_rev + right_rev
    return result

def test_module() -> int:
    passed: int = 0
    r1: list[int] = rotate_right([1, 2, 3, 4, 5], 2)
    if r1 == [4, 5, 1, 2, 3]:
        passed = passed + 1
    r2: list[int] = rotate_left([1, 2, 3, 4, 5], 2)
    if r2 == [3, 4, 5, 1, 2]:
        passed = passed + 1
    r3: list[int] = rotate_by_reversal([1, 2, 3, 4, 5], 2)
    if r3 == [4, 5, 1, 2, 3]:
        passed = passed + 1
    r4: list[int] = rotate_right([1, 2, 3], 0)
    if r4 == [1, 2, 3]:
        passed = passed + 1
    r5: list[int] = rotate_right([], 3)
    if r5 == []:
        passed = passed + 1
    r6: list[int] = rotate_left([1, 2, 3, 4], 6)
    if r6 == [3, 4, 1, 2]:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
