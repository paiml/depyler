"""Semantic parity test: clamp value."""


def clamp(x: int, lo: int, hi: int) -> int:
    if x < lo:
        return lo
    if x > hi:
        return hi
    return x


def main() -> None:
    print(clamp(15, 0, 10))


if __name__ == "__main__":
    main()
