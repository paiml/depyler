"""Semantic parity test: list operations."""


def sum_list(nums: list[int]) -> int:
    """Sum a list of integers."""
    total = 0
    for n in nums:
        total = total + n
    return total


def main() -> None:
    """Test sum_list."""
    print(sum_list([1, 2, 3, 4, 5]))


if __name__ == "__main__":
    main()
