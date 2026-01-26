"""Semantic parity test: string count."""


def count_char(s: str, c: str) -> int:
    return s.count(c)


def main() -> None:
    print(count_char("mississippi", "i"))


if __name__ == "__main__":
    main()
