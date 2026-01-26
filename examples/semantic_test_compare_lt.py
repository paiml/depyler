"""Semantic parity test: less than comparison."""


def is_smaller(a: int, b: int) -> bool:
    return a < b


def main() -> None:
    if is_smaller(3, 5):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
