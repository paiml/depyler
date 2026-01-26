"""Semantic parity test: dict membership."""


def has_key(d: dict[str, int], key: str) -> bool:
    return key in d


def main() -> None:
    data = {"hello": 1, "world": 2}
    if has_key(data, "hello"):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
