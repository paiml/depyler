"""Semantic parity test: check all equal."""


def all_equal(a: int, b: int, c: int) -> bool:
    return a == b and b == c


def main() -> None:
    if all_equal(5, 5, 5):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
