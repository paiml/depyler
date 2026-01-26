"""Semantic parity test: first index of character."""


def first_index(s: str, c: str) -> int:
    return s.find(c)


def main() -> None:
    print(first_index("hello", "l"))


if __name__ == "__main__":
    main()
