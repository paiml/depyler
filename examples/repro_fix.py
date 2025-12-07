"""Reproduction for DEPYLER-0769: Closure parameters using Rust keywords.

The transpiler generates 'fn' as a closure parameter name, but 'fn' is a
Rust keyword. This should be escaped as 'r#fn'.

Bug: safe_ident() not called for closure ASSIGNMENT parameter names.
The bug occurs when a nested function is RETURNED (becoming a closure assignment).

Pattern that triggers the bug:
    def outer() -> Callable:
        def inner(fn: Callable) -> Callable:  # 'fn' param on returned inner func
            ...
        return inner  # <- This causes inner to become a closure assignment

Expected: once_wrapper = | r#fn: ... | -> ... {
Actual:   once_wrapper = | fn: ... | -> ... {
"""
from typing import Callable


def make_once() -> Callable[[Callable[[], int]], Callable[[], int]]:
    """Create function that runs inner function only once.

    The inner 'once_wrapper' function takes 'fn' as a parameter.
    When returned, it becomes a closure assignment with 'fn' parameter.
    """

    def once_wrapper(fn: Callable[[], int]) -> Callable[[], int]:
        called = [False]
        result: list[int] = []

        def wrapped() -> int:
            if not called[0]:
                called[0] = True
                result.append(fn())
            return result[0]

        return wrapped

    return once_wrapper


def main() -> None:
    once = make_once()

    counter = [0]
    def get_value() -> int:
        counter[0] += 1
        return counter[0]

    cached = once(get_value)
    print(f"First call: {cached()}")
    print(f"Second call: {cached()}")
    print(f"Third call: {cached()}")


if __name__ == "__main__":
    main()
