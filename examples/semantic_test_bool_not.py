"""Semantic parity test: boolean not."""


def invert(b: bool) -> bool:
    return not b


def main() -> None:
    if invert(False):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
