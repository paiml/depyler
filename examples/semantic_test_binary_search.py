"""Semantic parity test: binary search."""


def binary_search(nums: list[int], target: int) -> int:
    left = 0
    right = len(nums) - 1
    while left <= right:
        mid = (left + right) // 2
        if nums[mid] == target:
            return mid
        if nums[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    return -1


def main() -> None:
    data = [1, 3, 5, 7, 9, 11, 13]
    print(binary_search(data, 7))


if __name__ == "__main__":
    main()
