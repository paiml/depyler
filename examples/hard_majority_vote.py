"""Boyer-Moore voting algorithm for majority element."""


def majority_element(arr: list[int]) -> int:
    """Find majority element (appears > n/2 times). Returns -1 if none."""
    n: int = len(arr)
    if n == 0:
        return -1
    candidate: int = arr[0]
    count: int = 1
    i: int = 1
    while i < n:
        if count == 0:
            candidate = arr[i]
            count = 1
        elif arr[i] == candidate:
            count = count + 1
        else:
            count = count - 1
        i = i + 1
    verify_count: int = 0
    j: int = 0
    while j < n:
        if arr[j] == candidate:
            verify_count = verify_count + 1
        j = j + 1
    if verify_count > n // 2:
        return candidate
    return -1


def majority_element_third(arr: list[int]) -> list[int]:
    """Find elements appearing > n/3 times (at most 2)."""
    n: int = len(arr)
    if n == 0:
        result: list[int] = []
        return result
    cand1: int = 0
    cand2: int = 1
    cnt1: int = 0
    cnt2: int = 0
    i: int = 0
    while i < n:
        if arr[i] == cand1:
            cnt1 = cnt1 + 1
        elif arr[i] == cand2:
            cnt2 = cnt2 + 1
        elif cnt1 == 0:
            cand1 = arr[i]
            cnt1 = 1
        elif cnt2 == 0:
            cand2 = arr[i]
            cnt2 = 1
        else:
            cnt1 = cnt1 - 1
            cnt2 = cnt2 - 1
        i = i + 1
    v1: int = 0
    v2: int = 0
    j: int = 0
    while j < n:
        if arr[j] == cand1:
            v1 = v1 + 1
        elif arr[j] == cand2:
            v2 = v2 + 1
        j = j + 1
    threshold: int = n // 3
    result: list[int] = []
    if v1 > threshold:
        result.append(cand1)
    if v2 > threshold:
        result.append(cand2)
    return result


def element_frequency(arr: list[int], target: int) -> int:
    """Count frequency of target in array."""
    count: int = 0
    i: int = 0
    n: int = len(arr)
    while i < n:
        if arr[i] == target:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0

    arr1: list[int] = [3, 3, 4, 2, 4, 4, 2, 4, 4]
    if majority_element(arr1) == 4:
        passed = passed + 1

    arr2: list[int] = [1, 2, 3, 4, 5]
    if majority_element(arr2) == -1:
        passed = passed + 1

    arr3: list[int] = [1, 1, 1, 3, 3, 2, 2, 3, 3]
    thirds: list[int] = majority_element_third(arr3)
    if len(thirds) == 1 and thirds[0] == 3:
        passed = passed + 1

    arr4: list[int] = [1, 2, 1, 2, 1, 2, 3]
    thirds2: list[int] = majority_element_third(arr4)
    if len(thirds2) == 2:
        passed = passed + 1

    arr5: list[int] = [7, 7, 7, 7, 7]
    if majority_element(arr5) == 7:
        passed = passed + 1

    if element_frequency(arr5, 7) == 5:
        passed = passed + 1

    if element_frequency(arr1, 4) == 5:
        passed = passed + 1

    return passed
