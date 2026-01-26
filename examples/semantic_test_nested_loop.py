"""Semantic parity test: nested loop."""


def count_pairs(n: int) -> int:
    count = 0
    for i in range(n):
        for j in range(n):
            if i < j:
                count = count + 1
    return count


def main() -> None:
    print(count_pairs(4))


if __name__ == "__main__":
    main()
