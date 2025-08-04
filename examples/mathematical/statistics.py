# @depyler: optimization_level = "aggressive"
# @depyler: bounds_checking = "explicit"
from typing import List, Optional

def mean(numbers: List[float]) -> float:
    """Calculate arithmetic mean"""
    if not numbers:
        return 0.0
    return sum(numbers) / len(numbers)

def median(numbers: List[float]) -> float:
    """Calculate median value"""
    if not numbers:
        return 0.0
    
    # Sort the numbers
    sorted_nums = sorted(numbers)
    n = len(sorted_nums)
    
    if n % 2 == 0:
        # Even number of elements - average of middle two
        return (sorted_nums[n // 2 - 1] + sorted_nums[n // 2]) / 2.0
    else:
        # Odd number of elements - middle element
        return sorted_nums[n // 2]

def mode(numbers: List[int]) -> Optional[int]:
    """Find the most frequently occurring number"""
    if not numbers:
        return None
    
    frequency: dict[int, int] = {}
    for num in numbers:
        if num in frequency:
            frequency[num] += 1
        else:
            frequency[num] = 1
    
    max_count = 0
    mode_value = numbers[0]
    
    for num, count in frequency.items():
        if count > max_count:
            max_count = count
            mode_value = num
    
    return mode_value

def variance(numbers: List[float]) -> float:
    """Calculate sample variance"""
    if len(numbers) < 2:
        return 0.0
    
    avg = mean(numbers)
    sum_squared_diff = 0.0
    
    for num in numbers:
        diff = num - avg
        sum_squared_diff += diff * diff
    
    return sum_squared_diff / (len(numbers) - 1)

def standard_deviation(numbers: List[float]) -> float:
    """Calculate sample standard deviation"""
    var = variance(numbers)
    # Simple square root approximation
    if var == 0.0:
        return 0.0
    
    # Newton's method for square root
    x = var / 2.0
    for _ in range(10):  # 10 iterations should be enough
        x = (x + var / x) / 2.0
    
    return x

def correlation(x: List[float], y: List[float]) -> float:
    """Calculate Pearson correlation coefficient"""
    if len(x) != len(y) or len(x) < 2:
        return 0.0
    
    n = len(x)
    mean_x = mean(x)
    mean_y = mean(y)
    
    numerator = 0.0
    sum_x_squared = 0.0
    sum_y_squared = 0.0
    
    for i in range(n):
        dx = x[i] - mean_x
        dy = y[i] - mean_y
        numerator += dx * dy
        sum_x_squared += dx * dx
        sum_y_squared += dy * dy
    
    denominator_squared = sum_x_squared * sum_y_squared
    if denominator_squared == 0.0:
        return 0.0
    
    # Approximate square root
    denominator = denominator_squared / 2.0
    for _ in range(10):
        denominator = (denominator + denominator_squared / denominator) / 2.0
    
    return numerator / denominator