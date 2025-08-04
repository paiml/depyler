# @depyler: string_strategy = "zero_copy"
# @depyler: optimization_level = "size"

def reverse_string(s: str) -> str:
    """Reverse a string"""
    result = ""
    for i in range(len(s) - 1, -1, -1):
        result = result + s[i]
    
    return result

def count_vowels(s: str) -> int:
    """Count vowels in string"""
    vowels = "aeiouAEIOU"
    count = 0
    
    for char in s:
        if char in vowels:
            count = count + 1
    
    return count

def is_palindrome_simple(s: str) -> bool:
    """Check if string is palindrome"""
    cleaned = ""
    for char in s:
        if char.isalpha():
            cleaned = cleaned + char.lower()
    
    length = len(cleaned)
    for i in range(length // 2):
        if cleaned[i] != cleaned[length - 1 - i]:
            return False
    
    return True

def count_words(text: str) -> int:
    """Count words in text"""
    if not text:
        return 0
    
    words = text.split()
    return len(words)

def capitalize_words(text: str) -> str:
    """Capitalize first letter of each word"""
    if not text:
        return ""
    
    words = text.split()
    result_words = []
    
    for word in words:
        if word:
            capitalized = word[0].upper() + word[1:].lower()
            result_words.append(capitalized)
    
    return " ".join(result_words)