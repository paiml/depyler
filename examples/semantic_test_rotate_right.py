"""Semantic parity test: rotate array right."""


def rotate_right(nums: list[int], k: int) -> int:
    n = len(nums)
    k = k % n
    return nums[n - k]


def main() -> None:
    print(rotate_right([1, 2, 3, 4, 5], 2))


if __name__ == "__main__":
    main()
