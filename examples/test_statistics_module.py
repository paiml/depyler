"""
Comprehensive test of Python statistics module transpilation to Rust.

This example demonstrates how Depyler transpiles Python's statistics module
functions to their Rust equivalents.

Expected Rust mappings:
- statistics.mean() -> manual calculation
- statistics.median() -> manual calculation with sorting
- statistics.stdev() -> manual standard deviation
- statistics.variance() -> manual variance calculation

Note: Statistics functions may need manual implementation.
"""

import statistics
from typing import List
import math


def test_mean() -> float:
    """Test calculating arithmetic mean"""
    data: List[float] = [1.0, 2.0, 3.0, 4.0, 5.0]

    # Calculate mean
    total: float = 0.0
    for value in data:
        total = total + value

    mean: float = total / float(len(data))

    return mean


def test_median_odd() -> float:
    """Test median with odd number of elements"""
    data: List[float] = [1.0, 3.0, 5.0, 7.0, 9.0]

    # Sort data (manual bubble sort)
    sorted_data: List[float] = data.copy()
    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j] < sorted_data[i]:
                temp: float = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    # Get middle element
    mid: int = len(sorted_data) // 2
    median: float = sorted_data[mid]

    return median


def test_median_even() -> float:
    """Test median with even number of elements"""
    data: List[float] = [1.0, 2.0, 3.0, 4.0]

    # Sort data
    sorted_data: List[float] = data.copy()
    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j] < sorted_data[i]:
                temp: float = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    # Average of two middle elements
    mid: int = len(sorted_data) // 2
    median: float = (sorted_data[mid - 1] + sorted_data[mid]) / 2.0

    return median


def test_mode() -> int:
    """Test finding mode (most common value)"""
    data: List[int] = [1, 2, 2, 3, 3, 3, 4, 4]

    # Count frequencies (manual implementation)
    max_count: int = 0
    mode_value: int = data[0]

    for i in range(len(data)):
        count: int = 0
        for j in range(len(data)):
            if data[j] == data[i]:
                count = count + 1

        if count > max_count:
            max_count = count
            mode_value = data[i]

    return mode_value


def test_variance() -> float:
    """Test calculating variance"""
    data: List[float] = [2.0, 4.0, 6.0, 8.0, 10.0]

    # Calculate mean
    total: float = 0.0
    for value in data:
        total = total + value
    mean: float = total / float(len(data))

    # Calculate variance
    variance_sum: float = 0.0
    for value in data:
        diff: float = value - mean
        variance_sum = variance_sum + (diff * diff)

    variance: float = variance_sum / float(len(data))

    return variance


def test_stdev() -> float:
    """Test calculating standard deviation"""
    data: List[float] = [2.0, 4.0, 6.0, 8.0, 10.0]

    # Calculate mean
    total: float = 0.0
    for value in data:
        total = total + value
    mean: float = total / float(len(data))

    # Calculate variance
    variance_sum: float = 0.0
    for value in data:
        diff: float = value - mean
        variance_sum = variance_sum + (diff * diff)

    variance: float = variance_sum / float(len(data))

    # Standard deviation is square root of variance
    stdev: float = math.sqrt(variance)

    return stdev


def test_min_max() -> tuple:
    """Test finding min and max"""
    data: List[float] = [3.5, 1.2, 7.8, 2.4, 9.1]

    if len(data) == 0:
        return (0.0, 0.0)

    min_val: float = data[0]
    max_val: float = data[0]

    for value in data:
        if value < min_val:
            min_val = value
        if value > max_val:
            max_val = value

    return (min_val, max_val)


def test_range() -> float:
    """Test calculating range (max - min)"""
    data: List[float] = [1.0, 5.0, 3.0, 9.0, 2.0]

    if len(data) == 0:
        return 0.0

    min_val: float = data[0]
    max_val: float = data[0]

    for value in data:
        if value < min_val:
            min_val = value
        if value > max_val:
            max_val = value

    data_range: float = max_val - min_val

    return data_range


def test_sum() -> float:
    """Test sum calculation"""
    data: List[float] = [1.5, 2.5, 3.5, 4.5]

    total: float = 0.0
    for value in data:
        total = total + value

    return total


def calculate_percentile(data: List[float], percentile: int) -> float:
    """Calculate percentile (simplified)"""
    # Sort data
    sorted_data: List[float] = data.copy()
    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j] < sorted_data[i]:
                temp: float = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    # Calculate index
    index: int = (percentile * len(sorted_data)) // 100

    if index >= len(sorted_data):
        index = len(sorted_data) - 1

    return sorted_data[index]


def calculate_quartiles(data: List[float]) -> tuple:
    """Calculate Q1, Q2 (median), Q3"""
    q1: float = calculate_percentile(data, 25)
    q2: float = calculate_percentile(data, 50)
    q3: float = calculate_percentile(data, 75)

    return (q1, q2, q3)


def calculate_iqr(data: List[float]) -> float:
    """Calculate interquartile range (IQR)"""
    quartiles: tuple = calculate_quartiles(data)
    q1: float = quartiles[0]
    q3: float = quartiles[2]

    iqr: float = q3 - q1

    return iqr


