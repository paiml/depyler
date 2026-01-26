"""Semantic parity test: difference of max and min."""


def difference(nums: list[int]) -> int:
    return max(nums) - min(nums)


def main() -> None:
    print(difference([5, 2, 8, 1, 9]))


if __name__ == "__main__":
    main()
