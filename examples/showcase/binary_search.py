from typing import List

def binary_search(arr: List[int], target: int) -> int:
    """Find target in sorted array, return -1 if not found."""
    left: int = 0
    right: int = len(arr) - 1
    
    while left <= right:
        mid: int = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1