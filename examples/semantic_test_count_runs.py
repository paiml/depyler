"""Semantic parity test: count runs."""


def count_runs(nums: list[int]) -> int:
    if len(nums) == 0:
        return 0
    count = 1
    i = 1
    while i < len(nums):
        if nums[i] != nums[i - 1]:
            count = count + 1
        i = i + 1
    return count


def main() -> None:
    print(count_runs([1, 1, 2, 2, 2, 3, 3]))


if __name__ == "__main__":
    main()
