"""Semantic parity test: max of two integers."""


def bigger(a: int, b: int) -> int:
    if a > b:
        return a
    return b


def main() -> None:
    print(bigger(7, 3))


if __name__ == "__main__":
    main()
