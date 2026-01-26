"""Simplest possible semantic parity test: add two integers.

DEPYLER-1361: Achieve ONE true success.

If this cannot achieve semantic parity, the architecture is falsified.
"""


def add(a: int, b: int) -> int:
    """Add two integers."""
    return a + b


def main() -> None:
    """Test the add function."""
    result = add(2, 3)
    print(result)


if __name__ == "__main__":
    main()
