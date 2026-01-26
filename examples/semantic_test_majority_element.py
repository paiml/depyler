"""Semantic parity test: find majority element (Boyer-Moore)."""


def majority_element(nums: list[int]) -> int:
    candidate = nums[0]
    count = 1
    i = 1
    while i < len(nums):
        if count == 0:
            candidate = nums[i]
            count = 1
        elif nums[i] == candidate:
            count = count + 1
        else:
            count = count - 1
        i = i + 1
    return candidate


def main() -> None:
    print(majority_element([2, 2, 1, 1, 1, 2, 2]))


if __name__ == "__main__":
    main()
