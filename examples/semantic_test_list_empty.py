"""Semantic parity test: empty list check."""


def is_empty(nums: list[int]) -> bool:
    return len(nums) == 0


def main() -> None:
    if is_empty([]):
        print("empty")
    else:
        print("not empty")


if __name__ == "__main__":
    main()
