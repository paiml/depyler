#!/usr/bin/env python3
"""
Demonstration of custom Rust attributes feature in Depyler.

This example shows how to use the custom_attribute annotation to add
custom Rust attributes to generated functions.
"""

# @depyler: custom_attribute = "inline"
def add_numbers(a: int, b: int) -> int:
    """Simple addition with inline hint."""
    return a + b


# @depyler: custom_attribute = "inline(always)"
# @depyler: performance_critical = "true"
def multiply_fast(x: int, y: int) -> int:
    """Performance-critical multiplication with aggressive inlining."""
    return x * y


# @depyler: custom_attribute = "must_use"
def calculate_checksum(data: list[int]) -> int:
    """Calculate checksum - result must be used."""
    checksum = 0
    for value in data:
        checksum ^= value
    return checksum


# @depyler: custom_attribute = "cold"
def handle_panic(message: str) -> None:
    """Error handler - rarely executed."""
    print(f"PANIC: {message}")


# @depyler: custom_attribute = "inline"
# @depyler: custom_attribute = "must_use"
def compute_hash(text: str) -> int:
    """Hash function with multiple attributes."""
    hash_val = 0
    for char in text:
        hash_val = (hash_val * 31 + ord(char)) % (2**32)
    return hash_val


if __name__ == "__main__":
    print(f"add_numbers(5, 3) = {add_numbers(5, 3)}")
    print(f"multiply_fast(4, 7) = {multiply_fast(4, 7)}")
    print(f"calculate_checksum([1, 2, 3, 4]) = {calculate_checksum([1, 2, 3, 4])}")
    print(f"compute_hash('hello') = {compute_hash('hello')}")
