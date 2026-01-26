"""Semantic parity test: find missing number."""


def missing_number(nums: list[int], n: int) -> int:
    expected = n * (n + 1) // 2
    actual = 0
    for x in nums:
        actual = actual + x
    return expected - actual


def main() -> None:
    print(missing_number([0, 1, 3, 4, 5], 5))


if __name__ == "__main__":
    main()
