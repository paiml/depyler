"""Semantic parity test: early return."""


def is_valid(n: int) -> bool:
    if n < 0:
        return False
    if n > 100:
        return False
    return True


def main() -> None:
    if is_valid(50):
        print("valid")
    else:
        print("invalid")


if __name__ == "__main__":
    main()
