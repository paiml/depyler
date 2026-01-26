"""Semantic parity test: break statement."""


def find_first_even(nums: list[int]) -> int:
    for n in nums:
        if n % 2 == 0:
            return n
    return -1


def main() -> None:
    print(find_first_even([1, 3, 5, 6, 7, 8]))


if __name__ == "__main__":
    main()
