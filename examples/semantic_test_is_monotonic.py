"""Semantic parity test: check if monotonic."""


def is_monotonic(nums: list[int]) -> bool:
    if len(nums) < 2:
        return True
    increasing = True
    decreasing = True
    i = 1
    while i < len(nums):
        if nums[i] > nums[i - 1]:
            decreasing = False
        if nums[i] < nums[i - 1]:
            increasing = False
        i = i + 1
    return increasing or decreasing


def main() -> None:
    if is_monotonic([1, 2, 3, 4, 5]):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
