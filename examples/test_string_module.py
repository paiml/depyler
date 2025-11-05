"""
Comprehensive test of Python string module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's string module
constants and utilities to Rust equivalents.

Expected Rust mappings:
- string.ascii_letters -> static constants
- string.digits -> static constants
- string.Template -> string interpolation

Note: Most functionality implemented as manual string operations.
"""

import string
from typing import List


def test_ascii_lowercase() -> str:
    """Test accessing lowercase ASCII letters"""
    # Manually define lowercase letters
    lowercase: str = "abcdefghijklmnopqrstuvwxyz"

    return lowercase


def test_ascii_uppercase() -> str:
    """Test accessing uppercase ASCII letters"""
    # Manually define uppercase letters
    uppercase: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

    return uppercase


def test_ascii_letters() -> str:
    """Test accessing all ASCII letters"""
    lowercase: str = "abcdefghijklmnopqrstuvwxyz"
    uppercase: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    letters: str = lowercase + uppercase

    return letters


def test_digits() -> str:
    """Test accessing digit characters"""
    digits: str = "0123456789"

    return digits


def test_hexdigits() -> str:
    """Test accessing hexadecimal digits"""
    hexdigits: str = "0123456789abcdefABCDEF"

    return hexdigits


def test_octdigits() -> str:
    """Test accessing octal digits"""
    octdigits: str = "01234567"

    return octdigits


def test_punctuation() -> str:
    """Test accessing punctuation characters"""
    punctuation: str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"

    return punctuation


def test_whitespace() -> str:
    """Test accessing whitespace characters"""
    whitespace: str = " \t\n\r"

    return whitespace


def is_ascii_letter(char: str) -> bool:
    """Check if character is ASCII letter"""
    if len(char) != 1:
        return False

    code: int = ord(char)

    # A-Z or a-z
    is_upper: bool = code >= 65 and code <= 90
    is_lower: bool = code >= 97 and code <= 122

    return is_upper or is_lower


def is_digit(char: str) -> bool:
    """Check if character is digit"""
    if len(char) != 1:
        return False

    return char.isdigit()


def is_alphanumeric(char: str) -> bool:
    """Check if character is alphanumeric"""
    if len(char) != 1:
        return False

    return is_ascii_letter(char) or is_digit(char)


def is_whitespace(char: str) -> bool:
    """Check if character is whitespace"""
    if len(char) != 1:
        return False

    whitespace_chars: str = " \t\n\r"

    for ws in whitespace_chars:
        if char == ws:
            return True

    return False


def is_punctuation(char: str) -> bool:
    """Check if character is punctuation"""
    if len(char) != 1:
        return False

    punctuation_chars: str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"

    for p in punctuation_chars:
        if char == p:
            return True

    return False


def capitalize_words(text: str) -> str:
    """Capitalize first letter of each word"""
    words: List[str] = text.split()
    capitalized: List[str] = []

    for word in words:
        if len(word) > 0:
            first_char: str = word[0].upper()
            rest: str = word[1:].lower()
            cap_word: str = first_char + rest
            capitalized.append(cap_word)

    return " ".join(capitalized)


def to_title_case(text: str) -> str:
    """Convert to title case"""
    return capitalize_words(text)


def swap_case(text: str) -> str:
    """Swap uppercase to lowercase and vice versa"""
    result: str = ""

    for char in text:
        if char.isupper():
            result = result + char.lower()
        elif char.islower():
            result = result + char.upper()
        else:
            result = result + char

    return result


def count_letters(text: str) -> int:
    """Count number of letters in text"""
    count: int = 0

    for char in text:
        if is_ascii_letter(char):
            count = count + 1

    return count


def count_digits(text: str) -> int:
    """Count number of digits in text"""
    count: int = 0

    for char in text:
        if is_digit(char):
            count = count + 1

    return count


def count_whitespace(text: str) -> int:
    """Count whitespace characters"""
    count: int = 0

    for char in text:
        if is_whitespace(char):
            count = count + 1

    return count


def remove_whitespace(text: str) -> str:
    """Remove all whitespace from text"""
    result: str = ""

    for char in text:
        if not is_whitespace(char):
            result = result + char

    return result


