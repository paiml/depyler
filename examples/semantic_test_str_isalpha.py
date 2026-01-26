"""Semantic parity test: string isalpha."""


def all_letters(s: str) -> bool:
    return s.isalpha()


def main() -> None:
    if all_letters("hello"):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
