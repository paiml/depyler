"""Semantic parity test: check strictly decreasing."""


def strictly_decreasing(nums: list[int]) -> bool:
    i = 1
    while i < len(nums):
        if nums[i] >= nums[i - 1]:
            return False
        i = i + 1
    return True


def main() -> None:
    if strictly_decreasing([5, 4, 3, 2, 1]):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
