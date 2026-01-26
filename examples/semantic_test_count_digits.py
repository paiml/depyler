"""Semantic parity test: count digits."""


def count_digits(n: int) -> int:
    if n == 0:
        return 1
    count = 0
    x = n
    if x < 0:
        x = -x
    while x > 0:
        count = count + 1
        x = x // 10
    return count


def main() -> None:
    print(count_digits(12345))


if __name__ == "__main__":
    main()
