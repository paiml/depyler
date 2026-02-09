"""Pathological matrix and linear algebra operations using nested lists.

Stress-tests Python-to-Rust transpilation with integer-only arithmetic,
triple-nested loops, cofactor expansion, and sparse matrix simulation.
All functions are pure, zero imports, fully type-annotated.
"""


# ---------- Matrix creation ----------


def mat_zeros(rows: int, cols: int) -> list[list[int]]:
    """Create a rows x cols zero matrix."""
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(0)
        result.append(row)
    return result


def mat_identity(n: int) -> list[list[int]]:
    """Create an n x n identity matrix."""
    result: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(n):
            if i == j:
                row.append(1)
            else:
                row.append(0)
        result.append(row)
    return result


def mat_from_flat(flat: list[int], rows: int, cols: int) -> list[list[int]]:
    """Create a matrix from a flat list in row-major order."""
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(flat[i * cols + j])
        result.append(row)
    return result


def mat_constant(rows: int, cols: int, val: int) -> list[list[int]]:
    """Create a matrix filled with a constant value."""
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(val)
        result.append(row)
    return result


def mat_diagonal(diag: list[int]) -> list[list[int]]:
    """Construct a diagonal matrix from a list of diagonal entries."""
    n: int = len(diag)
    result: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(n):
            if i == j:
                row.append(diag[i])
            else:
                row.append(0)
        result.append(row)
    return result


# ---------- Element-wise operations ----------


