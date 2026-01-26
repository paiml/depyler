"""Semantic parity test: count positive numbers."""


def count_positive(nums: list[int]) -> int:
    count = 0
    for n in nums:
        if n > 0:
            count = count + 1
    return count


def main() -> None:
    print(count_positive([-1, 2, -3, 4, 5, -6]))


if __name__ == "__main__":
    main()
