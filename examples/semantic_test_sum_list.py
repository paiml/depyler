"""Semantic parity test: sum function."""


def total(nums: list[int]) -> int:
    return sum(nums)


def main() -> None:
    print(total([1, 2, 3, 4, 5]))


if __name__ == "__main__":
    main()
