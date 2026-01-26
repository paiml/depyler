"""Semantic parity test: count divisible by k."""


def count_divisible(nums: list[int], k: int) -> int:
    count = 0
    for n in nums:
        if n % k == 0:
            count = count + 1
    return count


def main() -> None:
    print(count_divisible([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3))


if __name__ == "__main__":
    main()
