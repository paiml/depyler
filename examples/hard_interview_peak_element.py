def find_peak(arr: list[int]) -> int:
    n: int = len(arr)
    if n == 0:
        return 0 - 1
    if n == 1:
        return 0
    if arr[0] > arr[1]:
        return 0
    last: int = n - 1
    prev_last: int = n - 2
    if arr[last] > arr[prev_last]:
        return last
    i: int = 1
    while i < n - 1:
        prev: int = arr[i - 1]
        curr: int = arr[i]
        nxt: int = arr[i + 1]
        if curr >= prev and curr >= nxt:
            return i
        i = i + 1
    return 0 - 1

def find_valley(arr: list[int]) -> int:
    n: int = len(arr)
    if n == 0:
        return 0 - 1
    if n == 1:
        return 0
    if arr[0] < arr[1]:
        return 0
    last: int = n - 1
    prev_last: int = n - 2
    if arr[last] < arr[prev_last]:
        return last
    i: int = 1
    while i < n - 1:
        prev: int = arr[i - 1]
        curr: int = arr[i]
        nxt: int = arr[i + 1]
        if curr <= prev and curr <= nxt:
            return i
        i = i + 1
    return 0 - 1

def count_peaks(arr: list[int]) -> int:
    n: int = len(arr)
    if n < 3:
        return 0
    cnt: int = 0
    i: int = 1
    while i < n - 1:
        prev: int = arr[i - 1]
        curr: int = arr[i]
        nxt: int = arr[i + 1]
        if curr > prev and curr > nxt:
            cnt = cnt + 1
        i = i + 1
    return cnt

def test_module() -> int:
    passed: int = 0
    r1: int = find_peak([1, 3, 5, 4, 2])
    if r1 == 2:
        passed = passed + 1
    r2: int = find_valley([5, 3, 1, 4, 6])
    if r2 == 2:
        passed = passed + 1
    r3: int = count_peaks([1, 3, 2, 5, 1, 4, 2])
    if r3 == 3:
        passed = passed + 1
    r4: int = find_peak([1, 2, 3, 4, 5])
    if r4 == 4:
        passed = passed + 1
    r5: int = find_valley([5, 4, 3, 2, 1])
    if r5 == 4:
        passed = passed + 1
    r6: int = count_peaks([1, 2, 3])
    if r6 == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
