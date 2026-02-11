"""ROC curve computation for binary classifiers.

Computes True Positive Rate and False Positive Rate at various thresholds.
Uses integer scores and scaled rates (multiplied by 1000).
"""


def count_positives(labels: list[int]) -> int:
    """Count positive labels (1s)."""
    cnt: int = 0
    i: int = 0
    while i < len(labels):
        lv: int = labels[i]
        if lv == 1:
            cnt = cnt + 1
        i = i + 1
    return cnt


def count_negatives(labels: list[int]) -> int:
    """Count negative labels (0s)."""
    cnt: int = 0
    i: int = 0
    while i < len(labels):
        lv: int = labels[i]
        if lv == 0:
            cnt = cnt + 1
        i = i + 1
    return cnt


def tpr_at_threshold(scores: list[int], labels: list[int], thresh: int) -> int:
    """True positive rate * 1000 at given threshold."""
    tp: int = 0
    pos: int = count_positives(labels)
    if pos == 0:
        return 0
    i: int = 0
    while i < len(scores):
        sv: int = scores[i]
        lv: int = labels[i]
        if sv >= thresh:
            if lv == 1:
                tp = tp + 1
        i = i + 1
    return tp * 1000 // pos


def fpr_at_threshold(scores: list[int], labels: list[int], thresh: int) -> int:
    """False positive rate * 1000 at given threshold."""
    fp: int = 0
    neg: int = count_negatives(labels)
    if neg == 0:
        return 0
    i: int = 0
    while i < len(scores):
        sv: int = scores[i]
        lv: int = labels[i]
        if sv >= thresh:
            if lv == 0:
                fp = fp + 1
        i = i + 1
    return fp * 1000 // neg


def roc_points(scores: list[int], labels: list[int], thresholds: list[int]) -> list[int]:
    """Compute ROC curve as flat list: [fpr0, tpr0, fpr1, tpr1, ...]."""
    result: list[int] = []
    i: int = 0
    while i < len(thresholds):
        tv: int = thresholds[i]
        fpr: int = fpr_at_threshold(scores, labels, tv)
        tpr: int = tpr_at_threshold(scores, labels, tv)
        result.append(fpr)
        result.append(tpr)
        i = i + 1
    return result


def auc_trapezoidal(roc_pts: list[int]) -> int:
    """Approximate AUC * 1000000 using trapezoidal rule on ROC points.

    Points should be ordered by increasing FPR.
    """
    n: int = len(roc_pts) // 2
    if n < 2:
        return 0
    total: int = 0
    i: int = 1
    while i < n:
        x0: int = roc_pts[(i - 1) * 2]
        y0: int = roc_pts[(i - 1) * 2 + 1]
        x1: int = roc_pts[i * 2]
        y1: int = roc_pts[i * 2 + 1]
        width: int = x1 - x0
        height: int = y0 + y1
        total = total + width * height
        i = i + 1
    return total // 2


def unique_sorted_thresholds(scores: list[int]) -> list[int]:
    """Get unique scores sorted descending for threshold sweep."""
    seen: list[int] = []
    i: int = 0
    while i < len(scores):
        sv: int = scores[i]
        found: int = 0
        j: int = 0
        while j < len(seen):
            existing: int = seen[j]
            if existing == sv:
                found = 1
            j = j + 1
        if found == 0:
            seen.append(sv)
        i = i + 1
    k: int = 0
    while k < len(seen):
        m: int = k + 1
        while m < len(seen):
            vk: int = seen[k]
            vm: int = seen[m]
            if vk < vm:
                seen[k] = vm
                seen[m] = vk
            m = m + 1
        k = k + 1
    return seen


def test_module() -> int:
    """Test ROC curve computation."""
    ok: int = 0
    scores: list[int] = [9, 8, 7, 6, 5, 4, 3, 2]
    labs: list[int] = [1, 1, 1, 1, 0, 0, 0, 0]
    pos: int = count_positives(labs)
    neg: int = count_negatives(labs)
    if pos == 4:
        ok = ok + 1
    if neg == 4:
        ok = ok + 1
    tpr5: int = tpr_at_threshold(scores, labs, 5)
    if tpr5 == 1000:
        ok = ok + 1
    fpr5: int = fpr_at_threshold(scores, labs, 5)
    if fpr5 == 250:
        ok = ok + 1
    threshs: list[int] = unique_sorted_thresholds(scores)
    if len(threshs) == 8:
        ok = ok + 1
    return ok
