"""Semantic parity test: check perfect square (simplified)."""


def is_perfect_square(n: int) -> bool:
    if n < 0:
        return False
    i = 0
    square: int = i * i
    while square <= n:
        if square == n:
            return True
        i = i + 1
        square = i * i
    return False


def main() -> None:
    if is_perfect_square(16):
        print("yes")
    else:
        print("no")


if __name__ == "__main__":
    main()
