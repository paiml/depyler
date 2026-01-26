"""Semantic parity test: string isdigit."""


def all_digits(s: str) -> bool:
    return s.isdigit()


def main() -> None:
    if all_digits("12345"):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
