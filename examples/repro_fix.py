"""Reproduction for DEPYLER-0771: math.isqrt maps to nonexistent std::f64::isqrt.

Python's math.isqrt() computes the integer square root (floor of sqrt).
The transpiler incorrectly maps this to std::f64::isqrt which doesn't exist.

Expected: (n as f64).sqrt().floor() as i64 or similar
Actual:   std::f64::isqrt (doesn't exist)
"""
from math import isqrt


def is_perfect_square(n: int) -> bool:
    """Check if n is a perfect square using isqrt."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


def main() -> None:
    for i in [0, 1, 4, 9, 16, 25, 26, 100]:
        result = is_perfect_square(i)
        print(f"{i}: {result}")


if __name__ == "__main__":
    main()
