"""Pathological nested loop and 2D index access patterns for transpiler testing.

Tests: triple-nested loops, 2D array writes with computed indices,
dynamic programming tables, prefix sums, Floyd-Warshall, Pascal's triangle,
spiral traversal, Game of Life, transpose, diagonal sums, row sorting,
histogram building, and per-row Kadane's algorithm.

All functions use list[list[int]] or list[int] types with full annotations.
"""


def matrix_multiply(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    """Triple-nested matrix multiplication: result[i][j] += a[i][k] * b[k][j]."""
    rows_a: int = len(a)
    cols_a: int = len(a[0])
    cols_b: int = len(b[0])
    result: list[list[int]] = []
    for i in range(rows_a):
        row: list[int] = []
        for j in range(cols_b):
            total: int = 0
            for k in range(cols_a):
                total += a[i][k] * b[k][j]
            row.append(total)
        result.append(row)
    return result


def test_matrix_multiply() -> int:
    """Test matrix multiply with known 2x2 result."""
    a: list[list[int]] = [[1, 2], [3, 4]]
    b: list[list[int]] = [[5, 6], [7, 8]]
    r: list[list[int]] = matrix_multiply(a, b)
    # [[19,22],[43,50]] -> 19+22+43+50 = 134
    return r[0][0] + r[0][1] + r[1][0] + r[1][1]


def min_path_sum(grid: list[list[int]]) -> int:
    """DP table with computed indices: dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j]."""
    rows: int = len(grid)
    cols: int = len(grid[0])
    dp: list[list[int]] = []
    for i in range(rows):
        dp_row: list[int] = []
        for j in range(cols):
            dp_row.append(0)
        dp.append(dp_row)
    dp[0][0] = grid[0][0]
    for i in range(1, rows):
        dp[i][0] = dp[i - 1][0] + grid[i][0]
    for j in range(1, cols):
        dp[0][j] = dp[0][j - 1] + grid[0][j]
    for i in range(1, rows):
        for j in range(1, cols):
            top: int = dp[i - 1][j]
            left: int = dp[i][j - 1]
            if top < left:
                dp[i][j] = top + grid[i][j]
            else:
                dp[i][j] = left + grid[i][j]
    return dp[rows - 1][cols - 1]


def test_min_path_sum() -> int:
    """Test min path sum on 3x3 grid."""
    grid: list[list[int]] = [[1, 3, 1], [1, 5, 1], [4, 2, 1]]
    # min path: 1->3->1->1->1 = 7
    return min_path_sum(grid)


def prefix_sum_2d(grid: list[list[int]]) -> list[list[int]]:
    """2D prefix sum: prefix[i][j] = prefix[i-1][j] + prefix[i][j-1] - prefix[i-1][j-1] + grid[i][j]."""
    rows: int = len(grid)
    cols: int = len(grid[0])
    prefix: list[list[int]] = []
    for i in range(rows):
        prow: list[int] = []
        for j in range(cols):
            prow.append(0)
        prefix.append(prow)
    prefix[0][0] = grid[0][0]
    for i in range(1, rows):
        prefix[i][0] = prefix[i - 1][0] + grid[i][0]
    for j in range(1, cols):
        prefix[0][j] = prefix[0][j - 1] + grid[0][j]
    for i in range(1, rows):
        for j in range(1, cols):
            prefix[i][j] = prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1] + grid[i][j]
    return prefix


def test_prefix_sum_2d() -> int:
    """Test 2D prefix sum on 3x3 grid."""
    grid: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    p: list[list[int]] = prefix_sum_2d(grid)
    # prefix[2][2] should be sum of all = 45
    return p[2][2]


def floyd_warshall(dist: list[list[int]], n: int) -> list[list[int]]:
    """Floyd-Warshall shortest paths: dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])."""
    result: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(n):
            row.append(dist[i][j])
        result.append(row)
    for k in range(n):
        for i in range(n):
            for j in range(n):
                through_k: int = result[i][k] + result[k][j]
                if through_k < result[i][j]:
                    result[i][j] = through_k
    return result


