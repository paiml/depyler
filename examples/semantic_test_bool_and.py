"""Semantic parity test: boolean and."""


def both_positive(a: int, b: int) -> bool:
    return a > 0 and b > 0


def main() -> None:
    result = both_positive(5, 3)
    if result:
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
