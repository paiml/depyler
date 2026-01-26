"""Semantic parity test: tuple unpacking."""


def sum_tuple(t: tuple[int, int, int]) -> int:
    a, b, c = t
    return a + b + c


def main() -> None:
    print(sum_tuple((1, 2, 3)))


if __name__ == "__main__":
    main()
