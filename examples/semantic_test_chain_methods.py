"""Semantic parity test: chained string methods."""


def process_text(s: str) -> str:
    return s.strip().lower()


def main() -> None:
    print(process_text("  HELLO  "))


if __name__ == "__main__":
    main()
