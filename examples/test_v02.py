# @depyler: type_strategy = "conservative"
# @depyler: ownership = "borrowed"
# @depyler: bounds_checking = "explicit"
def fibonacci(n: int) -> int:
    """Calculate fibonacci with annotations"""
    # @depyler: performance_critical = "true"
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

# @depyler: optimization_hint = "vectorize"
def process_list(items: list[int]) -> list[int]:
    """Process list with performance hints"""
    # @depyler: bounds_checking = "explicit"
    result = []
    for item in items:
        if item > 0:
            result.append(item * 2)
    return result