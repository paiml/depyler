"""Array rotation and circular buffer operations."""


def rotate_left(arr: list[int], n: int) -> list[int]:
    """Rotate array left by n positions."""
    sz: int = len(arr)
    if sz == 0:
        return []
    shift: int = n % sz
    result: list[int] = []
    i: int = shift
    while i < sz:
        result.append(arr[i])
        i = i + 1
    i = 0
    while i < shift:
        result.append(arr[i])
        i = i + 1
    return result


def rotate_right(arr: list[int], n: int) -> list[int]:
    """Rotate array right by n positions."""
    sz: int = len(arr)
    if sz == 0:
        return []
    shift: int = n % sz
    return rotate_left(arr, sz - shift)


def reverse_subarray(arr: list[int], start: int, end: int) -> list[int]:
    """Reverse elements from start to end (inclusive) in a copy."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    lo: int = start
    hi: int = end
    while lo < hi:
        tmp: int = result[lo]
        result[lo] = result[hi]
        result[hi] = tmp
        lo = lo + 1
        hi = hi - 1
    return result


def rotate_by_reversal(arr: list[int], n: int) -> list[int]:
    """Rotate left using three reversals."""
    sz: int = len(arr)
    if sz == 0:
        return []
    shift: int = n % sz
    if shift == 0:
        result: list[int] = []
        i: int = 0
        while i < sz:
            result.append(arr[i])
            i = i + 1
        return result
    step1: list[int] = reverse_subarray(arr, 0, shift - 1)
    step2: list[int] = reverse_subarray(step1, shift, sz - 1)
    step3: list[int] = reverse_subarray(step2, 0, sz - 1)
    return step3


def circular_get(arr: list[int], idx: int) -> int:
    """Get element at circular index."""
    sz: int = len(arr)
    if sz == 0:
        return 0
    actual: int = idx % sz
    return arr[actual]


def circular_buffer_write(buf: list[int], write_pos: int, val: int) -> list[int]:
    """Write to circular buffer at position, return new buffer."""
    result: list[int] = []
    i: int = 0
    while i < len(buf):
        result.append(buf[i])
        i = i + 1
    sz: int = len(result)
    if sz > 0:
        actual: int = write_pos % sz
        result[actual] = val
    return result


def is_rotation_of(arr1: list[int], arr2: list[int]) -> int:
    """Return 1 if arr2 is a rotation of arr1."""
    n1: int = len(arr1)
    n2: int = len(arr2)
    if n1 != n2:
        return 0
    if n1 == 0:
        return 1
    r: int = 0
    while r < n1:
        matched: int = 1
        j: int = 0
        while j < n1:
            idx: int = (r + j) % n1
            if arr1[idx] != arr2[j]:
                matched = 0
                j = n1
            else:
                j = j + 1
        if matched == 1:
            return 1
        r = r + 1
    return 0


def find_rotation_point(arr: list[int]) -> int:
    """Find index where rotation starts in a rotated sorted array."""
    sz: int = len(arr)
    if sz <= 1:
        return 0
    lo: int = 0
    hi: int = sz - 1
    if arr[lo] <= arr[hi]:
        return 0
    while lo < hi:
        mid: int = (lo + hi) // 2
        if arr[mid] > arr[hi]:
            lo = mid + 1
        else:
            hi = mid
    return lo


def test_module() -> int:
    """Test all rotation functions."""
    passed: int = 0
    r1: list[int] = rotate_left([1, 2, 3, 4, 5], 2)
    if r1 == [3, 4, 5, 1, 2]:
        passed = passed + 1
    r2: list[int] = rotate_right([1, 2, 3, 4, 5], 2)
    if r2 == [4, 5, 1, 2, 3]:
        passed = passed + 1
    r3: list[int] = rotate_left([], 3)
    if len(r3) == 0:
        passed = passed + 1
    r4: list[int] = rotate_by_reversal([1, 2, 3, 4, 5], 2)
    if r4 == [3, 4, 5, 1, 2]:
        passed = passed + 1
    cv: int = circular_get([10, 20, 30], 5)
    if cv == 30:
        passed = passed + 1
    buf: list[int] = [0, 0, 0, 0]
    buf2: list[int] = circular_buffer_write(buf, 5, 42)
    if buf2[1] == 42:
        passed = passed + 1
    if is_rotation_of([1, 2, 3], [3, 1, 2]) == 1:
        passed = passed + 1
    if is_rotation_of([1, 2, 3], [1, 3, 2]) == 0:
        passed = passed + 1
    rp: int = find_rotation_point([4, 5, 1, 2, 3])
    if rp == 2:
        passed = passed + 1
    rp2: int = find_rotation_point([1, 2, 3, 4, 5])
    if rp2 == 0:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
