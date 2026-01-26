"""Semantic parity test: for loop sum."""


def sum_range(n: int) -> int:
    total = 0
    for i in range(n):
        total = total + i
    return total


def main() -> None:
    print(sum_range(10))


if __name__ == "__main__":
    main()
