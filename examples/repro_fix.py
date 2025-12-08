#!/usr/bin/env python3
"""Minimal repro for E0425: exception alias variable not declared.

DEPYLER-XXXX: Exception alias variable 'err' in 'except ... as err' clause
is not properly declared in generated Rust code.

Pattern: except SomeException as alias_var -> use alias_var
Root Cause: The 'as err' binding is not being declared in the match arm.
"""


def catch_and_convert(value: str) -> int:
    """Catch one exception, raise another using the exception alias."""
    try:
        return int(value)
    except ValueError as err:
        # This 'err' variable should be bound by the match arm
        # Currently generates: map_err(|_| err) - but err isn't declared!
        raise RuntimeError(f"Cannot parse: {value}") from err


def main() -> int:
    result = catch_and_convert("not_a_number")
    print(result)
    return 0


if __name__ == "__main__":
    main()
