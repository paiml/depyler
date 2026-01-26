"""Semantic parity test: sum of range."""


def range_sum(start: int, end: int) -> int:
    total = 0
    i = start
    while i <= end:
        total = total + i
        i = i + 1
    return total


def main() -> None:
    print(range_sum(1, 10))


if __name__ == "__main__":
    main()
