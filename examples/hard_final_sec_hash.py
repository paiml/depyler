"""SHA-like hash function using Merkle-Damgard construction.

Simplified hash operating on integer arrays. Uses mixing, rotation,
and compression steps inspired by real hash functions.
"""


def rotate_left_16(val: int, amount: int) -> int:
    """Left rotation of a 16-bit value."""
    val = val % 65536
    shifted: int = (val * pow2_val(amount)) % 65536
    wrapped: int = val // pow2_val(16 - amount)
    return (shifted + wrapped) % 65536


def pow2_val(n: int) -> int:
    """Compute 2^n."""
    result: int = 1
    i: int = 0
    while i < n:
        result = result * 2
        i = i + 1
    return result


def mix_round(a: int, b: int, c: int, d: int, msg: int) -> list[int]:
    """One round of mixing: updates a, b, c, d with message word."""
    a = (a + b + msg) % 65536
    a = rotate_left_16(a, 3)
    d = (d + a) % 65536
    d = rotate_left_16(d, 7)
    c = (c + d) % 65536
    b = (b + c) % 65536
    b = rotate_left_16(b, 11)
    return [a, b, c, d]


def compress_block(state: list[int], block: list[int]) -> list[int]:
    """Compress one 4-word block into state."""
    a: int = state[0]
    b: int = state[1]
    c: int = state[2]
    d: int = state[3]
    i: int = 0
    while i < len(block):
        mv: int = block[i]
        mixed: list[int] = mix_round(a, b, c, d, mv)
        a = mixed[0]
        b = mixed[1]
        c = mixed[2]
        d = mixed[3]
        i = i + 1
    s0: int = state[0]
    s1: int = state[1]
    s2: int = state[2]
    s3: int = state[3]
    return [(a + s0) % 65536, (b + s1) % 65536, (c + s2) % 65536, (d + s3) % 65536]


def pad_message(msg: list[int]) -> list[int]:
    """Pad message to multiple of 4 words. Append length."""
    padded: list[int] = []
    i: int = 0
    while i < len(msg):
        mv: int = msg[i]
        padded.append(mv)
        i = i + 1
    padded.append(len(msg))
    while len(padded) % 4 != 0:
        padded.append(0)
    return padded


def simple_hash(msg: list[int]) -> list[int]:
    """Hash a message to a 4-word (64-bit) digest."""
    padded: list[int] = pad_message(msg)
    state: list[int] = [27183, 31415, 14142, 17320]
    i: int = 0
    while i < len(padded):
        block: list[int] = [padded[i], padded[i + 1], padded[i + 2], padded[i + 3]]
        state = compress_block(state, block)
        i = i + 4
    return state


def hash_equal(h1: list[int], h2: list[int]) -> int:
    """Check if two hashes are equal."""
    if len(h1) != len(h2):
        return 0
    i: int = 0
    while i < len(h1):
        v1: int = h1[i]
        v2: int = h2[i]
        if v1 != v2:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test hash function."""
    ok: int = 0
    h1: list[int] = simple_hash([1, 2, 3])
    h2: list[int] = simple_hash([1, 2, 3])
    if hash_equal(h1, h2) == 1:
        ok = ok + 1
    h3: list[int] = simple_hash([1, 2, 4])
    if hash_equal(h1, h3) == 0:
        ok = ok + 1
    if len(h1) == 4:
        ok = ok + 1
    r: int = rotate_left_16(1, 1)
    if r == 2:
        ok = ok + 1
    h_empty: list[int] = simple_hash([])
    if len(h_empty) == 4:
        ok = ok + 1
    return ok
