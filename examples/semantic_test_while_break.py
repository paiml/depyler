"""Semantic parity test: while loop with break."""


def find_first_even(nums: list[int]) -> int:
    i = 0
    while i < len(nums):
        if nums[i] % 2 == 0:
            return nums[i]
        i = i + 1
    return -1


def main() -> None:
    print(find_first_even([1, 3, 5, 6, 7]))


if __name__ == "__main__":
    main()
