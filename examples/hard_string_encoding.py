"""String encoding and decoding: Caesar cipher variants and simple encodings."""


def caesar_encode(text: str, shift: int) -> str:
    """Encode a string using Caesar cipher on lowercase letters."""
    result: str = ""
    i: int = 0
    while i < len(text):
        ch: str = text[i]
        code: int = ord(ch)
        if code >= 97 and code <= 122:
            new_code: int = ((code - 97 + shift) % 26) + 97
            result = result + chr(new_code)
        else:
            result = result + ch
        i = i + 1
    return result


def caesar_decode(text: str, shift: int) -> str:
    """Decode a Caesar cipher encoded string."""
    decoded: str = caesar_encode(text, 26 - (shift % 26))
    return decoded


def count_char_types(text: str) -> list[int]:
    """Count lowercase, uppercase, digits, and other characters.
    Returns [lower, upper, digit, other]."""
    lower: int = 0
    upper: int = 0
    digit: int = 0
    other: int = 0
    i: int = 0
    while i < len(text):
        code: int = ord(text[i])
        if code >= 97 and code <= 122:
            lower = lower + 1
        elif code >= 65 and code <= 90:
            upper = upper + 1
        elif code >= 48 and code <= 57:
            digit = digit + 1
        else:
            other = other + 1
        i = i + 1
    result: list[int] = [lower, upper, digit, other]
    return result


def atbash_encode(text: str) -> str:
    """Encode using Atbash cipher (reverse alphabet) for lowercase."""
    result: str = ""
    i: int = 0
    while i < len(text):
        code: int = ord(text[i])
        if code >= 97 and code <= 122:
            new_code: int = 122 - (code - 97)
            result = result + chr(new_code)
        else:
            result = result + text[i]
        i = i + 1
    return result


def test_module() -> int:
    """Test string encoding functions."""
    ok: int = 0

    if caesar_encode("abc", 1) == "bcd":
        ok = ok + 1

    if caesar_encode("xyz", 3) == "abc":
        ok = ok + 1

    if caesar_decode("bcd", 1) == "abc":
        ok = ok + 1

    counts: list[int] = count_char_types("aB1!")
    if counts[0] == 1 and counts[1] == 1 and counts[2] == 1 and counts[3] == 1:
        ok = ok + 1

    if atbash_encode("abc") == "zyx":
        ok = ok + 1

    if atbash_encode("z") == "a":
        ok = ok + 1

    return ok
