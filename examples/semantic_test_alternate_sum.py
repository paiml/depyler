"""Semantic parity test: alternating sum."""


def alternate_sum(nums: list[int]) -> int:
    total = 0
    add = True
    for n in nums:
        if add:
            total = total + n
        else:
            total = total - n
        add = not add
    return total


def main() -> None:
    print(alternate_sum([1, 2, 3, 4, 5]))


if __name__ == "__main__":
    main()
