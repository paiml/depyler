from typing import List, Tuple

def mod_pow_dh(b: int, exp: int, modulus: int) -> int:
    result: int = 1
    base: int = b % modulus
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % modulus
        exp = exp >> 1
        base = (base * base) % modulus
    return result

def generate_public(g: int, private: int, p: int) -> int:
    result: int = 1
    base: int = g % p
    exp: int = private
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % p
        exp = exp >> 1
        base = (base * base) % p
    return result

def compute_shared(other_pub: int, private: int, p: int) -> int:
    result: int = 1
    base: int = other_pub % p
    exp: int = private
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % p
        exp = exp >> 1
        base = (base * base) % p
    return result

def derive_bytes(shared: int, length: int) -> List[int]:
    result: List[int] = []
    val: int = shared
    for i in range(length):
        val = ((val * 6364136223846793005) + 1) & 0xFFFFFFFF
        result.append(val & 0xFF)
    return result

def verify_exchange(a: int, b: int) -> bool:
    return a == b
