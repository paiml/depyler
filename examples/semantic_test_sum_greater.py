"""Semantic parity test: sum elements greater than threshold."""


def sum_greater(nums: list[int], threshold: int) -> int:
    total = 0
    for n in nums:
        if n > threshold:
            total = total + n
    return total


def main() -> None:
    print(sum_greater([1, 5, 3, 8, 2, 9], 4))


if __name__ == "__main__":
    main()
