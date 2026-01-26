"""Semantic parity test: check any negative."""


def any_negative(nums: list[int]) -> bool:
    for n in nums:
        if n < 0:
            return True
    return False


def main() -> None:
    if any_negative([1, -2, 3]):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
