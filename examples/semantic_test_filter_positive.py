"""Semantic parity test: filter positive."""


def filter_positive(nums: list[int]) -> list[int]:
    result: list[int] = []
    for n in nums:
        if n > 0:
            result.append(n)
    return result


def main() -> None:
    data = filter_positive([-1, 2, -3, 4, 5])
    print(len(data))


if __name__ == "__main__":
    main()
