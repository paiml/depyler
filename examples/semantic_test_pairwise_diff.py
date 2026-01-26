"""Semantic parity test: sum of pairwise differences."""


def pairwise_diff(nums: list[int]) -> int:
    total = 0
    i = 1
    while i < len(nums):
        diff = nums[i] - nums[i - 1]
        if diff < 0:
            diff = -diff
        total = total + diff
        i = i + 1
    return total


def main() -> None:
    print(pairwise_diff([1, 4, 2, 8, 3]))


if __name__ == "__main__":
    main()
