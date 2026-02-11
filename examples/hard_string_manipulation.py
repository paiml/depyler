"""Hard string manipulation patterns for transpiler stress-testing.

Tests: string split/join, strip, replace, find, startswith/endswith,
string formatting, character counting, substring operations,
case conversion patterns, and string building.
"""


def string_split_by_char(text: str, delimiter: str) -> list[str]:
    """Split a string by a single-character delimiter."""
    parts: list[str] = []
    current: str = ""
    for ch in text:
        if ch == delimiter:
            parts.append(current)
            current = ""
        else:
            current = current + ch
    parts.append(current)
    return parts


def string_join(parts: list[str], separator: str) -> str:
    """Join a list of strings with a separator."""
    if len(parts) == 0:
        return ""
    result: str = parts[0]
    for i in range(1, len(parts)):
        result = result + separator + parts[i]
    return result


def string_strip_chars(text: str, chars: str) -> str:
    """Strip specified characters from both ends of a string."""
    start: int = 0
    end: int = len(text)
    while start < end:
        found: bool = False
        for ch in chars:
            if text[start] == ch:
                found = True
        if found:
            start += 1
        else:
            break
    while end > start:
        found2: bool = False
        for ch in chars:
            if text[end - 1] == ch:
                found2 = True
        if found2:
            end -= 1
        else:
            break
    result: str = ""
    for i in range(start, end):
        result = result + text[i]
    return result


def string_replace_all(text: str, old: str, new: str) -> str:
    """Replace all occurrences of old substring with new substring."""
    if len(old) == 0:
        return text
    result: str = ""
    i: int = 0
    while i < len(text):
        match: bool = True
        if i + len(old) > len(text):
            match = False
        else:
            for j in range(len(old)):
                if text[i + j] != old[j]:
                    match = False
        if match:
            result = result + new
            i += len(old)
        else:
            result = result + text[i]
            i += 1
    return result


def string_find(text: str, pattern: str) -> int:
    """Find the first occurrence of pattern in text, return index or -1."""
    if len(pattern) == 0:
        return 0
    if len(pattern) > len(text):
        return -1
    for i in range(len(text) - len(pattern) + 1):
        match: bool = True
        for j in range(len(pattern)):
            if text[i + j] != pattern[j]:
                match = False
                break
        if match:
            return i
    return -1


def string_count_occurrences(text: str, pattern: str) -> int:
    """Count non-overlapping occurrences of pattern in text."""
    if len(pattern) == 0:
        return 0
    count: int = 0
    i: int = 0
    while i <= len(text) - len(pattern):
        match: bool = True
        for j in range(len(pattern)):
            if text[i + j] != pattern[j]:
                match = False
                break
        if match:
            count += 1
            i += len(pattern)
        else:
            i += 1
    return count


def string_starts_with(text: str, prefix: str) -> bool:
    """Check if text starts with the given prefix."""
    if len(prefix) > len(text):
        return False
    for i in range(len(prefix)):
        if text[i] != prefix[i]:
            return False
    return True


def string_ends_with(text: str, suffix: str) -> bool:
    """Check if text ends with the given suffix."""
    if len(suffix) > len(text):
        return False
    offset: int = len(text) - len(suffix)
    for i in range(len(suffix)):
        if text[offset + i] != suffix[i]:
            return False
    return True


def string_repeat(text: str, count: int) -> str:
    """Repeat a string count times."""
    result: str = ""
    for _ in range(count):
        result = result + text
    return result


def string_reverse(text: str) -> str:
    """Reverse a string."""
    result: str = ""
    for i in range(len(text) - 1, -1, -1):
        result = result + text[i]
    return result


def string_is_palindrome(text: str) -> bool:
    """Check if a string is a palindrome."""
    left: int = 0
    right: int = len(text) - 1
    while left < right:
        if text[left] != text[right]:
            return False
        left += 1
        right -= 1
    return True


def string_pad_left(text: str, width: int, pad_char: str) -> str:
    """Pad a string on the left to reach the desired width."""
    result: str = text
    while len(result) < width:
        result = pad_char + result
    return result


def string_pad_right(text: str, width: int, pad_char: str) -> str:
    """Pad a string on the right to reach the desired width."""
    result: str = text
    while len(result) < width:
        result = result + pad_char
    return result


def string_center(text: str, width: int, pad_char: str) -> str:
    """Center a string within a given width using pad characters."""
    if len(text) >= width:
        return text
    total_pad: int = width - len(text)
    left_pad: int = total_pad // 2
    right_pad: int = total_pad - left_pad
    result: str = ""
    for _ in range(left_pad):
        result = result + pad_char
    result = result + text
    for _ in range(right_pad):
        result = result + pad_char
    return result


def word_count(text: str) -> int:
    """Count words in a string (space-separated)."""
    if len(text) == 0:
        return 0
    words: list[str] = string_split_by_char(text, " ")
    count: int = 0
    for word in words:
        if len(word) > 0:
            count += 1
    return count


