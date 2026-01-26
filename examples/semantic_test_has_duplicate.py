"""Semantic parity test: check for duplicates."""


def has_duplicate(nums: list[int]) -> bool:
    i = 0
    while i < len(nums):
        j = i + 1
        while j < len(nums):
            if nums[i] == nums[j]:
                return True
            j = j + 1
        i = i + 1
    return False


def main() -> None:
    if has_duplicate([1, 2, 3, 2, 5]):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