def mat_add(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Element-wise addition of two matrices."""
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(a[i][j] + b[i][j])
        result.append(row)
    return result


def mat_sub(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Element-wise subtraction of two matrices."""
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(a[i][j] - b[i][j])
        result.append(row)
    return result


def mat_scale(a: list[list[int]], scalar: int) -> list[list[int]]:
    """Multiply every element of a matrix by a scalar."""
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(a[i][j] * scalar)
        result.append(row)
    return result


def mat_hadamard(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Hadamard (element-wise) product of two matrices."""
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(a[i][j] * b[i][j])
        result.append(row)
    return result


def mat_negate(a: list[list[int]]) -> list[list[int]]:
    """Negate every element of a matrix."""
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(-a[i][j])
        result.append(row)
    return result


# ---------- Matrix multiplication ----------


def mat_mul(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Standard matrix multiplication via triple nested loop."""
    rows_a: int = len(a)
    cols_a: int = len(a[0])
    cols_b: int = len(b[0])
    result: list[list[int]] = []
    for i in range(rows_a):
        row: list[int] = []
        for j in range(cols_b):
            s: int = 0
            for k in range(cols_a):
                s = s + a[i][k] * b[k][j]
            row.append(s)
        result.append(row)
    return result


def mat_vec_mul(a: list[list[int]], v: list[int]) -> list[int]:
    """Multiply a matrix by a column vector."""
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[int] = []
    for i in range(rows):
        s: int = 0
        for j in range(cols):
            s = s + a[i][j] * v[j]
        result.append(s)
    return result


def mat_power(a: list[list[int]], n: int) -> list[list[int]]:
    """Raise a square matrix to the n-th power by repeated squaring."""
    size: int = len(a)
    result: list[list[int]] = mat_identity(size)
    base: list[list[int]] = []
    for i in range(size):
        row: list[int] = []
        for j in range(size):
            row.append(a[i][j])
        base.append(row)
    p: int = n
    while p > 0:
        if p % 2 == 1:
            result = mat_mul(result, base)
        base = mat_mul(base, base)
        p = p // 2
    return result


# ---------- Transpose and structural queries ----------


def mat_transpose(a: list[list[int]]) -> list[list[int]]:
    """Transpose a matrix."""
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[list[int]] = []
    for j in range(cols):
        row: list[int] = []
        for i in range(rows):
            row.append(a[i][j])
        result.append(row)
    return result


def mat_trace(a: list[list[int]]) -> int:
    """Compute the trace (sum of diagonal elements) of a square matrix."""
    n: int = len(a)
    s: int = 0
    for i in range(n):
        s = s + a[i][i]
    return s


def mat_extract_diag(a: list[list[int]]) -> list[int]:
    """Extract the main diagonal of a square matrix."""
    n: int = len(a)
    result: list[int] = []
    for i in range(n):
        result.append(a[i][i])
    return result


def mat_is_symmetric(a: list[list[int]]) -> int:
    """Check if a square matrix is symmetric. Returns 1 if symmetric, 0 otherwise."""
    n: int = len(a)
    for i in range(n):
        for j in range(i + 1, n):
            if a[i][j] != a[j][i]:
                return 0
    return 1


def mat_is_upper_triangular(a: list[list[int]]) -> int:
    """Check if a square matrix is upper triangular. Returns 1 or 0."""
    n: int = len(a)
    for i in range(1, n):
        for j in range(0, i):
            if a[i][j] != 0:
                return 0
    return 1


def mat_is_lower_triangular(a: list[list[int]]) -> int:
    """Check if a square matrix is lower triangular. Returns 1 or 0."""
    n: int = len(a)
    for i in range(0, n):
        for j in range(i + 1, n):
            if a[i][j] != 0:
                return 0
    return 1


def mat_extract_upper(a: list[list[int]]) -> list[list[int]]:
    """Extract the upper triangular part of a matrix (zero out below diagonal)."""
    n: int = len(a)
    result: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(n):
            if j >= i:
                row.append(a[i][j])
            else:
                row.append(0)
        result.append(row)
    return result


def mat_extract_lower(a: list[list[int]]) -> list[list[int]]:
    """Extract the lower triangular part of a matrix (zero out above diagonal)."""
    n: int = len(a)
    result: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(n):
            if j <= i:
                row.append(a[i][j])
            else:
                row.append(0)
        result.append(row)
    return result


def mat_is_banded(a: list[list[int]], lower_bw: int, upper_bw: int) -> int:
    """Check if matrix is banded with given lower and upper bandwidth. Returns 1 or 0."""
    n: int = len(a)
    for i in range(n):
        for j in range(n):
            if j < i - lower_bw or j > i + upper_bw:
                if a[i][j] != 0:
                    return 0
    return 1


# ---------- Vector operations ----------


def vec_dot(a: list[int], b: list[int]) -> int:
    """Dot product of two integer vectors."""
    n: int = len(a)
    s: int = 0
    for i in range(n):
        s = s + a[i] * b[i]
    return s


def vec_cross(a: list[int], b: list[int]) -> list[int]:
    """Cross product of two 3D integer vectors."""
    result: list[int] = []
    result.append(a[1] * b[2] - a[2] * b[1])
    result.append(a[2] * b[0] - a[0] * b[2])
    result.append(a[0] * b[1] - a[1] * b[0])
    return result


def vec_norm_squared(v: list[int]) -> int:
    """Squared Euclidean norm of a vector (avoids sqrt)."""
    s: int = 0
    for i in range(len(v)):
        s = s + v[i] * v[i]
    return s


def vec_add(a: list[int], b: list[int]) -> list[int]:
    """Add two vectors element-wise."""
    result: list[int] = []
    for i in range(len(a)):
        result.append(a[i] + b[i])
    return result


def vec_scale(v: list[int], s: int) -> list[int]:
    """Scale a vector by an integer scalar."""
    result: list[int] = []
    for i in range(len(v)):
        result.append(v[i] * s)
    return result


# ---------- Determinant and submatrices ----------


def mat_minor(a: list[list[int]], row: int, col: int) -> list[list[int]]:
    """Extract the minor matrix by removing specified row and column."""
    n: int = len(a)
    result: list[list[int]] = []
    for i in range(n):
        if i == row:
            continue
        r: list[int] = []
        for j in range(n):
            if j == col:
                continue
            r.append(a[i][j])
        result.append(r)
    return result


def mat_submatrix(a: list[list[int]], r1: int, r2: int, c1: int, c2: int) -> list[list[int]]:
    """Extract submatrix from rows [r1, r2) and cols [c1, c2)."""
    result: list[list[int]] = []
    for i in range(r1, r2):
        row: list[int] = []
        for j in range(c1, c2):
            row.append(a[i][j])
        result.append(row)
    return result


def det2(a: list[list[int]]) -> int:
    """Determinant of a 2x2 matrix."""
    return a[0][0] * a[1][1] - a[0][1] * a[1][0]


def det3(a: list[list[int]]) -> int:
    """Determinant of a 3x3 matrix via cofactor expansion along first row."""
    s: int = 0
    for j in range(3):
        minor: list[list[int]] = mat_minor(a, 0, j)
        cofactor: int = det2(minor)
        if j % 2 == 0:
            s = s + a[0][j] * cofactor
        else:
            s = s - a[0][j] * cofactor
    return s


def det_recursive(a: list[list[int]]) -> int:
    """Determinant of an n x n matrix via recursive cofactor expansion."""
    n: int = len(a)
    if n == 1:
        return a[0][0]
    if n == 2:
        return det2(a)
    s: int = 0
    for j in range(n):
        minor: list[list[int]] = mat_minor(a, 0, j)
        cofactor: int = det_recursive(minor)
        if j % 2 == 0:
            s = s + a[0][j] * cofactor
        else:
            s = s - a[0][j] * cofactor
    return s


# ---------- Norms ----------


def mat_norm_max(a: list[list[int]]) -> int:
    """Maximum absolute element in the matrix (infinity-like norm)."""
    rows: int = len(a)
    cols: int = len(a[0])
    m: int = 0
    for i in range(rows):
        for j in range(cols):
            v: int = a[i][j]
            if v < 0:
                v = -v
            if v > m:
                m = v
    return m


def mat_norm_sum(a: list[list[int]]) -> int:
    """Sum of absolute values of all elements (Manhattan-like norm)."""
    rows: int = len(a)
    cols: int = len(a[0])
    s: int = 0
    for i in range(rows):
        for j in range(cols):
            v: int = a[i][j]
            if v < 0:
                v = -v
            s = s + v
    return s


def mat_frobenius_sq(a: list[list[int]]) -> int:
    """Squared Frobenius norm (sum of squares of all elements)."""
    rows: int = len(a)
    cols: int = len(a[0])
    s: int = 0
    for i in range(rows):
        for j in range(cols):
            s = s + a[i][j] * a[i][j]
    return s


# ---------- Kronecker product ----------


def mat_kronecker(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Kronecker product of two matrices."""
    ra: int = len(a)
    ca: int = len(a[0])
    rb: int = len(b)
    cb: int = len(b[0])
    result: list[list[int]] = []
    for i in range(ra * rb):
        row: list[int] = []
        for j in range(ca * cb):
            ai: int = i // rb
            bi: int = i % rb
            aj: int = j // cb
            bj: int = j % cb
            row.append(a[ai][aj] * b[bi][bj])
        result.append(row)
    return result


# ---------- Row echelon form (integer-only) ----------


def mat_copy(a: list[list[int]]) -> list[list[int]]:
    """Create a deep copy of a matrix."""
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(a[i][j])
        result.append(row)
    return result


def mat_swap_rows(a: list[list[int]], r1: int, r2: int) -> list[list[int]]:
    """Swap two rows in a matrix, returning a new matrix."""
    result: list[list[int]] = mat_copy(a)
    cols: int = len(a[0])
    for j in range(cols):
        tmp: int = result[r1][j]
        result[r1][j] = result[r2][j]
        result[r2][j] = tmp
    return result


def mat_row_echelon_int(a: list[list[int]]) -> list[list[int]]:
    """Integer-only row echelon form using scaled elimination.

    Avoids division by multiplying rows by pivot values.
    May produce large integers but stays exact.
    """
    rows: int = len(a)
    cols: int = len(a[0])
    m: list[list[int]] = mat_copy(a)
    pivot_row: int = 0
    for col in range(cols):
        if pivot_row >= rows:
            break
        found: int = -1
        for i in range(pivot_row, rows):
            if m[i][col] != 0:
                found = i
                break
        if found == -1:
            continue
        if found != pivot_row:
            m = mat_swap_rows(m, pivot_row, found)
        pivot_val: int = m[pivot_row][col]
        for i in range(pivot_row + 1, rows):
            if m[i][col] != 0:
                factor: int = m[i][col]
                for j in range(cols):
                    m[i][j] = m[i][j] * pivot_val - factor * m[pivot_row][j]
        pivot_row = pivot_row + 1
    return m


# ---------- Sparse matrix simulation ----------


def sparse_create(entries: list[list[int]]) -> list[list[int]]:
    """Create a sparse matrix representation from list of [row, col, value] triples."""
    result: list[list[int]] = []
    for i in range(len(entries)):
        triple: list[int] = []
        triple.append(entries[i][0])
        triple.append(entries[i][1])
        triple.append(entries[i][2])
        result.append(triple)
    return result


def sparse_to_dense(entries: list[list[int]], rows: int, cols: int) -> list[list[int]]:
    """Convert sparse representation to dense matrix."""
    result: list[list[int]] = mat_zeros(rows, cols)
    for i in range(len(entries)):
        r: int = entries[i][0]
        c: int = entries[i][1]
        v: int = entries[i][2]
        result[r][c] = v
    return result


def sparse_mat_vec_mul(entries: list[list[int]], v: list[int], rows: int) -> list[int]:
    """Multiply a sparse matrix (list of triples) by a dense vector."""
    result: list[int] = []
    for i in range(rows):
        result.append(0)
    for i in range(len(entries)):
        r: int = entries[i][0]
        c: int = entries[i][1]
        val: int = entries[i][2]
        result[r] = result[r] + val * v[c]
    return result


def sparse_count_nonzero(entries: list[list[int]]) -> int:
    """Count the number of nonzero entries in a sparse matrix."""
    count: int = 0
    for i in range(len(entries)):
        if entries[i][2] != 0:
            count = count + 1
    return count


# ---------- Additional matrix utilities ----------


def mat_flatten(a: list[list[int]]) -> list[int]:
    """Flatten a matrix into a row-major list."""
    result: list[int] = []
    for i in range(len(a)):
        for j in range(len(a[i])):
            result.append(a[i][j])
    return result


def mat_equal(a: list[list[int]], b: list[list[int]]) -> int:
    """Check if two matrices are equal. Returns 1 or 0."""
    if len(a) != len(b):
        return 0
    for i in range(len(a)):
        if len(a[i]) != len(b[i]):
            return 0
        for j in range(len(a[i])):
            if a[i][j] != b[i][j]:
                return 0
    return 1


def mat_column_sum(a: list[list[int]], col: int) -> int:
    """Sum a specific column of a matrix."""
    s: int = 0
    for i in range(len(a)):
        s = s + a[i][col]
    return s


def mat_row_sum(a: list[list[int]], row: int) -> int:
    """Sum a specific row of a matrix."""
    s: int = 0
    for j in range(len(a[row])):
        s = s + a[row][j]
    return s


def mat_max_row_sum(a: list[list[int]]) -> int:
    """Compute the infinity norm: max row sum of absolute values."""
    rows: int = len(a)
    cols: int = len(a[0])
    best: int = 0
    for i in range(rows):
        s: int = 0
        for j in range(cols):
            v: int = a[i][j]
            if v < 0:
                v = -v
            s = s + v
        if s > best:
            best = s
    return best


def mat_max_col_sum(a: list[list[int]]) -> int:
    """Compute the one norm: max column sum of absolute values."""
    rows: int = len(a)
    cols: int = len(a[0])
    best: int = 0
    for j in range(cols):
        s: int = 0
        for i in range(rows):
            v: int = a[i][j]
            if v < 0:
                v = -v
            s = s + v
        if s > best:
            best = s
    return best


# ---------- Test functions ----------


def test_zeros_and_identity() -> int:
    """Test zero matrix and identity matrix creation."""
    z: list[list[int]] = mat_zeros(3, 3)
    eye: list[list[int]] = mat_identity(3)
    s: int = 0
    for i in range(3):
        for j in range(3):
            s = s + z[i][j]
    t: int = mat_trace(eye)
    if s == 0 and t == 3:
        return 1
    return 0


def test_add_sub() -> int:
    """Test matrix addition and subtraction."""
    a: list[list[int]] = mat_from_flat([1, 2, 3, 4], 2, 2)
    b: list[list[int]] = mat_from_flat([5, 6, 7, 8], 2, 2)
    c: list[list[int]] = mat_add(a, b)
    d: list[list[int]] = mat_sub(c, b)
    if mat_equal(d, a) == 1:
        return 1
    return 0


def test_mat_mul_identity() -> int:
    """Test that multiplying by identity returns the original."""
    a: list[list[int]] = mat_from_flat([1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3)
    eye: list[list[int]] = mat_identity(3)
    b: list[list[int]] = mat_mul(a, eye)
    if mat_equal(a, b) == 1:
        return 1
    return 0


def test_transpose_twice() -> int:
    """Test that transposing twice returns the original."""
    a: list[list[int]] = mat_from_flat([1, 2, 3, 4, 5, 6], 2, 3)
    b: list[list[int]] = mat_transpose(mat_transpose(a))
    if mat_equal(a, b) == 1:
        return 1
    return 0


def test_dot_product() -> int:
    """Test vector dot product."""
    a: list[int] = [1, 2, 3]
    b: list[int] = [4, 5, 6]
    d: int = vec_dot(a, b)
    if d == 32:
        return 1
    return 0


def test_cross_product() -> int:
    """Test 3D vector cross product."""
    a: list[int] = [1, 0, 0]
    b: list[int] = [0, 1, 0]
    c: list[int] = vec_cross(a, b)
    if c[0] == 0 and c[1] == 0 and c[2] == 1:
        return 1
    return 0


def test_determinant_2x2() -> int:
    """Test 2x2 determinant."""
    a: list[list[int]] = mat_from_flat([3, 8, 4, 6], 2, 2)
    d: int = det2(a)
    if d == -14:
        return 1
    return 0


def test_determinant_3x3() -> int:
    """Test 3x3 determinant via cofactor expansion."""
    a: list[list[int]] = mat_from_flat([6, 1, 1, 4, -2, 5, 2, 8, 7], 3, 3)
    d: int = det3(a)
    if d == -306:
        return 1
    return 0


def test_hadamard() -> int:
    """Test Hadamard (element-wise) product."""
    a: list[list[int]] = mat_from_flat([1, 2, 3, 4], 2, 2)
    b: list[list[int]] = mat_from_flat([5, 6, 7, 8], 2, 2)
    c: list[list[int]] = mat_hadamard(a, b)
    if c[0][0] == 5 and c[0][1] == 12 and c[1][0] == 21 and c[1][1] == 32:
        return 1
    return 0


def test_kronecker() -> int:
    """Test Kronecker product dimensions and corner value."""
    a: list[list[int]] = mat_identity(2)
    b: list[list[int]] = mat_from_flat([1, 2, 3, 4], 2, 2)
    k: list[list[int]] = mat_kronecker(a, b)
    if len(k) == 4 and len(k[0]) == 4 and k[0][0] == 1 and k[2][2] == 1:
        return 1
    return 0


def test_mat_power() -> int:
    """Test matrix power by repeated squaring."""
    a: list[list[int]] = mat_from_flat([1, 1, 1, 0], 2, 2)
    a3: list[list[int]] = mat_power(a, 3)
    if a3[0][0] == 3 and a3[0][1] == 2 and a3[1][0] == 2 and a3[1][1] == 1:
        return 1
    return 0


def test_symmetric() -> int:
    """Test symmetric matrix check."""
    a: list[list[int]] = mat_from_flat([1, 2, 3, 2, 5, 6, 3, 6, 9], 3, 3)
    b: list[list[int]] = mat_from_flat([1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3)
    if mat_is_symmetric(a) == 1 and mat_is_symmetric(b) == 0:
        return 1
    return 0


def test_triangular() -> int:
    """Test upper and lower triangular checks."""
    u: list[list[int]] = mat_from_flat([1, 2, 3, 0, 4, 5, 0, 0, 6], 3, 3)
    lo: list[list[int]] = mat_from_flat([1, 0, 0, 2, 3, 0, 4, 5, 6], 3, 3)
    if mat_is_upper_triangular(u) == 1 and mat_is_lower_triangular(lo) == 1:
        return 1
    return 0


def test_norms() -> int:
    """Test matrix norms."""
    a: list[list[int]] = mat_from_flat([1, -2, 3, -4], 2, 2)
    mx: int = mat_norm_max(a)
    sm: int = mat_norm_sum(a)
    fr: int = mat_frobenius_sq(a)
    if mx == 4 and sm == 10 and fr == 30:
        return 1
    return 0


def test_sparse_mul() -> int:
    """Test sparse matrix-vector multiplication."""
    entries: list[list[int]] = [[0, 0, 2], [0, 2, 3], [1, 1, 5]]
    v: list[int] = [1, 2, 3]
    result: list[int] = sparse_mat_vec_mul(entries, v, 2)
    if result[0] == 11 and result[1] == 10:
        return 1
    return 0


def test_row_echelon() -> int:
    """Test integer row echelon form produces zeros below pivot."""
    a: list[list[int]] = mat_from_flat([2, 1, 1, 4, 3, 3, 8, 7, 9], 3, 3)
    r: list[list[int]] = mat_row_echelon_int(a)
    if r[1][0] == 0 and r[2][0] == 0 and r[2][1] == 0:
        return 1
    return 0


def test_minor_extraction() -> int:
    """Test minor matrix extraction."""
    a: list[list[int]] = mat_from_flat([1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3)
    m: list[list[int]] = mat_minor(a, 1, 1)
    if m[0][0] == 1 and m[0][1] == 3 and m[1][0] == 7 and m[1][1] == 9:
        return 1
    return 0


def test_banded() -> int:
    """Test band matrix detection."""
    a: list[list[int]] = mat_from_flat([1, 2, 0, 3, 4, 5, 0, 6, 7], 3, 3)
    if mat_is_banded(a, 1, 1) == 1:
        return 1
    return 0


def test_mat_vec_mul() -> int:
    """Test matrix-vector multiplication."""
    a: list[list[int]] = mat_from_flat([1, 2, 3, 4], 2, 2)
    v: list[int] = [5, 6]
    r: list[int] = mat_vec_mul(a, v)
    if r[0] == 17 and r[1] == 39:
        return 1
    return 0


def test_diagonal() -> int:
    """Test diagonal matrix construction and extraction."""
    d: list[int] = [3, 5, 7]
    m: list[list[int]] = mat_diagonal(d)
    e: list[int] = mat_extract_diag(m)
    if e[0] == 3 and e[1] == 5 and e[2] == 7 and m[0][1] == 0:
        return 1
    return 0


def test_det_recursive() -> int:
    """Test recursive determinant for 4x4 matrix."""
    a: list[list[int]] = mat_from_flat(
        [1, 0, 2, -1, 3, 0, 0, 5, 2, 1, 4, -3, 1, 0, 5, 0], 4, 4
    )
    d: int = det_recursive(a)
    if d == 30:
        return 1
    return 0


def test_flatten_roundtrip() -> int:
    """Test flatten and from_flat roundtrip."""
    a: list[list[int]] = mat_from_flat([1, 2, 3, 4, 5, 6], 2, 3)
    flat: list[int] = mat_flatten(a)
    b: list[list[int]] = mat_from_flat(flat, 2, 3)
    if mat_equal(a, b) == 1:
        return 1
    return 0


def test_scale_and_negate() -> int:
    """Test scalar multiplication and negation."""
    a: list[list[int]] = mat_from_flat([1, 2, 3, 4], 2, 2)
    b: list[list[int]] = mat_scale(a, 3)
    c: list[list[int]] = mat_negate(a)
    if b[0][0] == 3 and b[1][1] == 12 and c[0][0] == -1 and c[1][1] == -4:
        return 1
    return 0


def test_infinity_one_norms() -> int:
    """Test infinity norm (max row sum) and one norm (max col sum)."""
    a: list[list[int]] = mat_from_flat([1, -2, 3, -4, 5, -6], 2, 3)
    inf_norm: int = mat_max_row_sum(a)
    one_norm: int = mat_max_col_sum(a)
    if inf_norm == 15 and one_norm == 9:
        return 1
    return 0


def test_sparse_to_dense() -> int:
    """Test sparse to dense conversion."""
    entries: list[list[int]] = [[0, 0, 5], [1, 2, 7], [2, 1, 3]]
    d: list[list[int]] = sparse_to_dense(entries, 3, 3)
    if d[0][0] == 5 and d[1][2] == 7 and d[2][1] == 3 and d[1][1] == 0:
        return 1
    return 0


def test_vec_operations() -> int:
    """Test vector add, scale, and norm squared."""
    a: list[int] = [1, 2, 3]
    b: list[int] = [4, 5, 6]
    c: list[int] = vec_add(a, b)
    d: list[int] = vec_scale(a, 2)
    n: int = vec_norm_squared(a)
    if c[0] == 5 and c[1] == 7 and d[2] == 6 and n == 14:
        return 1
    return 0


def test_submatrix() -> int:
    """Test submatrix extraction."""
    a: list[list[int]] = mat_from_flat(
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], 4, 4
    )
    s: list[list[int]] = mat_submatrix(a, 1, 3, 1, 3)
    if s[0][0] == 6 and s[0][1] == 7 and s[1][0] == 10 and s[1][1] == 11:
        return 1
    return 0


# ---------- Master test runner ----------


def run_all_tests() -> int:
    """Run all test functions and return the sum of passing tests."""
    total: int = 0
    total = total + test_zeros_and_identity()
    total = total + test_add_sub()
    total = total + test_mat_mul_identity()
    total = total + test_transpose_twice()
    total = total + test_dot_product()
    total = total + test_cross_product()
    total = total + test_determinant_2x2()
    total = total + test_determinant_3x3()
    total = total + test_hadamard()
    total = total + test_kronecker()
    total = total + test_mat_power()
    total = total + test_symmetric()
    total = total + test_triangular()
    total = total + test_norms()
    total = total + test_sparse_mul()
    total = total + test_row_echelon()
    total = total + test_minor_extraction()
    total = total + test_banded()
    total = total + test_mat_vec_mul()
    total = total + test_diagonal()
    total = total + test_det_recursive()
    total = total + test_flatten_roundtrip()
    total = total + test_scale_and_negate()
    total = total + test_infinity_one_norms()
    total = total + test_sparse_to_dense()
    total = total + test_vec_operations()
    total = total + test_submatrix()
    return total


if __name__ == "__main__":
    passed: int = run_all_tests()
    expected: int = 27
    print("Passed " + str(passed) + " / " + str(expected) + " tests")
    if passed != expected:
        print("FAIL: not all tests passed")
    else:
        print("ALL TESTS PASSED")
