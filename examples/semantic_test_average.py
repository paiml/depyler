"""Semantic parity test: average of list."""


def average(nums: list[int]) -> int:
    total = 0
    for n in nums:
        total = total + n
    return total // len(nums)


def main() -> None:
    print(average([10, 20, 30, 40, 50]))


if __name__ == "__main__":
    main()
