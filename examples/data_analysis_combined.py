"""
Comprehensive Data Analysis Example
Combines: math, statistics, random, collections

This example demonstrates realistic data analysis scenarios using
multiple Python stdlib modules working together.

Tests transpiler's ability to handle:
- Multiple module imports
- Module interaction patterns
- Complex data flows between modules
- Real-world analytics operations
"""

import math
import random
from collections import Counter, defaultdict
from typing import List, Dict, Tuple


def generate_sample_data(size: int, mean: float, stddev: float) -> List[float]:
    """Generate sample data using normal distribution (simplified)"""
    data: List[float] = []

    for i in range(size):
        # Simple random data generation
        value: float = random.random() * stddev + mean
        data.append(value)

    return data


def calculate_statistics(data: List[float]) -> Dict[str, float]:
    """Calculate comprehensive statistics on dataset"""
    if len(data) == 0:
        return {}

    stats: Dict[str, float] = {}

    # Mean
    total: float = 0.0
    for value in data:
        total = total + value
    mean: float = total / float(len(data))
    stats["mean"] = mean

    # Variance and Standard Deviation
    variance_sum: float = 0.0
    for value in data:
        diff: float = value - mean
        variance_sum = variance_sum + (diff * diff)

    variance: float = variance_sum / float(len(data))
    stats["variance"] = variance
    stats["std_dev"] = math.sqrt(variance)

    # Min and Max
    min_val: float = data[0]
    max_val: float = data[0]
    for value in data:
        if value < min_val:
            min_val = value
        if value > max_val:
            max_val = value

    stats["min"] = min_val
    stats["max"] = max_val
    stats["range"] = max_val - min_val

    # Median (requires sorting)
    sorted_data: List[float] = data.copy()
    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j] < sorted_data[i]:
                temp: float = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    mid: int = len(sorted_data) // 2
    if len(sorted_data) % 2 == 1:
        stats["median"] = sorted_data[mid]
    else:
        stats["median"] = (sorted_data[mid - 1] + sorted_data[mid]) / 2.0

    return stats


def calculate_percentiles(data: List[float]) -> Dict[str, float]:
    """Calculate quartiles using math and sorting"""
    if len(data) == 0:
        return {}

    # Sort data
    sorted_data: List[float] = data.copy()
    for i in range(len(sorted_data)):
        for j in range(i + 1, len(sorted_data)):
            if sorted_data[j] < sorted_data[i]:
                temp: float = sorted_data[i]
                sorted_data[i] = sorted_data[j]
                sorted_data[j] = temp

    percentiles: Dict[str, float] = {}

    # Q1 (25th percentile)
    q1_idx: int = len(sorted_data) // 4
    percentiles["q1"] = sorted_data[q1_idx]

    # Q2 (50th percentile / median)
    q2_idx: int = len(sorted_data) // 2
    percentiles["q2"] = sorted_data[q2_idx]

    # Q3 (75th percentile)
    q3_idx: int = (3 * len(sorted_data)) // 4
    percentiles["q3"] = sorted_data[q3_idx]

    # IQR
    percentiles["iqr"] = percentiles["q3"] - percentiles["q1"]

    return percentiles


def detect_outliers(data: List[float]) -> List[float]:
    """Detect outliers using IQR method (combines statistics + collections)"""
    percentiles: Dict[str, float] = calculate_percentiles(data)

    if len(percentiles) == 0:
        return []

    q1: float = percentiles["q1"]
    q3: float = percentiles["q3"]
    iqr: float = percentiles["iqr"]

    lower_bound: float = q1 - 1.5 * iqr
    upper_bound: float = q3 + 1.5 * iqr

    outliers: List[float] = []
    for value in data:
        if value < lower_bound or value > upper_bound:
            outliers.append(value)

    return outliers


def bin_data(data: List[float], num_bins: int) -> Dict[int, int]:
    """Create histogram bins (uses collections + math)"""
    if len(data) == 0 or num_bins <= 0:
        return {}

    # Find min and max
    min_val: float = data[0]
    max_val: float = data[0]
    for value in data:
        if value < min_val:
            min_val = value
        if value > max_val:
            max_val = value

    # Calculate bin width
    bin_width: float = (max_val - min_val) / float(num_bins)

    # Count values in each bin
    bins: Dict[int, int] = {}
    for i in range(num_bins):
        bins[i] = 0

    for value in data:
        bin_index: int = int((value - min_val) / bin_width)
        if bin_index >= num_bins:
            bin_index = num_bins - 1

        bins[bin_index] = bins[bin_index] + 1

    return bins


