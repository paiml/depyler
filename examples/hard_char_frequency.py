"""Character frequency analysis for strings."""


def char_freq(text: str) -> dict[str, int]:
    """Count frequency of each character in text."""
    freq: dict[str, int] = {}
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        if ch in freq:
            freq[ch] = freq[ch] + 1
        else:
            freq[ch] = 1
        i = i + 1
    return freq


def most_frequent_char(text: str) -> str:
    """Find the most frequent character in text."""
    if len(text) == 0:
        return ""
    freq: dict[str, int] = char_freq(text)
    best_char: str = text[0]
    best_count: int = 0
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        count: int = freq[ch]
        if count > best_count:
            best_count = count
            best_char = ch
        i = i + 1
    return best_char


def unique_char_count(text: str) -> int:
    """Count the number of unique characters in text."""
    seen: dict[str, int] = {}
    count: int = 0
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        if ch not in seen:
            seen[ch] = 1
            count = count + 1
        i = i + 1
    return count


def is_anagram(s1: str, s2: str) -> int:
    """Check if s1 and s2 are anagrams. Returns 1 if yes, 0 if no."""
    if len(s1) != len(s2):
        return 0
    freq1: dict[str, int] = char_freq(s1)
    freq2: dict[str, int] = char_freq(s2)
    i: int = 0
    length: int = len(s1)
    while i < length:
        ch: str = s1[i]
        count1: int = freq1.get(ch, 0)
        count2: int = freq2.get(ch, 0)
        if count1 != count2:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test character frequency operations."""
    passed: int = 0

    freq: dict[str, int] = char_freq("aabbc")
    if freq["a"] == 2:
        passed = passed + 1

    if freq["c"] == 1:
        passed = passed + 1

    r3: str = most_frequent_char("aabbbcc")
    if r3 == "b":
        passed = passed + 1

    r4: int = unique_char_count("hello")
    if r4 == 4:
        passed = passed + 1

    if is_anagram("listen", "silent") == 1:
        passed = passed + 1

    if is_anagram("hello", "world") == 0:
        passed = passed + 1

    if unique_char_count("") == 0:
        passed = passed + 1

    r8: str = most_frequent_char("x")
    if r8 == "x":
        passed = passed + 1

    return passed
