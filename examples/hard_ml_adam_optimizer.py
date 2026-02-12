from typing import List, Tuple
import math

def adam_step(params: List[float], grads: List[float], m: List[float], v: List[float], t: int, lr: float, beta1: float, beta2: float) -> Tuple[List[float], List[float], List[float]]:
    eps: float = 0.00000001
    new_m: List[float] = []
    new_v: List[float] = []
    new_p: List[float] = []
    for i in range(len(params)):
        mi: float = beta1 * m[i] + (1.0 - beta1) * grads[i]
        vi: float = beta2 * v[i] + (1.0 - beta2) * grads[i] * grads[i]
        new_m.append(mi)
        new_v.append(vi)
        m_hat: float = mi / (1.0 - math.pow(beta1, float(t)))
        v_hat: float = vi / (1.0 - math.pow(beta2, float(t)))
        new_p.append(params[i] - lr * m_hat / (math.sqrt(v_hat) + eps))
    return (new_p, new_m, new_v)

def init_adam(n: int) -> Tuple[List[float], List[float]]:
    m: List[float] = [0.0] * n
    v: List[float] = [0.0] * n
    return (m, v)

def adam_train(params: List[float], grads_fn_seed: List[float], lr: float, epochs: int) -> List[float]:
    mv: Tuple[List[float], List[float]] = init_adam(len(params))
    m: List[float] = mv[0]
    v: List[float] = mv[1]
    current: List[float] = []
    for p in params:
        current.append(p)
    for t in range(1, epochs + 1):
        grads: List[float] = []
        for i in range(len(current)):
            grads.append(current[i] * grads_fn_seed[i % len(grads_fn_seed)])
        result: Tuple[List[float], List[float], List[float]] = adam_step(current, grads, m, v, t, lr, 0.9, 0.999)
        current = result[0]
        m = result[1]
        v = result[2]
    return current

def grad_norm(grads: List[float]) -> float:
    s: float = 0.0
    for g in grads:
        s = s + g * g
    return math.sqrt(s)

def clip_grads(grads: List[float], max_norm: float) -> List[float]:
    norm: float = grad_norm(grads)
    if norm <= max_norm:
        return grads
    result: List[float] = []
    for g in grads:
        result.append(g * max_norm / norm)
    return result
