"""Semantic parity test: if/else control flow."""


def max_val(a: int, b: int) -> int:
    if a > b:
        return a
    else:
        return b


def main() -> None:
    print(max_val(5, 8))


if __name__ == "__main__":
    main()
