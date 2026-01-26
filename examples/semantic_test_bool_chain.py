"""Semantic parity test: chained boolean operations."""


def check_range(x: int) -> bool:
    return x > 0 and x < 100


def main() -> None:
    if check_range(50):
        print("in range")
    else:
        print("out of range")


if __name__ == "__main__":
    main()
