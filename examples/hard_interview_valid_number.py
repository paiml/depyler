def is_digit(c: str) -> int:
    v: int = ord(c)
    if v >= 48 and v <= 57:
        return 1
    return 0

def is_valid_integer(s: str) -> int:
    n: int = len(s)
    if n == 0:
        return 0
    start: int = 0
    if s[0] == "+" or s[0] == "-":
        start = 1
    if start >= n:
        return 0
    i: int = start
    while i < n:
        d: int = is_digit(s[i])
        if d == 0:
            return 0
        i = i + 1
    return 1

def is_valid_float(s: str) -> int:
    n: int = len(s)
    if n == 0:
        return 0
    start: int = 0
    if s[0] == "+" or s[0] == "-":
        start = 1
    if start >= n:
        return 0
    dot_count: int = 0
    digit_count: int = 0
    i: int = start
    while i < n:
        ch: str = s[i]
        if ch == ".":
            dot_count = dot_count + 1
            if dot_count > 1:
                return 0
        else:
            d: int = is_digit(ch)
            if d == 0:
                return 0
            digit_count = digit_count + 1
        i = i + 1
    if digit_count == 0:
        return 0
    return 1

def parse_integer(s: str) -> int:
    n: int = len(s)
    if n == 0:
        return 0
    neg: int = 0
    start: int = 0
    if s[0] == "-":
        neg = 1
        start = 1
    elif s[0] == "+":
        start = 1
    result: int = 0
    i: int = start
    while i < n:
        d: int = ord(s[i]) - 48
        result = result * 10 + d
        i = i + 1
    if neg == 1:
        return 0 - result
    return result

def string_to_digits(s: str) -> list[int]:
    result: list[int] = []
    i: int = 0
    n: int = len(s)
    while i < n:
        d: int = is_digit(s[i])
        if d == 1:
            result.append(ord(s[i]) - 48)
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    r1: int = is_valid_integer("123")
    if r1 == 1:
        passed = passed + 1
    r2: int = is_valid_integer("-456")
    if r2 == 1:
        passed = passed + 1
    r3: int = is_valid_float("3.14")
    if r3 == 1:
        passed = passed + 1
    r4: int = is_valid_float("12.3.4")
    if r4 == 0:
        passed = passed + 1
    r5: int = parse_integer("-42")
    if r5 == 0 - 42:
        passed = passed + 1
    r6: list[int] = string_to_digits("a1b2c3")
    if r6 == [1, 2, 3]:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
