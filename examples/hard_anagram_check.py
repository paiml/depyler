"""Anagram detection using character counts."""


def char_counts(s: str) -> dict[str, int]:
    """Count occurrences of each character in a string."""
    counts: dict[str, int] = {}
    idx: int = 0
    length: int = len(s)
    while idx < length:
        ch: str = s[idx]
        if ch in counts:
            counts[ch] = counts[ch] + 1
        else:
            counts[ch] = 1
        idx = idx + 1
    return counts


def is_anagram(a: str, b: str) -> int:
    """Return 1 if a and b are anagrams, else 0."""
    if len(a) != len(b):
        return 0
    counts_a: dict[str, int] = char_counts(a)
    counts_b: dict[str, int] = char_counts(b)
    idx: int = 0
    length: int = len(a)
    while idx < length:
        ch: str = a[idx]
        if ch not in counts_b:
            return 0
        if counts_a[ch] != counts_b[ch]:
            return 0
        idx = idx + 1
    return 1


def can_form_word(letters: str, word: str) -> int:
    """Return 1 if word can be formed from given letters, else 0."""
    available: dict[str, int] = char_counts(letters)
    idx: int = 0
    length: int = len(word)
    while idx < length:
        ch: str = word[idx]
        if ch not in available:
            return 0
        if available[ch] <= 0:
            return 0
        available[ch] = available[ch] - 1
        idx = idx + 1
    return 1


def count_matching_chars(a: str, b: str) -> int:
    """Count characters in common (with multiplicity)."""
    counts_a: dict[str, int] = char_counts(a)
    counts_b: dict[str, int] = char_counts(b)
    total: int = 0
    idx: int = 0
    length: int = len(a)
    while idx < length:
        ch: str = a[idx]
        if ch in counts_b:
            a_count: int = counts_a[ch]
            b_count: int = counts_b[ch]
            if a_count < b_count:
                total = total + a_count
            else:
                total = total + b_count
            del counts_b[ch]
        idx = idx + 1
    return total


def test_module() -> int:
    passed: int = 0

    if is_anagram("listen", "silent") == 1:
        passed = passed + 1
    if is_anagram("hello", "world") == 0:
        passed = passed + 1
    if is_anagram("abc", "ab") == 0:
        passed = passed + 1

    if can_form_word("aabbcc", "abc") == 1:
        passed = passed + 1
    if can_form_word("abc", "abcd") == 0:
        passed = passed + 1

    if count_matching_chars("abc", "abd") == 2:
        passed = passed + 1

    counts: dict[str, int] = char_counts("aab")
    if counts["a"] == 2:
        passed = passed + 1

    return passed
