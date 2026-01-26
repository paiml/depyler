"""Semantic parity test: nested if."""


def classify(n: int) -> str:
    if n > 0:
        if n > 10:
            return "large"
        else:
            return "small"
    else:
        return "zero_or_neg"


def main() -> None:
    print(classify(5))


if __name__ == "__main__":
    main()
