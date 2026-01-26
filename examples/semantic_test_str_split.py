"""Semantic parity test: string split."""


def count_words(s: str) -> int:
    words = s.split(" ")
    return len(words)


def main() -> None:
    print(count_words("hello world foo bar"))


if __name__ == "__main__":
    main()
