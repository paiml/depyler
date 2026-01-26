"""Semantic parity test: product of three."""


def triple_product(a: int, b: int, c: int) -> int:
    return a * b * c


def main() -> None:
    print(triple_product(2, 3, 4))


if __name__ == "__main__":
    main()
