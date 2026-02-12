"""Cryptography: XOR cipher operations.
Tests: repeating key XOR, single-byte XOR, XOR properties.
"""
from typing import Dict, List, Tuple

def xor_encrypt(data: List[int], xkey: List[int]) -> List[int]:
    """XOR encrypt data with repeating key."""
    result: List[int] = []
    klen: int = len(xkey)
    i: int = 0
    while i < len(data):
        result.append(data[i] ^ xkey[i % klen])
        i += 1
    return result

def xor_decrypt(data: List[int], xkey: List[int]) -> List[int]:
    """XOR decrypt (same as encrypt due to XOR properties)."""
    return xor_encrypt(data, xkey)

def single_byte_xor(data: List[int], byte_val: int) -> List[int]:
    """XOR all bytes with single byte."""
    result: List[int] = []
    for d in data:
        result.append(d ^ byte_val)
    return result

def xor_strings_to_ints(text: str) -> List[int]:
    """Convert string to list of int codes."""
    result: List[int] = []
    i: int = 0
    while i < len(text):
        result.append(ord(text[i]))
        i += 1
    return result

def ints_to_string(codes: List[int]) -> str:
    """Convert list of int codes back to string."""
    result: List[str] = []
    for c in codes:
        result.append(chr(c))
    return "".join(result)

def test_xor() -> bool:
    ok: bool = True
    data: List[int] = [72, 101, 108, 108, 111]
    enc: List[int] = xor_encrypt(data, [42])
    dec: List[int] = xor_decrypt(enc, [42])
    if dec[0] != 72:
        ok = False
    if dec[4] != 111:
        ok = False
    return ok