def test_floyd_warshall() -> int:
    """Test Floyd-Warshall on 4-node graph."""
    big: int = 9999
    dist: list[list[int]] = [
        [0, 3, big, 7],
        [8, 0, 2, big],
        [5, big, 0, 1],
        [2, big, big, 0],
    ]
    r: list[list[int]] = floyd_warshall(dist, 4)
    # r[1][3] = min path from 1->3 = 1->2->3 = 2+1 = 3
    # r[0][2] = min path from 0->2 = 0->1->2 = 3+2 = 5
    # r[3][1] = min path from 3->1 = 3->0->1 = 2+3 = 5
    return r[1][3] + r[0][2] + r[3][1]


def pascals_triangle(n: int) -> list[list[int]]:
    """Build Pascal's triangle: triangle[i][j] = triangle[i-1][j-1] + triangle[i-1][j]."""
    triangle: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(i + 1):
            if j == 0 or j == i:
                row.append(1)
            else:
                row.append(triangle[i - 1][j - 1] + triangle[i - 1][j])
        triangle.append(row)
    return triangle


def test_pascals_triangle() -> int:
    """Test Pascal's triangle row 6."""
    tri: list[list[int]] = pascals_triangle(7)
    # Row 6: [1,6,15,20,15,6,1] sum=64
    total: int = 0
    for val in tri[6]:
        total += val
    return total


def spiral_order(matrix: list[list[int]]) -> list[int]:
    """Collect matrix elements in spiral order (right, down, left, up)."""
    if len(matrix) == 0:
        return []
    result: list[int] = []
    top: int = 0
    bottom: int = len(matrix) - 1
    left: int = 0
    right: int = len(matrix[0]) - 1
    while top <= bottom and left <= right:
        j: int = left
        while j <= right:
            result.append(matrix[top][j])
            j += 1
        top += 1
        i: int = top
        while i <= bottom:
            result.append(matrix[i][right])
            i += 1
        right -= 1
        if top <= bottom:
            j2: int = right
            while j2 >= left:
                result.append(matrix[bottom][j2])
                j2 -= 1
            bottom -= 1
        if left <= right:
            i2: int = bottom
            while i2 >= top:
                result.append(matrix[i2][left])
                i2 -= 1
            left += 1
    return result


def test_spiral_order() -> int:
    """Test spiral on 3x3 matrix."""
    m: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    s: list[int] = spiral_order(m)
    # [1,2,3,6,9,8,7,4,5] sum = 45
    total: int = 0
    for v in s:
        total += v
    return total


def game_of_life_step(grid: list[list[int]]) -> list[list[int]]:
    """One step of Conway's Game of Life on a 2D grid of 0s and 1s."""
    rows: int = len(grid)
    cols: int = len(grid[0])
    new_grid: list[list[int]] = []
    for i in range(rows):
        new_row: list[int] = []
        for j in range(cols):
            neighbors: int = 0
            di: int = -1
            while di <= 1:
                dj: int = -1
                while dj <= 1:
                    if di != 0 or dj != 0:
                        ni: int = i + di
                        nj: int = j + dj
                        if ni >= 0 and ni < rows and nj >= 0 and nj < cols:
                            neighbors += grid[ni][nj]
                    dj += 1
                di += 1
            cell: int = grid[i][j]
            if cell == 1 and (neighbors == 2 or neighbors == 3):
                new_row.append(1)
            elif cell == 0 and neighbors == 3:
                new_row.append(1)
            else:
                new_row.append(0)
        new_grid.append(new_row)
    return new_grid


def test_game_of_life_step() -> int:
    """Test Game of Life blinker oscillator."""
    # Blinker: vertical -> horizontal -> vertical
    grid: list[list[int]] = [
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
    ]
    after: list[list[int]] = game_of_life_step(grid)
    # After one step: horizontal blinker at row 2
    # after[2][1]=1, after[2][2]=1, after[2][3]=1, rest 0
    # Sum of all cells should be 3
    total: int = 0
    for row in after:
        for val in row:
            total += val
    return total


