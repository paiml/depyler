"""Simple cipher algorithms: Caesar, XOR, ROT13.

Tests: character manipulation via ordinals, string building.
"""


def caesar_encrypt(text: str, shift: int) -> str:
    """Caesar cipher encryption on lowercase letters."""
    result: str = ""
    for ch in text:
        code: int = ord(ch)
        if code >= 97 and code <= 122:
            shifted: int = ((code - 97 + shift) % 26) + 97
            result = result + chr(shifted)
        else:
            result = result + ch
    return result


def caesar_decrypt(text: str, shift: int) -> str:
    """Caesar cipher decryption."""
    return caesar_encrypt(text, 26 - (shift % 26))


def rot13(text: str) -> str:
    """ROT13 cipher."""
    return caesar_encrypt(text, 13)


def xor_cipher_values(values: list[int], key: int) -> list[int]:
    """XOR cipher on integer values."""
    result: list[int] = []
    for v in values:
        result.append(v ^ key)
    return result


def atbash_encrypt(text: str) -> str:
    """Atbash cipher: a->z, b->y, etc."""
    result: str = ""
    for ch in text:
        code: int = ord(ch)
        if code >= 97 and code <= 122:
            new_code: int = 122 - (code - 97)
            result = result + chr(new_code)
        else:
            result = result + ch
    return result


def test_module() -> int:
    """Test cipher operations."""
    ok: int = 0

    e: str = caesar_encrypt("abc", 3)
    if e == "def":
        ok += 1

    d: str = caesar_decrypt("def", 3)
    if d == "abc":
        ok += 1

    r: str = rot13("hello")
    r2: str = rot13(r)
    if r2 == "hello":
        ok += 1

    vals: list[int] = xor_cipher_values([1, 2, 3], 255)
    vals2: list[int] = xor_cipher_values(vals, 255)
    if vals2 == [1, 2, 3]:
        ok += 1

    a: str = atbash_encrypt("abc")
    if a == "zyx":
        ok += 1

    return ok
