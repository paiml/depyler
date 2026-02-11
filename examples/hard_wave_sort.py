"""Wave sort: arrange array so arr[0]>=arr[1]<=arr[2]>=arr[3]..."""


def wave_sort(arr: list[int]) -> list[int]:
    """Sort array into wave pattern using simple swaps."""
    length: int = len(arr)
    i: int = 0
    while i < length:
        next_i: int = i + 1
        if i > 0 and arr[i] < arr[i - 1] and i % 2 == 0:
            temp: int = arr[i]
            prev: int = i - 1
            arr[i] = arr[prev]
            arr[prev] = temp
        if next_i < length and arr[i] < arr[next_i] and i % 2 == 0:
            temp2: int = arr[i]
            arr[i] = arr[next_i]
            arr[next_i] = temp2
        i = i + 1
    return arr


def is_wave_sorted(arr: list[int]) -> int:
    """Check if array is in wave form. Returns 1 if yes, 0 if no."""
    i: int = 0
    length: int = len(arr)
    while i < length:
        if i % 2 == 0:
            if i > 0 and arr[i] < arr[i - 1]:
                return 0
            next_i: int = i + 1
            if next_i < length and arr[i] < arr[next_i]:
                return 0
        i = i + 1
    return 1


def wave_sort_stable(arr: list[int]) -> list[int]:
    """Sort array first, then swap adjacent pairs for wave pattern."""
    length: int = len(arr)
    i: int = 0
    while i < length:
        j: int = 0
        limit: int = length - 1
        while j < limit:
            next_j: int = j + 1
            if arr[j] > arr[next_j]:
                temp: int = arr[j]
                arr[j] = arr[next_j]
                arr[next_j] = temp
            j = j + 1
        i = i + 1
    k: int = 0
    step_limit: int = length - 1
    while k < step_limit:
        next_k: int = k + 1
        temp2: int = arr[k]
        arr[k] = arr[next_k]
        arr[next_k] = temp2
        k = k + 2
    return arr


def test_module() -> int:
    """Test wave sort operations."""
    passed: int = 0

    r1: list[int] = wave_sort_stable([3, 1, 2, 4, 5])
    if is_wave_sorted(r1) == 1:
        passed = passed + 1

    r2: list[int] = wave_sort_stable([1, 2, 3, 4])
    if is_wave_sorted(r2) == 1:
        passed = passed + 1

    if is_wave_sorted([3, 1, 4, 2, 5]) == 1:
        passed = passed + 1

    if is_wave_sorted([1, 2, 3, 4]) == 0:
        passed = passed + 1

    if is_wave_sorted([]) == 1:
        passed = passed + 1

    r6: list[int] = wave_sort_stable([5, 5, 5])
    if is_wave_sorted(r6) == 1:
        passed = passed + 1

    r7: list[int] = wave_sort_stable([10, 20])
    if r7[0] == 20 and r7[1] == 10:
        passed = passed + 1

    if is_wave_sorted([7]) == 1:
        passed = passed + 1

    return passed
