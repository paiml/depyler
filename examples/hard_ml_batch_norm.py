from typing import List, Tuple
import math

def batch_mean(values: List[float]) -> float:
    s: float = 0.0
    for v in values:
        s = s + v
    return s / float(len(values))

def batch_variance(values: List[float], mean: float) -> float:
    s: float = 0.0
    for v in values:
        d: float = v - mean
        s = s + d * d
    return s / float(len(values))

def batch_normalize(values: List[float], gamma: float, beta: float) -> List[float]:
    m: float = batch_mean(values)
    var: float = batch_variance(values, m)
    eps: float = 0.00001
    result: List[float] = []
    for v in values:
        normalized: float = (v - m) / math.sqrt(var + eps)
        result.append(gamma * normalized + beta)
    return result

def running_stats(running_mean: float, running_var: float, batch_mean_val: float, batch_var: float, momentum: float) -> Tuple[float, float]:
    new_mean: float = momentum * running_mean + (1.0 - momentum) * batch_mean_val
    new_var: float = momentum * running_var + (1.0 - momentum) * batch_var
    return (new_mean, new_var)

def bn_forward(values: List[float], gamma: float, beta: float, running_mean: float, running_var: float, training: bool) -> List[float]:
    if training:
        return batch_normalize(values, gamma, beta)
    eps: float = 0.00001
    result: List[float] = []
    for v in values:
        normalized: float = (v - running_mean) / math.sqrt(running_var + eps)
        result.append(gamma * normalized + beta)
    return result
