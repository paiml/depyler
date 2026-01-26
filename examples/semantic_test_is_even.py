"""Semantic parity test: is even."""


def is_even(n: int) -> bool:
    return n % 2 == 0


def main() -> None:
    if is_even(42):
        print("even")
    else:
        print("odd")


if __name__ == "__main__":
    main()
