from typing import List, Tuple

def relu(x: float) -> float:
    if x > 0.0:
        return x
    return 0.0

def relu_batch(values: List[float]) -> List[float]:
    result: List[float] = []
    for v in values:
        result.append(relu(v))
    return result

def leaky_relu(x: float, alpha: float) -> float:
    if x > 0.0:
        return x
    return alpha * x

def leaky_relu_batch(values: List[float], alpha: float) -> List[float]:
    result: List[float] = []
    for v in values:
        result.append(leaky_relu(v, alpha))
    return result

def relu_derivative(x: float) -> float:
    if x > 0.0:
        return 1.0
    return 0.0

def elu(x: float, alpha: float) -> float:
    if x > 0.0:
        return x
    return alpha * (2.718281828 ** x - 1.0)

def swish(x: float) -> float:
    if x > 20.0:
        return x
    if x < -20.0:
        return 0.0
    return x / (1.0 + 2.718281828 ** (0.0 - x))
