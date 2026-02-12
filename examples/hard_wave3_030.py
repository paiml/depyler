"""Text processing: Text alignment and formatting.

Tests: column alignment, text wrapping, justification,
table formatting, indent management.
"""

from typing import Dict, List, Tuple


def center_text(text: str, width: int) -> str:
    """Center text within given width."""
    n: int = len(text)
    if n >= width:
        return text
    total_pad: int = width - n
    left_pad: int = total_pad // 2
    right_pad: int = total_pad - left_pad
    result: List[str] = []
    i: int = 0
    while i < left_pad:
        result.append(" ")
        i += 1
    for ch in text:
        result.append(ch)
    i = 0
    while i < right_pad:
        result.append(" ")
        i += 1
    return "".join(result)


def right_align(text: str, width: int) -> str:
    """Right-align text within given width."""
    n: int = len(text)
    if n >= width:
        return text
    pad: int = width - n
    result: List[str] = []
    i: int = 0
    while i < pad:
        result.append(" ")
        i += 1
    for ch in text:
        result.append(ch)
    return "".join(result)


def truncate(text: str, max_len: int) -> str:
    """Truncate text to max_len, adding ... if truncated."""
    if len(text) <= max_len:
        return text
    if max_len <= 3:
        result: List[str] = []
        i: int = 0
        while i < max_len:
            result.append(".")
            i += 1
        return "".join(result)
    result2: List[str] = []
    i = 0
    cutoff: int = max_len - 3
    while i < cutoff:
        result2.append(text[i])
        i += 1
    result2.append(".")
    result2.append(".")
    result2.append(".")
    return "".join(result2)


def indent_lines(text: str, spaces: int) -> str:
    """Indent each line of text by given number of spaces."""
    result: List[str] = []
    i: int = 0
    while i < spaces:
        result.append(" ")
        i += 1
    prefix: str = "".join(result)
    output: List[str] = [prefix]
    for ch in text:
        output.append(ch)
        if ch == "\n":
            output.append(prefix)
    return "".join(output)


def wrap_text(text: str, width: int) -> str:
    """Word-wrap text at given width."""
    if width <= 0:
        return text
    result: List[str] = []
    col: int = 0
    for ch in text:
        if ch == " " and col >= width:
            result.append("\n")
            col = 0
        else:
            result.append(ch)
            col += 1
    return "".join(result)


def remove_duplicate_spaces(text: str) -> str:
    """Replace multiple consecutive spaces with single space."""
    result: List[str] = []
    prev_space: bool = False
    for ch in text:
        if ch == " ":
            if not prev_space:
                result.append(ch)
            prev_space = True
        else:
            result.append(ch)
            prev_space = False
    return "".join(result)


def title_case(text: str) -> str:
    """Convert text to title case (first letter of each word uppercase)."""
    result: List[str] = []
    capitalize_next: bool = True
    for ch in text:
        if ch == " ":
            result.append(ch)
            capitalize_next = True
        elif capitalize_next and ch >= "a" and ch <= "z":
            code: int = ord(ch) - ord("a") + ord("A")
            result.append(chr(code))
            capitalize_next = False
        else:
            result.append(ch)
            capitalize_next = False
    return "".join(result)


def test_formatting() -> bool:
    """Test text formatting functions."""
    ok: bool = True
    centered: str = center_text("hi", 6)
    if len(centered) != 6:
        ok = False
    trunc: str = truncate("hello world", 8)
    if len(trunc) != 8:
        ok = False
    dedup: str = remove_duplicate_spaces("a  b   c")
    if dedup != "a b c":
        ok = False
    titled: str = title_case("hello world")
    if titled != "Hello World":
        ok = False
    return ok
