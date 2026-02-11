"""Pathological operator overloading patterns for transpiler stress testing.

Tests arithmetic, comparison, container, and unary dunder methods
across Vector, Matrix, and Fraction classes.
"""

from typing import List, Tuple, Optional, Union
import math


class Vector:
    """2D/3D vector with full operator overloading including @matmul for dot product."""

    def __init__(self, *components: float):
        if len(components) < 2:
            raise ValueError("Vector requires at least 2 components")
        self._data: List[float] = list(components)

    def __repr__(self):
        parts = ", ".join(f"{c:.4g}" for c in self._data)
        return f"Vector({parts})"

    def __str__(self):
        parts = ", ".join(f"{c:.4g}" for c in self._data)
        return f"<{parts}>"

    def __len__(self):
        return len(self._data)

    def __getitem__(self, index):
        return self._data[index]

    def __setitem__(self, index: int, value: float):
        self._data[index] = value

    def __contains__(self, value):
        for c in self._data:
            if abs(c - value) < 1e-9:
                return True
        return False

    def __eq__(self, other):
        if not isinstance(other, Vector):
            return NotImplemented
        if len(self._data) != len(other._data):
            return False
        for a, b in zip(self._data, other._data):
            if abs(a - b) > 1e-9:
                return False
        return True

    def __add__(self, other):
        if len(self._data) != len(other._data):
            raise ValueError("Vector dimensions must match for addition")
        return Vector(*(a + b for a, b in zip(self._data, other._data)))

    def __sub__(self, other):
        if len(self._data) != len(other._data):
            raise ValueError("Vector dimensions must match for subtraction")
        return Vector(*(a - b for a, b in zip(self._data, other._data)))

    def __mul__(self, scalar):
        return Vector(*(c * scalar for c in self._data))


    def __truediv__(self, scalar):
        if abs(scalar) < 1e-15:
            raise ZeroDivisionError("Cannot divide vector by zero")
        return Vector(*(c / scalar for c in self._data))

    def __neg__(self):
        return Vector(*(-c for c in self._data))

    def __abs__(self):
        return math.sqrt(sum(c * c for c in self._data))

    def __matmul__(self, other):
        """Dot product via @ operator."""
        if len(self._data) != len(other._data):
            raise ValueError("Vector dimensions must match for dot product")
        total = 0.0
        for a, b in zip(self._data, other._data):
            total += a * b
        return total

    def __lt__(self, other):
        return abs(self) < abs(other)

    def __le__(self, other):
        return abs(self) <= abs(other)

    def __gt__(self, other):
        return abs(self) > abs(other)

    def __ge__(self, other):
        return abs(self) >= abs(other)

    def normalized(self) -> "Vector":
        magnitude = abs(self)
        if magnitude < 1e-15:
            raise ValueError("Cannot normalize zero vector")
        return self / magnitude

    def cross(self, other: "Vector") -> "Vector":
        """Cross product for 3D vectors only."""
        if len(self._data) != 3 or len(other._data) != 3:
            raise ValueError("Cross product requires 3D vectors")
        return Vector(
            self._data[1] * other._data[2] - self._data[2] * other._data[1],
            self._data[2] * other._data[0] - self._data[0] * other._data[2],
            self._data[0] * other._data[1] - self._data[1] * other._data[0],
        )


