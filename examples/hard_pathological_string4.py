# Pathological string: Character frequency with dict[str, int]
# Tests: building frequency maps from strings, comparing distributions


def char_frequency(text: str) -> dict[str, int]:
    """Count frequency of each character."""
    freq: dict[str, int] = {}
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if c in freq:
            freq[c] = freq[c] + 1
        else:
            freq[c] = 1
        i = i + 1
    return freq


def most_frequent_char(text: str) -> str:
    """Find the most frequent character. Ties broken by first occurrence."""
    if len(text) == 0:
        return ""
    freq: dict[str, int] = char_frequency(text)
    best_char: str = text[0]
    best_count: int = 0
    i: int = 0
    while i < len(text):
        c: str = text[i]
        count: int = freq[c]
        if count > best_count:
            best_count = count
            best_char = c
        i = i + 1
    return best_char


def unique_char_count(text: str) -> int:
    """Count number of unique characters in text."""
    freq: dict[str, int] = char_frequency(text)
    return len(freq)


def has_duplicate_chars(text: str) -> bool:
    """Check if any character appears more than once."""
    freq: dict[str, int] = char_frequency(text)
    # Check each character in order
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if freq[c] > 1:
            return True
        i = i + 1
    return False


def count_distinct_words(text: str) -> int:
    """Count distinct words (space-separated)."""
    words: dict[str, int] = {}
    current: str = ""
    i: int = 0
    while i < len(text):
        c: str = text[i]
        if c == " ":
            if len(current) > 0:
                words[current] = 1
                current = ""
        else:
            current = current + c
        i = i + 1
    if len(current) > 0:
        words[current] = 1
    return len(words)


def test_module() -> int:
    passed: int = 0
    # Test 1: char frequency
    freq: dict[str, int] = char_frequency("aabbc")
    if freq["a"] == 2:
        passed = passed + 1
    # Test 2: most frequent
    if most_frequent_char("abracadabra") == "a":
        passed = passed + 1
    # Test 3: unique count
    if unique_char_count("hello") == 4:
        passed = passed + 1
    # Test 4: has duplicates
    if has_duplicate_chars("hello") == True:
        passed = passed + 1
    # Test 5: no duplicates
    if has_duplicate_chars("abcde") == False:
        passed = passed + 1
    # Test 6: distinct words
    if count_distinct_words("the cat and the dog") == 4:
        passed = passed + 1
    # Test 7: empty string unique
    if unique_char_count("") == 0:
        passed = passed + 1
    return passed
