"""Hard string operation patterns for transpiler testing.

Tests: str.replace, str.split, str.join, f-strings, string slicing,
multi-line strings, escape sequences, string formatting.
"""


def reverse_string(s: str) -> str:
    """Reverse a string."""
    result: str = ""
    for char in s:
        result = char + result
    return result


def count_vowels(text: str) -> int:
    """Count vowels in text."""
    vowels: str = "aeiouAEIOU"
    count: int = 0
    for char in text:
        if char in vowels:
            count += 1
    return count


def is_palindrome(s: str) -> bool:
    """Check if string is palindrome."""
    cleaned: str = s.lower().strip()
    return cleaned == cleaned[::-1]


def caesar_cipher(text: str, shift: int) -> str:
    """Caesar cipher encryption."""
    result: list[str] = []
    for char in text:
        if char.isalpha():
            base: int = ord("a") if char.islower() else ord("A")
            shifted: int = (ord(char) - base + shift) % 26 + base
            result.append(chr(shifted))
        else:
            result.append(char)
    return "".join(result)


def word_frequency(text: str) -> dict[str, int]:
    """Count word frequencies."""
    counts: dict[str, int] = {}
    words: list[str] = text.lower().split()
    for word in words:
        if word in counts:
            counts[word] += 1
        else:
            counts[word] = 1
    return counts


def longest_common_prefix(strs: list[str]) -> str:
    """Find longest common prefix."""
    if not strs:
        return ""
    prefix: str = strs[0]
    for s in strs[1:]:
        while not s.startswith(prefix):
            prefix = prefix[:-1]
            if not prefix:
                return ""
    return prefix


def camel_to_snake(name: str) -> str:
    """Convert camelCase to snake_case."""
    result: list[str] = []
    for i, char in enumerate(name):
        if char.isupper() and i > 0:
            result.append("_")
        result.append(char.lower())
    return "".join(result)


def truncate(text: str, max_len: int) -> str:
    """Truncate text with ellipsis."""
    if len(text) <= max_len:
        return text
    return text[:max_len - 3] + "..."
