"""Semantic parity test: string repeat."""


def repeat_str(s: str, n: int) -> str:
    return s * n


def main() -> None:
    print(repeat_str("ab", 3))


if __name__ == "__main__":
    main()
