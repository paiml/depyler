"""Semantic parity test: elif chain."""


def classify_number(n: int) -> str:
    if n < 0:
        return "negative"
    elif n == 0:
        return "zero"
    elif n < 10:
        return "small"
    else:
        return "large"


def main() -> None:
    print(classify_number(5))


if __name__ == "__main__":
    main()
