"""Semantic parity test: swap values."""


def main() -> None:
    a = 5
    b = 10
    temp = a
    a = b
    b = temp
    print(a)
    print(b)


if __name__ == "__main__":
    main()
