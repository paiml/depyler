"""Semantic parity test: count negative numbers."""


def count_negative(nums: list[int]) -> int:
    count = 0
    for n in nums:
        if n < 0:
            count = count + 1
    return count


def main() -> None:
    print(count_negative([-1, 2, -3, 4, 5, -6]))


if __name__ == "__main__":
    main()
