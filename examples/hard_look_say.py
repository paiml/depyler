"""Look-and-say sequence represented as int arrays."""


def look_say_next(arr: list[int]) -> list[int]:
    """Given current sequence as list of digits, produce next look-and-say term."""
    result: list[int] = []
    length: int = len(arr)
    if length == 0:
        return result
    idx: int = 0
    while idx < length:
        current_digit: int = arr[idx]
        count: int = 1
        next_pos: int = idx + 1
        while next_pos < length and arr[next_pos] == current_digit:
            count = count + 1
            next_pos = next_pos + 1
        result.append(count)
        result.append(current_digit)
        idx = next_pos
    return result


def look_say_iterate(start: list[int], steps: int) -> list[int]:
    """Iterate look-and-say sequence 'steps' times from start."""
    current: list[int] = []
    ci: int = 0
    while ci < len(start):
        current.append(start[ci])
        ci = ci + 1
    step: int = 0
    while step < steps:
        current = look_say_next(current)
        step = step + 1
    return current


def digits_to_int(arr: list[int]) -> int:
    """Convert digit array to integer."""
    result: int = 0
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        result = result * 10 + arr[idx]
        idx = idx + 1
    return result


def int_to_digits(n: int) -> list[int]:
    """Convert positive integer to digit array."""
    if n == 0:
        result: list[int] = [0]
        return result
    digits: list[int] = []
    val: int = n
    while val > 0:
        digits.append(val % 10)
        val = val // 10
    result2: list[int] = []
    ridx: int = len(digits) - 1
    while ridx >= 0:
        result2.append(digits[ridx])
        ridx = ridx - 1
    return result2


def test_module() -> int:
    passed: int = 0

    seq1: list[int] = look_say_next([1])
    if len(seq1) == 2:
        passed = passed + 1
    if seq1[0] == 1:
        passed = passed + 1

    seq2: list[int] = look_say_next([1, 1])
    if seq2[0] == 2:
        passed = passed + 1
    if seq2[1] == 1:
        passed = passed + 1

    seq3: list[int] = look_say_iterate([1], 3)
    if seq3[0] == 2:
        passed = passed + 1

    if digits_to_int([1, 2, 3]) == 123:
        passed = passed + 1

    digs: list[int] = int_to_digits(456)
    if digs[0] == 4:
        passed = passed + 1
    if digs[2] == 6:
        passed = passed + 1

    return passed
