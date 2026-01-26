"""Semantic parity test: weighted sum."""


def weighted_sum(nums: list[int]) -> int:
    total = 0
    i = 0
    while i < len(nums):
        total = total + nums[i] * (i + 1)
        i = i + 1
    return total


def main() -> None:
    print(weighted_sum([1, 2, 3, 4]))


if __name__ == "__main__":
    main()
