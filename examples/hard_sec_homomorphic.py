from typing import List, Tuple

def he_encrypt(value: int, key: int, mod: int) -> int:
    return (value + key) % mod

def he_decrypt(cipher: int, key: int, mod: int) -> int:
    return (cipher - key + mod) % mod

def he_add(c1: int, c2: int, mod: int) -> int:
    return (c1 + c2) % mod

def he_sum(cts: List[int], mod: int) -> int:
    result: int = 0
    for c in cts:
        result = (result + c) % mod
    return result

def batch_encrypt(values: List[int], key: int, mod: int) -> List[int]:
    result: List[int] = []
    for v in values:
        result.append((v + key) % mod)
    return result

def batch_decrypt(cts: List[int], key: int, mod: int) -> List[int]:
    result: List[int] = []
    for c in cts:
        result.append((c - key + mod) % mod)
    return result
