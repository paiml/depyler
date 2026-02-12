from typing import List, Tuple
import math

def softmax(logits: List[float]) -> List[float]:
    max_val: float = logits[0]
    for v in logits:
        if v > max_val:
            max_val = v
    exps: List[float] = []
    sum_exp: float = 0.0
    for v in logits:
        e: float = math.exp(v - max_val)
        exps.append(e)
        sum_exp = sum_exp + e
    result: List[float] = []
    for e in exps:
        result.append(e / sum_exp)
    return result

def log_softmax(logits: List[float]) -> List[float]:
    max_val: float = logits[0]
    for v in logits:
        if v > max_val:
            max_val = v
    sum_exp: float = 0.0
    for v in logits:
        sum_exp = sum_exp + math.exp(v - max_val)
    log_sum: float = max_val + math.log(sum_exp)
    result: List[float] = []
    for v in logits:
        result.append(v - log_sum)
    return result

def softmax_jacobian_diag(probs: List[float]) -> List[float]:
    result: List[float] = []
    for p in probs:
        result.append(p * (1.0 - p))
    return result

def temperature_softmax(logits: List[float], temp: float) -> List[float]:
    scaled: List[float] = []
    for v in logits:
        scaled.append(v / temp)
    return softmax(scaled)

def argmax(values: List[float]) -> int:
    best: int = 0
    for i in range(1, len(values)):
        if values[i] > values[best]:
            best = i
    return best
