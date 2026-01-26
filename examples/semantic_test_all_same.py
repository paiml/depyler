"""Semantic parity test: check if all elements are same."""


def all_same(nums: list[int]) -> bool:
    if len(nums) == 0:
        return True
    first = nums[0]
    for n in nums:
        if n != first:
            return False
    return True


def main() -> None:
    if all_same([5, 5, 5, 5]):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
