"""String reversal operations: reverse words, reverse vowels, reverse pairs."""


def reverse_string(s: str) -> str:
    """Reverse an entire string."""
    chars: list[str] = []
    i: int = 0
    while i < len(s):
        chars.append(s[i])
        i = i + 1
    left: int = 0
    right: int = len(chars) - 1
    while left < right:
        tmp: str = chars[left]
        chars[left] = chars[right]
        chars[right] = tmp
        left = left + 1
        right = right - 1
    result: str = ""
    j: int = 0
    while j < len(chars):
        result = result + chars[j]
        j = j + 1
    return result


def reverse_words(s: str) -> str:
    """Reverse the order of words in a string."""
    words: list[str] = []
    current: str = ""
    i: int = 0
    while i < len(s):
        if s[i] == " ":
            if len(current) > 0:
                words.append(current)
                current = ""
        else:
            current = current + s[i]
        i = i + 1
    if len(current) > 0:
        words.append(current)
    result: str = ""
    idx: int = len(words) - 1
    while idx >= 0:
        if idx < len(words) - 1:
            result = result + " "
        result = result + words[idx]
        idx = idx - 1
    return result


def is_vowel(c: str) -> int:
    """Check if character is a vowel. Returns 1 or 0."""
    if c == "a" or c == "e" or c == "i" or c == "o" or c == "u":
        return 1
    if c == "A" or c == "E" or c == "I" or c == "O" or c == "U":
        return 1
    return 0


def reverse_vowels(s: str) -> str:
    """Reverse only the vowels in a string."""
    chars: list[str] = []
    i: int = 0
    while i < len(s):
        chars.append(s[i])
        i = i + 1
    left: int = 0
    right: int = len(chars) - 1
    while left < right:
        while left < right and is_vowel(chars[left]) == 0:
            left = left + 1
        while left < right and is_vowel(chars[right]) == 0:
            right = right - 1
        if left < right:
            tmp: str = chars[left]
            chars[left] = chars[right]
            chars[right] = tmp
            left = left + 1
            right = right - 1
    result: str = ""
    j: int = 0
    while j < len(chars):
        result = result + chars[j]
        j = j + 1
    return result


def test_module() -> int:
    passed: int = 0

    if reverse_string("hello") == "olleh":
        passed = passed + 1

    if reverse_string("") == "":
        passed = passed + 1

    if reverse_words("hello world foo") == "foo world hello":
        passed = passed + 1

    if reverse_words("single") == "single":
        passed = passed + 1

    if reverse_vowels("hello") == "holle":
        passed = passed + 1

    if reverse_vowels("xyz") == "xyz":
        passed = passed + 1

    if is_vowel("a") == 1:
        passed = passed + 1

    return passed
