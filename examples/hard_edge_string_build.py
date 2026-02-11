"""Building strings character by character with various patterns."""


def build_repeated(ch: str, n: int) -> str:
    """Build a string by repeating a character n times."""
    result: str = ""
    i: int = 0
    while i < n:
        result = result + ch
        i = i + 1
    return result


def build_alphabet(n: int) -> str:
    """Build first n letters of alphabet."""
    result: str = ""
    i: int = 0
    while i < n:
        code: int = 97 + i
        if code > 122:
            code = 122
        result = result + chr(code)
        i = i + 1
    return result


def build_digits(n: int) -> str:
    """Build string of digits 0 to n-1 (mod 10)."""
    result: str = ""
    i: int = 0
    while i < n:
        d: int = i % 10
        result = result + chr(48 + d)
        i = i + 1
    return result


def build_zigzag(s: str) -> str:
    """Build string alternating upper/lower case by code manipulation."""
    result: str = ""
    i: int = 0
    while i < len(s):
        code: int = ord(s[i])
        if i % 2 == 0:
            if code >= 97 and code <= 122:
                code = code - 32
        else:
            if code >= 65 and code <= 90:
                code = code + 32
        result = result + chr(code)
        i = i + 1
    return result


def build_reversed(s: str) -> str:
    """Build a reversed copy of string."""
    result: str = ""
    i: int = len(s) - 1
    while i >= 0:
        result = result + s[i]
        i = i - 1
    return result


def build_with_separator(words: list[str], sep: str) -> str:
    """Join words with separator."""
    if len(words) == 0:
        return ""
    result: str = words[0]
    i: int = 1
    while i < len(words):
        word: str = words[i]
        result = result + sep + word
        i = i + 1
    return result


def build_padded(s: str, width: int, pad_char: str) -> str:
    """Pad string on the right to given width."""
    result: str = s
    while len(result) < width:
        result = result + pad_char
    return result


def build_left_padded(s: str, width: int, pad_char: str) -> str:
    """Pad string on the left to given width."""
    result: str = s
    while len(result) < width:
        result = pad_char + result
    return result


def build_int_to_string(n: int) -> str:
    """Convert integer to string representation."""
    if n == 0:
        return "0"
    is_neg: int = 0
    val: int = n
    if val < 0:
        is_neg = 1
        val = 0 - val
    result: str = ""
    while val > 0:
        d: int = val % 10
        result = chr(48 + d) + result
        val = val // 10
    if is_neg == 1:
        result = "-" + result
    return result


def test_module() -> int:
    """Test all string building functions."""
    passed: int = 0
    r1: str = build_repeated("x", 5)
    if r1 == "xxxxx":
        passed = passed + 1
    r2: str = build_repeated("a", 0)
    if r2 == "":
        passed = passed + 1
    r3: str = build_alphabet(5)
    if r3 == "abcde":
        passed = passed + 1
    r4: str = build_digits(5)
    if r4 == "01234":
        passed = passed + 1
    r5: str = build_zigzag("hello")
    if r5 == "HeLlO":
        passed = passed + 1
    r6: str = build_reversed("abcd")
    if r6 == "dcba":
        passed = passed + 1
    r7: str = build_reversed("")
    if r7 == "":
        passed = passed + 1
    words: list[str] = ["one", "two", "three"]
    r8: str = build_with_separator(words, "-")
    if r8 == "one-two-three":
        passed = passed + 1
    r9: str = build_padded("hi", 5, ".")
    if r9 == "hi...":
        passed = passed + 1
    r10: str = build_left_padded("42", 5, "0")
    if r10 == "00042":
        passed = passed + 1
    r11: str = build_int_to_string(12345)
    if r11 == "12345":
        passed = passed + 1
    r12: str = build_int_to_string(0)
    if r12 == "0":
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
