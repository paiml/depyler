"""Cryptography: Caesar and substitution ciphers.
Tests: character shifting, modular arithmetic, encryption/decryption.
"""
from typing import Dict, List, Tuple

def caesar_encrypt(text: str, shift: int) -> str:
    result: List[str] = []
    i: int = 0
    while i < len(text):
        if text[i] >= "a" and text[i] <= "z":
            code: int = (ord(text[i]) - ord("a") + shift) % 26 + ord("a")
            result.append(chr(code))
        elif text[i] >= "A" and text[i] <= "Z":
            code2: int = (ord(text[i]) - ord("A") + shift) % 26 + ord("A")
            result.append(chr(code2))
        else:
            result.append(text[i])
        i += 1
    return "".join(result)

def caesar_decrypt(text: str, shift: int) -> str:
    return caesar_encrypt(text, 26 - (shift % 26))

def atbash_cipher(text: str) -> str:
    result: List[str] = []
    i: int = 0
    while i < len(text):
        if text[i] >= "a" and text[i] <= "z":
            code: int = ord("z") - (ord(text[i]) - ord("a"))
            result.append(chr(code))
        elif text[i] >= "A" and text[i] <= "Z":
            code2: int = ord("Z") - (ord(text[i]) - ord("A"))
            result.append(chr(code2))
        else:
            result.append(text[i])
        i += 1
    return "".join(result)

def affine_encrypt(text: str, a: int, b: int) -> str:
    result: List[str] = []
    i: int = 0
    while i < len(text):
        if text[i] >= "a" and text[i] <= "z":
            x: int = ord(text[i]) - ord("a")
            code: int = (a * x + b) % 26 + ord("a")
            result.append(chr(code))
        else:
            result.append(text[i])
        i += 1
    return "".join(result)

def test_ciphers() -> bool:
    ok: bool = True
    enc: str = caesar_encrypt("hello", 3)
    dec: str = caesar_decrypt(enc, 3)
    if dec != "hello":
        ok = False
    atb: str = atbash_cipher(atbash_cipher("hello"))
    if atb != "hello":
        ok = False
    return ok
