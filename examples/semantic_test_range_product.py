"""Semantic parity test: product of range."""


def range_product(start: int, end: int) -> int:
    result = 1
    i = start
    while i <= end:
        result = result * i
        i = i + 1
    return result


def main() -> None:
    print(range_product(1, 5))


if __name__ == "__main__":
    main()
