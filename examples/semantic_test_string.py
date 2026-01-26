"""Semantic parity test: string operations."""


def greet(name: str) -> str:
    """Create greeting."""
    return "Hello, " + name + "!"


def main() -> None:
    """Test greet."""
    print(greet("World"))


if __name__ == "__main__":
    main()
