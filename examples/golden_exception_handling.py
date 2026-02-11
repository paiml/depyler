"""Exception-safe pattern tests using return codes (no try/except).

Tests error handling via sentinel values and return codes,
avoiding transpiler bugs with try/except, raise, class, Optional, f-strings.
"""


def safe_divide(a: int, b: int) -> int:
    """Division with zero check returning sentinel on error."""
    if b == 0:
        return 0
    return a // b


def safe_divide_with_code(a: int, b: int) -> int:
    """Division returning -1 on error."""
    if b == 0:
        return -1
    return a // b


def validate_range(value: int, lo: int, hi: int) -> int:
    """Returns 1 if value in [lo, hi], 0 otherwise."""
    if value < lo:
        return 0
    if value > hi:
        return 0
    return 1


def validate_positive(x: int) -> int:
    """Returns x if positive, -1 as error sentinel."""
    if x < 0:
        return -1
    return x


def validate_even(x: int) -> int:
    """Returns x // 2 if x is even and in [0, 1000], else error code."""
    if x < 0:
        return -1
    if x > 1000:
        return -2
    if x % 2 != 0:
        return -3
    return x // 2


def clamp_result(x: int) -> int:
    """Clamp computation: if x < 0 return -1, if x*2 > 100 return 100, else x*2."""
    if x < 0:
        return -1
    result: int = x * 2
    if result > 100:
        return 100
    return result


def chain_computation(a: int, b: int, c: int) -> int:
    """Chain of operations: a // b * c, with error codes for failures."""
    if b == 0:
        return -1
    step1: int = a // b
    step2: int = step1 * c
    if step2 < 0:
        return -2
    return step2


def safe_lookup(keys: list[str], values: list[int], key: str) -> int:
    """Manual dict lookup using parallel lists. Returns -1 if not found."""
    i: int = 0
    while i < len(keys):
        k: str = keys[i]
        if k == key:
            return values[i]
        i = i + 1
    return -1


def parse_digit_string(s: str) -> int:
    """Parse a string of digits manually. Returns -1 if any non-digit found."""
    if len(s) == 0:
        return -1
    result: int = 0
    i: int = 0
    while i < len(s):
        ch: str = s[i]
        code: int = ord(ch)
        if code < 48:
            return -1
        if code > 57:
            return -1
        digit: int = code - 48
        result = result * 10 + digit
        i = i + 1
    return result


def sum_digit_strings(items: list[str]) -> int:
    """Sum parsed digit strings. Returns -1 if any parse fails."""
    total: int = 0
    i: int = 0
    while i < len(items):
        s: str = items[i]
        parsed: int = parse_digit_string(s)
        if parsed < 0:
            return -1
        total = total + parsed
        i = i + 1
    return total


def nested_validate(x: int) -> int:
    """Nested validation: outer adds 1, inner doubles, error if > 100."""
    outer: int = x + 1
    inner: int = outer * 2
    if inner > 100:
        return outer
    return inner


def multi_step_validate(s: str, keys: list[str], values: list[int]) -> int:
    """Multi-step: parse string then lookup. Returns error codes on failure."""
    num: int = parse_digit_string(s)
    if num < 0:
        return -1
    num_str: str = str(num)
    result: int = safe_lookup(keys, values, num_str)
    if result < 0:
        return -2
    return result


def describe_validation(value: int) -> str:
    """Return string description of validation result."""
    result: int = validate_even(value)
    if result == -1:
        return "negative"
    if result == -2:
        return "too_large"
    if result == -3:
        return "odd"
    return "ok"


def process_or_default(x: int) -> int:
    """Process x with fallback: if negative, return 0."""
    if x < 0:
        return 0
    return x * 2


def conditional_chain(a: int, b: int) -> int:
    """If b==0 return safe value, else compute and validate."""
    if b == 0:
        return 0
    step1: int = a // b
    if step1 < 0:
        return 0
    return step1 * 3


def test_module() -> int:
    """Test all exception-safe patterns."""
    ok: int = 0

    if safe_divide(10, 2) == 5:
        ok = ok + 1
    if safe_divide(10, 0) == 0:
        ok = ok + 1

    if safe_divide_with_code(10, 2) == 5:
        ok = ok + 1
    if safe_divide_with_code(10, 0) == -1:
        ok = ok + 1

    if validate_range(50, 0, 100) == 1:
        ok = ok + 1
    if validate_range(200, 0, 100) == 0:
        ok = ok + 1

    if validate_positive(10) == 10:
        ok = ok + 1
    if validate_positive(-5) == -1:
        ok = ok + 1

    if validate_even(100) == 50:
        ok = ok + 1
    if validate_even(-1) == -1:
        ok = ok + 1
    if validate_even(2000) == -2:
        ok = ok + 1
    if validate_even(3) == -3:
        ok = ok + 1

    if clamp_result(-5) == -1:
        ok = ok + 1
    if clamp_result(10) == 20:
        ok = ok + 1
    if clamp_result(100) == 100:
        ok = ok + 1

    if chain_computation(10, 2, 3) == 15:
        ok = ok + 1
    if chain_computation(10, 0, 3) == -1:
        ok = ok + 1
    if chain_computation(10, 2, -3) == -2:
        ok = ok + 1

    if parse_digit_string("42") == 42:
        ok = ok + 1
    if parse_digit_string("") == -1:
        ok = ok + 1

    items1: list[str] = ["1", "2", "3"]
    if sum_digit_strings(items1) == 6:
        ok = ok + 1

    if nested_validate(10) == 22:
        ok = ok + 1
    if nested_validate(100) == 101:
        ok = ok + 1

    if process_or_default(-5) == 0:
        ok = ok + 1
    if process_or_default(10) == 20:
        ok = ok + 1

    if conditional_chain(10, 2) == 15:
        ok = ok + 1
    if conditional_chain(10, 0) == 0:
        ok = ok + 1

    r1: str = describe_validation(100)
    if r1 == "ok":
        ok = ok + 1
    r2: str = describe_validation(-1)
    if r2 == "negative":
        ok = ok + 1
    r3: str = describe_validation(3)
    if r3 == "odd":
        ok = ok + 1

    return ok
