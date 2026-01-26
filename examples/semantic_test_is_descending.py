"""Semantic parity test: check descending order."""


def is_descending(a: int, b: int, c: int) -> bool:
    return a > b and b > c


def main() -> None:
    if is_descending(3, 2, 1):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
