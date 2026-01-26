"""Semantic parity test: while loop."""


def countdown(n: int) -> int:
    total = 0
    while n > 0:
        total = total + n
        n = n - 1
    return total


def main() -> None:
    print(countdown(5))


if __name__ == "__main__":
    main()
