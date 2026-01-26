"""Semantic parity test: empty string check."""


def is_empty(s: str) -> bool:
    return len(s) == 0


def main() -> None:
    if is_empty(""):
        print("empty")
    else:
        print("not empty")


if __name__ == "__main__":
    main()
