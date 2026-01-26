"""Semantic parity test: list pop."""


def get_last(nums: list[int]) -> int:
    return nums.pop()


def main() -> None:
    data = [1, 2, 3, 4, 5]
    print(get_last(data))


if __name__ == "__main__":
    main()
