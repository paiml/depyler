"""Semantic parity test: product of list."""


def product(nums: list[int]) -> int:
    result = 1
    for n in nums:
        result = result * n
    return result


def main() -> None:
    print(product([1, 2, 3, 4, 5]))


if __name__ == "__main__":
    main()
