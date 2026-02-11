"""Perceptron classifier for binary classification.

Implements single-layer perceptron with integer weights scaled by 1000.
Features stored as flat list with stride = num_features.
"""


def dot_int(weights: list[int], features: list[int], start: int, dim: int) -> int:
    """Dot product of weights and features[start:start+dim]."""
    total: int = 0
    j: int = 0
    while j < dim:
        wv: int = weights[j]
        fv: int = features[start + j]
        total = total + wv * fv
        j = j + 1
    return total


def perceptron_predict(weights: list[int], bias: int, features: list[int], start: int, dim: int) -> int:
    """Predict 0 or 1 using perceptron."""
    score: int = dot_int(weights, features, start, dim) + bias
    if score >= 0:
        return 1
    return 0


def perceptron_train(features: list[int], labels: list[int], dim: int, epochs: int, lr: int) -> list[int]:
    """Train perceptron. Returns weights followed by bias as last element.

    lr is learning rate scaled by 1000 (e.g., 100 = 0.1).
    """
    weights: list[int] = []
    j: int = 0
    while j < dim:
        weights.append(0)
        j = j + 1
    bias: int = 0
    n: int = len(labels)
    ep: int = 0
    while ep < epochs:
        i: int = 0
        while i < n:
            pred: int = perceptron_predict(weights, bias, features, i * dim, dim)
            lv: int = labels[i]
            err: int = lv - pred
            if err != 0:
                k: int = 0
                while k < dim:
                    old_w: int = weights[k]
                    fv: int = features[i * dim + k]
                    weights[k] = old_w + lr * err * fv
                    k = k + 1
                bias = bias + lr * err
            i = i + 1
        ep = ep + 1
    result: list[int] = []
    m: int = 0
    while m < dim:
        wv: int = weights[m]
        result.append(wv)
        m = m + 1
    result.append(bias)
    return result


def perceptron_accuracy(weights: list[int], bias: int, features: list[int], labels: list[int], dim: int) -> int:
    """Accuracy as percentage 0-100."""
    correct: int = 0
    n: int = len(labels)
    i: int = 0
    while i < n:
        pred: int = perceptron_predict(weights, bias, features, i * dim, dim)
        lv: int = labels[i]
        if pred == lv:
            correct = correct + 1
        i = i + 1
    if n == 0:
        return 0
    return correct * 100 // n


def test_module() -> int:
    """Test perceptron classifier."""
    ok: int = 0
    feats: list[int] = [0, 0, 0, 1, 1, 0, 1, 1]
    labs: list[int] = [0, 1, 1, 1]
    trained: list[int] = perceptron_train(feats, labs, 2, 10, 1)
    if len(trained) == 3:
        ok = ok + 1
    w0: int = trained[0]
    w1: int = trained[1]
    b: int = trained[2]
    acc: int = perceptron_accuracy([w0, w1], b, feats, labs, 2)
    if acc >= 75:
        ok = ok + 1
    p1: int = perceptron_predict([w0, w1], b, [1, 1, 0, 0], 0, 2)
    if p1 == 1:
        ok = ok + 1
    d: int = dot_int([1, 2, 3], [4, 5, 6], 0, 3)
    if d == 32:
        ok = ok + 1
    d2: int = dot_int([1, 0], [0, 1, 1, 0], 2, 2)
    if d2 == 1:
        ok = ok + 1
    return ok
