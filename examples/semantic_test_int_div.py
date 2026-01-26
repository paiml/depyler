"""Semantic parity test: integer division."""


def divide(a: int, b: int) -> int:
    return a // b


def main() -> None:
    print(divide(17, 5))


if __name__ == "__main__":
    main()
