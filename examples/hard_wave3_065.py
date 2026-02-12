"""Cryptography: CRC checksums.
Tests: bit-level operations, table generation, polynomial division.
"""
from typing import Dict, List, Tuple

def crc32_compute(data: str) -> int:
    poly: int = 0xEDB88320
    table: List[int] = []
    i: int = 0
    while i < 256:
        crc: int = i
        j: int = 0
        while j < 8:
            if (crc & 1) != 0:
                crc = (crc >> 1) ^ poly
            else:
                crc = crc >> 1
            j += 1
        table.append(crc & 0xFFFFFFFF)
        i += 1
    crc_val: int = 0xFFFFFFFF
    i = 0
    while i < len(data):
        byte_val: int = ord(data[i]) & 0xFF
        idx: int = (crc_val ^ byte_val) & 0xFF
        crc_val = (crc_val >> 8) ^ table[idx]
        i += 1
    return (crc_val ^ 0xFFFFFFFF) & 0xFFFFFFFF

def adler32_compute(data: str) -> int:
    a: int = 1
    b: int = 0
    mod: int = 65521
    i: int = 0
    while i < len(data):
        a = (a + ord(data[i])) % mod
        b = (b + a) % mod
        i += 1
    return (b << 16) | a

def fletcher16(data: List[int]) -> int:
    sum1: int = 0
    sum2: int = 0
    for byte_val in data:
        sum1 = (sum1 + byte_val) % 255
        sum2 = (sum2 + sum1) % 255
    return (sum2 << 8) | sum1

def test_crc() -> bool:
    ok: bool = True
    c: int = crc32_compute("hello")
    if c == 0:
        ok = False
    a: int = adler32_compute("hello")
    if a == 0:
        ok = False
    f: int = fletcher16([1, 2, 3, 4])
    if f == 0:
        ok = False
    return ok
