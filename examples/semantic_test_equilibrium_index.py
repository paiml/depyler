"""Semantic parity test: find equilibrium index."""


def equilibrium_index(nums: list[int]) -> int:
    total = 0
    for n in nums:
        total = total + n
    left_sum = 0
    i = 0
    while i < len(nums):
        right_sum = total - left_sum - nums[i]
        if left_sum == right_sum:
            return i
        left_sum = left_sum + nums[i]
        i = i + 1
    return -1


def main() -> None:
    print(equilibrium_index([1, 3, 5, 2, 2]))


if __name__ == "__main__":
    main()
