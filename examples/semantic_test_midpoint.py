"""Semantic parity test: midpoint of two numbers."""


def midpoint(a: int, b: int) -> int:
    return (a + b) // 2


def main() -> None:
    print(midpoint(10, 20))


if __name__ == "__main__":
    main()