def detect_outliers(data: List[float]) -> List[float]:
    """Detect outliers using IQR method"""
    quartiles: tuple = calculate_quartiles(data)
    q1: float = quartiles[0]
    q3: float = quartiles[2]
    iqr: float = q3 - q1

    lower_bound: float = q1 - 1.5 * iqr
    upper_bound: float = q3 + 1.5 * iqr

    outliers: List[float] = []
    for value in data:
        if value < lower_bound or value > upper_bound:
            outliers.append(value)

    return outliers


def normalize_data(data: List[float]) -> List[float]:
    """Normalize data to 0-1 range"""
    if len(data) == 0:
        return []

    min_val: float = data[0]
    max_val: float = data[0]

    for value in data:
        if value < min_val:
            min_val = value
        if value > max_val:
            max_val = value

    data_range: float = max_val - min_val

    if data_range == 0.0:
        return data

    normalized: List[float] = []
    for value in data:
        norm_value: float = (value - min_val) / data_range
        normalized.append(norm_value)

    return normalized


def standardize_data(data: List[float]) -> List[float]:
    """Standardize data (z-score)"""
    # Calculate mean
    total: float = 0.0
    for value in data:
        total = total + value
    mean: float = total / float(len(data))

    # Calculate stdev
    variance_sum: float = 0.0
    for value in data:
        diff: float = value - mean
        variance_sum = variance_sum + (diff * diff)
    variance: float = variance_sum / float(len(data))
    stdev: float = math.sqrt(variance)

    if stdev == 0.0:
        return data

    # Standardize
    standardized: List[float] = []
    for value in data:
        z_score: float = (value - mean) / stdev
        standardized.append(z_score)

    return standardized


def calculate_covariance(x: List[float], y: List[float]) -> float:
    """Calculate covariance between two datasets"""
    if len(x) != len(y) or len(x) == 0:
        return 0.0

    # Calculate means
    x_total: float = 0.0
    y_total: float = 0.0
    for i in range(len(x)):
        x_total = x_total + x[i]
        y_total = y_total + y[i]

    x_mean: float = x_total / float(len(x))
    y_mean: float = y_total / float(len(y))

    # Calculate covariance
    cov_sum: float = 0.0
    for i in range(len(x)):
        x_diff: float = x[i] - x_mean
        y_diff: float = y[i] - y_mean
        cov_sum = cov_sum + (x_diff * y_diff)

    covariance: float = cov_sum / float(len(x))

    return covariance


def calculate_correlation(x: List[float], y: List[float]) -> float:
    """Calculate Pearson correlation coefficient"""
    if len(x) != len(y) or len(x) == 0:
        return 0.0

    # Calculate covariance
    cov: float = calculate_covariance(x, y)

    # Calculate standard deviations
    x_total: float = 0.0
    for val in x:
        x_total = x_total + val
    x_mean: float = x_total / float(len(x))

    x_var_sum: float = 0.0
    for val in x:
        diff: float = val - x_mean
        x_var_sum = x_var_sum + (diff * diff)
    x_stdev: float = math.sqrt(x_var_sum / float(len(x)))

    y_total: float = 0.0
    for val in y:
        y_total = y_total + val
    y_mean: float = y_total / float(len(y))

    y_var_sum: float = 0.0
    for val in y:
        diff: float = val - y_mean
        y_var_sum = y_var_sum + (diff * diff)
    y_stdev: float = math.sqrt(y_var_sum / float(len(y)))

    if x_stdev == 0.0 or y_stdev == 0.0:
        return 0.0

    correlation: float = cov / (x_stdev * y_stdev)

    return correlation


def test_all_statistics_features() -> None:
    """Run all statistics module tests"""
    # Basic statistics
    data: List[float] = [1.0, 2.0, 3.0, 4.0, 5.0]
    mean: float = test_mean()
    median_odd: float = test_median_odd()
    median_even: float = test_median_even()

    # Mode
    mode_data: List[int] = [1, 2, 2, 3, 3, 3]
    mode: int = test_mode()

    # Variance and stdev
    variance: float = test_variance()
    stdev: float = test_stdev()

    # Min, max, range
    minmax: tuple = test_min_max()
    data_range: float = test_range()
    total: float = test_sum()

    # Percentiles and quartiles
    sample: List[float] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
    p50: float = calculate_percentile(sample, 50)
    quartiles: tuple = calculate_quartiles(sample)
    iqr: float = calculate_iqr(sample)

    # Outliers
    outlier_data: List[float] = [1.0, 2.0, 3.0, 4.0, 5.0, 100.0]
    outliers: List[float] = detect_outliers(outlier_data)

    # Normalization and standardization
    normalized: List[float] = normalize_data(sample)
    standardized: List[float] = standardize_data(sample)

    # Covariance and correlation
    x_data: List[float] = [1.0, 2.0, 3.0, 4.0, 5.0]
    y_data: List[float] = [2.0, 4.0, 6.0, 8.0, 10.0]
    cov: float = calculate_covariance(x_data, y_data)
    corr: float = calculate_correlation(x_data, y_data)

    print("All statistics module tests completed successfully")
