"""Semantic parity test: equality comparison."""


def are_equal(a: int, b: int) -> bool:
    return a == b


def main() -> None:
    if are_equal(5, 5):
        print("equal")
    else:
        print("not equal")


if __name__ == "__main__":
    main()
