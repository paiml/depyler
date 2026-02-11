"""
Comprehensive test of Python string operations transpilation to Rust.

This example demonstrates how Depyler transpiles Python string operations
to Rust equivalents using pure functions with full type annotations.

Note: All character classification done inline via ord() ranges to avoid
cross-function String/str mismatch in transpiled Rust.
"""


def test_ascii_lowercase() -> str:
    """Test accessing lowercase ASCII letters."""
    lowercase: str = "abcdefghijklmnopqrstuvwxyz"
    return lowercase


def test_ascii_uppercase() -> str:
    """Test accessing uppercase ASCII letters."""
    uppercase: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    return uppercase


def test_ascii_letters() -> str:
    """Test accessing all ASCII letters."""
    lowercase: str = "abcdefghijklmnopqrstuvwxyz"
    uppercase: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    letters: str = lowercase + uppercase
    return letters


def test_digits() -> str:
    """Test accessing digit characters."""
    digits: str = "0123456789"
    return digits


def test_hexdigits() -> str:
    """Test accessing hexadecimal digits."""
    hexdigits: str = "0123456789abcdefABCDEF"
    return hexdigits


def test_octdigits() -> str:
    """Test accessing octal digits."""
    octdigits: str = "01234567"
    return octdigits


def test_whitespace() -> str:
    """Test accessing whitespace characters."""
    whitespace: str = " \t\n\r"
    return whitespace


def is_letter_code(code: int) -> bool:
    """Check if character code is ASCII letter (A-Z or a-z)."""
    is_upper: bool = code >= 65 and code <= 90
    is_lower: bool = code >= 97 and code <= 122
    return is_upper or is_lower


def is_digit_code(code: int) -> bool:
    """Check if character code is digit (0-9)."""
    return code >= 48 and code <= 57


def is_whitespace_code(code: int) -> bool:
    """Check if character code is whitespace."""
    if code == 32:
        return True
    if code == 9:
        return True
    if code == 10:
        return True
    if code == 13:
        return True
    return False


def capitalize_words(text: str) -> str:
    """Capitalize first letter of each word."""
    words: list[str] = text.split()
    capitalized: list[str] = []
    for word in words:
        if len(word) > 0:
            first_char: str = word[0].upper()
            rest: str = word[1:].lower()
            cap_word: str = first_char + rest
            capitalized.append(cap_word)
    return " ".join(capitalized)


def to_title_case(text: str) -> str:
    """Convert to title case."""
    return capitalize_words(text)


def swap_case(text: str) -> str:
    """Swap uppercase to lowercase and vice versa."""
    result: str = ""
    for ch in text:
        if ch.isupper():
            result = result + ch.lower()
        elif ch.islower():
            result = result + ch.upper()
        else:
            result = result + ch
    return result


def count_letters(text: str) -> int:
    """Count number of letters in text using ord ranges."""
    count: int = 0
    for ch in text:
        code: int = ord(ch)
        if is_letter_code(code):
            count = count + 1
    return count


def count_digits_in_text(text: str) -> int:
    """Count number of digits in text using ord ranges."""
    count: int = 0
    for ch in text:
        code: int = ord(ch)
        if is_digit_code(code):
            count = count + 1
    return count


def count_whitespace(text: str) -> int:
    """Count whitespace characters using ord ranges."""
    count: int = 0
    for ch in text:
        code: int = ord(ch)
        if is_whitespace_code(code):
            count = count + 1
    return count


def remove_whitespace(text: str) -> str:
    """Remove all whitespace from text."""
    result: str = ""
    for ch in text:
        code: int = ord(ch)
        if not is_whitespace_code(code):
            result = result + ch
    return result


def keep_only_letters(text: str) -> str:
    """Keep only letters, remove everything else."""
    result: str = ""
    for ch in text:
        code: int = ord(ch)
        if is_letter_code(code):
            result = result + ch
    return result


def keep_only_digits(text: str) -> str:
    """Keep only digits, remove everything else."""
    result: str = ""
    for ch in text:
        code: int = ord(ch)
        if is_digit_code(code):
            result = result + ch
    return result


def keep_alphanumeric(text: str) -> str:
    """Keep only alphanumeric characters."""
    result: str = ""
    for ch in text:
        code: int = ord(ch)
        is_letter: bool = is_letter_code(code)
        is_dig: bool = is_digit_code(code)
        if is_letter or is_dig:
            result = result + ch
    return result


