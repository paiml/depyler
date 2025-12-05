# DEPYLER-0715: String iteration char comparison type mismatch
# Python pattern: for char in string1: for p in string2: if char == p
# Problem: Rust generates String vs char comparison which doesn't compile
# Expected: Both iterators should yield same type (both char or both String)

def find_punctuation(text: str, punctuation: str) -> int:
    """Count punctuation characters in text."""
    count: int = 0
    for char in text:
        for p in punctuation:
            if char == p:  # This comparison now works in Rust
                count += 1
    return count
