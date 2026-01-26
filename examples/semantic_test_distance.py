"""Semantic parity test: distance between two numbers."""


def distance(a: int, b: int) -> int:
    diff = a - b
    if diff < 0:
        return -diff
    return diff


def main() -> None:
    print(distance(5, 12))


if __name__ == "__main__":
    main()
