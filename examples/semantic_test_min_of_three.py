"""Semantic parity test: minimum of three."""


def min_of_three(a: int, b: int, c: int) -> int:
    if a <= b and a <= c:
        return a
    if b <= c:
        return b
    return c


def main() -> None:
    print(min_of_three(5, 2, 8))


if __name__ == "__main__":
    main()
