"""Semantic parity test: recursive digit sum."""


def digit_sum(n: int) -> int:
    if n < 10:
        return n
    return n % 10 + digit_sum(n // 10)


def main() -> None:
    print(digit_sum(12345))


if __name__ == "__main__":
    main()
