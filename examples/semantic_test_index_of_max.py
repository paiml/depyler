"""Semantic parity test: index of maximum."""


def index_of_max(nums: list[int]) -> int:
    max_idx = 0
    i = 1
    while i < len(nums):
        if nums[i] > nums[max_idx]:
            max_idx = i
        i = i + 1
    return max_idx


def main() -> None:
    print(index_of_max([3, 1, 4, 1, 5, 9, 2, 6]))


if __name__ == "__main__":
    main()
