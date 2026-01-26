"""Semantic parity test: check all positive."""


def all_positive(nums: list[int]) -> bool:
    for n in nums:
        if n <= 0:
            return False
    return True


def main() -> None:
    if all_positive([1, 2, 3, 4, 5]):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
