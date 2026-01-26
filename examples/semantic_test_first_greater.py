"""Semantic parity test: find first greater than threshold."""


def first_greater(nums: list[int], threshold: int) -> int:
    for n in nums:
        if n > threshold:
            return n
    return -1


def main() -> None:
    print(first_greater([1, 3, 5, 7, 9], 4))


if __name__ == "__main__":
    main()
