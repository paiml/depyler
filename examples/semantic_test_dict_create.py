"""Semantic parity test: dict creation."""


def create_dict() -> dict[str, int]:
    return {"a": 1, "b": 2, "c": 3}


def main() -> None:
    d = create_dict()
    print(len(d))


if __name__ == "__main__":
    main()
