from typing import List, Tuple

def mod_pow_i(base: int, exp: int, mod: int) -> int:
    result: int = 1
    b: int = base % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * b) % mod
        exp = exp >> 1
        b = (b * b) % mod
    return result

def hash_data(data: List[int]) -> int:
    h: int = 0x811C9DC5
    for b in data:
        h = h ^ b
        h = (h * 0x01000193) & 0xFFFFFFFF
    return h

def sign_value(msg_hash: int, priv_key: int, n: int) -> int:
    result: int = 1
    b: int = msg_hash % n
    exp: int = priv_key
    while exp > 0:
        if exp % 2 == 1:
            result = (result * b) % n
        exp = exp >> 1
        b = (b * b) % n
    return result

def verify_value(sig: int, pub_key: int, n: int) -> int:
    result: int = 1
    b: int = sig % n
    exp: int = pub_key
    while exp > 0:
        if exp % 2 == 1:
            result = (result * b) % n
        exp = exp >> 1
        b = (b * b) % n
    return result

def sign_batch(hashes: List[int], priv_key: int, n: int) -> List[int]:
    sigs: List[int] = []
    for h in hashes:
        result: int = 1
        b: int = h % n
        exp: int = priv_key
        while exp > 0:
            if exp % 2 == 1:
                result = (result * b) % n
            exp = exp >> 1
            b = (b * b) % n
        sigs.append(result)
    return sigs
