"""Semantic parity test: tuple creation."""


def make_pair(a: int, b: int) -> tuple[int, int]:
    return (a, b)


def main() -> None:
    t = make_pair(3, 7)
    print(t[0])


if __name__ == "__main__":
    main()
