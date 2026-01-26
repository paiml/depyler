"""Semantic parity test: sum of pairs."""


def pair_sum(a: int, b: int, c: int, d: int) -> int:
    return (a + b) + (c + d)


def main() -> None:
    print(pair_sum(1, 2, 3, 4))


if __name__ == "__main__":
    main()
