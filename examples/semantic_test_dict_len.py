"""Semantic parity test: dict length."""


def dict_size(d: dict[str, int]) -> int:
    return len(d)


def main() -> None:
    data = {"one": 1, "two": 2, "three": 3, "four": 4}
    print(dict_size(data))


if __name__ == "__main__":
    main()
