"""Semantic parity test: sum within bounds."""


def bounded_sum(nums: list[int], lo: int, hi: int) -> int:
    total = 0
    for n in nums:
        if n >= lo and n <= hi:
            total = total + n
    return total


def main() -> None:
    print(bounded_sum([1, 5, 3, 8, 2, 9], 3, 7))


if __name__ == "__main__":
    main()
