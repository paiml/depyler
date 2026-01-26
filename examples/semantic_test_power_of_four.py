"""Semantic parity test: check power of four."""


def is_power_of_four(n: int) -> bool:
    if n <= 0:
        return False
    while n > 1:
        if n % 4 != 0:
            return False
        n = n // 4
    return True


def main() -> None:
    if is_power_of_four(64):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
