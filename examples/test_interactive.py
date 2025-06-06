# Test case for interactive annotation suggestions

from typing import List, Dict

def compute_sum(numbers: List[int]) -> int:
    """Compute the sum of a list of numbers."""
    total = 0
    for num in numbers:
        total += num
    return total

def binary_search(arr: List[int], target: int) -> int:
    """Binary search with nested loops and array access."""
    left = 0
    right = len(arr) - 1
    
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1

def process_strings(strings: List[str]) -> str:
    """Process strings with concatenation."""
    result = ""
    for s in strings:
        result += s + " "
    return result.strip()

def lookup_values(data: Dict[str, int], keys: List[str]) -> List[int]:
    """Function with frequent dictionary lookups."""
    results = []
    for key in keys:
        if key in data:
            results.append(data[key])
        else:
            results.append(0)
    return results