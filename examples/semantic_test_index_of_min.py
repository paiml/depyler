"""Semantic parity test: index of minimum."""


def index_of_min(nums: list[int]) -> int:
    min_idx = 0
    i = 1
    while i < len(nums):
        if nums[i] < nums[min_idx]:
            min_idx = i
        i = i + 1
    return min_idx


def main() -> None:
    print(index_of_min([3, 1, 4, 1, 5, 9, 2, 6]))


if __name__ == "__main__":
    main()
