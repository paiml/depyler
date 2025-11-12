#!/usr/bin/env python3
"""Test script to verify keyword arguments are preserved in HIR"""
import argparse

def main() -> int:
    parser = argparse.ArgumentParser(description="Test kwargs preservation")
    # This call has kwargs: nargs, type, help
    parser.add_argument("files", nargs="+", type=str, help="Files to process")
    args = parser.parse_args()
    print(f"Files: {args.files}")
    return 0

if __name__ == "__main__":
    exit(main())