def caesar_cipher(text: str, shift: int) -> str:
    """Simple Caesar cipher."""
    result: str = ""
    for ch in text:
        code: int = ord(ch)
        is_upper: bool = code >= 65 and code <= 90
        is_lower: bool = code >= 97 and code <= 122
        if is_upper:
            off: int = 65
            shifted: int = (code - off + shift) % 26
            new_char: str = chr(off + shifted)
            result = result + new_char
        elif is_lower:
            off2: int = 97
            shifted2: int = (code - off2 + shift) % 26
            new_char2: str = chr(off2 + shifted2)
            result = result + new_char2
        else:
            result = result + ch
    return result


def reverse_string(text: str) -> str:
    """Reverse a string."""
    result: str = ""
    for i in range(len(text) - 1, -1, -1):
        result = result + text[i]
    return result


def is_palindrome(text: str) -> bool:
    """Check if text is palindrome (ignoring case and spaces)."""
    normalized: str = ""
    lower_text: str = text.lower()
    for ch in lower_text:
        code: int = ord(ch)
        is_letter: bool = is_letter_code(code)
        is_dig: bool = is_digit_code(code)
        if is_letter or is_dig:
            normalized = normalized + ch
    left: int = 0
    right: int = len(normalized) - 1
    while left < right:
        if normalized[left] != normalized[right]:
            return False
        left = left + 1
        right = right - 1
    return True


def count_vowels(text: str) -> int:
    """Count vowels in text."""
    vowels: str = "aeiouAEIOU"
    count: int = 0
    for ch in text:
        if ch in vowels:
            count = count + 1
    return count


def count_consonants(text: str) -> int:
    """Count consonants in text."""
    vowels: str = "aeiouAEIOU"
    count: int = 0
    for ch in text:
        code: int = ord(ch)
        if is_letter_code(code):
            if ch not in vowels:
                count = count + 1
    return count


def test_module() -> int:
    """Run all string module tests and return count of passed."""
    passed: int = 0

    # Test constants
    lowercase: str = test_ascii_lowercase()
    if len(lowercase) == 26:
        passed += 1

    uppercase: str = test_ascii_uppercase()
    if len(uppercase) == 26:
        passed += 1

    letters: str = test_ascii_letters()
    if len(letters) == 52:
        passed += 1

    digits: str = test_digits()
    if len(digits) == 10:
        passed += 1

    hexdigits: str = test_hexdigits()
    if len(hexdigits) == 22:
        passed += 1

    octdigits: str = test_octdigits()
    if len(octdigits) == 8:
        passed += 1

    ws: str = test_whitespace()
    if len(ws) == 4:
        passed += 1

    # Test character classification via code
    if is_letter_code(65):
        passed += 1
    if is_letter_code(97):
        passed += 1
    if not is_letter_code(48):
        passed += 1

    if is_digit_code(48):
        passed += 1
    if not is_digit_code(65):
        passed += 1

    if is_whitespace_code(32):
        passed += 1
    if not is_whitespace_code(65):
        passed += 1

    # Test text transformations
    capitalized: str = capitalize_words("hello world")
    if capitalized == "Hello World":
        passed += 1

    title: str = to_title_case("hello world")
    if title == "Hello World":
        passed += 1

    swapped: str = swap_case("Hello")
    if swapped == "hELLO":
        passed += 1

    # Test counting
    sample: str = "Hello World 123"
    letter_count: int = count_letters(sample)
    if letter_count == 10:
        passed += 1

    digit_count: int = count_digits_in_text(sample)
    if digit_count == 3:
        passed += 1

    ws_count: int = count_whitespace(sample)
    if ws_count == 2:
        passed += 1

    # Test filtering
    no_ws: str = remove_whitespace("a b c")
    if no_ws == "abc":
        passed += 1

    only_letters: str = keep_only_letters("abc123")
    if only_letters == "abc":
        passed += 1

    only_digits: str = keep_only_digits("abc123")
    if only_digits == "123":
        passed += 1

    only_alnum: str = keep_alphanumeric("abc 123!")
    if only_alnum == "abc123":
        passed += 1

    # Test cipher
    encrypted: str = caesar_cipher("HELLO", 3)
    if encrypted == "KHOOR":
        passed += 1

    decrypted: str = caesar_cipher("KHOOR", -3)
    if decrypted == "HELLO":
        passed += 1

    # Test string operations
    reversed_text: str = reverse_string("hello")
    if reversed_text == "olleh":
        passed += 1

    if is_palindrome("racecar"):
        passed += 1

    vowel_count: int = count_vowels("hello world")
    if vowel_count == 3:
        passed += 1

    consonant_count: int = count_consonants("hello world")
    if consonant_count == 7:
        passed += 1

    return passed


if __name__ == "__main__":
    result: int = test_module()
    print("PASSED: " + str(result))
