#!/usr/bin/env python3
"""Simple CLI tool demonstrating argparse → clap transformation"""
import argparse

def main() -> int:
    # Create argument parser
    parser = argparse.ArgumentParser(
        description="A simple CLI tool example"
    )

    # Add arguments
    parser.add_argument(
        "name",
        help="Your name"
    )

    # Parse arguments
    args = parser.parse_args()

    # Use the arguments
    print(f"Hello, {args.name}!")

    return 0

if __name__ == "__main__":
    exit(main())
