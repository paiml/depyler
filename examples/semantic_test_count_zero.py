"""Semantic parity test: count zeros."""


def count_zero(nums: list[int]) -> int:
    count = 0
    for n in nums:
        if n == 0:
            count = count + 1
    return count


def main() -> None:
    print(count_zero([0, 1, 0, 2, 0, 3]))


if __name__ == "__main__":
    main()
