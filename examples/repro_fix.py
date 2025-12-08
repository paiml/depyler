"""Reproduction for ord() on char from string iteration.

The issue: When iterating `for char in text`, Python gives string chars.
`ord(char)` gets the Unicode code point of that character.

In Rust, `for char in text.chars()` gives `char` type directly.
`ord(char)` incorrectly generates `char.chars().next().unwrap() as i32`
but `char` type has no `.chars()` method.

Error: E0599: no method named `chars` found for type `char`

Python: ord(char)  # char from `for char in text:`

WRONG Rust:
  char.chars().next().unwrap() as i32  // ERROR: char has no .chars()

RIGHT Rust:
  char as u32  # or `char as i32` for signed
"""


def compute_hash(text: str) -> int:
    """Hash a string by summing character ordinals."""
    total = 0
    for char in text:
        total += ord(char)
    return total
