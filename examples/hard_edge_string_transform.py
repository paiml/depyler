"""String transforms: case conversion, reverse, rotate, strip."""


def to_upper(s: str) -> str:
    """Convert string to uppercase."""
    result: str = ""
    i: int = 0
    while i < len(s):
        code: int = ord(s[i])
        if code >= 97 and code <= 122:
            result = result + chr(code - 32)
        else:
            result = result + s[i]
        i = i + 1
    return result


def to_lower(s: str) -> str:
    """Convert string to lowercase."""
    result: str = ""
    i: int = 0
    while i < len(s):
        code: int = ord(s[i])
        if code >= 65 and code <= 90:
            result = result + chr(code + 32)
        else:
            result = result + s[i]
        i = i + 1
    return result


def swap_case(s: str) -> str:
    """Swap upper and lower case characters."""
    result: str = ""
    i: int = 0
    while i < len(s):
        code: int = ord(s[i])
        if code >= 65 and code <= 90:
            result = result + chr(code + 32)
        elif code >= 97 and code <= 122:
            result = result + chr(code - 32)
        else:
            result = result + s[i]
        i = i + 1
    return result


def capitalize_first(s: str) -> str:
    """Capitalize first character, lowercase rest."""
    if len(s) == 0:
        return ""
    first_code: int = ord(s[0])
    result: str = ""
    if first_code >= 97 and first_code <= 122:
        result = chr(first_code - 32)
    else:
        result = s[0]
    i: int = 1
    while i < len(s):
        code: int = ord(s[i])
        if code >= 65 and code <= 90:
            result = result + chr(code + 32)
        else:
            result = result + s[i]
        i = i + 1
    return result


def reverse_string(s: str) -> str:
    """Reverse a string."""
    result: str = ""
    i: int = len(s) - 1
    while i >= 0:
        result = result + s[i]
        i = i - 1
    return result


def rotate_left(s: str, n: int) -> str:
    """Rotate string left by n positions."""
    slen: int = len(s)
    if slen == 0:
        return ""
    shift: int = n % slen
    result: str = ""
    i: int = shift
    while i < slen:
        result = result + s[i]
        i = i + 1
    i = 0
    while i < shift:
        result = result + s[i]
        i = i + 1
    return result


def rotate_right(s: str, n: int) -> str:
    """Rotate string right by n positions."""
    slen: int = len(s)
    if slen == 0:
        return ""
    shift: int = n % slen
    return rotate_left(s, slen - shift)


def is_palindrome(s: str) -> int:
    """Return 1 if s is a palindrome, 0 otherwise."""
    lo: int = 0
    hi: int = len(s) - 1
    while lo < hi:
        if s[lo] != s[hi]:
            return 0
        lo = lo + 1
        hi = hi - 1
    return 1


def strip_char(s: str, ch_code: int) -> str:
    """Remove all occurrences of character with given code from s."""
    result: str = ""
    i: int = 0
    while i < len(s):
        c: int = ord(s[i])
        if c != ch_code:
            result = result + s[i]
        i = i + 1
    return result


def replace_char(s: str, old_code: int, new_code: int) -> str:
    """Replace all occurrences of old_code char with new_code char."""
    result: str = ""
    i: int = 0
    while i < len(s):
        c: int = ord(s[i])
        if c == old_code:
            result = result + chr(new_code)
        else:
            result = result + s[i]
        i = i + 1
    return result


def test_module() -> int:
    """Test all string transform functions."""
    passed: int = 0
    if to_upper("hello") == "HELLO":
        passed = passed + 1
    if to_lower("HELLO") == "hello":
        passed = passed + 1
    if swap_case("Hello") == "hELLO":
        passed = passed + 1
    if capitalize_first("hello") == "Hello":
        passed = passed + 1
    if reverse_string("abcd") == "dcba":
        passed = passed + 1
    if reverse_string("") == "":
        passed = passed + 1
    if rotate_left("abcde", 2) == "cdeab":
        passed = passed + 1
    if rotate_right("abcde", 2) == "deabc":
        passed = passed + 1
    if is_palindrome("racecar") == 1:
        passed = passed + 1
    if is_palindrome("hello") == 0:
        passed = passed + 1
    if strip_char("abcabc", 98) == "acac":
        passed = passed + 1
    if replace_char("hello", 108, 114) == "herro":
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
