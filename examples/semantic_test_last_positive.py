"""Semantic parity test: find last positive."""


def last_positive(nums: list[int]) -> int:
    result = -1
    for n in nums:
        if n > 0:
            result = n
    return result


def main() -> None:
    print(last_positive([1, -2, 3, -4]))


if __name__ == "__main__":
    main()
