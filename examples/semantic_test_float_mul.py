"""Semantic parity test: float multiplication."""


def mul_floats(a: float, b: float) -> float:
    return a * b


def main() -> None:
    result = mul_floats(2.0, 3.0)
    print(int(result))


if __name__ == "__main__":
    main()
