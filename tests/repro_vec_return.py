# DEPYLER-1160: Reproduction case for Vec return type inference
# Problem: Empty list in CLOSURE with explicit return type

def test_lambda_list_return() -> list[int]:
    """The real bug: lambda with typed return but untyped local list."""
    numbers = [1, 2, 3, 4, 5]

    # This closure has explicit return type Vec<i32>
    # But result = [] is inferred as Vec<DepylerValue>
    apply_func = lambda lst, f: (lambda: (
        result := [],
        [result.append(f(x)) for x in lst],
        result
    )[-1])()

    doubled = apply_func(numbers, lambda x: x * 2)
    return doubled


def test_higher_order_map(nums: list[int]) -> list[int]:
    """Higher-order function pattern."""
    def mapper(f, items: list[int]) -> list[int]:
        result = []  # Should be Vec<i32> from return type
        for item in items:
            result.append(f(item))  # f(item) returns int
        return result

    return mapper(lambda x: x * 2, nums)


def main() -> None:
    result = test_higher_order_map([1, 2, 3])
    print(result)
