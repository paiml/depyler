"""Tests DEPYLER-0811: Triple list concat with function calls fails."""


def get_list(n: int) -> list[int]:
    """Returns a list, may raise (so Result in Rust)."""
    if n < 0:
        raise ValueError("negative")
    return [n]


def triple_concat() -> list[int]:
    """DEPYLER-0811: get_list(1) + [2] + get_list(3) should chain iterators."""
    return get_list(1) + [2] + get_list(3)
