"""Semantic parity test: dot product of two lists."""


def dot_product(a: list[int], b: list[int]) -> int:
    total = 0
    i = 0
    while i < len(a):
        total = total + a[i] * b[i]
        i = i + 1
    return total


def main() -> None:
    print(dot_product([1, 2, 3], [4, 5, 6]))


if __name__ == "__main__":
    main()