def transpose(matrix: list[list[int]]) -> list[list[int]]:
    """Transpose matrix: result[j][i] = matrix[i][j]."""
    rows: int = len(matrix)
    cols: int = len(matrix[0])
    result: list[list[int]] = []
    for j in range(cols):
        new_row: list[int] = []
        for i in range(rows):
            new_row.append(matrix[i][j])
        result.append(new_row)
    return result


def test_transpose() -> int:
    """Test transpose of 2x3 matrix."""
    m: list[list[int]] = [[1, 2, 3], [4, 5, 6]]
    t: list[list[int]] = transpose(m)
    # t = [[1,4],[2,5],[3,6]], t[0][1]=4, t[1][0]=2, t[2][1]=6
    return t[0][1] + t[1][0] + t[2][1]


def diagonal_sums(matrix: list[list[int]]) -> int:
    """Sum main diagonal and anti-diagonal: matrix[i][i] + matrix[i][n-1-i]."""
    n: int = len(matrix)
    total: int = 0
    for i in range(n):
        total += matrix[i][i]
        anti_j: int = n - 1 - i
        if anti_j != i:
            total += matrix[i][anti_j]
    return total


def test_diagonal_sums() -> int:
    """Test diagonal sums on 4x4 matrix."""
    m: list[list[int]] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ]
    # Main diag: 1+6+11+16 = 34
    # Anti diag: 4+7+10+13 = 34
    # But center overlap removed for odd n; n=4 no overlap
    # Total = 34 + 34 = 68
    return diagonal_sums(m)


def row_sort_2d(matrix: list[list[int]]) -> list[list[int]]:
    """Sort each row of 2D matrix using bubble sort: row[j], row[j+1] swap."""
    rows: int = len(matrix)
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for val in matrix[i]:
            row.append(val)
        n: int = len(row)
        for p in range(n):
            for q in range(0, n - p - 1):
                if row[q] > row[q + 1]:
                    temp: int = row[q]
                    row[q] = row[q + 1]
                    row[q + 1] = temp
        result.append(row)
    return result


def test_row_sort_2d() -> int:
    """Test row sorting of 3x4 matrix."""
    m: list[list[int]] = [[4, 2, 7, 1], [9, 3, 5, 8], [6, 1, 3, 2]]
    r: list[list[int]] = row_sort_2d(m)
    # row0 sorted: [1,2,4,7], row1: [3,5,8,9], row2: [1,2,3,6]
    # r[0][0] + r[1][3] + r[2][2] = 1 + 9 + 3 = 13
    return r[0][0] + r[1][3] + r[2][2]


def histogram_2d(grid: list[list[int]]) -> list[int]:
    """Build histogram of values in 2D grid, returning counts for values 0..9."""
    counts: list[int] = []
    for i in range(10):
        counts.append(0)
    for row in grid:
        for val in row:
            if val >= 0 and val <= 9:
                counts[val] += 1
    return counts


def test_histogram_2d() -> int:
    """Test histogram building from 3x3 grid."""
    grid: list[list[int]] = [[1, 2, 3], [2, 3, 3], [1, 1, 5]]
    h: list[int] = histogram_2d(grid)
    # counts: 0->0, 1->3, 2->2, 3->3, 4->0, 5->1
    # h[1] + h[2] + h[3] + h[5] = 3+2+3+1 = 9
    return h[1] + h[2] + h[3] + h[5]


def max_subarray_per_row(matrix: list[list[int]]) -> list[int]:
    """Apply Kadane's algorithm to each row, returning max subarray sum per row."""
    result: list[int] = []
    for row in matrix:
        if len(row) == 0:
            result.append(0)
        else:
            max_sum: int = row[0]
            current: int = row[0]
            for idx in range(1, len(row)):
                candidate: int = current + row[idx]
                if candidate > row[idx]:
                    current = candidate
                else:
                    current = row[idx]
                if current > max_sum:
                    max_sum = current
            result.append(max_sum)
    return result


