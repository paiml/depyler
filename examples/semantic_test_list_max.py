"""Semantic parity test: find max in list."""


def find_max(nums: list[int]) -> int:
    result = nums[0]
    for n in nums:
        if n > result:
            result = n
    return result


def main() -> None:
    print(find_max([3, 1, 4, 1, 5, 9, 2, 6]))


if __name__ == "__main__":
    main()
