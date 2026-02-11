"""K-fold cross validation framework with integer scoring.

Splits data into k folds, trains on k-1, tests on 1, averages accuracy.
Uses a simple threshold classifier for demonstration.
"""


def split_folds(n: int, k_folds: int) -> list[int]:
    """Assign each index to a fold number (0..k_folds-1)."""
    assignments: list[int] = []
    i: int = 0
    while i < n:
        assignments.append(i % k_folds)
        i = i + 1
    return assignments


def threshold_train(features: list[int], labels: list[int]) -> int:
    """Simple threshold classifier: find threshold that minimizes error."""
    best_t: int = 0
    best_err: int = len(features) + 1
    t: int = 0
    while t <= 10:
        err: int = 0
        i: int = 0
        while i < len(features):
            fv: int = features[i]
            lv: int = labels[i]
            pred: int = 0
            if fv >= t:
                pred = 1
            if pred != lv:
                err = err + 1
            i = i + 1
        if err < best_err:
            best_err = err
            best_t = t
        t = t + 1
    return best_t


def threshold_test(features: list[int], labels: list[int], thresh: int) -> int:
    """Test threshold classifier accuracy as percentage 0-100."""
    correct: int = 0
    i: int = 0
    while i < len(features):
        fv: int = features[i]
        lv: int = labels[i]
        pred: int = 0
        if fv >= thresh:
            pred = 1
        if pred == lv:
            correct = correct + 1
        i = i + 1
    if len(features) == 0:
        return 0
    return correct * 100 // len(features)


def cross_validate(features: list[int], labels: list[int], k_folds: int) -> int:
    """K-fold cross validation. Returns average accuracy 0-100."""
    folds: list[int] = split_folds(len(features), k_folds)
    total_acc: int = 0
    fold: int = 0
    while fold < k_folds:
        train_f: list[int] = []
        train_l: list[int] = []
        test_f: list[int] = []
        test_l: list[int] = []
        i: int = 0
        while i < len(features):
            fv: int = features[i]
            lv: int = labels[i]
            fa: int = folds[i]
            if fa == fold:
                test_f.append(fv)
                test_l.append(lv)
            else:
                train_f.append(fv)
                train_l.append(lv)
            i = i + 1
        if len(test_f) > 0:
            thresh: int = threshold_train(train_f, train_l)
            acc: int = threshold_test(test_f, test_l, thresh)
            total_acc = total_acc + acc
        fold = fold + 1
    return total_acc // k_folds


def stratified_count(labels: list[int], fold_assigns: list[int], fold_id: int, label_val: int) -> int:
    """Count items with given label in given fold."""
    cnt: int = 0
    i: int = 0
    while i < len(labels):
        lv: int = labels[i]
        fa: int = fold_assigns[i]
        if fa == fold_id:
            if lv == label_val:
                cnt = cnt + 1
        i = i + 1
    return cnt


def test_module() -> int:
    """Test cross validation."""
    ok: int = 0
    feats: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    labs: list[int] = [0, 0, 0, 0, 0, 1, 1, 1, 1, 1]
    folds: list[int] = split_folds(10, 5)
    if len(folds) == 10:
        ok = ok + 1
    f0: int = folds[0]
    f5: int = folds[5]
    if f0 == f5:
        ok = ok + 1
    thresh: int = threshold_train(feats, labs)
    if thresh == 6:
        ok = ok + 1
    acc: int = threshold_test(feats, labs, 6)
    if acc == 100:
        ok = ok + 1
    cv_acc: int = cross_validate(feats, labs, 5)
    if cv_acc >= 50:
        ok = ok + 1
    return ok
