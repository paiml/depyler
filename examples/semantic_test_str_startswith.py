"""Semantic parity test: string startswith."""


def check_prefix(s: str, prefix: str) -> bool:
    return s.startswith(prefix)


def main() -> None:
    if check_prefix("hello world", "hello"):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
