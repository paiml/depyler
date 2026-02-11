def bigint_from_int(n: int) -> list[int]:
    if n == 0:
        return [0]
    digits: list[int] = []
    val: int = n
    if val < 0:
        val = -val
    while val > 0:
        digits.append(val % 10)
        val = val // 10
    return digits


def bigint_add(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    carry: int = 0
    i: int = 0
    la: int = len(a)
    lb: int = len(b)
    max_len: int = la
    if lb > max_len:
        max_len = lb
    while i < max_len or carry > 0:
        da: int = 0
        db: int = 0
        if i < la:
            da = a[i]
        if i < lb:
            db = b[i]
        s: int = da + db + carry
        result.append(s % 10)
        carry = s // 10
        i = i + 1
    return result


def bigint_subtract(a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    borrow: int = 0
    i: int = 0
    while i < len(a):
        da: int = a[i]
        db: int = 0
        if i < len(b):
            db = b[i]
        diff: int = da - db - borrow
        if diff < 0:
            diff = diff + 10
            borrow = 1
        else:
            borrow = 0
        result.append(diff)
        i = i + 1
    while len(result) > 1 and result[len(result) - 1] == 0:
        result.pop()
    return result


def bigint_to_int(digits: list[int]) -> int:
    result: int = 0
    i: int = len(digits) - 1
    while i >= 0:
        result = result * 10 + digits[i]
        i = i - 1
    return result


def test_module() -> int:
    passed: int = 0
    a: list[int] = bigint_from_int(123)
    if a == [3, 2, 1]:
        passed = passed + 1
    if bigint_to_int([3, 2, 1]) == 123:
        passed = passed + 1
    s: list[int] = bigint_add([9, 9], [1])
    if s == [0, 0, 1]:
        passed = passed + 1
    d: list[int] = bigint_subtract([0, 0, 1], [1])
    if d == [9, 9]:
        passed = passed + 1
    if bigint_add([0], [0]) == [0]:
        passed = passed + 1
    if bigint_from_int(0) == [0]:
        passed = passed + 1
    s2: list[int] = bigint_add([5, 4, 3], [6, 7, 8])
    if bigint_to_int(s2) == 1211:
        passed = passed + 1
    return passed
