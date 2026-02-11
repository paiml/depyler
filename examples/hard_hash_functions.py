def simple_hash(data: list[int], table_size: int) -> int:
    h: int = 0
    i: int = 0
    while i < len(data):
        h = h + data[i]
        i = i + 1
    return h % table_size


def polynomial_hash(data: list[int], base: int, mod: int) -> int:
    h: int = 0
    i: int = 0
    while i < len(data):
        h = (h * base + data[i]) % mod
        i = i + 1
    return h


def rolling_hash_init(data: list[int], window: int, base: int, mod: int) -> int:
    h: int = 0
    i: int = 0
    while i < window and i < len(data):
        h = (h * base + data[i]) % mod
        i = i + 1
    return h


def rolling_hash_slide(old_hash: int, old_val: int, new_val: int, base_power: int, base: int, mod: int) -> int:
    h: int = (old_hash - old_val * base_power % mod + mod) % mod
    h = (h * base + new_val) % mod
    return h


def rabin_karp_count(text: list[int], pattern: list[int]) -> int:
    n: int = len(text)
    m: int = len(pattern)
    if m > n or m == 0:
        return 0
    base: int = 31
    mod: int = 1000000007
    bp: int = 1
    i: int = 0
    while i < m - 1:
        bp = (bp * base) % mod
        i = i + 1
    ph: int = polynomial_hash(pattern, base, mod)
    th: int = rolling_hash_init(text, m, base, mod)
    count: int = 0
    pos: int = 0
    while pos <= n - m:
        if th == ph:
            match: int = 1
            j: int = 0
            while j < m:
                if text[pos + j] != pattern[j]:
                    match = 0
                    j = m
                j = j + 1
            if match == 1:
                count = count + 1
        if pos < n - m:
            th = rolling_hash_slide(th, text[pos], text[pos + m], bp, base, mod)
        pos = pos + 1
    return count


def test_module() -> int:
    passed: int = 0
    if simple_hash([1, 2, 3], 10) == 6:
        passed = passed + 1
    if simple_hash([], 10) == 0:
        passed = passed + 1
    if polynomial_hash([1, 2, 3], 31, 1000) == ((1 * 31 + 2) * 31 + 3) % 1000:
        passed = passed + 1
    if rabin_karp_count([1, 2, 1, 2, 1], [1, 2]) == 2:
        passed = passed + 1
    if rabin_karp_count([1, 1, 1], [1]) == 3:
        passed = passed + 1
    if rabin_karp_count([], [1]) == 0:
        passed = passed + 1
    if rabin_karp_count([1, 2, 3], [4, 5]) == 0:
        passed = passed + 1
    return passed
