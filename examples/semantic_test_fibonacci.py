"""Semantic parity test: fibonacci."""


def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)


def main() -> None:
    print(fib(10))


if __name__ == "__main__":
    main()
