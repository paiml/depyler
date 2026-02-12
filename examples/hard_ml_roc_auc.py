from typing import List, Tuple
import math

def roc_auc_forward(inputs: List[float], weights: List[float]) -> List[float]:
    result: List[float] = []
    for i in range(len(inputs)):
        val: float = 0.0
        for j in range(len(weights)):
            val = val + inputs[i] * weights[j]
        result.append(val)
    return result

def roc_auc_backward(outputs: List[float], targets: List[float]) -> List[float]:
    grads: List[float] = []
    for i in range(len(outputs)):
        grads.append(outputs[i] - targets[i])
    return grads

def roc_auc_loss(predictions: List[float], targets: List[float]) -> float:
    total: float = 0.0
    for i in range(len(predictions)):
        diff: float = predictions[i] - targets[i]
        total = total + diff * diff
    return total / float(len(predictions))

def roc_auc_update(weights: List[float], grads: List[float], lr: float) -> List[float]:
    result: List[float] = []
    for i in range(len(weights)):
        if i < len(grads):
            result.append(weights[i] - lr * grads[i])
        else:
            result.append(weights[i])
    return result

def roc_auc_evaluate(predictions: List[float], targets: List[float]) -> float:
    correct: int = 0
    for i in range(len(predictions)):
        if abs(predictions[i] - targets[i]) < 0.5:
            correct = correct + 1
    if len(predictions) == 0:
        return 0.0
    return float(correct) / float(len(predictions))
