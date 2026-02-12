from typing import List, Tuple
import math

def euclidean_dist(a: List[float], b: List[float]) -> float:
    s: float = 0.0
    for i in range(len(a)):
        d: float = a[i] - b[i]
        s = s + d * d
    return math.sqrt(s)

def assign_clusters(points: List[List[float]], centroids: List[List[float]]) -> List[int]:
    assignments: List[int] = []
    for p in points:
        best: int = 0
        best_dist: float = euclidean_dist(p, centroids[0])
        for c in range(1, len(centroids)):
            d: float = euclidean_dist(p, centroids[c])
            if d < best_dist:
                best_dist = d
                best = c
        assignments.append(best)
    return assignments

def update_centroids(points: List[List[float]], assignments: List[int], k: int, dims: int) -> List[List[float]]:
    sums: List[List[float]] = []
    counts: List[int] = [0] * k
    for i in range(k):
        sums.append([0.0] * dims)
    for i in range(len(points)):
        c: int = assignments[i]
        counts[c] = counts[c] + 1
        for d in range(dims):
            sums[c][d] = sums[c][d] + points[i][d]
    centroids: List[List[float]] = []
    for i in range(k):
        row: List[float] = []
        for d in range(dims):
            if counts[i] > 0:
                row.append(sums[i][d] / float(counts[i]))
            else:
                row.append(0.0)
        centroids.append(row)
    return centroids

def kmeans(points: List[List[float]], k: int, dims: int, iters: int) -> List[int]:
    centroids: List[List[float]] = []
    for i in range(k):
        if i < len(points):
            centroids.append(points[i])
    assignments: List[int] = assign_clusters(points, centroids)
    for it in range(iters):
        centroids = update_centroids(points, assignments, k, dims)
        assignments = assign_clusters(points, centroids)
    return assignments

def inertia(points: List[List[float]], centroids: List[List[float]], assignments: List[int]) -> float:
    total: float = 0.0
    for i in range(len(points)):
        d: float = euclidean_dist(points[i], centroids[assignments[i]])
        total = total + d * d
    return total