def char_frequency(text: str) -> dict[str, int]:
    """Count the frequency of each character in a string."""
    freq: dict[str, int] = {}
    for ch in text:
        if ch in freq:
            freq[ch] += 1
        else:
            freq[ch] = 1
    return freq


def longest_common_prefix(strings: list[str]) -> str:
    """Find the longest common prefix among a list of strings."""
    if len(strings) == 0:
        return ""
    prefix: str = strings[0]
    for i in range(1, len(strings)):
        new_prefix: str = ""
        s: str = strings[i]
        j: int = 0
        while j < len(prefix) and j < len(s):
            if prefix[j] == s[j]:
                new_prefix = new_prefix + prefix[j]
                j += 1
            else:
                break
        prefix = new_prefix
    return prefix


def caesar_cipher(text: str, shift: int) -> str:
    """Apply Caesar cipher to lowercase letters only, leaving other chars unchanged."""
    result: str = ""
    for ch in text:
        if ch >= "a" and ch <= "z":
            code: int = ord(ch) - ord("a")
            shifted: int = (code + shift) % 26
            result = result + chr(shifted + ord("a"))
        else:
            result = result + ch
    return result


def test_all() -> bool:
    """Comprehensive test exercising all string manipulation functions."""
    # Test string_split_by_char
    parts: list[str] = string_split_by_char("hello,world,foo", ",")
    assert len(parts) == 3
    assert parts[0] == "hello"
    assert parts[1] == "world"
    assert parts[2] == "foo"
    assert string_split_by_char("abc", ",") == ["abc"]

    # Test string_join
    assert string_join(["a", "b", "c"], "-") == "a-b-c"
    assert string_join(["hello"], " ") == "hello"
    assert string_join([], ",") == ""

    # Test string_strip_chars
    assert string_strip_chars("  hello  ", " ") == "hello"
    assert string_strip_chars("xxhelloxx", "x") == "hello"
    assert string_strip_chars("abc", "x") == "abc"

    # Test string_replace_all
    assert string_replace_all("hello world", "world", "rust") == "hello rust"
    assert string_replace_all("aabaa", "aa", "x") == "xbx"
    assert string_replace_all("abc", "d", "e") == "abc"

    # Test string_find
    assert string_find("hello world", "world") == 6
    assert string_find("hello world", "xyz") == -1
    assert string_find("hello", "") == 0
    assert string_find("abc", "abcd") == -1

    # Test string_count_occurrences
    assert string_count_occurrences("abababab", "ab") == 4
    assert string_count_occurrences("aaaa", "aa") == 2
    assert string_count_occurrences("hello", "xyz") == 0

    # Test string_starts_with
    assert string_starts_with("hello world", "hello") == True
    assert string_starts_with("hello", "world") == False
    assert string_starts_with("hi", "hello") == False

    # Test string_ends_with
    assert string_ends_with("hello world", "world") == True
    assert string_ends_with("hello", "world") == False
    assert string_ends_with("hi", "hello") == False

    # Test string_repeat
    assert string_repeat("ab", 3) == "ababab"
    assert string_repeat("x", 0) == ""
    assert string_repeat("hi", 1) == "hi"

    # Test string_reverse
    assert string_reverse("hello") == "olleh"
    assert string_reverse("a") == "a"
    assert string_reverse("") == ""

    # Test string_is_palindrome
    assert string_is_palindrome("racecar") == True
    assert string_is_palindrome("hello") == False
    assert string_is_palindrome("a") == True
    assert string_is_palindrome("") == True

    # Test string_pad_left
    assert string_pad_left("42", 5, "0") == "00042"
    assert string_pad_left("hello", 3, " ") == "hello"

    # Test string_pad_right
    assert string_pad_right("hi", 5, ".") == "hi..."
    assert string_pad_right("hello", 3, " ") == "hello"

    # Test string_center
    assert string_center("hi", 6, "-") == "--hi--"
    assert string_center("x", 4, "*") == "*x**"
    assert string_center("long", 2, " ") == "long"

    # Test word_count
    assert word_count("hello world foo bar") == 4
    assert word_count("single") == 1
    assert word_count("") == 0

    # Test char_frequency
    freq: dict[str, int] = char_frequency("aabbc")
    assert freq["a"] == 2
    assert freq["b"] == 2
    assert freq["c"] == 1

    # Test longest_common_prefix
    assert longest_common_prefix(["flower", "flow", "flight"]) == "fl"
    assert longest_common_prefix(["dog", "racecar"]) == ""
    assert longest_common_prefix(["abc"]) == "abc"
    assert longest_common_prefix([]) == ""

    # Test caesar_cipher
    assert caesar_cipher("abc", 1) == "bcd"
    assert caesar_cipher("xyz", 3) == "abc"
    assert caesar_cipher("hello world", 13) == "uryyb jbeyq"
    # Double rotation by 13 should return to original
    assert caesar_cipher(caesar_cipher("hello", 13), 13) == "hello"

    return True


def main() -> None:
    """Run all tests and report results."""
    result: bool = test_all()
    if result:
        print("All string manipulation tests passed!")


if __name__ == "__main__":
    main()
