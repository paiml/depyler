"""Semantic parity test: integer modulo."""


def modulo(a: int, b: int) -> int:
    return a % b


def main() -> None:
    print(modulo(17, 5))


if __name__ == "__main__":
    main()
