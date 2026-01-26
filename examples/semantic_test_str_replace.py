"""Semantic parity test: string replace."""


def replace_char(s: str, old: str, new: str) -> str:
    return s.replace(old, new)


def main() -> None:
    print(replace_char("hello", "l", "x"))


if __name__ == "__main__":
    main()
