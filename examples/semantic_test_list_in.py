"""Semantic parity test: list membership."""


def contains(nums: list[int], target: int) -> bool:
    return target in nums


def main() -> None:
    data = [1, 2, 3, 4, 5]
    if contains(data, 3):
        print("found")
    else:
        print("not found")


if __name__ == "__main__":
    main()
