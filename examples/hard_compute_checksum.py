"""Checksum algorithms: simple hash, CRC-like, Luhn, Adler-like.

Tests: simple_hash, crc_like, luhn_check, adler_checksum.
"""


def simple_hash(text: str, modulus: int) -> int:
    """Simple polynomial rolling hash of string."""
    h: int = 0
    multiplier: int = 31
    i: int = 0
    length: int = len(text)
    while i < length:
        ch_val: int = ord(text[i])
        h = (h * multiplier + ch_val) % modulus
        i = i + 1
    return h


def djb2_hash(text: str) -> int:
    """DJB2 hash function."""
    h: int = 5381
    i: int = 0
    length: int = len(text)
    while i < length:
        ch_val: int = ord(text[i])
        h = ((h * 33) + ch_val) % 2147483648
        i = i + 1
    return h


def fnv1a_hash(text: str) -> int:
    """FNV-1a hash (simplified, reduced modulus to avoid overflow)."""
    h: int = 2166136
    i: int = 0
    length: int = len(text)
    while i < length:
        ch_val: int = ord(text[i])
        h = h ^ ch_val
        h = (h * 16777) % 1000000007
        i = i + 1
    return h


def luhn_check(digits: list[int]) -> int:
    """Luhn algorithm check. Returns 1 if valid, 0 otherwise."""
    n: int = len(digits)
    total: int = 0
    alt: int = 0
    i: int = n - 1
    while i >= 0:
        d: int = digits[i]
        if alt == 1:
            d = d * 2
            if d > 9:
                d = d - 9
        total = total + d
        if alt == 0:
            alt = 1
        else:
            alt = 0
        i = i - 1
    if total % 10 == 0:
        return 1
    return 0


def luhn_generate_check_digit(digits: list[int]) -> int:
    """Generate check digit for Luhn algorithm."""
    extended: list[int] = []
    i: int = 0
    n: int = len(digits)
    while i < n:
        extended.append(digits[i])
        i = i + 1
    extended.append(0)
    check: int = 0
    while check < 10:
        extended[n] = check
        if luhn_check(extended) == 1:
            return check
        check = check + 1
    return -1


def adler32_like(data: list[int]) -> int:
    """Adler-32 like checksum over list of byte values."""
    mod_val: int = 65521
    a: int = 1
    b: int = 0
    i: int = 0
    n: int = len(data)
    while i < n:
        a = (a + data[i]) % mod_val
        b = (b + a) % mod_val
        i = i + 1
    return b * 65536 + a


def fletcher16_like(data: list[int]) -> int:
    """Fletcher-16 like checksum."""
    sum1: int = 0
    sum2: int = 0
    i: int = 0
    n: int = len(data)
    while i < n:
        sum1 = (sum1 + data[i]) % 255
        sum2 = (sum2 + sum1) % 255
        i = i + 1
    return sum2 * 256 + sum1


def test_module() -> int:
    """Test checksum algorithms."""
    passed: int = 0

    h1: int = simple_hash("hello", 1000000)
    h2: int = simple_hash("hello", 1000000)
    if h1 == h2:
        passed = passed + 1

    h3: int = simple_hash("world", 1000000)
    if h1 != h3:
        passed = passed + 1

    if luhn_check([7, 9, 9, 2, 7, 3, 9, 8, 7, 1]) == 1:
        passed = passed + 1

    if luhn_check([1, 2, 3, 4, 5]) == 0:
        passed = passed + 1

    cd: int = luhn_generate_check_digit([7, 9, 9, 2, 7, 3, 9, 8, 7])
    if cd == 1:
        passed = passed + 1

    a1: int = adler32_like([72, 101, 108, 108, 111])
    if a1 > 0:
        passed = passed + 1

    f1: int = fletcher16_like([1, 2, 3])
    if f1 > 0:
        passed = passed + 1

    return passed
