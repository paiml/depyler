# @depyler: string_strategy = "zero_copy"
# @depyler: optimization_level = "size"
from typing import Dict, List, Set

def word_frequency(text: str) -> Dict[str, int]:
    """Count word frequencies in text"""
    words = text.lower().split()
    frequency: Dict[str, int] = {}
    
    for word in words:
        # Remove punctuation
        clean_word = ""
        for char in word:
            if char.isalpha():
                clean_word += char
        
        if clean_word:
            if clean_word in frequency:
                frequency[clean_word] += 1
            else:
                frequency[clean_word] = 1
    
    return frequency

def find_anagrams(words: List[str]) -> List[List[str]]:
    """Group words that are anagrams of each other"""
    groups: Dict[str, List[str]] = {}
    
    for word in words:
        # Sort characters to create signature
        sorted_chars = "".join(sorted(word.lower()))
        
        if sorted_chars in groups:
            groups[sorted_chars].append(word)
        else:
            groups[sorted_chars] = [word]
    
    # Return groups with more than one word
    result: List[List[str]] = []
    for group in groups.values():
        if len(group) > 1:
            result.append(group)
    
    return result

def longest_common_prefix(strings: List[str]) -> str:
    """Find the longest common prefix among strings"""
    if not strings:
        return ""
    
    if len(strings) == 1:
        return strings[0]
    
    # Find minimum length
    min_length = len(strings[0])
    for s in strings[1:]:
        if len(s) < min_length:
            min_length = len(s)
    
    # Check character by character
    prefix = ""
    for i in range(min_length):
        char = strings[0][i]
        
        # Check if all strings have same character at position i
        all_match = True
        for s in strings[1:]:
            if s[i] != char:
                all_match = False
                break
        
        if all_match:
            prefix += char
        else:
            break
    
    return prefix

def is_palindrome(s: str) -> bool:
    """Check if string is a palindrome (ignoring case and non-alphanumeric)"""
    # Build cleaned string
    cleaned = ""
    for char in s.lower():
        if char.isalnum():
            cleaned += char
    
    # Check if cleaned string reads same forwards and backwards
    length = len(cleaned)
    for i in range(length // 2):
        if cleaned[i] != cleaned[length - 1 - i]:
            return False
    
    return True