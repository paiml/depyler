"""Semantic parity test: check if between."""


def is_between(x: int, lo: int, hi: int) -> bool:
    return lo <= x and x <= hi


def main() -> None:
    if is_between(5, 1, 10):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
