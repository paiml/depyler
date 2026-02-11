"""Find top-k elements via partial sort (selection-based)."""


def find_kth_smallest(arr: list[int], k: int) -> int:
    """Find kth smallest element (1-indexed) using partial selection sort."""
    n: int = len(arr)
    work: list[int] = []
    i: int = 0
    while i < n:
        work.append(arr[i])
        i = i + 1
    i = 0
    while i < k:
        min_idx: int = i
        j: int = i + 1
        while j < n:
            if work[j] < work[min_idx]:
                min_idx = j
            j = j + 1
        tmp: int = work[i]
        work[i] = work[min_idx]
        work[min_idx] = tmp
        i = i + 1
    return work[k - 1]


def top_k_largest(arr: list[int], k: int) -> list[int]:
    """Find top-k largest elements, sorted descending."""
    n: int = len(arr)
    work: list[int] = []
    i: int = 0
    while i < n:
        work.append(arr[i])
        i = i + 1
    i = 0
    while i < k and i < n:
        max_idx: int = i
        j: int = i + 1
        while j < n:
            if work[j] > work[max_idx]:
                max_idx = j
            j = j + 1
        tmp: int = work[i]
        work[i] = work[max_idx]
        work[max_idx] = tmp
        i = i + 1
    result: list[int] = []
    i = 0
    while i < k and i < n:
        result.append(work[i])
        i = i + 1
    return result


def top_k_smallest(arr: list[int], k: int) -> list[int]:
    """Find top-k smallest elements, sorted ascending."""
    n: int = len(arr)
    work: list[int] = []
    i: int = 0
    while i < n:
        work.append(arr[i])
        i = i + 1
    i = 0
    while i < k and i < n:
        min_idx: int = i
        j: int = i + 1
        while j < n:
            if work[j] < work[min_idx]:
                min_idx = j
            j = j + 1
        tmp: int = work[i]
        work[i] = work[min_idx]
        work[min_idx] = tmp
        i = i + 1
    result: list[int] = []
    i = 0
    while i < k and i < n:
        result.append(work[i])
        i = i + 1
    return result


def test_module() -> int:
    """Test top-k elements."""
    passed: int = 0

    arr: list[int] = [3, 1, 4, 1, 5, 9, 2, 6]
    if find_kth_smallest(arr, 1) == 1:
        passed = passed + 1

    if find_kth_smallest(arr, 3) == 2:
        passed = passed + 1

    top3: list[int] = top_k_largest(arr, 3)
    if top3[0] == 9 and top3[1] == 6 and top3[2] == 5:
        passed = passed + 1

    bot3: list[int] = top_k_smallest(arr, 3)
    if bot3[0] == 1 and bot3[1] == 1 and bot3[2] == 2:
        passed = passed + 1

    single: list[int] = top_k_largest([7], 1)
    if len(single) == 1 and single[0] == 7:
        passed = passed + 1

    top2: list[int] = top_k_largest([10, 20, 30], 2)
    if top2[0] == 30 and top2[1] == 20:
        passed = passed + 1

    return passed
