"""Semantic parity test: is power of two."""


def is_power_of_two(n: int) -> bool:
    if n <= 0:
        return False
    while n > 1:
        if n % 2 != 0:
            return False
        n = n // 2
    return True


def main() -> None:
    if is_power_of_two(16):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
