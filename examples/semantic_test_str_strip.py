"""Semantic parity test: string strip."""


def clean(s: str) -> str:
    return s.strip()


def main() -> None:
    result = clean("  hello  ")
    print(len(result))


if __name__ == "__main__":
    main()
