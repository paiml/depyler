"""Semantic parity test: tuple length."""


def tuple_size(t: tuple[int, int, int, int]) -> int:
    return len(t)


def main() -> None:
    data = (1, 2, 3, 4)
    print(tuple_size(data))


if __name__ == "__main__":
    main()
