"""Semantic parity test: count even and odd."""


def count_even_odd(nums: list[int]) -> int:
    even = 0
    odd = 0
    for n in nums:
        if n % 2 == 0:
            even = even + 1
        else:
            odd = odd + 1
    return even - odd


def main() -> None:
    print(count_even_odd([1, 2, 3, 4, 5, 6]))


if __name__ == "__main__":
    main()