def test_max_subarray_per_row() -> int:
    """Test Kadane's per row on 3-row matrix."""
    m: list[list[int]] = [
        [-2, 1, -3, 4, -1, 2, 1, -5, 4],
        [1, 2, 3, 4, 5],
        [-1, -2, -3],
    ]
    r: list[int] = max_subarray_per_row(m)
    # row0: max subarray = [4,-1,2,1] = 6
    # row1: max subarray = [1,2,3,4,5] = 15
    # row2: max subarray = [-1]
    # 6 + 15 + (-1) = 20
    return r[0] + r[1] + r[2]


def matrix_rotate_90(matrix: list[list[int]]) -> list[list[int]]:
    """Rotate NxN matrix 90 degrees clockwise: result[j][n-1-i] = matrix[i][j]."""
    n: int = len(matrix)
    result: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(n):
            row.append(0)
        result.append(row)
    for i in range(n):
        for j in range(n):
            result[j][n - 1 - i] = matrix[i][j]
    return result


def test_matrix_rotate_90() -> int:
    """Test 90-degree rotation of 3x3 matrix."""
    m: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    r: list[list[int]] = matrix_rotate_90(m)
    # Rotated: [[7,4,1],[8,5,2],[9,6,3]]
    # r[0][0] + r[1][1] + r[2][2] = 7+5+3 = 15
    return r[0][0] + r[1][1] + r[2][2]


def wave_collapse_count(grid: list[list[int]], threshold: int) -> int:
    """Count cells where sum of 4-directional neighbors exceeds threshold."""
    rows: int = len(grid)
    cols: int = len(grid[0])
    count: int = 0
    for i in range(rows):
        for j in range(cols):
            neighbor_sum: int = 0
            if i > 0:
                neighbor_sum += grid[i - 1][j]
            if i < rows - 1:
                neighbor_sum += grid[i + 1][j]
            if j > 0:
                neighbor_sum += grid[i][j - 1]
            if j < cols - 1:
                neighbor_sum += grid[i][j + 1]
            if neighbor_sum > threshold:
                count += 1
    return count


def test_wave_collapse_count() -> int:
    """Test neighbor sum counting on 3x3 grid."""
    grid: list[list[int]] = [[5, 5, 5], [5, 1, 5], [5, 5, 5]]
    # Center (1,1): neighbors = 5+5+5+5=20, threshold=15 -> yes
    # Corner (0,0): neighbors = 5+5=10, threshold=15 -> no
    # Edge (0,1): neighbors = 5+1+5=11, threshold=15 -> no
    # With threshold=8:
    # corners have 10 > 8 (4 corners), edges have 11 > 8 (4 edges), center has 20 > 8
    # All 9 cells exceed threshold 8
    return wave_collapse_count(grid, 8)


def edit_distance_table(s1: list[int], s2: list[int]) -> int:
    """Edit distance via full DP table: dp[i][j] = min(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1]+cost)."""
    m: int = len(s1)
    n: int = len(s2)
    dp: list[list[int]] = []
    for i in range(m + 1):
        row: list[int] = []
        for j in range(n + 1):
            row.append(0)
        dp.append(row)
    for i in range(m + 1):
        dp[i][0] = i
    for j in range(n + 1):
        dp[0][j] = j
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                cost: int = 0
            else:
                cost = 1
            del_cost: int = dp[i - 1][j] + 1
            ins_cost: int = dp[i][j - 1] + 1
            sub_cost: int = dp[i - 1][j - 1] + cost
            best: int = del_cost
            if ins_cost < best:
                best = ins_cost
            if sub_cost < best:
                best = sub_cost
            dp[i][j] = best
    return dp[m][n]


def test_edit_distance_table() -> int:
    """Test edit distance between two integer sequences."""
    s1: list[int] = [1, 2, 3, 4, 5]
    s2: list[int] = [1, 3, 4, 5, 6]
    # Operations: delete 2, insert 6 -> distance = 2
    return edit_distance_table(s1, s2)
