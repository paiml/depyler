from typing import List, Tuple
import math

def cross_entropy(predictions: List[float], targets: List[float]) -> float:
    eps: float = 0.0000001
    loss: float = 0.0
    for i in range(len(predictions)):
        p: float = predictions[i]
        if p < eps:
            p = eps
        if p > 1.0 - eps:
            p = 1.0 - eps
        loss = loss - targets[i] * math.log(p)
    return loss / float(len(predictions))

def softmax_ce(logits: List[float], target_idx: int) -> float:
    max_val: float = logits[0]
    for v in logits:
        if v > max_val:
            max_val = v
    sum_exp: float = 0.0
    for v in logits:
        sum_exp = sum_exp + math.exp(v - max_val)
    return 0.0 - (logits[target_idx] - max_val - math.log(sum_exp))

def ce_gradient(predictions: List[float], targets: List[float]) -> List[float]:
    grads: List[float] = []
    eps: float = 0.0000001
    for i in range(len(predictions)):
        p: float = predictions[i]
        if p < eps:
            p = eps
        grads.append(0.0 - targets[i] / p)
    return grads

def kl_divergence(p: List[float], q: List[float]) -> float:
    eps: float = 0.0000001
    kl: float = 0.0
    for i in range(len(p)):
        if p[i] > eps:
            kl = kl + p[i] * math.log(p[i] / (q[i] + eps))
    return kl

def label_smoothing(targets: List[float], epsilon: float, num_classes: int) -> List[float]:
    result: List[float] = []
    for t in targets:
        result.append(t * (1.0 - epsilon) + epsilon / float(num_classes))
    return result
