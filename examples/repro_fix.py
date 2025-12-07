"""Reproduction for DEPYLER-0769: Closure parameters using Rust keywords.

The transpiler generates 'fn' as a closure parameter name, but 'fn' is a
Rust keyword. This should be escaped as 'r#fn'.

Bug: safe_ident() not called for closure ASSIGNMENT parameter names.
The bug occurs when a nested function is RETURNED (becoming a closure assignment).

Pattern that triggers the bug:
    def outer():
        def inner(fn: int) -> int:  # 'fn' param - Rust keyword
            return fn * 2
        return inner

Expected: wrapper = | r#fn: i32 | -> i32 {
Actual:   wrapper = | fn: i32 | -> i32 {
"""


def make_doubler() -> int:
    """Create a function that doubles its input.

    The inner 'wrapper' function takes 'fn' as a parameter name.
    'fn' is a Rust keyword and must be escaped as 'r#fn'.
    """

    def wrapper(fn: int) -> int:
        return fn * 2

    return wrapper(21)


def main() -> None:
    result = make_doubler()
    print(f"Result: {result}")


if __name__ == "__main__":
    main()
