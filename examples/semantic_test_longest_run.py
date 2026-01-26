"""Semantic parity test: longest run of same value."""


def longest_run(nums: list[int]) -> int:
    if len(nums) == 0:
        return 0
    max_run = 1
    current_run = 1
    i = 1
    while i < len(nums):
        if nums[i] == nums[i - 1]:
            current_run = current_run + 1
            if current_run > max_run:
                max_run = current_run
        else:
            current_run = 1
        i = i + 1
    return max_run


def main() -> None:
    print(longest_run([1, 1, 2, 2, 2, 3, 3]))


if __name__ == "__main__":
    main()
