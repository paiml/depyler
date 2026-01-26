"""Semantic parity test: nested function calls."""


def square(x: int) -> int:
    return x * x


def cube(x: int) -> int:
    return x * x * x


def main() -> None:
    print(square(3) + cube(2))


if __name__ == "__main__":
    main()
