"""Semantic parity test: find first positive."""


def first_positive(nums: list[int]) -> int:
    for n in nums:
        if n > 0:
            return n
    return -1


def main() -> None:
    print(first_positive([-1, -2, 3, 4]))


if __name__ == "__main__":
    main()
