"""Semantic parity test: check if contains zero."""


def contains_zero(nums: list[int]) -> bool:
    for n in nums:
        if n == 0:
            return True
    return False


def main() -> None:
    if contains_zero([1, 2, 0, 4]):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
