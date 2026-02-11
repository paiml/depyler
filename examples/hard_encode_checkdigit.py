def isbn10_check(digits: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < 9:
        weight: int = 10 - i
        total = total + digits[i] * weight
        i = i + 1
    rem: int = total % 11
    check: int = 11 - rem
    if check == 11:
        check = 0
    return check

def isbn13_check(digits: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < 12:
        weight: int = 1
        rem: int = i % 2
        if rem == 1:
            weight = 3
        total = total + digits[i] * weight
        i = i + 1
    rem2: int = total % 10
    check: int = 10 - rem2
    if check == 10:
        check = 0
    return check

def luhn_check(digits: list[int]) -> int:
    n: int = len(digits)
    total: int = 0
    i: int = n - 1
    dbl: int = 0
    while i >= 0:
        d: int = digits[i]
        if dbl == 1:
            d = d * 2
            if d > 9:
                d = d - 9
        total = total + d
        if dbl == 0:
            dbl = 1
        else:
            dbl = 0
        i = i - 1
    return total % 10

def ean8_check(digits: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < 7:
        weight: int = 1
        rem: int = i % 2
        if rem == 0:
            weight = 3
        total = total + digits[i] * weight
        i = i + 1
    rem2: int = total % 10
    check: int = 10 - rem2
    if check == 10:
        check = 0
    return check

def verhoeff_digit_sum(a: int, b: int) -> int:
    return (a + b) % 10

def test_module() -> int:
    passed: int = 0
    isbn: list[int] = [0, 3, 0, 6, 4, 0, 6, 1, 5]
    c: int = isbn10_check(isbn)
    if c == 2:
        passed = passed + 1
    isbn13: list[int] = [9, 7, 8, 0, 3, 0, 6, 4, 0, 6, 1, 5]
    c2: int = isbn13_check(isbn13)
    if c2 == 7:
        passed = passed + 1
    luhn: list[int] = [4, 9, 9, 2, 7, 3, 9, 8, 7, 1, 6]
    c3: int = luhn_check(luhn)
    if c3 == 0:
        passed = passed + 1
    ean: list[int] = [5, 5, 1, 2, 3, 4, 5]
    c4: int = ean8_check(ean)
    if c4 >= 0:
        passed = passed + 1
    vs: int = verhoeff_digit_sum(7, 8)
    if vs == 5:
        passed = passed + 1
    return passed
