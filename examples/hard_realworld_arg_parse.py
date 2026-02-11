"""Real-world command line argument parsing without imports.

Mimics: argparse, click, sys.argv processing patterns.
Handles flags, named options, positional args, help text.
"""


def starts_with_dash(text: str) -> bool:
    """Check if text starts with dash."""
    if len(text) == 0:
        return False
    return text[0] == "-"


def starts_with_double_dash(text: str) -> bool:
    """Check if argument starts with --."""
    if len(text) < 3:
        return False
    return text[0] == "-" and text[1] == "-"


def is_flag(arg: str) -> int:
    """Check if argument is a flag (starts with -). Returns 1 if yes, 0 if no."""
    if len(arg) > 1 and arg[0] == "-":
        return 1
    return 0


def extract_option_name(arg: str) -> str:
    """Extract option name without dashes."""
    result: str = ""
    start: int = 0
    if len(arg) > 2 and arg[0] == "-" and arg[1] == "-":
        start = 2
    elif len(arg) > 1 and arg[0] == "-":
        start = 1
    idx: int = start
    while idx < len(arg):
        ch: str = arg[idx]
        if ch == "=":
            return result
        result = result + ch
        idx = idx + 1
    return result


def has_equals(arg: str) -> int:
    """Check if argument contains equals sign. Returns 1 if yes, 0 if no."""
    idx: int = 0
    while idx < len(arg):
        if arg[idx] == "=":
            return 1
        idx = idx + 1
    return 0


def extract_after_equals(arg: str) -> str:
    """Extract value after = sign in --option=value format."""
    idx: int = 0
    while idx < len(arg):
        if arg[idx] == "=":
            result: str = ""
            vi: int = idx + 1
            while vi < len(arg):
                result = result + arg[vi]
                vi = vi + 1
            return result
        idx = idx + 1
    return ""


def parse_args_to_pairs(argv: list[str]) -> list[list[str]]:
    """Parse argument list into [name, value] pairs."""
    pairs: list[list[str]] = []
    idx: int = 0
    while idx < len(argv):
        arg: str = argv[idx]
        if is_flag(arg) == 1:
            opt_name: str = extract_option_name(arg)
            if has_equals(arg) == 1:
                eq_val: str = extract_after_equals(arg)
                pairs.append([opt_name, eq_val])
            elif idx + 1 < len(argv) and is_flag(argv[idx + 1]) == 0:
                pairs.append([opt_name, argv[idx + 1]])
                idx = idx + 1
            else:
                pairs.append([opt_name, "true"])
        else:
            pairs.append(["_pos", arg])
        idx = idx + 1
    return pairs


def get_arg_value(pairs: list[list[str]], name: str, default_val: str) -> str:
    """Get argument value by name, with default."""
    idx: int = 0
    while idx < len(pairs):
        if pairs[idx][0] == name:
            return pairs[idx][1]
        idx = idx + 1
    return default_val


def has_arg(pairs: list[list[str]], name: str) -> int:
    """Check if an argument exists. Returns 1 if yes, 0 if no."""
    idx: int = 0
    while idx < len(pairs):
        if pairs[idx][0] == name:
            return 1
        idx = idx + 1
    return 0


def get_positional_args(pairs: list[list[str]]) -> list[str]:
    """Get all positional (non-flag) arguments."""
    result: list[str] = []
    idx: int = 0
    while idx < len(pairs):
        if pairs[idx][0] == "_pos":
            result.append(pairs[idx][1])
        idx = idx + 1
    return result


def count_flags(pairs: list[list[str]]) -> int:
    """Count number of flag arguments."""
    count: int = 0
    idx: int = 0
    while idx < len(pairs):
        if pairs[idx][0] != "_pos":
            count = count + 1
        idx = idx + 1
    return count


def test_module() -> int:
    """Test argument parsing module."""
    passed: int = 0

    # Test 1: flag detection
    if is_flag("-v") == 1 and is_flag("hello") == 0:
        passed = passed + 1

    # Test 2: extract option name from long option
    nm: str = extract_option_name("--output")
    if nm == "output":
        passed = passed + 1

    # Test 3: extract option name from short option
    nm2: str = extract_option_name("-v")
    if nm2 == "v":
        passed = passed + 1

    # Test 4: extract after equals
    ev: str = extract_after_equals("--file=test.txt")
    if ev == "test.txt":
        passed = passed + 1

    # Test 5: parse simple args
    argv: list[str] = ["--verbose", "--output", "result.txt", "input.py"]
    pairs: list[list[str]] = parse_args_to_pairs(argv)
    if len(pairs) == 3:
        passed = passed + 1

    # Test 6: get arg value
    outfile: str = get_arg_value(pairs, "output", "default.txt")
    if outfile == "result.txt":
        passed = passed + 1

    # Test 7: has arg check
    if has_arg(pairs, "verbose") == 1 and has_arg(pairs, "debug") == 0:
        passed = passed + 1

    # Test 8: positional args
    positionals: list[str] = get_positional_args(pairs)
    if len(positionals) == 1 and positionals[0] == "input.py":
        passed = passed + 1

    return passed
