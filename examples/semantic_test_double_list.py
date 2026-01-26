"""Semantic parity test: double each element."""


def double_list(nums: list[int]) -> list[int]:
    result: list[int] = []
    for n in nums:
        result.append(n * 2)
    return result


def main() -> None:
    data = double_list([1, 2, 3, 4, 5])
    print(sum(data))


if __name__ == "__main__":
    main()
