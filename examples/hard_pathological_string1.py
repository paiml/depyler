# Pathological string: Character-by-character parsing and tokenizing
# Tests: string indexing, char comparison, token extraction loops


def tokenize_words(text: str) -> list[str]:
    """Split text into words by spaces, character by character."""
    words: list[str] = []
    current: str = ""
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if c == " ":
            if len(current) > 0:
                words.append(current)
                current = ""
        else:
            current = current + c
        i = i + 1
    if len(current) > 0:
        words.append(current)
    return words


def count_vowels(text: str) -> int:
    """Count vowels in text (case-insensitive via checking both)."""
    count: int = 0
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if c == "a" or c == "e" or c == "i" or c == "o" or c == "u":
            count = count + 1
        elif c == "A" or c == "E" or c == "I" or c == "O" or c == "U":
            count = count + 1
        i = i + 1
    return count


def extract_digits(text: str) -> str:
    """Extract only digit characters from text."""
    result: str = ""
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if c >= "0" and c <= "9":
            result = result + c
        i = i + 1
    return result


def is_palindrome(text: str) -> bool:
    """Check if string is a palindrome character by character."""
    left: int = 0
    right: int = len(text) - 1
    while left < right:
        if text[left] != text[right]:
            return False
        left = left + 1
        right = right - 1
    return True


def reverse_words(text: str) -> str:
    """Reverse order of words in text."""
    words: list[str] = tokenize_words(text)
    if len(words) == 0:
        return ""
    result: str = ""
    i: int = len(words) - 1
    while i >= 0:
        if i < len(words) - 1:
            result = result + " "
        result = result + words[i]
        i = i - 1
    return result


def test_module() -> int:
    passed: int = 0
    # Test 1: tokenize
    words: list[str] = tokenize_words("hello world foo")
    if len(words) == 3:
        passed = passed + 1
    # Test 2: count vowels
    if count_vowels("hello world") == 3:
        passed = passed + 1
    # Test 3: extract digits
    if extract_digits("abc123def456") == "123456":
        passed = passed + 1
    # Test 4: palindrome true
    if is_palindrome("racecar") == True:
        passed = passed + 1
    # Test 5: palindrome false
    if is_palindrome("hello") == False:
        passed = passed + 1
    # Test 6: reverse words
    if reverse_words("hello world") == "world hello":
        passed = passed + 1
    # Test 7: empty string
    if count_vowels("") == 0:
        passed = passed + 1
    return passed