class Matrix:
    """2D matrix with operator overloading for arithmetic and element access."""

    def __init__(self, rows: int, cols: int, data: Optional[List[List[float]]] = None):
        self.rows = rows
        self.cols = cols
        if data is not None:
            if len(data) != rows:
                raise ValueError(f"Expected {rows} rows, got {len(data)}")
            self._data = [list(row) for row in data]
        else:
            self._data = [[0.0] * cols for _ in range(rows)]

    def __repr__(self) -> str:
        row_strs = []
        for row in self._data:
            row_strs.append("[" + ", ".join(f"{v:.4g}" for v in row) + "]")
        return f"Matrix({self.rows}x{self.cols}, [{', '.join(row_strs)}])"

    def __str__(self) -> str:
        lines = []
        for row in self._data:
            lines.append("  ".join(f"{v:8.4f}" for v in row))
        return "\n".join(lines)

    def __eq__(self, other):
        if not isinstance(other, Matrix):
            return NotImplemented
        if self.rows != other.rows or self.cols != other.cols:
            return False
        for i in range(self.rows):
            for j in range(self.cols):
                if abs(self._data[i][j] - other._data[i][j]) > 1e-9:
                    return False
        return True

    def __getitem__(self, key: Tuple[int, int]) -> float:
        r, c = key
        return self._data[r][c]

    def __setitem__(self, key: Tuple[int, int], value: float):
        r, c = key
        self._data[r][c] = value

    def __add__(self, other):
        if self.rows != other.rows or self.cols != other.cols:
            raise ValueError("Matrix dimensions must match for addition")
        result = Matrix(self.rows, self.cols)
        for i in range(self.rows):
            for j in range(self.cols):
                result._data[i][j] = self._data[i][j] + other._data[i][j]
        return result

    def __sub__(self, other):
        if self.rows != other.rows or self.cols != other.cols:
            raise ValueError("Matrix dimensions must match for subtraction")
        result = Matrix(self.rows, self.cols)
        for i in range(self.rows):
            for j in range(self.cols):
                result._data[i][j] = self._data[i][j] - other._data[i][j]
        return result

    def __mul__(self, scalar):
        result = Matrix(self.rows, self.cols)
        for i in range(self.rows):
            for j in range(self.cols):
                result._data[i][j] = self._data[i][j] * scalar
        return result

    def __matmul__(self, other):
        """Matrix multiplication via @ operator."""
        if self.cols != other.rows:
            raise ValueError(f"Cannot multiply {self.rows}x{self.cols} by {other.rows}x{other.cols}")
        result = Matrix(self.rows, other.cols)
        for i in range(self.rows):
            for j in range(other.cols):
                total = 0.0
                for k in range(self.cols):
                    total += self._data[i][k] * other._data[k][j]
                result._data[i][j] = total
        return result

    def __neg__(self):
        return self * (-1.0)

    def __contains__(self, value):
        for row in self._data:
            for v in row:
                if abs(v - value) < 1e-9:
                    return True
        return False

    def __len__(self) -> int:
        return self.rows * self.cols

    def transpose(self) -> "Matrix":
        result = Matrix(self.cols, self.rows)
        for i in range(self.rows):
            for j in range(self.cols):
                result._data[j][i] = self._data[i][j]
        return result

    def determinant(self) -> float:
        """Compute determinant for square matrices up to 3x3."""
        if self.rows != self.cols:
            raise ValueError("Determinant requires square matrix")
        if self.rows == 1:
            return self._data[0][0]
        if self.rows == 2:
            return self._data[0][0] * self._data[1][1] - self._data[0][1] * self._data[1][0]
        if self.rows == 3:
            a = self._data
            return (a[0][0] * (a[1][1]*a[2][2] - a[1][2]*a[2][1])
                  - a[0][1] * (a[1][0]*a[2][2] - a[1][2]*a[2][0])
                  + a[0][2] * (a[1][0]*a[2][1] - a[1][1]*a[2][0]))
        raise ValueError("Determinant only implemented up to 3x3")

    def trace_val(self) -> float:
        """Compute trace (sum of diagonal elements)."""
        if self.rows != self.cols:
            raise ValueError("Trace requires square matrix")
        total = 0.0
        for i in range(self.rows):
            total += self._data[i][i]
        return total


class Fraction:
    """Fraction with full arithmetic and comparison operators."""

    def __init__(self, numerator: int, denominator: int = 1):
        if denominator == 0:
            raise ZeroDivisionError("Fraction denominator cannot be zero")
        g = self._gcd(abs(numerator), abs(denominator))
        sign = -1 if (numerator < 0) != (denominator < 0) else 1
        self.num = sign * abs(numerator) // g
        self.den = abs(denominator) // g

    @staticmethod
    def _gcd(a: int, b: int) -> int:
        while b:
            a, b = b, a % b
        return a if a != 0 else 1

    def __repr__(self) -> str:
        if self.den == 1:
            return f"Fraction({self.num})"
        return f"Fraction({self.num}, {self.den})"

    def __str__(self) -> str:
        if self.den == 1:
            return str(self.num)
        return f"{self.num}/{self.den}"

    def __add__(self, other):
        return Fraction(self.num * other.den + other.num * self.den, self.den * other.den)

    def __sub__(self, other):
        return Fraction(self.num * other.den - other.num * self.den, self.den * other.den)

    def __mul__(self, other):
        return Fraction(self.num * other.num, self.den * other.den)

    def __truediv__(self, other):
        if other.num == 0:
            raise ZeroDivisionError("Cannot divide by zero fraction")
        return Fraction(self.num * other.den, self.den * other.num)

    def __mod__(self, other):
        """Modulo: self - (self // other) * other, where // is integer division of values."""
        if other.num == 0:
            raise ZeroDivisionError("Cannot mod by zero fraction")
        quotient = (self.num * other.den) // (other.num * self.den)
        return self - Fraction(quotient, 1) * other

    def __neg__(self):
        return Fraction(-self.num, self.den)

    def __abs__(self):
        return Fraction(abs(self.num), self.den)

    def __pow__(self, exp):
        if exp < 0:
            return Fraction(self.den ** (-exp), self.num ** (-exp))
        return Fraction(self.num ** exp, self.den ** exp)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Fraction):
            return NotImplemented
        return self.num == other.num and self.den == other.den

    def __lt__(self, other):
        return self.num * other.den < other.num * self.den

    def __le__(self, other):
        return self.num * other.den <= other.num * self.den

    def __gt__(self, other):
        return self.num * other.den > other.num * self.den

    def __ge__(self, other):
        return self.num * other.den >= other.num * self.den

    def to_float(self):
        return self.num / self.den


