"""Word counting and text statistics using simple parsing.

Tests: count words by separator, count sentences, count chars, avg word length.
"""


def count_spaces(text: str) -> int:
    """Count space characters in text."""
    count: int = 0
    i: int = 0
    while i < len(text):
        if text[i] == " ":
            count = count + 1
        i = i + 1
    return count


def count_words_simple(text: str) -> int:
    """Count words (space-separated). Empty string returns 0."""
    if len(text) == 0:
        return 0
    spaces: int = count_spaces(text)
    return spaces + 1


def count_vowels(text: str) -> int:
    """Count vowels (a, e, i, o, u) in text."""
    count: int = 0
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if c == "a" or c == "e" or c == "i" or c == "o" or c == "u":
            count = count + 1
        if c == "A" or c == "E" or c == "I" or c == "O" or c == "U":
            count = count + 1
        i = i + 1
    return count


def count_consonants(text: str) -> int:
    """Count letter consonants in text."""
    total_letters: int = 0
    i: int = 0
    while i < len(text):
        c: str = text[i]
        is_letter: int = 0
        if c >= "a" and c <= "z":
            is_letter = 1
        if c >= "A" and c <= "Z":
            is_letter = 1
        if is_letter == 1:
            total_letters = total_letters + 1
        i = i + 1
    return total_letters - count_vowels(text)


def test_module() -> int:
    """Test word counting."""
    ok: int = 0
    if count_spaces("hello world foo") == 2:
        ok = ok + 1
    if count_words_simple("hello world") == 2:
        ok = ok + 1
    if count_words_simple("") == 0:
        ok = ok + 1
    if count_vowels("hello") == 2:
        ok = ok + 1
    if count_vowels("xyz") == 0:
        ok = ok + 1
    if count_consonants("hello") == 3:
        ok = ok + 1
    return ok
