# Hunt Mode tracking file - used to verify repro fixes compile
# Current status: GREEN (last verified: E0384 - nested function mutability)
# DEPYLER-0766: Variables in nested functions now correctly get `let mut`

def calculate(values: list[int]) -> int:
    """Calculate sum using nested helper."""

    def sum_values(lst: list[int]) -> int:
        total = 0
        for x in lst:
            total += x
        return total

    return sum_values(values)


def main() -> None:
    """Main entry point."""
    result = calculate([1, 2, 3, 4, 5])
    print(f"Sum: {result}")


if __name__ == "__main__":
    main()
