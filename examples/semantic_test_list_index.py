"""Semantic parity test: list indexing."""


def get_item(items: list[int], idx: int) -> int:
    return items[idx]


def main() -> None:
    nums = [10, 20, 30, 40, 50]
    print(get_item(nums, 2))


if __name__ == "__main__":
    main()
