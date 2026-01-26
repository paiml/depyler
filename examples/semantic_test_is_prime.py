"""Semantic parity test: primality check."""


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i = i + 1
    return True


def main() -> None:
    if is_prime(17):
        print("prime")
    else:
        print("not prime")


if __name__ == "__main__":
    main()
