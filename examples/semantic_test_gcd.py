"""Semantic parity test: greatest common divisor."""


def gcd(a: int, b: int) -> int:
    while b != 0:
        temp = b
        b = a % b
        a = temp
    return a


def main() -> None:
    print(gcd(48, 18))


if __name__ == "__main__":
    main()
