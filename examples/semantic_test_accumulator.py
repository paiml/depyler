"""Semantic parity test: accumulator pattern."""


def sum_squares(n: int) -> int:
    total = 0
    i = 1
    while i <= n:
        total = total + i * i
        i = i + 1
    return total


def main() -> None:
    print(sum_squares(5))


if __name__ == "__main__":
    main()
