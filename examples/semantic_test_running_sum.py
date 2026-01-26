"""Semantic parity test: running sum."""


def running_sum(nums: list[int]) -> int:
    total = 0
    for n in nums:
        total = total + n
    return total


def main() -> None:
    print(running_sum([1, 2, 3, 4, 5]))


if __name__ == "__main__":
    main()
