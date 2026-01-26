"""Semantic parity test: string find."""


def find_pos(s: str, sub: str) -> int:
    return s.find(sub)


def main() -> None:
    print(find_pos("hello world", "wor"))


if __name__ == "__main__":
    main()
