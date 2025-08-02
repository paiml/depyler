def binary_search(items: list[int], target: int) -> int:
    """
    Binary search implementation with contracts.
    
    @requires items is not None
    @requires all(items[i] <= items[i+1] for i in range(len(items)-1))
    @ensures result >= -1
    @ensures result < len(items)
    @invariant low <= high
    """
    low = 0
    high = len(items) - 1
    
    while low <= high:
        mid = int((low + high) / 2)
        if items[mid] == target:
            return mid
        elif items[mid] < target:
            low = mid + 1
        else:
            high = mid - 1
    
    return -1


def safe_divide(numerator: float, denominator: float) -> float:
    """
    Safe division with contracts.
    
    @requires denominator != 0
    @ensures result == numerator / denominator
    """
    return numerator / denominator


def list_sum(numbers: list[float]) -> float:
    """
    Sum all numbers in a list.
    
    @requires numbers is not None
    @ensures result >= 0 if all(n >= 0 for n in numbers) else True
    """
    total = 0.0
    for num in numbers:
        total += num
    return total