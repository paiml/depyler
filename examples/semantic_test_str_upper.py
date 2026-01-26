"""Semantic parity test: string upper."""


def shout(s: str) -> str:
    return s.upper()


def main() -> None:
    print(shout("hello"))


if __name__ == "__main__":
    main()
