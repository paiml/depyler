"""Semantic parity test: count character occurrences."""


def count_char(s: str, c: str) -> int:
    return s.count(c)


def main() -> None:
    print(count_char("banana", "a"))


if __name__ == "__main__":
    main()
