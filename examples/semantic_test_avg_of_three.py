"""Semantic parity test: average of three."""


def avg_of_three(a: int, b: int, c: int) -> int:
    return (a + b + c) // 3


def main() -> None:
    print(avg_of_three(10, 20, 30))


if __name__ == "__main__":
    main()
