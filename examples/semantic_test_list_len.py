"""Semantic parity test: list length."""


def get_length(items: list[int]) -> int:
    return len(items)


def main() -> None:
    nums = [1, 2, 3, 4, 5, 6, 7]
    print(get_length(nums))


if __name__ == "__main__":
    main()
