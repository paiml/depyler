"""Semantic parity test: count elements in range."""


def count_in_range(nums: list[int], lo: int, hi: int) -> int:
    count = 0
    for n in nums:
        if n >= lo and n <= hi:
            count = count + 1
    return count


def main() -> None:
    print(count_in_range([1, 5, 3, 8, 2, 9], 3, 7))


if __name__ == "__main__":
    main()
