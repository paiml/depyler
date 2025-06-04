from typing import List, Optional

# Test Case 11: Sum all elements in a list
def sum_list(numbers: List[int]) -> int:
    total: int = 0
    for num in numbers:
        total += num
    return total

# Test Case 12: Find maximum in list
def find_max(numbers: List[int]) -> Optional[int]:
    if not numbers:
        return None
    max_val: int = numbers[0]
    for num in numbers:
        if num > max_val:
            max_val = num
    return max_val

# Test Case 13: Count elements
def count_elements(numbers: List[int]) -> int:
    return len(numbers)

# Test Case 14: Filter positive numbers
def filter_positive(numbers: List[int]) -> List[int]:
    result: List[int] = []
    for num in numbers:
        if num > 0:
            result.append(num)
    return result

# Test Case 15: Find element at index
def get_element(numbers: List[int], index: int) -> Optional[int]:
    if 0 <= index < len(numbers):
        return numbers[index]
    return None

# Test Case 16: Reverse list
def reverse_list(numbers: List[int]) -> List[int]:
    result: List[int] = []
    for i in range(len(numbers) - 1, -1, -1):
        result.append(numbers[i])
    return result

# Test Case 17: List contains element
def contains_element(numbers: List[int], target: int) -> bool:
    for num in numbers:
        if num == target:
            return True
    return False

# Test Case 18: First element
def first_element(numbers: List[int]) -> Optional[int]:
    if numbers:
        return numbers[0]
    return None

# Test Case 19: Last element
def last_element(numbers: List[int]) -> Optional[int]:
    if numbers:
        return numbers[-1]
    return None

# Test Case 20: Average of numbers
def average_numbers(numbers: List[int]) -> float:
    if not numbers:
        return 0.0
    return sum(numbers) / len(numbers)