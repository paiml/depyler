"""Semantic parity test: find min in list."""


def find_min(nums: list[int]) -> int:
    result = nums[0]
    for n in nums:
        if n < result:
            result = n
    return result


def main() -> None:
    print(find_min([3, 1, 4, 1, 5, 9, 2, 6]))


if __name__ == "__main__":
    main()
