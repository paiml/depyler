from typing import List, Tuple

def create_centroid(mean_scaled: int, weight: int) -> Tuple[int, int]:
    return (mean_scaled, weight)

def add_value(centroids: List[Tuple[int, int]], value: int) -> List[Tuple[int, int]]:
    if len(centroids) == 0:
        return [(value, 1)]
    best: int = 0
    best_dist: int = abs(value - centroids[0][0])
    for i in range(1, len(centroids)):
        d: int = abs(value - centroids[i][0])
        if d < best_dist:
            best_dist = d
            best = i
    result: List[Tuple[int, int]] = []
    for i in range(len(centroids)):
        if i == best:
            ow: int = centroids[i][1]
            nw: int = ow + 1
            nm: int = (centroids[i][0] * ow + value) // nw
            result.append((nm, nw))
        else:
            result.append(centroids[i])
    return result

def quantile_scaled(centroids: List[Tuple[int, int]], q_scaled: int) -> int:
    total: int = 0
    for c in centroids:
        total = total + c[1]
    target: int = (q_scaled * total) // 1000
    running: int = 0
    for c in centroids:
        running = running + c[1]
        if running >= target:
            return c[0]
    if len(centroids) > 0:
        return centroids[len(centroids) - 1][0]
    return 0

def digest_size(centroids: List[Tuple[int, int]]) -> int:
    return len(centroids)

def total_weight(centroids: List[Tuple[int, int]]) -> int:
    t: int = 0
    for c in centroids:
        t = t + c[1]
    return t
