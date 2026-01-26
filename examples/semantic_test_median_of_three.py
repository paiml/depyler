"""Semantic parity test: median of three."""


def median_of_three(a: int, b: int, c: int) -> int:
    if (a <= b and b <= c) or (c <= b and b <= a):
        return b
    if (b <= a and a <= c) or (c <= a and a <= b):
        return a
    return c


def main() -> None:
    print(median_of_three(5, 2, 8))


if __name__ == "__main__":
    main()
