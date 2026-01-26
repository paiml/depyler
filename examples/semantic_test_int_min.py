"""Semantic parity test: min of two integers."""


def smaller(a: int, b: int) -> int:
    if a < b:
        return a
    return b


def main() -> None:
    print(smaller(7, 3))


if __name__ == "__main__":
    main()