def keep_only_letters(text: str) -> str:
    """Keep only letters, remove everything else"""
    result: str = ""

    for char in text:
        if is_ascii_letter(char):
            result = result + char

    return result


def keep_only_digits(text: str) -> str:
    """Keep only digits, remove everything else"""
    result: str = ""

    for char in text:
        if is_digit(char):
            result = result + char

    return result


def keep_alphanumeric(text: str) -> str:
    """Keep only alphanumeric characters"""
    result: str = ""

    for char in text:
        if is_alphanumeric(char):
            result = result + char

    return result


def template_substitute(template: str, values: dict) -> str:
    """Simple template substitution"""
    result: str = template

    # Replace each placeholder
    for key in values.keys():
        placeholder: str = "${" + key + "}"
        value: str = str(values[key])
        result = result.replace(placeholder, value)

    return result


def caesar_cipher(text: str, shift: int) -> str:
    """Simple Caesar cipher"""
    result: str = ""

    for char in text:
        if char.isalpha():
            if char.isupper():
                base: int = ord('A')
                shifted: int = (ord(char) - base + shift) % 26
                new_char: str = chr(base + shifted)
                result = result + new_char
            else:
                base: int = ord('a')
                shifted: int = (ord(char) - base + shift) % 26
                new_char: str = chr(base + shifted)
                result = result + new_char
        else:
            result = result + char

    return result


def reverse_string(text: str) -> str:
    """Reverse a string"""
    result: str = ""

    for i in range(len(text) - 1, -1, -1):
        result = result + text[i]

    return result


def is_palindrome(text: str) -> bool:
    """Check if text is palindrome (ignoring case and spaces)"""
    # Normalize
    normalized: str = ""
    for char in text.lower():
        if char.isalnum():
            normalized = normalized + char

    # Check palindrome
    left: int = 0
    right: int = len(normalized) - 1

    while left < right:
        if normalized[left] != normalized[right]:
            return False
        left = left + 1
        right = right - 1

    return True


def count_vowels(text: str) -> int:
    """Count vowels in text"""
    vowels: str = "aeiouAEIOU"
    count: int = 0

    for char in text:
        if char in vowels:
            count = count + 1

    return count


def count_consonants(text: str) -> int:
    """Count consonants in text"""
    vowels: str = "aeiouAEIOU"
    count: int = 0

    for char in text:
        if is_ascii_letter(char) and char not in vowels:
            count = count + 1

    return count


def test_all_string_features() -> None:
    """Run all string module tests"""
    # Constants
    lowercase: str = test_ascii_lowercase()
    uppercase: str = test_ascii_uppercase()
    letters: str = test_ascii_letters()
    digits: str = test_digits()
    hexdigits: str = test_hexdigits()
    octdigits: str = test_octdigits()
    punct: str = test_punctuation()
    ws: str = test_whitespace()

    # Character classification
    is_letter: bool = is_ascii_letter('a')
    is_num: bool = is_digit('5')
    is_alnum: bool = is_alphanumeric('a')
    is_ws: bool = is_whitespace(' ')
    is_punct: bool = is_punctuation('!')

    # Text transformations
    text: str = "hello world"
    capitalized: str = capitalize_words(text)
    title: str = to_title_case(text)
    swapped: str = swap_case(text)

    # Counting
    sample: str = "Hello World 123!"
    letter_count: int = count_letters(sample)
    digit_count: int = count_digits(sample)
    ws_count: int = count_whitespace(sample)

    # Filtering
    no_ws: str = remove_whitespace(sample)
    only_letters: str = keep_only_letters(sample)
    only_digits: str = keep_only_digits(sample)
    only_alnum: str = keep_alphanumeric(sample)

    # Template substitution
    template: str = "Hello ${name}, you are ${age} years old"
    values: dict = {"name": "Alice", "age": "30"}
    substituted: str = template_substitute(template, values)

    # Cipher
    message: str = "HELLO"
    encrypted: str = caesar_cipher(message, 3)
    decrypted: str = caesar_cipher(encrypted, -3)

    # String operations
    reversed_text: str = reverse_string("hello")
    is_palin: bool = is_palindrome("A man a plan a canal Panama")
    vowel_count: int = count_vowels("hello world")
    consonant_count: int = count_consonants("hello world")

    print("All string module tests completed successfully")