# --- Untyped helper functions ---

def vector_angle(v1, v2):
    """Compute angle between two vectors in radians - untyped."""
    dot = v1 @ v2
    mag1 = abs(v1)
    mag2 = abs(v2)
    if mag1 < 1e-15 or mag2 < 1e-15:
        return 0.0
    cos_theta = max(-1.0, min(1.0, dot / (mag1 * mag2)))
    return math.acos(cos_theta)


def matrix_from_vectors(vectors):
    """Build a matrix from a list of row vectors - untyped."""
    rows = len(vectors)
    cols = len(vectors[0])
    data = []
    for v in vectors:
        row = []
        for i in range(cols):
            row.append(float(v[i]))
        data.append(row)
    return Matrix(rows, cols, data)


def identity_matrix(n):
    """Create an n x n identity matrix - untyped."""
    m = Matrix(n, n)
    for i in range(n):
        m[(i, i)] = 1.0
    return m


def fraction_sum(fractions):
    """Sum a list of fractions - untyped."""
    result = Fraction(0)
    for f in fractions:
        result = result + f
    return result


def harmonic_number(n):
    """Compute the nth harmonic number as a Fraction - untyped."""
    result = Fraction(0)
    for i in range(1, n + 1):
        result = result + Fraction(1, i)
    return result


def vector_projection(v, onto):
    """Project vector v onto vector 'onto' - untyped."""
    dot = v @ onto
    mag_sq = onto @ onto
    if mag_sq < 1e-15:
        return onto * 0.0
    scalar = dot / mag_sq
    return onto * scalar


# --- Typed test functions ---

def test_vector_arithmetic() -> bool:
    """Test vector arithmetic operations."""
    v1 = Vector(1.0, 2.0, 3.0)
    v2 = Vector(4.0, 5.0, 6.0)

    v_add = v1 + v2
    assert v_add == Vector(5.0, 7.0, 9.0)

    v_sub = v2 - v1
    assert v_sub == Vector(3.0, 3.0, 3.0)

    v_scaled = v1 * 2.0
    assert v_scaled == Vector(2.0, 4.0, 6.0)

    v_div = v_scaled / 2.0
    assert v_div == v1

    v_neg = -v1
    assert v_neg == Vector(-1.0, -2.0, -3.0)

    mag = abs(v1)
    assert abs(mag - math.sqrt(14.0)) < 1e-9
    return True


def test_vector_dot_cross() -> bool:
    """Test dot product (@) and cross product."""
    v1 = Vector(1.0, 0.0, 0.0)
    v2 = Vector(0.0, 1.0, 0.0)

    dot = v1 @ v2
    assert abs(dot) < 1e-9  # Perpendicular

    cross = v1.cross(v2)
    assert cross == Vector(0.0, 0.0, 1.0)

    v3 = Vector(1.0, 2.0, 3.0)
    v4 = Vector(4.0, 5.0, 6.0)
    assert abs((v3 @ v4) - 32.0) < 1e-9
    return True


def test_vector_comparison():
    """Test vector comparison (by magnitude)."""
    short = Vector(1.0, 0.0)
    long = Vector(3.0, 4.0)

    assert short < long
    assert long > short
    assert short <= long
    assert long >= short
    assert short <= Vector(1.0, 0.0)
    return True


def test_vector_container():
    """Test __len__, __getitem__, __setitem__, __contains__."""
    v = Vector(1.0, 2.0, 3.0)
    assert len(v) == 3
    assert v[0] == 1.0
    assert v[2] == 3.0
    assert 2.0 in v
    assert 99.0 not in v

    v[1] = 10.0
    assert v[1] == 10.0
    return True


