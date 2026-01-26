"""Semantic parity test: second largest."""


def second_largest(nums: list[int]) -> int:
    first = nums[0]
    second = nums[0]
    for n in nums:
        if n > first:
            second = first
            first = n
        elif n > second and n != first:
            second = n
    return second


def main() -> None:
    print(second_largest([5, 2, 8, 1, 9, 3]))


if __name__ == "__main__":
    main()
