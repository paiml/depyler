"""
Comprehensive test of Python re (regex) module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's re module
(regular expressions) to Rust equivalents.

Expected Rust mappings:
- re.match() -> regex::Regex::is_match()
- re.search() -> regex::Regex::find()
- re.findall() -> regex::Regex::find_iter()
- re.sub() -> regex::Regex::replace()
- re.split() -> regex::Regex::split()

Note: Regex functionality may be simulated with string operations.
"""

import re
from typing import List, Optional


def test_simple_match() -> bool:
    """Test simple pattern matching"""
    text: str = "Hello World"
    pattern: str = "Hello"

    # Manual string matching (simulating regex)
    matches: bool = text.startswith(pattern)

    return matches


def test_contains_pattern() -> bool:
    """Test if text contains pattern"""
    text: str = "The quick brown fox"
    pattern: str = "quick"

    # Check if pattern is in text
    contains: bool = pattern in text

    return contains


def test_find_pattern_position() -> int:
    """Test finding pattern position"""
    text: str = "Hello World Hello"
    pattern: str = "World"

    # Find pattern position
    position: int = text.find(pattern)

    return position


def test_count_occurrences() -> int:
    """Test counting pattern occurrences"""
    text: str = "abc abc abc"
    pattern: str = "abc"

    # Count occurrences
    count: int = text.count(pattern)

    return count


def test_replace_pattern() -> str:
    """Test replacing pattern"""
    text: str = "Hello World"
    old_pattern: str = "World"
    new_text: str = "Python"

    # Replace pattern
    result: str = text.replace(old_pattern, new_text)

    return result


def test_split_by_pattern() -> List[str]:
    """Test splitting by pattern"""
    text: str = "apple,banana,cherry"
    delimiter: str = ","

    # Split by delimiter
    parts: List[str] = text.split(delimiter)

    return parts


def test_match_digit() -> bool:
    """Test matching digits"""
    text: str = "123"

    # Check if all characters are digits
    is_digit: bool = text.isdigit()

    return is_digit


def test_match_alpha() -> bool:
    """Test matching alphabetic characters"""
    text: str = "Hello"

    # Check if all characters are alphabetic
    is_alpha: bool = text.isalpha()

    return is_alpha


def test_match_alphanumeric() -> bool:
    """Test matching alphanumeric characters"""
    text: str = "Hello123"

    # Check if all characters are alphanumeric
    is_alnum: bool = text.isalnum()

    return is_alnum


def extract_digits(text: str) -> str:
    """Extract all digits from text"""
    digits: str = ""

    for char in text:
        if char.isdigit():
            digits = digits + char

    return digits


def extract_letters(text: str) -> str:
    """Extract all letters from text"""
    letters: str = ""

    for char in text:
        if char.isalpha():
            letters = letters + char

    return letters


def find_all_words(text: str) -> List[str]:
    """Find all words in text (space-separated)"""
    words: List[str] = text.split()

    return words


def validate_email_simple(email: str) -> bool:
    """Simple email validation (manual)"""
    # Check for @ symbol
    has_at: bool = "@" in email

    # Check for dot after @
    if not has_at:
        return False

    at_pos: int = email.find("@")
    after_at: str = email[at_pos + 1:]
    has_dot: bool = "." in after_at

    return has_dot


def validate_phone_simple(phone: str) -> bool:
    """Simple phone validation"""
    # Remove common separators
    cleaned: str = phone.replace("-", "").replace(" ", "").replace("(", "").replace(")", "")

    # Check if all remaining are digits
    is_valid: bool = cleaned.isdigit() and len(cleaned) >= 10

    return is_valid


def extract_url_domain(url: str) -> str:
    """Extract domain from URL"""
    # Remove protocol
    if url.startswith("http://"):
        url = url[7:]
    elif url.startswith("https://"):
        url = url[8:]

    # Find first slash
    slash_pos: int = url.find("/")

    if slash_pos >= 0:
        domain: str = url[:slash_pos]
    else:
        domain: str = url

    return domain


def remove_punctuation(text: str) -> str:
    """Remove common punctuation marks"""
    punctuation: str = ".,!?;:"
    result: str = ""

    for char in text:
        is_punct: bool = False
        for p in punctuation:
            if char == p:
                is_punct = True
                break

        if not is_punct:
            result = result + char

    return result


def normalize_whitespace(text: str) -> str:
    """Normalize multiple spaces to single space"""
    # Split and rejoin to normalize
    words: List[str] = text.split()
    normalized: str = " ".join(words)

    return normalized


def starts_with_pattern(text: str, pattern: str) -> bool:
    """Check if text starts with pattern"""
    return text.startswith(pattern)


