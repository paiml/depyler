def base64_char(idx: int) -> str:
    chars: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    return chars[idx]

def encode_triplet(a: int, b: int, c: int) -> str:
    combined: int = (a * 65536) + (b * 256) + c
    i0: int = (combined // 262144) % 64
    i1: int = (combined // 4096) % 64
    i2: int = (combined // 64) % 64
    i3: int = combined % 64
    r: str = base64_char(i0) + base64_char(i1) + base64_char(i2) + base64_char(i3)
    return r

def bytes_to_ints(data: list[int]) -> list[int]:
    result: list[int] = []
    n: int = len(data)
    i: int = 0
    while i < n:
        result.append(data[i] % 256)
        i = i + 1
    return result

def base64_encode_ints(data: list[int]) -> str:
    n: int = len(data)
    result: str = ""
    i: int = 0
    while i + 2 < n:
        a: int = data[i]
        b: int = data[i + 1]
        c: int = data[i + 2]
        result = result + encode_triplet(a, b, c)
        i = i + 3
    rem: int = n % 3
    if rem == 1:
        result = result + encode_triplet(data[n - 1], 0, 0)
    if rem == 2:
        a2: int = data[n - 2]
        b2: int = data[n - 1]
        result = result + encode_triplet(a2, b2, 0)
    return result

def count_padding(data_len: int) -> int:
    rem: int = data_len % 3
    if rem == 0:
        return 0
    return 3 - rem

def test_module() -> int:
    passed: int = 0
    c: str = base64_char(0)
    if c == "A":
        passed = passed + 1
    c2: str = base64_char(25)
    if c2 == "Z":
        passed = passed + 1
    c3: str = base64_char(26)
    if c3 == "a":
        passed = passed + 1
    t: str = encode_triplet(77, 97, 110)
    if t == "TWFu":
        passed = passed + 1
    p: int = count_padding(4)
    if p == 2:
        passed = passed + 1
    return passed
