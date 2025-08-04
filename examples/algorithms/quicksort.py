# @depyler: optimization_level = "aggressive"
# @depyler: bounds_checking = "explicit"
from typing import List

def quicksort(arr: List[int]) -> List[int]:
    """Classic quicksort algorithm implementation"""
    if len(arr) <= 1:
        return arr
    
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    
    return quicksort(left) + middle + quicksort(right)

def partition(arr: List[int], low: int, high: int) -> int:
    """In-place partition for quicksort"""
    pivot = arr[high]
    i = low - 1
    
    for j in range(low, high):
        if arr[j] <= pivot:
            i += 1
            arr[i], arr[j] = arr[j], arr[i]
    
    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return i + 1

def quicksort_inplace(arr: List[int], low: int, high: int) -> None:
    """In-place quicksort implementation"""
    if low < high:
        pi = partition(arr, low, high)
        quicksort_inplace(arr, low, pi - 1)
        quicksort_inplace(arr, pi + 1, high)