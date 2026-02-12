"""Text processing: String tokenizer and word counter.

Tests: character classification, token boundary detection,
frequency counting, word length analysis.
"""

from typing import Dict, List, Tuple


def count_words(text: str) -> int:
    """Count words in text (space-separated)."""
    if len(text) == 0:
        return 0
    count: int = 0
    in_word: bool = False
    i: int = 0
    while i < len(text):
        is_space: bool = text[i] == " " or text[i] == "\t" or text[i] == "\n"
        if is_space:
            if in_word:
                in_word = False
        else:
            if not in_word:
                count += 1
                in_word = True
        i += 1
    return count


def count_lines(text: str) -> int:
    """Count number of lines in text."""
    if len(text) == 0:
        return 0
    count: int = 1
    i: int = 0
    while i < len(text):
        if text[i] == "\n":
            count += 1
        i += 1
    return count


def count_chars(text: str) -> int:
    """Count non-whitespace characters."""
    count: int = 0
    i: int = 0
    while i < len(text):
        if text[i] != " " and text[i] != "\t" and text[i] != "\n":
            count += 1
        i += 1
    return count


def longest_word_length(text: str) -> int:
    """Find length of longest word."""
    max_len: int = 0
    current_len: int = 0
    i: int = 0
    while i < len(text):
        if text[i] == " " or text[i] == "\t" or text[i] == "\n":
            if current_len > max_len:
                max_len = current_len
            current_len = 0
        else:
            current_len += 1
        i += 1
    if current_len > max_len:
        max_len = current_len
    return max_len


def count_spaces(text: str) -> int:
    """Count space characters in text."""
    count: int = 0
    i: int = 0
    while i < len(text):
        if text[i] == " ":
            count += 1
        i += 1
    return count


def average_word_length(text: str) -> float:
    """Compute average word length."""
    total_chars: int = 0
    word_count: int = 0
    current_len: int = 0
    i: int = 0
    while i < len(text):
        if text[i] == " " or text[i] == "\t" or text[i] == "\n":
            if current_len > 0:
                total_chars = total_chars + current_len
                word_count += 1
                current_len = 0
        else:
            current_len += 1
        i += 1
    if current_len > 0:
        total_chars = total_chars + current_len
        word_count += 1
    if word_count == 0:
        return 0.0
    return float(total_chars) / float(word_count)


def shortest_word_length(text: str) -> int:
    """Find length of shortest word."""
    min_len: int = 999999
    current_len: int = 0
    found: bool = False
    i: int = 0
    while i < len(text):
        if text[i] == " " or text[i] == "\t" or text[i] == "\n":
            if current_len > 0:
                found = True
                if current_len < min_len:
                    min_len = current_len
                current_len = 0
        else:
            current_len += 1
        i += 1
    if current_len > 0:
        found = True
        if current_len < min_len:
            min_len = current_len
    if not found:
        return 0
    return min_len


def test_text_tokenizer() -> bool:
    """Test text processing functions."""
    ok: bool = True
    wc: int = count_words("hello world foo")
    if wc != 3:
        ok = False
    lc: int = count_lines("a\nb\nc")
    if lc != 3:
        ok = False
    ll: int = longest_word_length("hi hello world")
    if ll != 5:
        ok = False
    sp: int = count_spaces("a b c")
    if sp != 2:
        ok = False
    return ok
