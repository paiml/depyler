"""Semantic parity test: find last less than threshold."""


def last_less(nums: list[int], threshold: int) -> int:
    result = -1
    for n in nums:
        if n < threshold:
            result = n
    return result


def main() -> None:
    print(last_less([1, 3, 5, 7, 9], 6))


if __name__ == "__main__":
    main()
