"""Semantic parity test: tuple indexing."""


def get_second(t: tuple[int, int, int]) -> int:
    return t[1]


def main() -> None:
    data = (10, 20, 30)
    print(get_second(data))


if __name__ == "__main__":
    main()
