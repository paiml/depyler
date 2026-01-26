"""Semantic parity test: ternary expression."""


def abs_value(n: int) -> int:
    return n if n >= 0 else -n


def main() -> None:
    print(abs_value(-5))


if __name__ == "__main__":
    main()
