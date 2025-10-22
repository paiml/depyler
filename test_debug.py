#!/usr/bin/env python3
"""Simple test file for debugging integration"""

def greet(name: str) -> str:
    """Greet someone by name"""
    message = f"Hello, {name}!"
    return message

def add_numbers(a: int, b: int) -> int:
    """Add two numbers"""
    result = a + b
    return result

def main() -> None:
    """Main entry point"""
    name = "World"
    greeting = greet(name)
    print(greeting)

    x = 10
    y = 20
    sum_result = add_numbers(x, y)
    print(f"Sum: {sum_result}")

if __name__ == "__main__":
    main()
