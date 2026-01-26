"""Semantic parity test: element-wise sum."""


def element_wise_sum(a: list[int], b: list[int]) -> int:
    total = 0
    i = 0
    while i < len(a):
        total = total + a[i] + b[i]
        i = i + 1
    return total


def main() -> None:
    print(element_wise_sum([1, 2, 3], [4, 5, 6]))


if __name__ == "__main__":
    main()
