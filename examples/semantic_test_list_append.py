"""Semantic parity test: list append."""


def build_list(n: int) -> int:
    result: list[int] = []
    for i in range(n):
        result.append(i)
    return len(result)


def main() -> None:
    print(build_list(5))


if __name__ == "__main__":
    main()
