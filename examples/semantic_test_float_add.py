"""Semantic parity test: float addition."""


def add_floats(a: float, b: float) -> float:
    return a + b


def main() -> None:
    result = add_floats(1.5, 2.5)
    print(int(result))


if __name__ == "__main__":
    main()
