"""Stream cipher using XOR with pseudo-random keystream.

Generates keystream using linear congruential generator (LCG).
XOR-based encryption/decryption (symmetric).
"""


def lcg_next(state: int, a_mult: int, c_add: int, modulus: int) -> int:
    """Linear congruential generator step."""
    return (a_mult * state + c_add) % modulus


def generate_keystream(seed: int, length: int) -> list[int]:
    """Generate keystream of given length using LCG."""
    stream: list[int] = []
    state: int = seed
    i: int = 0
    while i < length:
        state = lcg_next(state, 1103515245, 12345, 2147483648)
        byte_val: int = (state // 65536) % 256
        stream.append(byte_val)
        i = i + 1
    return stream


def xor_encrypt(plaintext: list[int], keystream: list[int]) -> list[int]:
    """XOR plaintext with keystream."""
    result: list[int] = []
    i: int = 0
    while i < len(plaintext):
        pv: int = plaintext[i]
        kv: int = keystream[i]
        xor_val: int = xor_bits(pv, kv)
        result.append(xor_val)
        i = i + 1
    return result


def xor_bits(a: int, b: int) -> int:
    """Bitwise XOR using arithmetic (works for 0-255)."""
    result: int = 0
    bit: int = 1
    pos: int = 0
    while pos < 8:
        a_bit: int = (a // bit) % 2
        b_bit: int = (b // bit) % 2
        if a_bit != b_bit:
            result = result + bit
        bit = bit * 2
        pos = pos + 1
    return result


def stream_encrypt(plaintext: list[int], seed: int) -> list[int]:
    """Full stream encryption: generate keystream then XOR."""
    ks: list[int] = generate_keystream(seed, len(plaintext))
    return xor_encrypt(plaintext, ks)


def stream_decrypt(ciphertext: list[int], seed: int) -> list[int]:
    """Decryption is same as encryption for XOR cipher."""
    ks: list[int] = generate_keystream(seed, len(ciphertext))
    return xor_encrypt(ciphertext, ks)


def lists_match(a: list[int], b: list[int]) -> int:
    """Check list equality."""
    if len(a) != len(b):
        return 0
    i: int = 0
    while i < len(a):
        va: int = a[i]
        vb: int = b[i]
        if va != vb:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test stream cipher."""
    ok: int = 0
    plain: list[int] = [72, 101, 108, 108, 111]
    enc: list[int] = stream_encrypt(plain, 42)
    dec: list[int] = stream_decrypt(enc, 42)
    if lists_match(dec, plain) == 1:
        ok = ok + 1
    if lists_match(enc, plain) == 0:
        ok = ok + 1
    x: int = xor_bits(170, 85)
    if x == 255:
        ok = ok + 1
    x2: int = xor_bits(255, 255)
    if x2 == 0:
        ok = ok + 1
    ks: list[int] = generate_keystream(0, 5)
    if len(ks) == 5:
        ok = ok + 1
    return ok
