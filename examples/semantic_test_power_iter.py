"""Semantic parity test: iterative power."""


def power(base: int, exp: int) -> int:
    result = 1
    i = 0
    while i < exp:
        result = result * base
        i = i + 1
    return result


def main() -> None:
    print(power(2, 10))


if __name__ == "__main__":
    main()
