# DEPYLER-1134: NumPy Bridge Validation Test
# Tests that the Sovereign Type DB correctly resolves numpy types
import numpy as np
from typing import List

def numpy_sum_test() -> int:
    """Test basic numpy array creation and sum"""
    a = np.array([1, 2, 3, 4, 5])
    return np.sum(a)

def numpy_dot_product(x: List[float], y: List[float]) -> float:
    """Test numpy dot product"""
    a = np.array(x)
    b = np.array(y)
    return np.dot(a, b)

def numpy_mean_std() -> float:
    """Test numpy statistics"""
    data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
    mean = np.mean(data)
    std = np.std(data)
    return mean + std