def test_matrix_arithmetic() -> bool:
    """Test matrix arithmetic operations."""
    m1 = Matrix(2, 2, [[1.0, 2.0], [3.0, 4.0]])
    m2 = Matrix(2, 2, [[5.0, 6.0], [7.0, 8.0]])

    m_add = m1 + m2
    assert m_add == Matrix(2, 2, [[6.0, 8.0], [10.0, 12.0]])

    m_sub = m2 - m1
    assert m_sub == Matrix(2, 2, [[4.0, 4.0], [4.0, 4.0]])

    m_scaled = m1 * 2.0
    assert m_scaled == Matrix(2, 2, [[2.0, 4.0], [6.0, 8.0]])

    m_neg = -m1
    assert m_neg == Matrix(2, 2, [[-1.0, -2.0], [-3.0, -4.0]])
    return True


def test_matrix_multiply() -> bool:
    """Test matrix multiplication via @ operator."""
    m1 = Matrix(2, 3, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]])
    m2 = Matrix(3, 2, [[7.0, 8.0], [9.0, 10.0], [11.0, 12.0]])

    result = m1 @ m2
    assert result.rows == 2 and result.cols == 2
    assert abs(result[(0, 0)] - 58.0) < 1e-9
    assert abs(result[(0, 1)] - 64.0) < 1e-9
    assert abs(result[(1, 0)] - 139.0) < 1e-9
    assert abs(result[(1, 1)] - 154.0) < 1e-9

    # Identity multiplication
    eye = identity_matrix(2)
    m3 = Matrix(2, 2, [[3.0, 7.0], [2.0, 5.0]])
    assert (eye @ m3) == m3
    return True


def test_matrix_properties():
    """Test determinant, trace, transpose, contains."""
    m = Matrix(3, 3, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]])

    det = m.determinant()
    assert abs(det - (-3.0)) < 1e-9

    tr = m.trace_val()
    assert abs(tr - 16.0) < 1e-9

    mt = m.transpose()
    assert mt[(0, 1)] == m[(1, 0)]
    assert mt[(2, 0)] == m[(0, 2)]

    assert 5.0 in m
    assert 99.0 not in m
    assert len(m) == 9
    return True


def test_fraction_arithmetic() -> bool:
    """Test fraction arithmetic operations."""
    f1 = Fraction(1, 3)
    f2 = Fraction(1, 6)

    f_add = f1 + f2
    assert f_add == Fraction(1, 2)

    f_sub = f1 - f2
    assert f_sub == Fraction(1, 6)

    f_mul = f1 * f2
    assert f_mul == Fraction(1, 18)

    f_div = f1 / f2
    assert f_div == Fraction(2, 1)

    f_neg = -f1
    assert f_neg == Fraction(-1, 3)

    f_abs = abs(Fraction(-3, 4))
    assert f_abs == Fraction(3, 4)

    f_pow = Fraction(2, 3) ** 3
    assert f_pow == Fraction(8, 27)

    f_neg_pow = Fraction(2, 3) ** -1
    assert f_neg_pow == Fraction(3, 2)
    return True


def test_fraction_comparison():
    """Test fraction comparison operators."""
    assert Fraction(1, 3) < Fraction(1, 2)
    assert Fraction(1, 2) > Fraction(1, 3)
    assert Fraction(1, 3) <= Fraction(1, 3)
    assert Fraction(1, 2) >= Fraction(1, 3)
    assert Fraction(2, 4) == Fraction(1, 2)
    assert str(Fraction(3, 6)) == "1/2"
    return True


def test_harmonic_number():
    """Test harmonic number computation via fractions."""
    h4 = harmonic_number(4)
    # H(4) = 1 + 1/2 + 1/3 + 1/4 = 25/12
    assert h4 == Fraction(25, 12)

    fsum = fraction_sum([Fraction(1, 2), Fraction(1, 3), Fraction(1, 6)])
    assert fsum == Fraction(1, 1)
    return True


def test_vector_projection():
    """Test vector projection."""
    v = Vector(3.0, 4.0)
    onto = Vector(1.0, 0.0)
    proj = vector_projection(v, onto)
    assert abs(proj[0] - 3.0) < 1e-9
    assert abs(proj[1]) < 1e-9
    return True


def test_all() -> bool:
    """Run all tests."""
    assert test_vector_arithmetic()
    assert test_vector_dot_cross()
    assert test_vector_comparison()
    assert test_vector_container()
    assert test_matrix_arithmetic()
    assert test_matrix_multiply()
    assert test_matrix_properties()
    assert test_fraction_arithmetic()
    assert test_fraction_comparison()
    assert test_harmonic_number()
    assert test_vector_projection()
    return True


def main():
    """Entry point."""
    if test_all():
        print("hard_operator_overloading: ALL TESTS PASSED")
    else:
        print("hard_operator_overloading: TESTS FAILED")


if __name__ == "__main__":
    main()
