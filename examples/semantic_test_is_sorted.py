"""Semantic parity test: check if sorted."""


def is_sorted(nums: list[int]) -> bool:
    i = 0
    while i < len(nums) - 1:
        if nums[i] > nums[i + 1]:
            return False
        i = i + 1
    return True


def main() -> None:
    if is_sorted([1, 2, 3, 4, 5]):
        print("sorted")
    else:
        print("not sorted")


if __name__ == "__main__":
    main()
