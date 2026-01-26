"""Semantic parity test: count odd numbers."""


def count_odd(nums: list[int]) -> int:
    count = 0
    for n in nums:
        if n % 2 != 0:
            count = count + 1
    return count


def main() -> None:
    print(count_odd([1, 2, 3, 4, 5, 6, 7]))


if __name__ == "__main__":
    main()
