"""Simple trie-like operations using flat arrays.

Tests: prefix counting, common prefix length, prefix matching.
"""


def common_prefix_len(s1: str, s2: str) -> int:
    """Find length of common prefix between two strings."""
    i: int = 0
    limit: int = len(s1)
    if len(s2) < limit:
        limit = len(s2)
    while i < limit:
        if s1[i] != s2[i]:
            return i
        i = i + 1
    return i


def longest_common_prefix(words: list[str]) -> str:
    """Find longest common prefix among all words."""
    if len(words) == 0:
        return ""
    prefix: str = words[0]
    i: int = 1
    while i < len(words):
        new_len: int = common_prefix_len(prefix, words[i])
        prefix = prefix[:new_len]
        if new_len == 0:
            return ""
        i = i + 1
    return prefix


def count_with_prefix(words: list[str], prefix: str) -> int:
    """Count words that start with a given prefix."""
    count: int = 0
    i: int = 0
    while i < len(words):
        word: str = words[i]
        if len(word) >= len(prefix):
            match: bool = True
            j: int = 0
            while j < len(prefix):
                if word[j] != prefix[j]:
                    match = False
                    j = len(prefix)
                else:
                    j = j + 1
            if match:
                count = count + 1
        i = i + 1
    return count


def unique_prefixes_count(words: list[str], length: int) -> int:
    """Count unique prefixes of given length."""
    seen: list[str] = []
    i: int = 0
    while i < len(words):
        word: str = words[i]
        if len(word) >= length:
            prefix: str = word[:length]
            found: bool = False
            j: int = 0
            while j < len(seen):
                if seen[j] == prefix:
                    found = True
                    j = len(seen)
                else:
                    j = j + 1
            if not found:
                seen.append(prefix)
        i = i + 1
    return len(seen)


def test_module() -> None:
    assert common_prefix_len("abc", "abd") == 2
    assert common_prefix_len("abc", "abc") == 3
    assert common_prefix_len("abc", "xyz") == 0
    words: list[str] = ["apple", "app", "application", "apt"]
    assert longest_common_prefix(words) == "ap"
    assert count_with_prefix(words, "app") == 3
    assert count_with_prefix(words, "apt") == 1
    assert count_with_prefix(words, "xyz") == 0
    assert unique_prefixes_count(words, 2) == 2
    assert unique_prefixes_count(words, 3) == 2
    assert longest_common_prefix([]) == ""
