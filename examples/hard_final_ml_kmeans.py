"""K-means clustering with flat 2D point representation.

Implements Lloyd's algorithm: assign points to nearest centroid, recompute centroids.
Points stored as flat list[float] with stride 2 (x, y alternating).
"""


def distance_sq(x1: float, y1: float, x2: float, y2: float) -> float:
    """Squared Euclidean distance between two 2D points."""
    dx: float = x1 - x2
    dy: float = y1 - y2
    return dx * dx + dy * dy


def assign_clusters(points: list[float], centroids: list[float], k: int) -> list[int]:
    """Assign each point to its nearest centroid. Returns cluster index per point."""
    n: int = len(points) // 2
    assignments: list[int] = []
    i: int = 0
    while i < n:
        idx_x: int = i * 2
        idx_y: int = i * 2 + 1
        px: float = points[idx_x]
        py: float = points[idx_y]
        best: int = 0
        best_dist: float = distance_sq(px, py, centroids[0], centroids[1])
        j: int = 1
        while j < k:
            jx: int = j * 2
            jy: int = j * 2 + 1
            cx: float = centroids[jx]
            cy: float = centroids[jy]
            d: float = distance_sq(px, py, cx, cy)
            if d < best_dist:
                best_dist = d
                best = j
            j = j + 1
        assignments.append(best)
        i = i + 1
    return assignments


def update_centroids(points: list[float], assignments: list[int], k: int) -> list[float]:
    """Recompute centroids as mean of assigned points."""
    sums_x: list[float] = []
    sums_y: list[float] = []
    counts: list[int] = []
    j: int = 0
    while j < k:
        sums_x.append(0.0)
        sums_y.append(0.0)
        counts.append(0)
        j = j + 1
    n: int = len(assignments)
    i: int = 0
    while i < n:
        c: int = assignments[i]
        ix: int = i * 2
        iy: int = i * 2 + 1
        px: float = points[ix]
        py: float = points[iy]
        old_sx: float = sums_x[c]
        sums_x[c] = old_sx + px
        old_sy: float = sums_y[c]
        sums_y[c] = old_sy + py
        old_cnt: int = counts[c]
        counts[c] = old_cnt + 1
        i = i + 1
    new_centroids: list[float] = []
    j2: int = 0
    while j2 < k:
        cnt: int = counts[j2]
        if cnt > 0:
            new_centroids.append(sums_x[j2] / (cnt * 1.0))
            new_centroids.append(sums_y[j2] / (cnt * 1.0))
        else:
            new_centroids.append(0.0)
            new_centroids.append(0.0)
        j2 = j2 + 1
    return new_centroids


def kmeans(points: list[float], k: int, iters: int) -> list[int]:
    """Run k-means for fixed iterations. Initial centroids = first k points."""
    centroids: list[float] = []
    i: int = 0
    while i < k:
        kx: int = i * 2
        ky: int = i * 2 + 1
        centroids.append(points[kx])
        centroids.append(points[ky])
        i = i + 1
    assignments: list[int] = []
    step: int = 0
    while step < iters:
        assignments = assign_clusters(points, centroids, k)
        centroids = update_centroids(points, assignments, k)
        step = step + 1
    return assignments


def test_module() -> int:
    """Test k-means clustering."""
    ok: int = 0
    pts: list[float] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 10.0, 10.0, 11.0, 10.0, 10.0, 11.0]
    a: list[int] = kmeans(pts, 2, 5)
    if len(a) == 6:
        ok = ok + 1
    c0: int = a[0]
    c3: int = a[3]
    if c0 != c3:
        ok = ok + 1
    c1: int = a[1]
    c2: int = a[2]
    if c0 == c1:
        ok = ok + 1
    if c3 == a[4]:
        ok = ok + 1
    d: float = distance_sq(0.0, 0.0, 3.0, 4.0)
    if d > 24.0:
        if d < 26.0:
            ok = ok + 1
    return ok
