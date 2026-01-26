"""Semantic parity test: count peaks."""


def count_peaks(nums: list[int]) -> int:
    if len(nums) < 3:
        return 0
    count = 0
    i = 1
    while i < len(nums) - 1:
        if nums[i] > nums[i - 1] and nums[i] > nums[i + 1]:
            count = count + 1
        i = i + 1
    return count


def main() -> None:
    print(count_peaks([1, 3, 2, 4, 1, 5, 2]))


if __name__ == "__main__":
    main()
