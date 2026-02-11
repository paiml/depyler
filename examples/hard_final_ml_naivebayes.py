"""Naive Bayes classifier using integer feature counts.

Implements a simple categorical Naive Bayes with Laplace smoothing.
Features and labels are integers. Uses log-likelihood approximation via
integer arithmetic to avoid float precision issues.
"""


def count_label(labels: list[int], target: int) -> int:
    """Count occurrences of target in labels."""
    cnt: int = 0
    i: int = 0
    while i < len(labels):
        lab: int = labels[i]
        if lab == target:
            cnt = cnt + 1
        i = i + 1
    return cnt


def count_feature_given_label(features: list[int], labels: list[int], feat_val: int, label_val: int) -> int:
    """Count how many times feat_val appears with label_val."""
    cnt: int = 0
    i: int = 0
    while i < len(features):
        fv: int = features[i]
        lv: int = labels[i]
        if fv == feat_val:
            if lv == label_val:
                cnt = cnt + 1
        i = i + 1
    return cnt


def unique_labels(labels: list[int]) -> list[int]:
    """Get sorted unique labels."""
    seen: list[int] = []
    i: int = 0
    while i < len(labels):
        lab: int = labels[i]
        found: int = 0
        j: int = 0
        while j < len(seen):
            sv: int = seen[j]
            if sv == lab:
                found = 1
            j = j + 1
        if found == 0:
            seen.append(lab)
        i = i + 1
    k: int = 0
    while k < len(seen):
        m: int = k + 1
        while m < len(seen):
            vk: int = seen[k]
            vm: int = seen[m]
            if vk > vm:
                seen[k] = vm
                seen[m] = vk
            m = m + 1
        k = k + 1
    return seen


def nb_predict(features: list[int], labels: list[int], query: int, num_feat_vals: int) -> int:
    """Predict label for query feature using Naive Bayes with Laplace smoothing.

    Uses scaled integer scores: score = count_label * (count_feat|label + 1).
    """
    ulabels: list[int] = unique_labels(labels)
    n: int = len(labels)
    best_label: int = 0
    best_score: int = 0 - 1
    li: int = 0
    while li < len(ulabels):
        lab: int = ulabels[li]
        lab_count: int = count_label(labels, lab)
        feat_count: int = count_feature_given_label(features, labels, query, lab)
        score: int = lab_count * (feat_count + 1)
        if score > best_score:
            best_score = score
            best_label = lab
        li = li + 1
    return best_label


def nb_accuracy(features: list[int], labels: list[int], num_feat_vals: int) -> int:
    """Leave-one-out accuracy (percentage 0-100)."""
    correct: int = 0
    i: int = 0
    while i < len(features):
        train_f: list[int] = []
        train_l: list[int] = []
        j: int = 0
        while j < len(features):
            if j != i:
                train_f.append(features[j])
                train_l.append(labels[j])
            j = j + 1
        query: int = features[i]
        pred: int = nb_predict(train_f, train_l, query, num_feat_vals)
        actual: int = labels[i]
        if pred == actual:
            correct = correct + 1
        i = i + 1
    return correct * 100 // len(features)


def test_module() -> int:
    """Test naive Bayes classifier."""
    ok: int = 0
    feats: list[int] = [0, 0, 0, 1, 1, 1, 2, 2, 2]
    labs: list[int] = [0, 0, 1, 1, 1, 0, 0, 0, 1]
    pred0: int = nb_predict(feats, labs, 0, 3)
    if pred0 == 0:
        ok = ok + 1
    pred1: int = nb_predict(feats, labs, 1, 3)
    if pred1 == 1:
        ok = ok + 1
    ul: list[int] = unique_labels(labs)
    if len(ul) == 2:
        ok = ok + 1
    c0: int = count_label(labs, 0)
    c1: int = count_label(labs, 1)
    if c0 + c1 == 9:
        ok = ok + 1
    acc: int = nb_accuracy(feats, labs, 3)
    if acc >= 40:
        ok = ok + 1
    return ok
