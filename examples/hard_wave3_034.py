"""Text processing: Anagram and permutation operations.

Tests: character sorting, anagram detection, permutation generation,
frequency comparison, canonical form computation.
"""

from typing import Dict, List, Tuple


def sort_chars(s: str) -> str:
    """Sort characters in a string using insertion sort on codes."""
    chars: List[int] = []
    i: int = 0
    while i < len(s):
        chars.append(ord(s[i]))
        i += 1
    n: int = len(chars)
    i = 1
    while i < n:
        val: int = chars[i]
        j: int = i - 1
        while j >= 0 and chars[j] > val:
            chars[j + 1] = chars[j]
            j -= 1
        chars[j + 1] = val
        i += 1
    result: List[str] = []
    for c in chars:
        result.append(chr(c))
    return "".join(result)


def are_anagrams(a: str, b: str) -> bool:
    """Check if two strings are anagrams using char frequency comparison."""
    if len(a) != len(b):
        return False
    freq_a: Dict[str, int] = {}
    freq_b: Dict[str, int] = {}
    i: int = 0
    while i < len(a):
        ch_a: str = a[i]
        ch_b: str = b[i]
        if ch_a in freq_a:
            freq_a[ch_a] = freq_a[ch_a] + 1
        else:
            freq_a[ch_a] = 1
        if ch_b in freq_b:
            freq_b[ch_b] = freq_b[ch_b] + 1
        else:
            freq_b[ch_b] = 1
        i += 1
    for k in freq_a:
        if k not in freq_b:
            return False
        if freq_a[k] != freq_b[k]:
            return False
    for k in freq_b:
        if k not in freq_a:
            return False
    return True


def count_anagram_groups(words: List[str]) -> int:
    """Count distinct anagram groups using sorted form."""
    groups: Dict[str, int] = {}
    for word in words:
        sorted_w: str = sort_chars(word)
        if sorted_w not in groups:
            groups[sorted_w] = 1
        else:
            groups[sorted_w] = groups[sorted_w] + 1
    count: int = 0
    for g in groups:
        count += 1
    return count


def char_frequency(s: str) -> Dict[str, int]:
    """Build character frequency map."""
    freq: Dict[str, int] = {}
    i: int = 0
    while i < len(s):
        ch: str = s[i]
        if ch in freq:
            freq[ch] = freq[ch] + 1
        else:
            freq[ch] = 1
        i += 1
    return freq


def is_subset_chars(a: str, b: str) -> bool:
    """Check if all chars of a appear in b with sufficient frequency."""
    fa: Dict[str, int] = char_frequency(a)
    fb: Dict[str, int] = char_frequency(b)
    for ch in fa:
        if ch not in fb:
            return False
        if fa[ch] > fb[ch]:
            return False
    return True


def unique_chars(s: str) -> str:
    """Return string with duplicate characters removed, preserving order."""
    seen: Dict[str, int] = {}
    result: List[str] = []
    i: int = 0
    while i < len(s):
        ch: str = s[i]
        if ch not in seen:
            seen[ch] = 1
            result.append(ch)
        i += 1
    return "".join(result)


def test_anagrams() -> bool:
    """Test anagram functions."""
    ok: bool = True
    if not are_anagrams("listen", "silent"):
        ok = False
    if are_anagrams("hello", "world"):
        ok = False
    gc: int = count_anagram_groups(["eat", "tea", "ate", "tan", "nat"])
    if gc != 2:
        ok = False
    uniq: str = unique_chars("abracadabra")
    if uniq != "abrcd":
        ok = False
    return ok
