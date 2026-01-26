"""Semantic parity test: factorial."""


def factorial(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial(n - 1)


def main() -> None:
    print(factorial(5))


if __name__ == "__main__":
    main()
