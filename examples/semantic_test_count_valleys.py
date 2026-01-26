"""Semantic parity test: count valleys."""


def count_valleys(nums: list[int]) -> int:
    if len(nums) < 3:
        return 0
    count = 0
    i = 1
    while i < len(nums) - 1:
        if nums[i] < nums[i - 1] and nums[i] < nums[i + 1]:
            count = count + 1
        i = i + 1
    return count


def main() -> None:
    print(count_valleys([3, 1, 4, 1, 5, 2, 6]))


if __name__ == "__main__":
    main()
