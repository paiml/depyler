"""Semantic parity test: integer square root (simplified)."""


def integer_sqrt(n: int) -> int:
    if n < 0:
        return -1
    i = 0
    next_i: int = i + 1
    square: int = next_i * next_i
    while square <= n:
        i = i + 1
        next_i = i + 1
        square = next_i * next_i
    return i


def main() -> None:
    print(integer_sqrt(17))


if __name__ == "__main__":
    main()
