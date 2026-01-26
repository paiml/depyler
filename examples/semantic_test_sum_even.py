"""Semantic parity test: sum even numbers."""


def sum_even(nums: list[int]) -> int:
    total = 0
    for n in nums:
        if n % 2 == 0:
            total = total + n
    return total


def main() -> None:
    print(sum_even([1, 2, 3, 4, 5, 6]))


if __name__ == "__main__":
    main()
