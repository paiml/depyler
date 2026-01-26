"""Semantic parity test: function composition."""


def double(n: int) -> int:
    return n * 2


def add_one(n: int) -> int:
    return n + 1


def main() -> None:
    result = add_one(double(5))
    print(result)


if __name__ == "__main__":
    main()
