"""Semantic parity test: boolean or."""


def either_positive(a: int, b: int) -> bool:
    return a > 0 or b > 0


def main() -> None:
    result = either_positive(-5, 3)
    if result:
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
