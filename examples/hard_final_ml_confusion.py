"""Confusion matrix computation for binary classifiers.

Computes TP, TN, FP, FN, precision, recall, F1 (all as scaled integers).
"""


def compute_tp(preds: list[int], actuals: list[int]) -> int:
    """True positives: predicted 1 and actual 1."""
    cnt: int = 0
    i: int = 0
    while i < len(preds):
        pv: int = preds[i]
        av: int = actuals[i]
        if pv == 1:
            if av == 1:
                cnt = cnt + 1
        i = i + 1
    return cnt


def compute_tn(preds: list[int], actuals: list[int]) -> int:
    """True negatives: predicted 0 and actual 0."""
    cnt: int = 0
    i: int = 0
    while i < len(preds):
        pv: int = preds[i]
        av: int = actuals[i]
        if pv == 0:
            if av == 0:
                cnt = cnt + 1
        i = i + 1
    return cnt


def compute_fp(preds: list[int], actuals: list[int]) -> int:
    """False positives: predicted 1 and actual 0."""
    cnt: int = 0
    i: int = 0
    while i < len(preds):
        pv: int = preds[i]
        av: int = actuals[i]
        if pv == 1:
            if av == 0:
                cnt = cnt + 1
        i = i + 1
    return cnt


def compute_fn(preds: list[int], actuals: list[int]) -> int:
    """False negatives: predicted 0 and actual 1."""
    cnt: int = 0
    i: int = 0
    while i < len(preds):
        pv: int = preds[i]
        av: int = actuals[i]
        if pv == 0:
            if av == 1:
                cnt = cnt + 1
        i = i + 1
    return cnt


def precision_scaled(preds: list[int], actuals: list[int]) -> int:
    """Precision * 1000. Returns 0 if no positive predictions."""
    tp: int = compute_tp(preds, actuals)
    fp: int = compute_fp(preds, actuals)
    denom: int = tp + fp
    if denom == 0:
        return 0
    return tp * 1000 // denom


def recall_scaled(preds: list[int], actuals: list[int]) -> int:
    """Recall * 1000. Returns 0 if no actual positives."""
    tp: int = compute_tp(preds, actuals)
    fn: int = compute_fn(preds, actuals)
    denom: int = tp + fn
    if denom == 0:
        return 0
    return tp * 1000 // denom


def f1_scaled(preds: list[int], actuals: list[int]) -> int:
    """F1 score * 1000. Harmonic mean of precision and recall."""
    prec: int = precision_scaled(preds, actuals)
    rec: int = recall_scaled(preds, actuals)
    if prec + rec == 0:
        return 0
    return 2 * prec * rec // (prec + rec)


def accuracy_scaled(preds: list[int], actuals: list[int]) -> int:
    """Accuracy * 1000."""
    tp: int = compute_tp(preds, actuals)
    tn: int = compute_tn(preds, actuals)
    n: int = len(preds)
    if n == 0:
        return 0
    return (tp + tn) * 1000 // n


def test_module() -> int:
    """Test confusion matrix metrics."""
    ok: int = 0
    preds: list[int] = [1, 1, 0, 0, 1, 0, 1, 1]
    actuals: list[int] = [1, 0, 0, 1, 1, 0, 1, 0]
    tp: int = compute_tp(preds, actuals)
    if tp == 3:
        ok = ok + 1
    tn: int = compute_tn(preds, actuals)
    if tn == 2:
        ok = ok + 1
    fp: int = compute_fp(preds, actuals)
    fn_val: int = compute_fn(preds, actuals)
    if fp + fn_val == 3:
        ok = ok + 1
    prec: int = precision_scaled(preds, actuals)
    if prec == 600:
        ok = ok + 1
    acc: int = accuracy_scaled(preds, actuals)
    if acc == 625:
        ok = ok + 1
    return ok
