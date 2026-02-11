def hash_fn1(val: int, size: int) -> int:
    h: int = (val * 2654435761) % size
    if h < 0:
        h = 0 - h
    return h % size

def hash_fn2(val: int, size: int) -> int:
    h: int = (val * 40503) % size
    if h < 0:
        h = 0 - h
    return h % size

def hash_fn3(val: int, size: int) -> int:
    h: int = (val * 11400714819323198485) % size
    if h < 0:
        h = 0 - h
    return h % size

def bloom_create(size: int) -> list[int]:
    bits: list[int] = []
    i: int = 0
    while i < size:
        bits.append(0)
        i = i + 1
    return bits

def bloom_add(bits: list[int], val: int) -> int:
    size: int = len(bits)
    h1: int = hash_fn1(val, size)
    h2: int = hash_fn2(val, size)
    h3: int = hash_fn3(val, size)
    bits[h1] = 1
    bits[h2] = 1
    bits[h3] = 1
    return 1

def bloom_check(bits: list[int], val: int) -> int:
    size: int = len(bits)
    h1: int = hash_fn1(val, size)
    h2: int = hash_fn2(val, size)
    h3: int = hash_fn3(val, size)
    b1: int = bits[h1]
    b2: int = bits[h2]
    b3: int = bits[h3]
    if b1 == 1 and b2 == 1 and b3 == 1:
        return 1
    return 0

def bloom_fill_ratio(bits: list[int]) -> float:
    size: int = len(bits)
    ones: int = 0
    i: int = 0
    while i < size:
        v: int = bits[i]
        ones = ones + v
        i = i + 1
    return ones * 1.0 / (size * 1.0)

def test_module() -> int:
    passed: int = 0
    bf: list[int] = bloom_create(100)
    c: int = bloom_check(bf, 42)
    if c == 0:
        passed = passed + 1
    bloom_add(bf, 42)
    c2: int = bloom_check(bf, 42)
    if c2 == 1:
        passed = passed + 1
    bloom_add(bf, 100)
    c3: int = bloom_check(bf, 100)
    if c3 == 1:
        passed = passed + 1
    fr: float = bloom_fill_ratio(bf)
    if fr > 0.0 and fr < 1.0:
        passed = passed + 1
    nb: int = len(bf)
    if nb == 100:
        passed = passed + 1
    return passed
