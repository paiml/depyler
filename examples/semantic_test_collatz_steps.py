"""Semantic parity test: Collatz sequence steps (simplified)."""


def collatz_steps(n: int) -> int:
    steps = 0
    while n != 1:
        if n % 2 == 0:
            n = n // 2
        else:
            # Avoid 3 * n + 1 type inference issue
            triple: int = n + n + n
            n = triple + 1
        steps = steps + 1
    return steps


def main() -> None:
    print(collatz_steps(27))


if __name__ == "__main__":
    main()
