"""Cryptography: Bit manipulation suite.
Tests: popcount, bit reversal, byte swap, rotation.
"""
from typing import Dict, List, Tuple

def popcount(x: int) -> int:
    v: int = x & 0xFFFFFFFF
    count: int = 0
    while v > 0:
        v = v & (v - 1)
        count += 1
    return count

def bit_reverse_32(x: int) -> int:
    v: int = x & 0xFFFFFFFF
    result: int = 0
    i: int = 0
    while i < 32:
        result = (result << 1) | (v & 1)
        v = v >> 1
        i += 1
    return result & 0xFFFFFFFF

def byte_swap_32(x: int) -> int:
    v: int = x & 0xFFFFFFFF
    b0: int = (v >> 24) & 0xFF
    b1: int = (v >> 16) & 0xFF
    b2: int = (v >> 8) & 0xFF
    b3: int = v & 0xFF
    return (b3 << 24) | (b2 << 16) | (b1 << 8) | b0

def rotate_left_32(x: int, n: int) -> int:
    v: int = x & 0xFFFFFFFF
    n = n % 32
    return ((v << n) | (v >> (32 - n))) & 0xFFFFFFFF

def rotate_right_32(x: int, n: int) -> int:
    v: int = x & 0xFFFFFFFF
    n = n % 32
    return ((v >> n) | (v << (32 - n))) & 0xFFFFFFFF

def leading_zeros(x: int) -> int:
    if x == 0:
        return 32
    v: int = x & 0xFFFFFFFF
    count: int = 0
    bit: int = 31
    while bit >= 0:
        if (v >> bit) & 1 == 1:
            return count
        count += 1
        bit -= 1
    return 32

def test_bits() -> bool:
    ok: bool = True
    if popcount(0xFF) != 8:
        ok = False
    if popcount(0) != 0:
        ok = False
    bs: int = byte_swap_32(0x01020304)
    if bs != 0x04030201:
        ok = False
    return ok
