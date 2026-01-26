"""Semantic parity test: prefix sum (first n elements)."""


def prefix_sum(nums: list[int], n: int) -> int:
    total = 0
    i = 0
    while i < n and i < len(nums):
        total = total + nums[i]
        i = i + 1
    return total


def main() -> None:
    print(prefix_sum([1, 2, 3, 4, 5], 3))


if __name__ == "__main__":
    main()
