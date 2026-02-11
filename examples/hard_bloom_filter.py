def hash_fn1(val: int, size: int) -> int:
    return val % size


def hash_fn2(val: int, size: int) -> int:
    return (val * 7 + 3) % size


def hash_fn3(val: int, size: int) -> int:
    return (val * 13 + 17) % size


def bloom_create(size: int) -> list[int]:
    bits: list[int] = []
    i: int = 0
    while i < size:
        bits.append(0)
        i = i + 1
    return bits


def bloom_add(bits: list[int], val: int) -> list[int]:
    size: int = len(bits)
    bits[hash_fn1(val, size)] = 1
    bits[hash_fn2(val, size)] = 1
    bits[hash_fn3(val, size)] = 1
    return bits


def bloom_check(bits: list[int], val: int) -> int:
    size: int = len(bits)
    if bits[hash_fn1(val, size)] == 0:
        return 0
    if bits[hash_fn2(val, size)] == 0:
        return 0
    if bits[hash_fn3(val, size)] == 0:
        return 0
    return 1


def bloom_count_bits(bits: list[int]) -> int:
    count: int = 0
    i: int = 0
    while i < len(bits):
        if bits[i] == 1:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0
    b: list[int] = bloom_create(64)
    if len(b) == 64:
        passed = passed + 1
    if bloom_count_bits(b) == 0:
        passed = passed + 1
    b = bloom_add(b, 42)
    if bloom_check(b, 42) == 1:
        passed = passed + 1
    if bloom_count_bits(b) > 0:
        passed = passed + 1
    b = bloom_add(b, 100)
    if bloom_check(b, 100) == 1:
        passed = passed + 1
    b2: list[int] = bloom_create(100)
    if bloom_check(b2, 50) == 0:
        passed = passed + 1
    b2 = bloom_add(b2, 1)
    b2 = bloom_add(b2, 2)
    b2 = bloom_add(b2, 3)
    if bloom_check(b2, 1) == 1:
        passed = passed + 1
    return passed
