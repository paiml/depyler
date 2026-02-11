# 1D k-means clustering


def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x


def assign_clusters(data: list[int], centroids: list[int]) -> list[int]:
    # Assign each data point to nearest centroid
    assignments: list[int] = []
    i: int = 0
    while i < len(data):
        best_cluster: int = 0
        best_dist: int = abs_val(data[i] - centroids[0])
        j: int = 1
        while j < len(centroids):
            dist: int = abs_val(data[i] - centroids[j])
            if dist < best_dist:
                best_dist = dist
                best_cluster = j
            j = j + 1
        assignments.append(best_cluster)
        i = i + 1
    return assignments


def update_centroids(data: list[int], assignments: list[int], k: int) -> list[int]:
    sums: list[int] = []
    counts: list[int] = []
    i: int = 0
    while i < k:
        sums.append(0)
        counts.append(0)
        i = i + 1
    j: int = 0
    while j < len(data):
        cluster: int = assignments[j]
        sums[cluster] = sums[cluster] + data[j]
        counts[cluster] = counts[cluster] + 1
        j = j + 1
    centroids: list[int] = []
    i = 0
    while i < k:
        if counts[i] > 0:
            centroids.append(sums[i] // counts[i])
        else:
            centroids.append(0)
        i = i + 1
    return centroids


def kmeans_1d(data: list[int], k: int, max_iter: int) -> list[int]:
    # Initialize centroids from first k data points
    centroids: list[int] = []
    i: int = 0
    while i < k:
        centroids.append(data[i])
        i = i + 1
    iteration: int = 0
    while iteration < max_iter:
        assignments: list[int] = assign_clusters(data, centroids)
        new_centroids: list[int] = update_centroids(data, assignments, k)
        changed: int = 0
        j: int = 0
        while j < k:
            if new_centroids[j] != centroids[j]:
                changed = 1
            j = j + 1
        centroids = new_centroids
        if changed == 0:
            break
        iteration = iteration + 1
    return centroids


def cluster_inertia(data: list[int], centroids: list[int]) -> int:
    # Sum of squared distances to nearest centroid
    assignments: list[int] = assign_clusters(data, centroids)
    total: int = 0
    i: int = 0
    while i < len(data):
        diff: int = data[i] - centroids[assignments[i]]
        total = total + diff * diff
        i = i + 1
    return total


def sort_list(arr: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    i = 1
    while i < len(result):
        key: int = result[i]
        j: int = i - 1
        while j >= 0 and result[j] > key:
            result[j + 1] = result[j]
            j = j - 1
        result[j + 1] = key
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0

    # Test 1: two clear clusters
    data: list[int] = [1, 2, 3, 100, 101, 102]
    centroids: list[int] = kmeans_1d(data, 2, 20)
    cs: list[int] = sort_list(centroids)
    if cs[0] == 2 and cs[1] == 101:
        passed = passed + 1

    # Test 2: assignments for clear clusters
    assignments: list[int] = assign_clusters(data, centroids)
    a0: int = assignments[0]
    a5: int = assignments[5]
    if a0 != a5:
        passed = passed + 1

    # Test 3: single cluster = mean
    data2: list[int] = [10, 20, 30]
    c1: list[int] = kmeans_1d(data2, 1, 20)
    if c1[0] == 20:
        passed = passed + 1

    # Test 4: inertia for perfect clusters = 0
    perfect: list[int] = [5, 5, 10, 10]
    cents: list[int] = [5, 10]
    if cluster_inertia(perfect, cents) == 0:
        passed = passed + 1

    # Test 5: inertia for imperfect
    data3: list[int] = [0, 1, 10, 11]
    cents2: list[int] = [0, 10]
    # 0->0: 0, 1->0: 1, 10->10: 0, 11->10: 1 = 2
    if cluster_inertia(data3, cents2) == 2:
        passed = passed + 1

    # Test 6: three clusters
    data4: list[int] = [1, 2, 50, 51, 100, 101]
    c3: list[int] = kmeans_1d(data4, 3, 20)
    sc3: list[int] = sort_list(c3)
    if sc3[0] < 10 and sc3[1] > 40 and sc3[1] < 60 and sc3[2] > 90:
        passed = passed + 1

    # Test 7: convergence (should not change after convergence)
    c_a: list[int] = kmeans_1d(data, 2, 5)
    c_b: list[int] = kmeans_1d(data, 2, 50)
    sa: list[int] = sort_list(c_a)
    sb: list[int] = sort_list(c_b)
    if sa[0] == sb[0] and sa[1] == sb[1]:
        passed = passed + 1

    return passed
