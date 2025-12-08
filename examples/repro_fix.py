#!/usr/bin/env python3
"""Minimal repro for E0507: filter closure borrow pattern.

DEPYLER-0820: Filter closure uses |&(k, v)| but String doesn't implement Copy,
causing "cannot move out of shared reference" error.

Pattern: {k: v for k, v in d.items() if k.startswith(prefix)}
Generated: .filter(|&(k, v)| k.starts_with(prefix))  // E0507
Should be: .filter(|(k, v)| k.starts_with(prefix))   // OK
"""


def filter_by_key(d: dict[str, int], prefix: str) -> dict[str, int]:
    """Filter dict by key prefix."""
    return {k: v for k, v in d.items() if k.startswith(prefix)}


def main() -> int:
    data = {"apple": 1, "apricot": 2, "banana": 3}
    result = filter_by_key(data, "ap")
    print(result)
    return 0


if __name__ == "__main__":
    main()
