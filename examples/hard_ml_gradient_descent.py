from typing import List, Tuple

def gd_step(params: List[float], grads: List[float], lr: float) -> List[float]:
    result: List[float] = []
    for i in range(len(params)):
        result.append(params[i] - lr * grads[i])
    return result

def compute_gradient(params: List[float], data: List[float], targets: List[float]) -> List[float]:
    grads: List[float] = [0.0] * len(params)
    for i in range(len(data)):
        pred: float = 0.0
        for j in range(len(params)):
            pred = pred + params[j] * (data[i] + float(j))
        err: float = pred - targets[i]
        for j in range(len(params)):
            grads[j] = grads[j] + err * (data[i] + float(j))
    n: float = float(len(data))
    result: List[float] = []
    for g in grads:
        result.append(g / n)
    return result

def gd_train(params: List[float], data: List[float], targets: List[float], lr: float, epochs: int) -> List[float]:
    current: List[float] = []
    for p in params:
        current.append(p)
    for e in range(epochs):
        grads: List[float] = compute_gradient(current, data, targets)
        current = gd_step(current, grads, lr)
    return current

def momentum_step(params: List[float], grads: List[float], velocity: List[float], lr: float, beta: float) -> Tuple[List[float], List[float]]:
    new_v: List[float] = []
    new_p: List[float] = []
    for i in range(len(params)):
        v: float = beta * velocity[i] + grads[i]
        new_v.append(v)
        new_p.append(params[i] - lr * v)
    return (new_p, new_v)

def learning_rate_decay(initial_lr: float, epoch: int, decay: float) -> float:
    return initial_lr / (1.0 + decay * float(epoch))
