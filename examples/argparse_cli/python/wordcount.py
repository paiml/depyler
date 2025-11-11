#!/usr/bin/env python3
"""
wordcount.py - Count words, lines, and characters in files
"""
import argparse
import sys
from pathlib import Path
from typing import NamedTuple

class Stats(NamedTuple):
    """Statistics for a file"""
    lines: int
    words: int
    chars: int
    filename: str

def count_file(filepath: Path) -> Stats:
    """Count statistics for a single file"""
    try:
        content = filepath.read_text()
        lines = len(content.splitlines())
        words = len(content.split())
        chars = len(content)
        return Stats(lines, words, chars, str(filepath))
    except IOError as e:
        print(f"Error reading {filepath}: {e}", file=sys.stderr)
        return Stats(0, 0, 0, str(filepath))

def format_stats(stats: Stats, show_filename: bool = True) -> str:
    """Format statistics for output"""
    result = f"{stats.lines:8} {stats.words:8} {stats.chars:8}"
    if show_filename:
        result += f" {stats.filename}"
    return result

def main() -> int:
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description="Count lines, words, and characters in files",
        epilog="Similar to wc(1) Unix command"
    )
    parser.add_argument(
        "files",
        nargs="+",
        type=Path,
        help="Files to process"
    )
    parser.add_argument(
        "-l", "--lines",
        action="store_true",
        help="Show only line count"
    )
    parser.add_argument(
        "-w", "--words",
        action="store_true",
        help="Show only word count"
    )
    parser.add_argument(
        "-c", "--chars",
        action="store_true",
        help="Show only character count"
    )

    args = parser.parse_args()

    total_lines = 0
    total_words = 0
    total_chars = 0

    for filepath in args.files:
        stats = count_file(filepath)
        total_lines += stats.lines
        total_words += stats.words
        total_chars += stats.chars

        if args.lines:
            print(f"{stats.lines:8} {stats.filename}")
        elif args.words:
            print(f"{stats.words:8} {stats.filename}")
        elif args.chars:
            print(f"{stats.chars:8} {stats.filename}")
        else:
            print(format_stats(stats))

    # Show totals if multiple files
    if len(args.files) > 1:
        total_stats = Stats(total_lines, total_words, total_chars, "total")
        print(format_stats(total_stats, show_filename=True))

    return 0

if __name__ == "__main__":
    sys.exit(main())
