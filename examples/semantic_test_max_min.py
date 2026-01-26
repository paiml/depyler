"""Semantic parity test: max and min functions."""


def find_range(nums: list[int]) -> int:
    return max(nums) - min(nums)


def main() -> None:
    data = [3, 1, 4, 1, 5, 9, 2, 6]
    print(find_range(data))


if __name__ == "__main__":
    main()
