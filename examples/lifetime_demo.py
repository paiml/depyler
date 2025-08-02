# Example demonstrating lifetime inference

def get_length(s: str) -> int:
    """Get the length of a string without consuming it"""
    return len(s)

def first_word(s: str) -> str:
    """Extract the first word from a string"""
    words = s.split()
    if words:
        return words[0]
    return ""

def append_exclamation(s: str) -> str:
    """Append an exclamation mark to a string"""
    s = s + "!"
    return s

def longest(x: str, y: str) -> str:
    """Return the longest of two strings"""
    if len(x) > len(y):
        return x
    else:
        return y

def modify_string(s: str):
    """Modify a string in place"""
    s += " modified"
    return None