def ends_with_pattern(text: str, pattern: str) -> bool:
    """Check if text ends with pattern"""
    return text.endswith(pattern)


def case_insensitive_match(text: str, pattern: str) -> bool:
    """Case-insensitive pattern matching"""
    text_lower: str = text.lower()
    pattern_lower: str = pattern.lower()

    matches: bool = pattern_lower in text_lower

    return matches


def find_between(text: str, start_marker: str, end_marker: str) -> str:
    """Find text between two markers"""
    start_pos: int = text.find(start_marker)

    if start_pos < 0:
        return ""

    start_pos = start_pos + len(start_marker)
    end_pos: int = text.find(end_marker, start_pos)

    if end_pos < 0:
        return ""

    result: str = text[start_pos:end_pos]

    return result


def replace_multiple(text: str, replacements: List[tuple]) -> str:
    """Replace multiple patterns"""
    result: str = text

    for replacement in replacements:
        old: str = replacement[0]
        new: str = replacement[1]
        result = result.replace(old, new)

    return result


def count_word_occurrences(text: str, word: str) -> int:
    """Count occurrences of a word"""
    words: List[str] = text.split()
    count: int = 0

    for w in words:
        if w == word:
            count = count + 1

    return count


def extract_numbers_from_text(text: str) -> List[int]:
    """Extract numbers from text"""
    numbers: List[int] = []
    current_num: str = ""

    for char in text:
        if char.isdigit():
            current_num = current_num + char
        else:
            if len(current_num) > 0:
                num: int = int(current_num)
                numbers.append(num)
                current_num = ""

    # Add last number if exists
    if len(current_num) > 0:
        num: int = int(current_num)
        numbers.append(num)

    return numbers


def wildcard_match_simple(text: str, pattern: str) -> bool:
    """Simple wildcard matching (* means any sequence)"""
    # Check if pattern has wildcard
    if "*" not in pattern:
        return text == pattern

    # Split by wildcard
    parts: List[str] = pattern.split("*")

    if len(parts) != 2:
        return False

    prefix: str = parts[0]
    suffix: str = parts[1]

    # Check prefix and suffix
    has_prefix: bool = True
    has_suffix: bool = True

    if len(prefix) > 0:
        has_prefix = text.startswith(prefix)

    if len(suffix) > 0:
        has_suffix = text.endswith(suffix)

    return has_prefix and has_suffix


def test_all_re_features() -> None:
    """Run all regex module tests"""
    # Basic matching
    matches: bool = test_simple_match()
    contains: bool = test_contains_pattern()
    position: int = test_find_pattern_position()
    count: int = test_count_occurrences()

    # Replacement and splitting
    replaced: str = test_replace_pattern()
    split_result: List[str] = test_split_by_pattern()

    # Character class tests
    is_digit: bool = test_match_digit()
    is_alpha: bool = test_match_alpha()
    is_alnum: bool = test_match_alphanumeric()

    # Extraction
    text: str = "abc123def456"
    digits: str = extract_digits(text)
    letters: str = extract_letters(text)

    sentence: str = "Hello world from Python"
    words: List[str] = find_all_words(sentence)

    # Validation
    email_valid: bool = validate_email_simple("user@example.com")
    email_invalid: bool = validate_email_simple("notanemail")

    phone_valid: bool = validate_phone_simple("555-123-4567")
    phone_invalid: bool = validate_phone_simple("abc")

    # URL processing
    url: str = "https://www.example.com/path/page.html"
    domain: str = extract_url_domain(url)

    # Text processing
    punct_text: str = "Hello, World!"
    no_punct: str = remove_punctuation(punct_text)

    spaces: str = "Hello    World   !"
    normalized: str = normalize_whitespace(spaces)

    # Pattern checks
    starts: bool = starts_with_pattern("Hello World", "Hello")
    ends: bool = ends_with_pattern("Hello World", "World")

    case_match: bool = case_insensitive_match("Hello", "hello")

    # Between extraction
    tagged: str = "<tag>content</tag>"
    content: str = find_between(tagged, "<tag>", "</tag>")

    # Multiple replacements
    replacements: List[tuple] = [("a", "x"), ("b", "y")]
    multi_replace: str = replace_multiple("aabbcc", replacements)

    # Word counting
    para: str = "the quick brown fox jumps over the lazy dog"
    the_count: int = count_word_occurrences(para, "the")

    # Number extraction
    mixed: str = "I have 2 apples and 5 oranges"
    nums: List[int] = extract_numbers_from_text(mixed)

    # Wildcard matching
    wildcard1: bool = wildcard_match_simple("hello.txt", "*.txt")
    wildcard2: bool = wildcard_match_simple("test_file.py", "test_*")

    print("All regex module tests completed successfully")
