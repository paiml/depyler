"""Semantic parity test: multiple return paths."""


def classify(n: int) -> str:
    if n < 0:
        return "negative"
    if n == 0:
        return "zero"
    return "positive"


def main() -> None:
    print(classify(-5))
    print(classify(0))
    print(classify(5))


if __name__ == "__main__":
    main()
