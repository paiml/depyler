"""Semantic parity test: double then halve."""


def double_halve(n: int) -> int:
    doubled = n * 2
    return doubled // 2


def main() -> None:
    print(double_halve(12345))


if __name__ == "__main__":
    main()
