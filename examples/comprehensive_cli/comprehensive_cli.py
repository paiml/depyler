#!/usr/bin/env python3
"""Comprehensive CLI test using all 28 ArgumentParser features"""
import argparse
from pathlib import Path
import json

def main():
    parser = argparse.ArgumentParser(
        description="Comprehensive CLI application testing all ArgumentParser features",
        epilog="Example: ./cli input.txt --verbose -o output.txt"
    )

    # Feature 1: Positional
    parser.add_argument("input_file", help="Input file to process")

    # Features 2-4: Flags
    parser.add_argument("-v", help="Verbose mode (short only)")
    parser.add_argument("--debug", action="store_true", help="Debug mode (long only)")
    parser.add_argument("-o", "--output", help="Output file (short + long)")

    # Features 5-8: Actions
    parser.add_argument("-q", "--quiet", action="store_true", help="Quiet mode")
    parser.add_argument("--no-color", action="store_false", dest="color", help="Disable colors")
    parser.add_argument("-V", action="count", default=0, help="Verbosity level")
    parser.add_argument("-I", "--include", action="append", help="Include paths")

    # Features 9-12: nargs
    parser.add_argument("extras", nargs="+", help="Extra files (1 or more)")
    parser.add_argument("--optional", nargs="*", help="Optional args (0 or more)")
    parser.add_argument("--config", nargs="?", help="Config file (0 or 1)")
    parser.add_argument("--coords", nargs=2, type=float, help="X Y coordinates")

    # Features 13-16: Types
    parser.add_argument("-n", "--count", type=int, help="Item count")
    parser.add_argument("-r", "--rate", type=float, help="Processing rate")
    parser.add_argument("-m", "--message", type=str, help="Message text")
    parser.add_argument("-p", "--path", type=Path, help="Path argument")

    # Features 17-20: Defaults
    parser.add_argument("--timeout", type=int, default=30, help="Timeout seconds")
    parser.add_argument("--threshold", type=float, default=0.95, help="Threshold")
    parser.add_argument("--name", default="unnamed", help="Project name")

    # Features 21-22: Required
    parser.add_argument("--api-key", required=True, help="API key (required)")
    parser.add_argument("--token", required=False, help="Token (optional)")

    # Features 23-25: Advanced parameters
    parser.add_argument("--input-path", dest="custom_input", help="Custom dest")
    parser.add_argument("--file", metavar="FILE", help="With metavar")
    parser.add_argument("--format", choices=["json", "yaml", "xml"], help="Output format")

    # Feature 26: const + nargs
    parser.add_argument("--mode", nargs="?", const="auto", default="manual", help="Processing mode")

    # Feature 27: store_const
    parser.add_argument("--fast", action="store_const", const=True, dest="speed", help="Fast mode")

    args = parser.parse_args()

    # Output results as JSON for easy verification
    result = {
        "input_file": args.input_file,
        "debug": args.debug,
        "output": args.output,
        "quiet": args.quiet,
        "color": args.color,
        "verbosity": args.V,
        "includes": args.include if args.include else [],
        "extras": args.extras,
        "config": args.config,
        "count": args.count,
        "rate": args.rate,
        "timeout": args.timeout,
        "threshold": args.threshold,
        "name": args.name,
        "api_key": args.api_key,
        "format": args.format,
        "mode": args.mode,
        "speed": args.speed if args.speed else False,
    }

    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
