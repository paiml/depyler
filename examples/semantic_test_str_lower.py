"""Semantic parity test: string lower."""


def whisper(s: str) -> str:
    return s.lower()


def main() -> None:
    print(whisper("HELLO"))


if __name__ == "__main__":
    main()
