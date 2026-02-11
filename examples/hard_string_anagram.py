"""Anagram detection and grouping by sorted key."""


def sort_string(s: str) -> str:
    """Sort characters of a string using insertion sort."""
    chars: list[int] = []
    i: int = 0
    while i < len(s):
        chars.append(ord(s[i]))
        i = i + 1
    # Insertion sort
    j: int = 1
    while j < len(chars):
        key: int = chars[j]
        k: int = j - 1
        while k >= 0 and chars[k] > key:
            chars[k + 1] = chars[k]
            k = k - 1
        chars[k + 1] = key
        j = j + 1
    result: str = ""
    m: int = 0
    while m < len(chars):
        result = result + chr(chars[m])
        m = m + 1
    return result


def is_anagram(s1: str, s2: str) -> int:
    """Check if two strings are anagrams. Returns 1 or 0."""
    if len(s1) != len(s2):
        return 0
    sorted1: str = sort_string(s1)
    sorted2: str = sort_string(s2)
    if sorted1 == sorted2:
        return 1
    return 0


def char_frequency(s: str) -> list[int]:
    """Return frequency array of 26 lowercase letters."""
    freq: list[int] = []
    i: int = 0
    while i < 26:
        freq.append(0)
        i = i + 1
    j: int = 0
    while j < len(s):
        idx: int = ord(s[j]) - ord("a")
        if idx >= 0 and idx < 26:
            freq[idx] = freq[idx] + 1
        j = j + 1
    return freq


def count_anagram_pairs(words: list[str]) -> int:
    """Count pairs of anagrams in a list of words."""
    count: int = 0
    i: int = 0
    while i < len(words):
        j: int = i + 1
        while j < len(words):
            if is_anagram(words[i], words[j]) == 1:
                count = count + 1
            j = j + 1
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0

    if is_anagram("listen", "silent") == 1:
        passed = passed + 1

    if is_anagram("hello", "world") == 0:
        passed = passed + 1

    if is_anagram("abc", "ab") == 0:
        passed = passed + 1

    if sort_string("cba") == "abc":
        passed = passed + 1

    freq: list[int] = char_frequency("aab")
    if freq[0] == 2 and freq[1] == 1:
        passed = passed + 1

    pairs: int = count_anagram_pairs(["eat", "tea", "tan", "ate", "nat"])
    if pairs == 4:
        passed = passed + 1

    if is_anagram("", "") == 1:
        passed = passed + 1

    return passed
