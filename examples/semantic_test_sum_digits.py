"""Semantic parity test: sum of digits."""


def sum_digits(n: int) -> int:
    total = 0
    x = n
    if x < 0:
        x = -x
    while x > 0:
        total = total + x % 10
        x = x // 10
    return total


def main() -> None:
    print(sum_digits(12345))


if __name__ == "__main__":
    main()
