from typing import List, Tuple

def create_bloom(size: int) -> List[int]:
    bits: List[int] = []
    for i in range(size):
        bits.append(0)
    return bits

def bloom_add(bits: List[int], value: int) -> List[int]:
    size: int = len(bits)
    result: List[int] = []
    for b in bits:
        result.append(b)
    h1: int = ((value * 0x5BD1E995) ^ (value >> 13)) & 0x7FFFFFFF
    h2: int = ((value * 0x85EBCA6B) ^ (value >> 16)) & 0x7FFFFFFF
    h3: int = ((value * 0xCC9E2D51) ^ (value >> 11)) & 0x7FFFFFFF
    result[h1 % size] = 1
    result[h2 % size] = 1
    result[h3 % size] = 1
    return result

def bloom_check(bits: List[int], value: int) -> bool:
    size: int = len(bits)
    h1: int = ((value * 0x5BD1E995) ^ (value >> 13)) & 0x7FFFFFFF
    h2: int = ((value * 0x85EBCA6B) ^ (value >> 16)) & 0x7FFFFFFF
    h3: int = ((value * 0xCC9E2D51) ^ (value >> 11)) & 0x7FFFFFFF
    if bits[h1 % size] == 0:
        return False
    if bits[h2 % size] == 0:
        return False
    if bits[h3 % size] == 0:
        return False
    return True

def bloom_union(a: List[int], b: List[int]) -> List[int]:
    result: List[int] = []
    for i in range(len(a)):
        if a[i] == 1 or b[i] == 1:
            result.append(1)
        else:
            result.append(0)
    return result

def bloom_fp_rate(bits: List[int]) -> float:
    ones: int = 0
    for b in bits:
        if b == 1:
            ones = ones + 1
    ratio: float = float(ones) / float(len(bits))
    return ratio * ratio * ratio
