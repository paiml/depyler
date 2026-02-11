"""Array rotation operations: left rotate, right rotate by k positions."""


def rotate_left(arr: list[int], k: int) -> list[int]:
    """Rotate array left by k positions."""
    n: int = len(arr)
    if n == 0:
        return []
    k = k % n
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


def rotate_right(arr: list[int], k: int) -> list[int]:
    """Rotate array right by k positions."""
    n: int = len(arr)
    if n == 0:
        return []
    k = k % n
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


def reverse_array(arr: list[int], start: int, end: int) -> list[int]:
    """Reverse a portion of the array in-place style, returning new array."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    left: int = start
    right: int = end
    while left < right:
        tmp: int = result[left]
        result[left] = result[right]
        result[right] = tmp
        left = left + 1
        right = right - 1
    return result


def rotate_using_reverse(arr: list[int], k: int) -> list[int]:
    """Rotate left using triple reverse technique."""
    n: int = len(arr)
    if n == 0:
        return []
    k = k % n
    step1: list[int] = reverse_array(arr, 0, k - 1)
    step2: list[int] = reverse_array(step1, k, n - 1)
    step3: list[int] = reverse_array(step2, 0, n - 1)
    return step3


def test_module() -> int:
    passed: int = 0

    r1: list[int] = rotate_left([1, 2, 3, 4, 5], 2)
    if r1 == [3, 4, 5, 1, 2]:
        passed = passed + 1

    r2: list[int] = rotate_right([1, 2, 3, 4, 5], 2)
    if r2 == [4, 5, 1, 2, 3]:
        passed = passed + 1

    r3: list[int] = rotate_left([], 3)
    if r3 == []:
        passed = passed + 1

    r4: list[int] = rotate_right([1, 2, 3], 0)
    if r4 == [1, 2, 3]:
        passed = passed + 1

    r5: list[int] = rotate_left([1, 2, 3, 4, 5], 7)
    if r5 == [3, 4, 5, 1, 2]:
        passed = passed + 1

    r6: list[int] = rotate_using_reverse([1, 2, 3, 4, 5], 2)
    if r6 == [3, 4, 5, 1, 2]:
        passed = passed + 1

    r7: list[int] = reverse_array([1, 2, 3, 4, 5], 1, 3)
    if r7 == [1, 4, 3, 2, 5]:
        passed = passed + 1

    return passed
