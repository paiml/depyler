"""Semantic parity test: square each element."""


def square_list(nums: list[int]) -> list[int]:
    result: list[int] = []
    for n in nums:
        result.append(n * n)
    return result


def main() -> None:
    data = square_list([1, 2, 3, 4, 5])
    print(sum(data))


if __name__ == "__main__":
    main()
