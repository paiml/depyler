"""Semantic parity test: dict get."""


def main() -> None:
    data: dict[str, int] = {"x": 10, "y": 20}
    print(data["x"])


if __name__ == "__main__":
    main()
