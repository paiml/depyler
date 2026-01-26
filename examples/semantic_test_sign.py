"""Semantic parity test: sign function."""


def sign(n: int) -> int:
    if n > 0:
        return 1
    if n < 0:
        return -1
    return 0


def main() -> None:
    print(sign(-42))


if __name__ == "__main__":
    main()
