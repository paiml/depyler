"""Semantic parity test: suffix sum (last n elements)."""


def suffix_sum(nums: list[int], n: int) -> int:
    total = 0
    start = len(nums) - n
    if start < 0:
        start = 0
    i = start
    while i < len(nums):
        total = total + nums[i]
        i = i + 1
    return total


def main() -> None:
    print(suffix_sum([1, 2, 3, 4, 5], 3))


if __name__ == "__main__":
    main()
