# @depyler: optimization_level = "aggressive"
# @depyler: bounds_checking = "explicit"
from typing import List

def filter_even_numbers(numbers: List[int]) -> List[int]:
    """Filter out even numbers from list"""
    result: List[int] = []
    
    for num in numbers:
        if num % 2 == 0:
            result.append(num)
    
    return result

def find_duplicates(numbers: List[int]) -> List[int]:
    """Find duplicate numbers in list"""
    seen: List[int] = []
    duplicates: List[int] = []
    
    for num in numbers:
        if num in seen:
            if num not in duplicates:
                duplicates.append(num)
        else:
            seen.append(num)
    
    return duplicates

def merge_sorted_lists(list1: List[int], list2: List[int]) -> List[int]:
    """Merge two sorted lists into one sorted list"""
    result: List[int] = []
    i = 0
    j = 0
    
    while i < len(list1) and j < len(list2):
        if list1[i] <= list2[j]:
            result.append(list1[i])
            i = i + 1
        else:
            result.append(list2[j])
            j = j + 1
    
    # Add remaining elements
    while i < len(list1):
        result.append(list1[i])
        i = i + 1
    
    while j < len(list2):
        result.append(list2[j])
        j = j + 1
    
    return result

def calculate_running_sum(numbers: List[int]) -> List[int]:
    """Calculate running sum of list"""
    if not numbers:
        return []
    
    result: List[int] = []
    running_total = 0
    
    for num in numbers:
        running_total = running_total + num
        result.append(running_total)
    
    return result

def rotate_list_left(numbers: List[int], positions: int) -> List[int]:
    """Rotate list left by specified positions"""
    if not numbers or positions <= 0:
        return numbers
    
    length = len(numbers)
    positions = positions % length  # Handle positions > length
    
    result: List[int] = []
    
    # Add elements from position onwards
    for i in range(positions, length):
        result.append(numbers[i])
    
    # Add elements from beginning to position
    for i in range(positions):
        result.append(numbers[i])
    
    return result