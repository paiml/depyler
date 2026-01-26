"""Semantic parity test: comparison operators."""


def main() -> None:
    a = 5
    b = 10
    c = 5
    if a < b:
        print("less")
    if a <= c:
        print("leq")
    if a >= c:
        print("geq")
    if b > a:
        print("greater")


if __name__ == "__main__":
    main()
