from typing import List, Tuple
import math

def distance(a: List[float], b: List[float]) -> float:
    s: float = 0.0
    for i in range(len(a)):
        d: float = a[i] - b[i]
        s = s + d * d
    return math.sqrt(s)

def find_k_nearest(train: List[List[float]], query: List[float], k: int) -> List[int]:
    dists: List[Tuple[float, int]] = []
    for i in range(len(train)):
        d: float = distance(train[i], query)
        dists.append((d, i))
    for i in range(len(dists)):
        for j in range(i + 1, len(dists)):
            if dists[j][0] < dists[i][0]:
                temp: Tuple[float, int] = dists[i]
                dists[i] = dists[j]
                dists[j] = temp
    result: List[int] = []
    for i in range(k):
        if i < len(dists):
            result.append(dists[i][1])
    return result

def knn_classify(train: List[List[float]], labels: List[int], query: List[float], k: int) -> int:
    neighbors: List[int] = find_k_nearest(train, query, k)
    votes: List[int] = [0] * 10
    for n in neighbors:
        if labels[n] < 10:
            votes[labels[n]] = votes[labels[n]] + 1
    best: int = 0
    for i in range(10):
        if votes[i] > votes[best]:
            best = i
    return best

def knn_regress(train: List[List[float]], values: List[float], query: List[float], k: int) -> float:
    neighbors: List[int] = find_k_nearest(train, query, k)
    s: float = 0.0
    for n in neighbors:
        s = s + values[n]
    return s / float(k)

def weighted_knn(train: List[List[float]], labels: List[int], query: List[float], k: int) -> int:
    neighbors: List[int] = find_k_nearest(train, query, k)
    weights: List[float] = [0.0] * 10
    for n in neighbors:
        d: float = distance(train[n], query) + 0.001
        if labels[n] < 10:
            weights[labels[n]] = weights[labels[n]] + 1.0 / d
    best: int = 0
    for i in range(10):
        if weights[i] > weights[best]:
            best = i
    return best
