"""Reproduction for isalpha() on char from string iteration.

The issue: When iterating `for char in text`, Python gives string chars.
`char.isalpha()` checks if that single-char string is alphabetic.

In Rust, `for char in text.chars()` gives `char` type directly.
`char.isalpha()` incorrectly generates `char.chars().all(|c| c.is_alphabetic())`
but `char` type has no `.chars()` method.

Error: E0599: no method named `chars` found for type `char`

Should generate: `char.is_alphabetic()` directly.
"""


def count_alpha(text: str) -> int:
    """Count alphabetic characters in a string."""
    count = 0
    for char in text:
        if char.isalpha():
            count += 1
    return count
