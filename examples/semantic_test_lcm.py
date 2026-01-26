"""Semantic parity test: least common multiple."""


def gcd(a: int, b: int) -> int:
    while b != 0:
        temp = b
        b = a % b
        a = temp
    return a


def lcm(a: int, b: int) -> int:
    product = a * b
    divisor = gcd(a, b)
    return product // divisor


def main() -> None:
    print(lcm(12, 18))


if __name__ == "__main__":
    main()
