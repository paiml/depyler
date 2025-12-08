"""Reproduction for E0308: Array literal in class method.

The issue: Class methods passing list literals may not get &vec![] conversion.
"""


def process(items: list[str]) -> int:
    """Process list of items."""
    return len(items)


class TestCase:
    def test_it(self):
        # List literal in class method - does it convert?
        result = process(["a", "b", "c"])
        print(result)
