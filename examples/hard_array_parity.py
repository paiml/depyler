"""Array parity operations: separate evens/odds, parity checks, and transforms."""


def separate_even_odd(arr: list[int]) -> list[int]:
    """Rearrange array so all even numbers come before odd numbers."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    left: int = 0
    right_idx: int = len(result) - 1
    while left < right_idx:
        while left < right_idx and result[left] % 2 == 0:
            left = left + 1
        while left < right_idx and result[right_idx] % 2 != 0:
            right_idx = right_idx - 1
        if left < right_idx:
            tmp: int = result[left]
            result[left] = result[right_idx]
            result[right_idx] = tmp
            left = left + 1
            right_idx = right_idx - 1
    return result


def parity_checksum(arr: list[int]) -> int:
    """Calculate parity checksum: XOR of all elements then return 0 for even, 1 for odd."""
    xor_val: int = 0
    i: int = 0
    while i < len(arr):
        xor_val = xor_val ^ arr[i]
        i = i + 1
    return xor_val & 1


def even_indexed_sum(arr: list[int]) -> int:
    """Sum elements at even indices (0, 2, 4, ...)."""
    total: int = 0
    i: int = 0
    while i < len(arr):
        total = total + arr[i]
        i = i + 2
    return total


def odd_indexed_sum(arr: list[int]) -> int:
    """Sum elements at odd indices (1, 3, 5, ...)."""
    total: int = 0
    i: int = 1
    while i < len(arr):
        total = total + arr[i]
        i = i + 2
    return total


def count_parity_transitions(arr: list[int]) -> int:
    """Count transitions between even and odd consecutive elements."""
    if len(arr) <= 1:
        return 0
    transitions: int = 0
    i: int = 1
    while i < len(arr):
        prev_idx: int = i - 1
        prev_parity: int = arr[prev_idx] % 2
        curr_parity: int = arr[i] % 2
        if prev_parity != curr_parity:
            transitions = transitions + 1
        i = i + 1
    return transitions


def test_module() -> int:
    """Test array parity operations."""
    ok: int = 0

    arr1: list[int] = [3, 2, 5, 4, 1, 6]
    sep: list[int] = separate_even_odd(arr1)
    # First elements should be even
    if sep[0] % 2 == 0 and sep[1] % 2 == 0 and sep[2] % 2 == 0:
        ok = ok + 1

    arr2: list[int] = [1, 2, 3]
    if parity_checksum(arr2) == 0:
        ok = ok + 1

    arr3: list[int] = [10, 20, 30, 40, 50]
    if even_indexed_sum(arr3) == 90:
        ok = ok + 1

    if odd_indexed_sum(arr3) == 60:
        ok = ok + 1

    arr4: list[int] = [1, 2, 3, 4, 5]
    if count_parity_transitions(arr4) == 4:
        ok = ok + 1

    arr5: list[int] = [2, 4, 6]
    if count_parity_transitions(arr5) == 0:
        ok = ok + 1

    return ok
