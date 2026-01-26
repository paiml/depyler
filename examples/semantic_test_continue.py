"""Semantic parity test: continue statement."""


def sum_positive(nums: list[int]) -> int:
    total = 0
    for n in nums:
        if n < 0:
            continue
        total = total + n
    return total


def main() -> None:
    print(sum_positive([1, -2, 3, -4, 5]))


if __name__ == "__main__":
    main()
