"""Semantic parity test: list slicing."""


def first_three(nums: list[int]) -> list[int]:
    return nums[:3]


def main() -> None:
    data = [1, 2, 3, 4, 5]
    result = first_three(data)
    print(len(result))


if __name__ == "__main__":
    main()
