"""Block cipher modes of operation (ECB and CBC) with simple XOR block cipher.

Operates on integer arrays. Block size is fixed at 4 integers.
Uses XOR-based block cipher for simplicity.
"""


def xor_byte(a: int, b: int) -> int:
    """XOR two integers (0-255) using arithmetic."""
    result: int = 0
    bit: int = 1
    pos: int = 0
    while pos < 8:
        ab: int = (a // bit) % 2
        bb: int = (b // bit) % 2
        if ab != bb:
            result = result + bit
        bit = bit * 2
        pos = pos + 1
    return result


def block_encrypt(block: list[int], cipher_key: list[int]) -> list[int]:
    """Encrypt a 4-element block by XOR with key."""
    result: list[int] = []
    i: int = 0
    while i < 4:
        bv: int = block[i]
        kv: int = cipher_key[i]
        xv: int = xor_byte(bv, kv)
        result.append(xv)
        i = i + 1
    return result


def block_decrypt(block: list[int], cipher_key: list[int]) -> list[int]:
    """Decrypt is same as encrypt for XOR."""
    return block_encrypt(block, cipher_key)


def ecb_encrypt(data: list[int], cipher_key: list[int]) -> list[int]:
    """ECB mode: encrypt each block independently."""
    result: list[int] = []
    i: int = 0
    while i < len(data):
        block: list[int] = [data[i], data[i + 1], data[i + 2], data[i + 3]]
        enc: list[int] = block_encrypt(block, cipher_key)
        j: int = 0
        while j < 4:
            ev: int = enc[j]
            result.append(ev)
            j = j + 1
        i = i + 4
    return result


def ecb_decrypt(data: list[int], cipher_key: list[int]) -> list[int]:
    """ECB mode decryption."""
    return ecb_encrypt(data, cipher_key)


def cbc_encrypt(data: list[int], cipher_key: list[int], iv: list[int]) -> list[int]:
    """CBC mode encryption: XOR with previous ciphertext block before encrypting."""
    result: list[int] = []
    prev: list[int] = [iv[0], iv[1], iv[2], iv[3]]
    i: int = 0
    while i < len(data):
        xored: list[int] = []
        j: int = 0
        while j < 4:
            dv: int = data[i + j]
            pv: int = prev[j]
            xv2: int = xor_byte(dv, pv)
            xored.append(xv2)
            j = j + 1
        enc: list[int] = block_encrypt(xored, cipher_key)
        k: int = 0
        while k < 4:
            ev: int = enc[k]
            result.append(ev)
            k = k + 1
        prev = enc
        i = i + 4
    return result


def cbc_decrypt(data: list[int], cipher_key: list[int], iv: list[int]) -> list[int]:
    """CBC mode decryption."""
    result: list[int] = []
    prev: list[int] = [iv[0], iv[1], iv[2], iv[3]]
    i: int = 0
    while i < len(data):
        block: list[int] = [data[i], data[i + 1], data[i + 2], data[i + 3]]
        dec: list[int] = block_decrypt(block, cipher_key)
        j: int = 0
        while j < 4:
            dv: int = dec[j]
            pv: int = prev[j]
            xv3: int = xor_byte(dv, pv)
            result.append(xv3)
            j = j + 1
        prev = block
        i = i + 4
    return result


def lists_identical(a: list[int], b: list[int]) -> int:
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
    """Test block cipher modes."""
    ok: int = 0
    cipher_key: list[int] = [0, 0, 0, 0]
    plain: list[int] = [1, 2, 3, 4, 5, 6, 7, 8]
    ecb_enc: list[int] = ecb_encrypt(plain, cipher_key)
    ecb_dec: list[int] = ecb_decrypt(ecb_enc, cipher_key)
    if lists_identical(ecb_dec, plain) == 1:
        ok = ok + 1
    real_key: list[int] = [10, 20, 30, 40]
    ecb2: list[int] = ecb_encrypt(plain, real_key)
    ecb2d: list[int] = ecb_decrypt(ecb2, real_key)
    if lists_identical(ecb2d, plain) == 1:
        ok = ok + 1
    iv: list[int] = [1, 1, 1, 1]
    cbc_enc: list[int] = cbc_encrypt(plain, real_key, iv)
    cbc_dec: list[int] = cbc_decrypt(cbc_enc, real_key, iv)
    if lists_identical(cbc_dec, plain) == 1:
        ok = ok + 1
    xv: int = xor_byte(0, 0)
    if xv == 0:
        ok = ok + 1
    xv2: int = xor_byte(255, 0)
    if xv2 == 255:
        ok = ok + 1
    return ok
