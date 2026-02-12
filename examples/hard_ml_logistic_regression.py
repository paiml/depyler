from typing import List, Tuple
import math

def sigmoid(x: float) -> float:
    if x > 20.0:
        return 1.0
    if x < -20.0:
        return 0.0
    return 1.0 / (1.0 + math.exp(0.0 - x))

def log_predict(weights: List[float], features: List[float], bias: float) -> float:
    z: float = bias
    for i in range(len(weights)):
        z = z + weights[i] * features[i]
    return sigmoid(z)

def binary_cross_entropy(pred: float, target: float) -> float:
    eps: float = 0.0000001
    p: float = pred
    if p < eps:
        p = eps
    if p > 1.0 - eps:
        p = 1.0 - eps
    return 0.0 - (target * math.log(p) + (1.0 - target) * math.log(1.0 - p))

def log_gradient(weights: List[float], features: List[List[float]], targets: List[float], bias: float, lr: float) -> Tuple[List[float], float]:
    n: int = len(targets)
    new_w: List[float] = [0.0] * len(weights)
    b_grad: float = 0.0
    for i in range(n):
        pred: float = log_predict(weights, features[i], bias)
        err: float = pred - targets[i]
        for j in range(len(weights)):
            new_w[j] = new_w[j] + err * features[i][j]
        b_grad = b_grad + err
    result: List[float] = []
    for j in range(len(weights)):
        result.append(weights[j] - lr * new_w[j] / float(n))
    return (result, bias - lr * b_grad / float(n))

def accuracy(predictions: List[float], targets: List[float], threshold: float) -> float:
    correct: int = 0
    for i in range(len(predictions)):
        pred_class: int = 0
        if predictions[i] >= threshold:
            pred_class = 1
        target_class: int = 0
        if targets[i] >= 0.5:
            target_class = 1
        if pred_class == target_class:
            correct = correct + 1
    return float(correct) / float(len(predictions))
