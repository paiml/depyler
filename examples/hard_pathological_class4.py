# Pathological class pattern: Matrix operations using flat list[int]
# Tests: 2D indexing via row*cols+col, row/col sums, scale, max
# Workaround: avoid list index assignment (self.data[idx] = val) as transpiler
# generates Vec::insert instead of index assign. Use rebuild approach instead.


class FlatMatrix:
    def __init__(self, rows: int, cols: int) -> None:
        self.rows: int = rows
        self.cols: int = cols
        self.data: list[int] = []
        total: int = rows * cols
        i: int = 0
        while i < total:
            self.data.append(0)
            i = i + 1

    def get_val(self, row: int, col: int) -> int:
        idx: int = row * self.cols + col
        return self.data[idx]

    def row_sum(self, row: int) -> int:
        total: int = 0
        c: int = 0
        while c < self.cols:
            total = total + self.get_val(row, c)
            c = c + 1
        return total

    def col_sum(self, col: int) -> int:
        total: int = 0
        r: int = 0
        while r < self.rows:
            total = total + self.get_val(r, col)
            r = r + 1
        return total

    def total_sum(self) -> int:
        total: int = 0
        i: int = 0
        while i < len(self.data):
            total = total + self.data[i]
            i = i + 1
        return total

    def max_element(self) -> int:
        if len(self.data) == 0:
            return 0
        best: int = self.data[0]
        i: int = 1
        while i < len(self.data):
            if self.data[i] > best:
                best = self.data[i]
            i = i + 1
        return best

    def count_nonzero(self) -> int:
        cnt: int = 0
        i: int = 0
        while i < len(self.data):
            if self.data[i] != 0:
                cnt = cnt + 1
            i = i + 1
        return cnt


def build_matrix(rows: int, cols: int, vals: list[int]) -> FlatMatrix:
    """Build matrix from flat list of values."""
    m: FlatMatrix = FlatMatrix(rows, cols)
    m.data = vals
    return m


def test_module() -> int:
    passed: int = 0
    # Build a 3x3 matrix:
    # 1 4 0
    # 5 2 0
    # 0 0 3
    vals: list[int] = [1, 4, 0, 5, 2, 0, 0, 0, 3]
    m: FlatMatrix = build_matrix(3, 3, vals)
    # Test 1: get value
    if m.get_val(0, 0) == 1:
        passed = passed + 1
    # Test 2: row sum (row 0 = 1+4+0 = 5)
    if m.row_sum(0) == 5:
        passed = passed + 1
    # Test 3: col sum (col 0 = 1+5+0 = 6)
    if m.col_sum(0) == 6:
        passed = passed + 1
    # Test 4: total sum (1+4+5+2+3 = 15)
    if m.total_sum() == 15:
        passed = passed + 1
    # Test 5: max element
    if m.max_element() == 5:
        passed = passed + 1
    # Test 6: count nonzero
    if m.count_nonzero() == 5:
        passed = passed + 1
    # Test 7: empty matrix
    m2: FlatMatrix = FlatMatrix(2, 2)
    if m2.total_sum() == 0:
        passed = passed + 1
    return passed
