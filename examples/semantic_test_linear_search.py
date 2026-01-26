"""Semantic parity test: linear search."""


def linear_search(nums: list[int], target: int) -> int:
    i = 0
    while i < len(nums):
        if nums[i] == target:
            return i
        i = i + 1
    return -1


def main() -> None:
    data = [4, 2, 7, 1, 9]
    print(linear_search(data, 7))


if __name__ == "__main__":
    main()
