"""Semantic parity test: count set bits."""


def hamming_weight(n: int) -> int:
    count = 0
    while n > 0:
        count = count + (n & 1)
        n = n >> 1
    return count


def main() -> None:
    print(hamming_weight(11))


if __name__ == "__main__":
    main()
