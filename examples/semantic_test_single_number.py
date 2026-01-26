"""Semantic parity test: find single number (XOR)."""


def single_number(nums: list[int]) -> int:
    result = 0
    for n in nums:
        result = result ^ n
    return result


def main() -> None:
    print(single_number([4, 1, 2, 1, 2]))


if __name__ == "__main__":
    main()
