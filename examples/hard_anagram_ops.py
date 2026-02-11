"""Anagram detection and character counting.

Tests: character frequency analysis, sorted comparison, string ops.
"""


def char_counts(s: str) -> list[int]:
    """Return a list of 26 counts for lowercase a-z."""
    counts: list[int] = []
    i: int = 0
    while i < 26:
        counts.append(0)
        i += 1
    for ch in s:
        code: int = ord(ch)
        if code >= 97 and code <= 122:
            idx: int = code - 97
            counts[idx] = counts[idx] + 1
    return counts


def is_anagram(s1: str, s2: str) -> bool:
    """Check if two strings are anagrams."""
    c1: list[int] = char_counts(s1)
    c2: list[int] = char_counts(s2)
    i: int = 0
    while i < 26:
        if c1[i] != c2[i]:
            return False
        i += 1
    return True


def count_anagram_pairs(words: list[str]) -> int:
    """Count pairs of anagrams in a list."""
    count: int = 0
    n: int = len(words)
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            if is_anagram(words[i], words[j]):
                count += 1
            j += 1
        i += 1
    return count


def sort_string_chars(s: str) -> str:
    """Sort characters of a string using selection sort on codes."""
    codes: list[int] = []
    for ch in s:
        codes.append(ord(ch))
    n: int = len(codes)
    i: int = 0
    while i < n:
        min_idx: int = i
        j: int = i + 1
        while j < n:
            if codes[j] < codes[min_idx]:
                min_idx = j
            j += 1
        temp: int = codes[i]
        codes[i] = codes[min_idx]
        codes[min_idx] = temp
        i += 1
    result: str = ""
    for c in codes:
        result = result + chr(c)
    return result


def most_common_char(s: str) -> str:
    """Find most common lowercase character."""
    counts: list[int] = char_counts(s)
    best_idx: int = 0
    best_count: int = 0
    i: int = 0
    while i < 26:
        if counts[i] > best_count:
            best_count = counts[i]
            best_idx = i
        i += 1
    return chr(97 + best_idx)


def test_module() -> int:
    """Test anagram operations."""
    ok: int = 0

    if is_anagram("listen", "silent"):
        ok += 1
    if not is_anagram("hello", "world"):
        ok += 1

    pairs: int = count_anagram_pairs(["eat", "tea", "ate", "dog"])
    if pairs == 3:
        ok += 1

    sorted_s: str = sort_string_chars("cba")
    if sorted_s == "abc":
        ok += 1

    mc: str = most_common_char("abracadabra")
    if mc == "a":
        ok += 1

    return ok
