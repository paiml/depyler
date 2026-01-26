"""Semantic parity test: check ascending order."""


def is_ascending(a: int, b: int, c: int) -> bool:
    return a < b and b < c


def main() -> None:
    if is_ascending(1, 2, 3):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
