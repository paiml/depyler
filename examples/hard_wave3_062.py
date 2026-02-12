"""Cryptography: Vigenere cipher.
Tests: repeating key, polyalphabetic substitution.
"""
from typing import Dict, List, Tuple

def vigenere_encrypt(text: str, vkey: str) -> str:
    result: List[str] = []
    klen: int = len(vkey)
    ki: int = 0
    i: int = 0
    while i < len(text):
        if text[i] >= "a" and text[i] <= "z":
            k: int = ord(vkey[ki % klen]) - ord("a")
            code: int = (ord(text[i]) - ord("a") + k) % 26 + ord("a")
            result.append(chr(code))
            ki += 1
        elif text[i] >= "A" and text[i] <= "Z":
            k2: int = ord(vkey[ki % klen]) - ord("a")
            code2: int = (ord(text[i]) - ord("A") + k2) % 26 + ord("A")
            result.append(chr(code2))
            ki += 1
        else:
            result.append(text[i])
        i += 1
    return "".join(result)

def vigenere_decrypt(text: str, vkey: str) -> str:
    result: List[str] = []
    klen: int = len(vkey)
    ki: int = 0
    i: int = 0
    while i < len(text):
        if text[i] >= "a" and text[i] <= "z":
            k: int = ord(vkey[ki % klen]) - ord("a")
            code: int = (ord(text[i]) - ord("a") - k + 26) % 26 + ord("a")
            result.append(chr(code))
            ki += 1
        elif text[i] >= "A" and text[i] <= "Z":
            k2: int = ord(vkey[ki % klen]) - ord("a")
            code2: int = (ord(text[i]) - ord("A") - k2 + 26) % 26 + ord("A")
            result.append(chr(code2))
            ki += 1
        else:
            result.append(text[i])
        i += 1
    return "".join(result)

def test_vigenere() -> bool:
    ok: bool = True
    enc: str = vigenere_encrypt("attackatdawn", "lemon")
    dec: str = vigenere_decrypt(enc, "lemon")
    if dec != "attackatdawn":
        ok = False
    return ok
