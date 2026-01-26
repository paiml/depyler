"""Semantic parity test: count trailing zeros in factorial."""


def trailing_zeros(n: int) -> int:
    count = 0
    while n >= 5:
        n = n // 5
        count = count + n
    return count


def main() -> None:
    print(trailing_zeros(25))


if __name__ == "__main__":
    main()