def calculate_correlation(x: List[float], y: List[float]) -> float:
    """Calculate Pearson correlation coefficient"""
    if len(x) != len(y) or len(x) == 0:
        return 0.0

    # Calculate means
    x_sum: float = 0.0
    y_sum: float = 0.0
    for i in range(len(x)):
        x_sum = x_sum + x[i]
        y_sum = y_sum + y[i]

    x_mean: float = x_sum / float(len(x))
    y_mean: float = y_sum / float(len(y))

    # Calculate correlation components
    numerator: float = 0.0
    x_variance_sum: float = 0.0
    y_variance_sum: float = 0.0

    for i in range(len(x)):
        x_diff: float = x[i] - x_mean
        y_diff: float = y[i] - y_mean

        numerator = numerator + (x_diff * y_diff)
        x_variance_sum = x_variance_sum + (x_diff * x_diff)
        y_variance_sum = y_variance_sum + (y_diff * y_diff)

    denominator: float = math.sqrt(x_variance_sum * y_variance_sum)

    if denominator == 0.0:
        return 0.0

    correlation: float = numerator / denominator
    return correlation


def normalize_data(data: List[float]) -> List[float]:
    """Z-score normalization using statistics"""
    if len(data) == 0:
        return []

    # Calculate mean
    total: float = 0.0
    for value in data:
        total = total + value
    mean: float = total / float(len(data))

    # Calculate standard deviation
    variance_sum: float = 0.0
    for value in data:
        diff: float = value - mean
        variance_sum = variance_sum + (diff * diff)

    stddev: float = math.sqrt(variance_sum / float(len(data)))

    if stddev == 0.0:
        return data

    # Normalize
    normalized: List[float] = []
    for value in data:
        z_score: float = (value - mean) / stddev
        normalized.append(z_score)

    return normalized


def group_by_range(data: List[float], ranges: List[Tuple[float, float]]) -> Dict[str, List[float]]:
    """Group data by ranges using collections"""
    groups: Dict[str, List[float]] = {}

    for i in range(len(ranges)):
        range_tuple: Tuple[float, float] = ranges[i]
        range_key: str = f"{range_tuple[0]}-{range_tuple[1]}"
        groups[range_key] = []

    for value in data:
        for i in range(len(ranges)):
            range_tuple: Tuple[float, float] = ranges[i]
            if value >= range_tuple[0] and value < range_tuple[1]:
                range_key: str = f"{range_tuple[0]}-{range_tuple[1]}"
                groups[range_key].append(value)
                break

    return groups


def monte_carlo_simulation(num_trials: int) -> Dict[str, float]:
    """Monte Carlo simulation combining random + math + statistics"""
    results: List[float] = []

    for trial in range(num_trials):
        # Simulate random process
        x: float = random.random() * 10.0
        y: float = random.random() * 10.0

        # Calculate result using math
        distance: float = math.sqrt(x * x + y * y)
        results.append(distance)

    # Analyze results with statistics
    stats: Dict[str, float] = calculate_statistics(results)

    return stats


def analyze_dataset() -> None:
    """Main analysis pipeline combining all modules"""
    print("=== Comprehensive Data Analysis Demo ===")

    # Generate sample data
    random.seed(42)
    sample_size: int = 100
    dataset: List[float] = generate_sample_data(sample_size, 50.0, 10.0)

    # Calculate comprehensive statistics
    stats: Dict[str, float] = calculate_statistics(dataset)
    print(f"Mean: {stats['mean']}, StdDev: {stats['std_dev']}")

    # Calculate percentiles
    percentiles: Dict[str, float] = calculate_percentiles(dataset)
    print(f"Q1: {percentiles['q1']}, Median: {percentiles['q2']}, Q3: {percentiles['q3']}")

    # Detect outliers
    outliers: List[float] = detect_outliers(dataset)
    print(f"Outliers found: {len(outliers)}")

    # Create histogram
    histogram: Dict[int, int] = bin_data(dataset, 10)
    print(f"Histogram bins created: {len(histogram)}")

    # Normalize data
    normalized: List[float] = normalize_data(dataset)
    normalized_stats: Dict[str, float] = calculate_statistics(normalized)
    print(f"Normalized mean: {normalized_stats['mean']}")

    # Generate second dataset for correlation
    dataset2: List[float] = generate_sample_data(sample_size, 60.0, 12.0)

    # Calculate correlation
    corr: float = calculate_correlation(dataset, dataset2)
    print(f"Correlation: {corr}")

    # Group by ranges
    ranges: List[Tuple[float, float]] = [(0.0, 25.0), (25.0, 50.0), (50.0, 75.0), (75.0, 100.0)]
    groups: Dict[str, List[float]] = group_by_range(dataset, ranges)
    print(f"Range groups created: {len(groups)}")

    # Run Monte Carlo simulation
    mc_stats: Dict[str, float] = monte_carlo_simulation(1000)
    print(f"Monte Carlo mean: {mc_stats['mean']}")

    print("=== Analysis Complete ===")


if __name__ == "__main__":
    analyze_dataset()
