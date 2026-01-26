"""Semantic parity test: check strictly increasing."""


def strictly_increasing(nums: list[int]) -> bool:
    i = 1
    while i < len(nums):
        if nums[i] <= nums[i - 1]:
            return False
        i = i + 1
    return True


def main() -> None:
    if strictly_increasing([1, 2, 3, 4, 5]):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
