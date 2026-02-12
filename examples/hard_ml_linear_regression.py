from typing import List, Tuple
import math

def dot_product(a: List[float], b: List[float]) -> float:
    result: float = 0.0
    for i in range(len(a)):
        result = result + a[i] * b[i]
    return result

def predict(weights: List[float], features: List[float], bias: float) -> float:
    return dot_product(weights, features) + bias

def mse_loss(predictions: List[float], targets: List[float]) -> float:
    total: float = 0.0
    for i in range(len(predictions)):
        diff: float = predictions[i] - targets[i]
        total = total + diff * diff
    return total / float(len(predictions))

def gradient_step(weights: List[float], features: List[List[float]], targets: List[float], lr: float) -> List[float]:
    n: int = len(targets)
    new_weights: List[float] = []
    for j in range(len(weights)):
        grad: float = 0.0
        for i in range(n):
            pred: float = dot_product(weights, features[i])
            grad = grad + (pred - targets[i]) * features[i][j]
        grad = grad * 2.0 / float(n)
        new_weights.append(weights[j] - lr * grad)
    return new_weights

def r_squared(predictions: List[float], targets: List[float]) -> float:
    mean: float = 0.0
    for t in targets:
        mean = mean + t
    mean = mean / float(len(targets))
    ss_res: float = 0.0
    ss_tot: float = 0.0
    for i in range(len(targets)):
        ss_res = ss_res + (targets[i] - predictions[i]) * (targets[i] - predictions[i])
        ss_tot = ss_tot + (targets[i] - mean) * (targets[i] - mean)
    if ss_tot == 0.0:
        return 0.0
    return 1.0 - ss_res / ss_tot
