from typing import List, Optional

# Test Case 21: String length
def string_length(s: str) -> int:
    return len(s)

# Test Case 22: String concatenation
def concat_strings(a: str, b: str) -> str:
    return a + b

# Test Case 23: String repetition
def repeat_string(s: str, times: int) -> str:
    result: str = ""
    for _ in range(times):
        result += s
    return result

# Test Case 24: String uppercase
def to_uppercase(s: str) -> str:
    return s.upper()

# Test Case 25: String lowercase
def to_lowercase(s: str) -> str:
    return s.lower()

# Test Case 26: String contains substring
def contains_substring(text: str, substring: str) -> bool:
    return substring in text

# Test Case 27: String starts with
def starts_with(text: str, prefix: str) -> bool:
    return text.startswith(prefix)

# Test Case 28: String ends with
def ends_with(text: str, suffix: str) -> bool:
    return text.endswith(suffix)

# Test Case 29: String replace
def replace_substring(text: str, old: str, new: str) -> str:
    return text.replace(old, new)

# Test Case 30: String split
def split_string(text: str, delimiter: str) -> List[str]:
    return text.split(delimiter)

# Test Case 31: String strip whitespace
def strip_whitespace(s: str) -> str:
    return s.strip()

# Test Case 32: String is empty
def is_empty_string(s: str) -> bool:
    return len(s) == 0

# Test Case 33: String character at index
def char_at(s: str, index: int) -> Optional[str]:
    if 0 <= index < len(s):
        return s[index]
    return None

# Test Case 34: Count character occurrences
def count_char(text: str, char: str) -> int:
    count: int = 0
    for c in text:
        if c == char:
            count += 1
    return count

# Test Case 35: Reverse string
def reverse_string(s: str) -> str:
    result: str = ""
    for i in range(len(s) - 1, -1, -1):
        result += s[i]
    return result