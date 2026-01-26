"""Semantic parity test: count matches in two lists."""


def count_matches(a: list[int], b: list[int]) -> int:
    count = 0
    i = 0
    while i < len(a) and i < len(b):
        if a[i] == b[i]:
            count = count + 1
        i = i + 1
    return count


def main() -> None:
    print(count_matches([1, 2, 3, 4], [1, 0, 3, 0]))


if __name__ == "__main__":
    main()
