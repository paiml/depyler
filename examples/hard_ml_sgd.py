from typing import List, Tuple

def sgd_update(params: List[float], grad: List[float], lr: float) -> List[float]:
    result: List[float] = []
    for i in range(len(params)):
        result.append(params[i] - lr * grad[i])
    return result

def sample_gradient(params: List[float], x: float, y: float) -> List[float]:
    pred: float = params[0] * x + params[1]
    err: float = pred - y
    return [err * x, err]

def sgd_train(params: List[float], xs: List[float], ys: List[float], lr: float, epochs: int) -> List[float]:
    current: List[float] = [params[0], params[1]]
    for e in range(epochs):
        for i in range(len(xs)):
            grad: List[float] = sample_gradient(current, xs[i], ys[i])
            current = sgd_update(current, grad, lr)
    return current

def mini_batch_grad(params: List[float], xs: List[float], ys: List[float], start: int, batch_size: int) -> List[float]:
    grads: List[float] = [0.0, 0.0]
    end: int = start + batch_size
    if end > len(xs):
        end = len(xs)
    for i in range(start, end):
        g: List[float] = sample_gradient(params, xs[i], ys[i])
        grads[0] = grads[0] + g[0]
        grads[1] = grads[1] + g[1]
    n: float = float(end - start)
    return [grads[0] / n, grads[1] / n]

def sgd_loss(params: List[float], xs: List[float], ys: List[float]) -> float:
    total: float = 0.0
    for i in range(len(xs)):
        pred: float = params[0] * xs[i] + params[1]
        diff: float = pred - ys[i]
        total = total + diff * diff
    return total / float(len(xs))
