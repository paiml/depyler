"""Semantic parity test: sum odd numbers."""


def sum_odd(nums: list[int]) -> int:
    total = 0
    for n in nums:
        if n % 2 != 0:
            total = total + n
    return total


def main() -> None:
    print(sum_odd([1, 2, 3, 4, 5, 6, 7]))


if __name__ == "__main__":
    main()
