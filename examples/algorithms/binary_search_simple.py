# @depyler: optimization_level = "aggressive"
# @depyler: bounds_checking = "explicit"
from typing import List

def binary_search(arr: List[int], target: int) -> int:
    """Binary search implementation - returns index or -1"""
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

def linear_search(arr: List[int], target: int) -> int:
    """Linear search for comparison"""
    for i in range(len(arr)):
        if arr[i] == target:
            return i
    return -1

def find_maximum(arr: List[int]) -> int:
    """Find maximum element in array"""
    if not arr:
        return 0
    
    max_val = arr[0]
    for val in arr:
        if val > max_val:
            max_val = val
    
    return max_val