"""Principal Component Analysis (PCA) via power iteration.

Implements PCA using the power iteration method to find the dominant eigenvector
of the covariance matrix. Data stored as flat list[float] with stride 2.
All index arithmetic uses pre-computed int variables.
"""


def compute_col_mean_2d(data: list[float], num_rows: int, which_col: int) -> float:
    """Compute mean of column which_col in data with stride 2."""
    total: float = 0.0
    i: int = 0
    while i < num_rows:
        idx: int = i * 2 + which_col
        v: float = data[idx]
        total = total + v
        i = i + 1
    if num_rows == 0:
        return 0.0
    return total / (num_rows * 1.0)


def center_data_2d(data: list[float], num_rows: int) -> list[float]:
    """Subtract column means from 2D data (stride 2)."""
    m0: float = compute_col_mean_2d(data, num_rows, 0)
    m1: float = compute_col_mean_2d(data, num_rows, 1)
    result: list[float] = []
    i: int = 0
    while i < num_rows:
        idx0: int = i * 2
        idx1: int = i * 2 + 1
        v0: float = data[idx0]
        v1: float = data[idx1]
        result.append(v0 - m0)
        result.append(v1 - m1)
        i = i + 1
    return result


def cov_2d(centered: list[float], num_rows: int, ci: int, cj: int) -> float:
    """Compute 2x2 covariance matrix entry."""
    total: float = 0.0
    i: int = 0
    while i < num_rows:
        idx_i: int = i * 2 + ci
        idx_j: int = i * 2 + cj
        vi: float = centered[idx_i]
        vj: float = centered[idx_j]
        total = total + vi * vj
        i = i + 1
    if num_rows <= 1:
        return 0.0
    return total / ((num_rows - 1) * 1.0)


def vec_norm_2(a: float, b: float) -> float:
    """Compute L2 norm of 2D vector."""
    sq: float = a * a + b * b
    return sqrt_approx(sq)


def sqrt_approx(x: float) -> float:
    """Newton's method square root."""
    if x < 0.0001:
        return 0.0
    guess: float = x / 2.0
    step: int = 0
    while step < 20:
        guess = (guess + x / guess) / 2.0
        step = step + 1
    return guess


def power_iter_2d(centered: list[float], num_rows: int, iters: int) -> list[float]:
    """Find dominant eigenvector of 2x2 covariance via power iteration."""
    v0: float = 1.0
    v1: float = 1.0
    c00: float = cov_2d(centered, num_rows, 0, 0)
    c01: float = cov_2d(centered, num_rows, 0, 1)
    c10: float = cov_2d(centered, num_rows, 1, 0)
    c11: float = cov_2d(centered, num_rows, 1, 1)
    step: int = 0
    while step < iters:
        nv0: float = c00 * v0 + c01 * v1
        nv1: float = c10 * v0 + c11 * v1
        nrm: float = vec_norm_2(nv0, nv1)
        if nrm < 0.0001:
            return [nv0, nv1]
        v0 = nv0 / nrm
        v1 = nv1 / nrm
        step = step + 1
    return [v0, v1]


def project_onto_vec(centered: list[float], num_rows: int, ev0: float, ev1: float) -> list[float]:
    """Project data onto a 2D eigenvector."""
    projections: list[float] = []
    i: int = 0
    while i < num_rows:
        idx0: int = i * 2
        idx1: int = i * 2 + 1
        d0: float = centered[idx0]
        d1: float = centered[idx1]
        dot: float = d0 * ev0 + d1 * ev1
        projections.append(dot)
        i = i + 1
    return projections


def approx_eq(a: float, b: float) -> int:
    """Check approximate equality."""
    diff: float = a - b
    if diff < 0.0:
        diff = 0.0 - diff
    if diff < 0.1:
        return 1
    return 0


def test_module() -> int:
    """Test PCA implementation."""
    ok: int = 0
    data: list[float] = [1.0, 2.0, 2.0, 4.0, 3.0, 6.0, 4.0, 8.0]
    centered: list[float] = center_data_2d(data, 4)
    if len(centered) == 8:
        ok = ok + 1
    mc0: float = compute_col_mean_2d(centered, 4, 0)
    if approx_eq(mc0, 0.0) == 1:
        ok = ok + 1
    eigv: list[float] = power_iter_2d(centered, 4, 20)
    if len(eigv) == 2:
        ok = ok + 1
    e0: float = eigv[0]
    e1: float = eigv[1]
    nrm: float = vec_norm_2(e0, e1)
    if approx_eq(nrm, 1.0) == 1:
        ok = ok + 1
    proj: list[float] = project_onto_vec(centered, 4, e0, e1)
    if len(proj) == 4:
        ok = ok + 1
    return ok
