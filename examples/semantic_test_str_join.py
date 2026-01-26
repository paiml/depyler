"""Semantic parity test: string join."""


def join_words(words: list[str]) -> str:
    return "-".join(words)


def main() -> None:
    print(join_words(["a", "b", "c"]))


if __name__ == "__main__":
    main()
