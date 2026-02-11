"""Decision stump classifier (single-split decision tree).

Finds optimal threshold on a single feature to split binary labels.
Uses integer features and labels (0 or 1).
"""


def find_best_threshold(features: list[int], labels: list[int]) -> int:
    """Find threshold that minimizes misclassification. Returns threshold value."""
    min_vals: list[int] = []
    i: int = 0
    while i < len(features):
        fv: int = features[i]
        found: int = 0
        j: int = 0
        while j < len(min_vals):
            mv: int = min_vals[j]
            if mv == fv:
                found = 1
            j = j + 1
        if found == 0:
            min_vals.append(fv)
        i = i + 1
    best_thresh: int = 0
    best_err: int = len(features) + 1
    ti: int = 0
    while ti < len(min_vals):
        thresh: int = min_vals[ti]
        err: int = 0
        k: int = 0
        while k < len(features):
            fv2: int = features[k]
            lv: int = labels[k]
            pred: int = 0
            if fv2 >= thresh:
                pred = 1
            if pred != lv:
                err = err + 1
            k = k + 1
        if err < best_err:
            best_err = err
            best_thresh = thresh
        ti = ti + 1
    return best_thresh


def stump_predict(features: list[int], thresh: int) -> list[int]:
    """Predict using decision stump."""
    preds: list[int] = []
    i: int = 0
    while i < len(features):
        fv: int = features[i]
        if fv >= thresh:
            preds.append(1)
        else:
            preds.append(0)
        i = i + 1
    return preds


def stump_accuracy(preds: list[int], labels: list[int]) -> int:
    """Compute accuracy as percentage 0-100."""
    correct: int = 0
    i: int = 0
    while i < len(preds):
        pv: int = preds[i]
        lv: int = labels[i]
        if pv == lv:
            correct = correct + 1
        i = i + 1
    if len(preds) == 0:
        return 0
    return correct * 100 // len(preds)


def weighted_error(features: list[int], labels: list[int], weights: list[int], thresh: int) -> int:
    """Weighted misclassification error (sum of weights for misclassified)."""
    err: int = 0
    i: int = 0
    while i < len(features):
        fv: int = features[i]
        lv: int = labels[i]
        wt: int = weights[i]
        pred: int = 0
        if fv >= thresh:
            pred = 1
        if pred != lv:
            err = err + wt
        i = i + 1
    return err


def gini_impurity(labels: list[int]) -> int:
    """Gini impurity * 1000 for integer arithmetic. Lower is purer."""
    n: int = len(labels)
    if n == 0:
        return 0
    c1: int = 0
    i: int = 0
    while i < n:
        lv: int = labels[i]
        if lv == 1:
            c1 = c1 + 1
        i = i + 1
    c0: int = n - c1
    return (2 * c0 * c1 * 1000) // (n * n)


def test_module() -> int:
    """Test decision stump classifier."""
    ok: int = 0
    feats: list[int] = [1, 2, 3, 4, 5, 6, 7, 8]
    labs: list[int] = [0, 0, 0, 0, 1, 1, 1, 1]
    thresh: int = find_best_threshold(feats, labs)
    if thresh == 5:
        ok = ok + 1
    preds: list[int] = stump_predict(feats, thresh)
    acc: int = stump_accuracy(preds, labs)
    if acc == 100:
        ok = ok + 1
    g_pure: int = gini_impurity([0, 0, 0, 0])
    if g_pure == 0:
        ok = ok + 1
    g_mixed: int = gini_impurity([0, 1, 0, 1])
    if g_mixed > 400:
        ok = ok + 1
    w: list[int] = [1, 1, 1, 1, 1, 1, 1, 1]
    we: int = weighted_error(feats, labs, w, 5)
    if we == 0:
        ok = ok + 1
    return ok
