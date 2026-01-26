"""Semantic parity test: multiple function calls."""


def double(x: int) -> int:
    return x * 2


def increment(x: int) -> int:
    return x + 1


def main() -> None:
    result = double(increment(5))
    print(result)


if __name__ == "__main__":
    main()
