from typing import List, Tuple
import math

def prior_prob(labels: List[int], c: int) -> float:
    count: int = 0
    for l in labels:
        if l == c:
            count = count + 1
    return float(count) / float(len(labels))

def likelihood(features: List[float], labels: List[int], col: int, threshold: float, c: int) -> float:
    match: int = 0
    total: int = 0
    for i in range(len(labels)):
        if labels[i] == c:
            total = total + 1
            if features[i] > threshold:
                match = match + 1
    if total == 0:
        return 0.0
    return float(match) / float(total)

def gaussian_pdf(x: float, mean: float, var: float) -> float:
    if var <= 0.0:
        return 0.0
    exp_val: float = 0.0 - ((x - mean) * (x - mean)) / (2.0 * var)
    return math.exp(exp_val) / math.sqrt(2.0 * 3.14159265 * var)

def nb_predict(priors: List[float], means: List[List[float]], variances: List[List[float]], features: List[float]) -> int:
    best: int = 0
    best_prob: float = -999999.0
    for c in range(len(priors)):
        log_prob: float = math.log(priors[c] + 0.0001)
        for j in range(len(features)):
            pdf: float = gaussian_pdf(features[j], means[c][j], variances[c][j])
            if pdf > 0.0:
                log_prob = log_prob + math.log(pdf)
        if log_prob > best_prob:
            best_prob = log_prob
            best = c
    return best

def class_stats(features: List[List[float]], labels: List[int], c: int) -> Tuple[List[float], List[float]]:
    n: int = 0
    dims: int = len(features[0]) if len(features) > 0 else 0
    sums: List[float] = [0.0] * dims
    for i in range(len(labels)):
        if labels[i] == c:
            n = n + 1
            for j in range(dims):
                sums[j] = sums[j] + features[i][j]
    means: List[float] = []
    for s in sums:
        if n > 0:
            means.append(s / float(n))
        else:
            means.append(0.0)
    variances: List[float] = [0.0] * dims
    for i in range(len(labels)):
        if labels[i] == c:
            for j in range(dims):
                d: float = features[i][j] - means[j]
                variances[j] = variances[j] + d * d
    result_var: List[float] = []
    for v in variances:
        if n > 1:
            result_var.append(v / float(n - 1))
        else:
            result_var.append(0.01)
    return (means, result_var)
