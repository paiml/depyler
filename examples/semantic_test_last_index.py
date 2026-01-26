"""Semantic parity test: last index of character."""


def last_index(s: str, c: str) -> int:
    return s.rfind(c)


def main() -> None:
    print(last_index("hello", "l"))


if __name__ == "__main__":
    main()
