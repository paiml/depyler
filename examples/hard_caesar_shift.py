"""Caesar cipher with shift operations for encoding and decoding."""


def shift_char(ch: str, amount: int) -> str:
    """Shift a single lowercase letter by amount positions."""
    base: int = ord("a")
    code: int = ord(ch)
    if code < base or code > ord("z"):
        return ch
    shifted: int = ((code - base + amount) % 26) + base
    return chr(shifted)


def caesar_encrypt(text: str, shift: int) -> str:
    """Encrypt text using Caesar cipher with given shift."""
    result: str = ""
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        encrypted: str = shift_char(ch, shift)
        result = result + encrypted
        i = i + 1
    return result


def caesar_decrypt(text: str, shift: int) -> str:
    """Decrypt Caesar cipher text by shifting in reverse."""
    reverse_shift: int = 26 - (shift % 26)
    result: str = caesar_encrypt(text, reverse_shift)
    return result


def caesar_brute_force(cipher: str) -> list[str]:
    """Try all 26 possible shifts and return results."""
    results: list[str] = []
    attempt: int = 0
    while attempt < 26:
        decoded: str = caesar_decrypt(cipher, attempt)
        results.append(decoded)
        attempt = attempt + 1
    return results


def test_module() -> int:
    """Test Caesar cipher operations."""
    passed: int = 0

    r1: str = shift_char("a", 3)
    if r1 == "d":
        passed = passed + 1

    r2: str = shift_char("z", 1)
    if r2 == "a":
        passed = passed + 1

    r3: str = caesar_encrypt("abc", 3)
    if r3 == "def":
        passed = passed + 1

    r4: str = caesar_encrypt("xyz", 3)
    if r4 == "abc":
        passed = passed + 1

    r5: str = caesar_decrypt("def", 3)
    if r5 == "abc":
        passed = passed + 1

    roundtrip: str = caesar_decrypt(caesar_encrypt("hello", 7), 7)
    if roundtrip == "hello":
        passed = passed + 1

    brute: list[str] = caesar_brute_force("def")
    if brute[3] == "abc":
        passed = passed + 1

    r8: str = shift_char("5", 3)
    if r8 == "5":
        passed = passed + 1

    return passed
