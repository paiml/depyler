"""String hashing for fast comparison and duplicate detection."""


def polynomial_hash(s: str, base: int, mod: int) -> int:
    """Compute polynomial rolling hash of string."""
    h: int = 0
    i: int = 0
    n: int = len(s)
    while i < n:
        h = (h * base + ord(s[i])) % mod
        i = i + 1
    return h


def double_hash(s: str) -> list[int]:
    """Compute two independent hashes for reduced collision probability."""
    h1: int = polynomial_hash(s, 31, 1000000007)
    h2: int = polynomial_hash(s, 37, 1000000009)
    result: list[int] = [h1, h2]
    return result


def count_unique_substrings(s: str, length: int) -> int:
    """Count unique substrings of given length using hashing."""
    n: int = len(s)
    if length > n or length <= 0:
        return 0
    hashes: list[int] = []
    i: int = 0
    while i <= n - length:
        h: int = polynomial_hash(s[i:i + length], 31, 1000000007)
        found: int = 0
        j: int = 0
        num_h: int = len(hashes)
        while j < num_h:
            if hashes[j] == h:
                found = 1
                break
            j = j + 1
        if found == 0:
            hashes.append(h)
        i = i + 1
    return len(hashes)


def strings_equal_by_hash(a: str, b: str) -> int:
    """Check string equality using double hashing. Returns 1/0."""
    if len(a) != len(b):
        return 0
    ha: list[int] = double_hash(a)
    hb: list[int] = double_hash(b)
    if ha[0] == hb[0] and ha[1] == hb[1]:
        return 1
    return 0


def test_module() -> int:
    passed: int = 0

    h1: int = polynomial_hash("hello", 31, 1000000007)
    h2: int = polynomial_hash("hello", 31, 1000000007)
    if h1 == h2:
        passed = passed + 1

    h3: int = polynomial_hash("world", 31, 1000000007)
    if h1 != h3:
        passed = passed + 1

    if strings_equal_by_hash("abc", "abc") == 1:
        passed = passed + 1

    if strings_equal_by_hash("abc", "abd") == 0:
        passed = passed + 1

    uniq: int = count_unique_substrings("abab", 2)
    if uniq == 2:
        passed = passed + 1

    uniq2: int = count_unique_substrings("aaaa", 1)
    if uniq2 == 1:
        passed = passed + 1

    if strings_equal_by_hash("test", "tess") == 0:
        passed = passed + 1

    return passed
