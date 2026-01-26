"""Semantic parity test: dict iteration."""


def sum_values(d: dict[str, int]) -> int:
    total = 0
    for k in d:
        total = total + d[k]
    return total


def main() -> None:
    data = {"a": 10, "b": 20, "c": 30}
    print(sum_values(data))


if __name__ == "__main__":
    main()
