def partition(arr: list[int], lo: int, hi: int) -> int:
    pivot: int = arr[hi]
    store: int = lo
    j: int = lo
    while j < hi:
        if arr[j] >= pivot:
            tmp: int = arr[store]
            arr[store] = arr[j]
            arr[j] = tmp
            store = store + 1
        j = j + 1
    tmp2: int = arr[store]
    arr[store] = arr[hi]
    arr[hi] = tmp2
    return store

def kth_largest(arr: list[int], k: int) -> int:
    work: list[int] = []
    i: int = 0
    while i < len(arr):
        work.append(arr[i])
        i = i + 1
    lo: int = 0
    hi: int = len(work) - 1
    target: int = k - 1
    while lo <= hi:
        p: int = partition(work, lo, hi)
        if p == target:
            return work[p]
        elif p < target:
            lo = p + 1
        else:
            hi = p - 1
    return -1

def find_max(arr: list[int]) -> int:
    mx: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] > mx:
            mx = arr[i]
        i = i + 1
    return mx

def find_min(arr: list[int]) -> int:
    mn: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] < mn:
            mn = arr[i]
        i = i + 1
    return mn

def test_module() -> int:
    passed: int = 0
    if kth_largest([3, 1, 4, 1, 5, 9, 2, 6], 1) == 9:
        passed = passed + 1
    if kth_largest([3, 1, 4, 1, 5, 9, 2, 6], 2) == 6:
        passed = passed + 1
    if kth_largest([3, 1, 4, 1, 5, 9, 2, 6], 3) == 5:
        passed = passed + 1
    if find_max([7, 2, 9, 1]) == 9:
        passed = passed + 1
    if find_min([7, 2, 9, 1]) == 1:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
