# Hunt Mode repro for DEPYLER-0762: Panic on --format as identifier
# The transpiler tries to use raw argparse flags (--format) as Rust identifiers
# which panics because "--format" is not a valid Rust identifier

import argparse


def cmd_show(args):
    """Show data in specified format. Depyler: proven to terminate"""
    if args.format == "json":
        print('{"result": "ok"}')
    elif args.format == "text":
        print("result: ok")
    else:
        print(args.format)


def cmd_info(args):
    """Show info. Depyler: proven to terminate"""
    print(f"Name: {args.name}")


def main():
    """Main entry point. Depyler: proven to terminate"""
    parser = argparse.ArgumentParser(description="Format tool")
    subparsers = parser.add_subparsers(dest="command", required=True)

    # show subcommand with --format option
    show_parser = subparsers.add_parser("show", help="Show data")
    show_parser.add_argument("--format", choices=["json", "text"], default="text")

    # info subcommand
    info_parser = subparsers.add_parser("info", help="Show info")
    info_parser.add_argument("name", help="Name to show")

    args = parser.parse_args()

    if args.command == "show":
        cmd_show(args)
    elif args.command == "info":
        cmd_info(args)


if __name__ == "__main__":
    main()
