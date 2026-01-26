"""Semantic parity test: absolute value."""


def absolute(n: int) -> int:
    if n < 0:
        return -n
    return n


def main() -> None:
    print(absolute(-7))


if __name__ == "__main__":
    main()
