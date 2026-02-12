"""Cryptography: Base64 encoding/decoding.
Tests: 6-bit grouping, padding, table lookup.
"""
from typing import Dict, List, Tuple

def base64_encode(data: str) -> str:
    table: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    result: List[str] = []
    bytes_list: List[int] = []
    i: int = 0
    while i < len(data):
        bytes_list.append(ord(data[i]))
        i += 1
    i = 0
    n: int = len(bytes_list)
    while i < n:
        b0: int = bytes_list[i]
        b1: int = 0
        b2: int = 0
        if i + 1 < n:
            b1 = bytes_list[i + 1]
        if i + 2 < n:
            b2 = bytes_list[i + 2]
        triple: int = (b0 << 16) | (b1 << 8) | b2
        result.append(table[(triple >> 18) & 0x3F])
        result.append(table[(triple >> 12) & 0x3F])
        if i + 1 < n:
            result.append(table[(triple >> 6) & 0x3F])
        else:
            result.append("=")
        if i + 2 < n:
            result.append(table[triple & 0x3F])
        else:
            result.append("=")
        i += 3
    return "".join(result)

def hex_encode(data: str) -> str:
    hex_chars: str = "0123456789abcdef"
    result: List[str] = []
    i: int = 0
    while i < len(data):
        byte_val: int = ord(data[i])
        result.append(hex_chars[(byte_val >> 4) & 0xF])
        result.append(hex_chars[byte_val & 0xF])
        i += 1
    return "".join(result)

def test_encoding() -> bool:
    ok: bool = True
    b64: str = base64_encode("Hi")
    if len(b64) != 4:
        ok = False
    hx: str = hex_encode("AB")
    if hx != "4142":
        ok = False
    return ok
