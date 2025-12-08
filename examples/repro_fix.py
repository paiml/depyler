"""Tests DEPYLER-0812: Module-level slice with negative step generates usize for -1."""

data: list[int] = [1, 2, 3, 4, 5]
reversed_data: list[int] = data[::-1]  # E0277: usize: Neg not satisfied
