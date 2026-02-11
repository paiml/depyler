"""K-Nearest Neighbors classifier using Manhattan distance.

Features stored as flat list[int] with stride = dim.
Labels stored separately. Uses majority voting among k neighbors.
"""


def manhattan_dist(features: list[int], idx_a: int, idx_b: int, dim: int) -> int:
    """Manhattan distance between two points in flat feature array."""
    total: int = 0
    j: int = 0
    while j < dim:
        va: int = features[idx_a * dim + j]
        vb: int = features[idx_b * dim + j]
        diff: int = va - vb
        if diff < 0:
            diff = 0 - diff
        total = total + diff
        j = j + 1
    return total


def find_k_nearest(features: list[int], labels: list[int], query_idx: int, dim: int, k_val: int) -> list[int]:
    """Find k nearest neighbor labels for query point. Excludes query itself."""
    n: int = len(labels)
    dists: list[int] = []
    indices: list[int] = []
    i: int = 0
    while i < n:
        if i != query_idx:
            d: int = manhattan_dist(features, query_idx, i, dim)
            dists.append(d)
            indices.append(i)
        i = i + 1
    m: int = len(dists)
    p: int = 0
    while p < m:
        q: int = p + 1
        while q < m:
            dp: int = dists[p]
            dq: int = dists[q]
            if dp > dq:
                dists[p] = dq
                dists[q] = dp
                ip: int = indices[p]
                iq: int = indices[q]
                indices[p] = iq
                indices[q] = ip
            q = q + 1
        p = p + 1
    result: list[int] = []
    r: int = 0
    while r < k_val:
        if r < m:
            idx: int = indices[r]
            lv: int = labels[idx]
            result.append(lv)
        r = r + 1
    return result


def majority_vote(neighbor_labels: list[int]) -> int:
    """Return most common label. Tie-breaks by smallest label."""
    if len(neighbor_labels) == 0:
        return 0
    unique_labs: list[int] = []
    counts: list[int] = []
    i: int = 0
    while i < len(neighbor_labels):
        lv: int = neighbor_labels[i]
        found: int = 0 - 1
        j: int = 0
        while j < len(unique_labs):
            uv: int = unique_labs[j]
            if uv == lv:
                found = j
            j = j + 1
        if found >= 0:
            old_c: int = counts[found]
            counts[found] = old_c + 1
        else:
            unique_labs.append(lv)
            counts.append(1)
        i = i + 1
    best_lab: int = unique_labs[0]
    best_cnt: int = counts[0]
    k2: int = 1
    while k2 < len(unique_labs):
        cv: int = counts[k2]
        uv2: int = unique_labs[k2]
        if cv > best_cnt:
            best_cnt = cv
            best_lab = uv2
        k2 = k2 + 1
    return best_lab


def knn_predict(features: list[int], labels: list[int], query_idx: int, dim: int, k_val: int) -> int:
    """Predict label for query using KNN."""
    neighbors: list[int] = find_k_nearest(features, labels, query_idx, dim, k_val)
    return majority_vote(neighbors)


def knn_accuracy(features: list[int], labels: list[int], dim: int, k_val: int) -> int:
    """Leave-one-out accuracy as percentage 0-100."""
    correct: int = 0
    n: int = len(labels)
    i: int = 0
    while i < n:
        pred: int = knn_predict(features, labels, i, dim, k_val)
        actual: int = labels[i]
        if pred == actual:
            correct = correct + 1
        i = i + 1
    if n == 0:
        return 0
    return correct * 100 // n


def test_module() -> int:
    """Test KNN classifier."""
    ok: int = 0
    feats: list[int] = [0, 0, 1, 0, 0, 1, 10, 10, 11, 10, 10, 11]
    labs: list[int] = [0, 0, 0, 1, 1, 1]
    p0: int = knn_predict(feats, labs, 0, 2, 3)
    if p0 == 0:
        ok = ok + 1
    p3: int = knn_predict(feats, labs, 3, 2, 3)
    if p3 == 1:
        ok = ok + 1
    d: int = manhattan_dist(feats, 0, 3, 2)
    if d == 20:
        ok = ok + 1
    mv: int = majority_vote([0, 1, 1, 0, 1])
    if mv == 1:
        ok = ok + 1
    acc: int = knn_accuracy(feats, labs, 2, 3)
    if acc >= 80:
        ok = ok + 1
    return ok
