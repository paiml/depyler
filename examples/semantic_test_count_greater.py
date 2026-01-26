"""Semantic parity test: count elements greater than threshold."""


def count_greater(nums: list[int], threshold: int) -> int:
    count = 0
    for n in nums:
        if n > threshold:
            count = count + 1
    return count


def main() -> None:
    print(count_greater([1, 5, 3, 8, 2, 9], 4))


if __name__ == "__main__":
    main()
