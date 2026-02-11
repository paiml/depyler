"""Join algorithms: nested loop, hash join, and sort-merge join.

Tables represented as flat arrays with stride = num_columns.
Join on equality of specified columns.
"""


def nested_loop_join(table_a: list[int], cols_a: int, table_b: list[int], cols_b: int, join_col_a: int, join_col_b: int) -> list[int]:
    """Nested loop join. Returns flat result [a_cols..., b_cols..., ...]."""
    result: list[int] = []
    rows_a: int = len(table_a) // cols_a
    rows_b: int = len(table_b) // cols_b
    i: int = 0
    while i < rows_a:
        va: int = table_a[i * cols_a + join_col_a]
        j: int = 0
        while j < rows_b:
            vb: int = table_b[j * cols_b + join_col_b]
            if va == vb:
                ka: int = 0
                while ka < cols_a:
                    rv: int = table_a[i * cols_a + ka]
                    result.append(rv)
                    ka = ka + 1
                kb: int = 0
                while kb < cols_b:
                    rv2: int = table_b[j * cols_b + kb]
                    result.append(rv2)
                    kb = kb + 1
            j = j + 1
        i = i + 1
    return result


def hash_join(table_a: list[int], cols_a: int, table_b: list[int], cols_b: int, join_col_a: int, join_col_b: int) -> list[int]:
    """Hash join using dict for build phase."""
    build_map: dict[int, int] = {}
    rows_a: int = len(table_a) // cols_a
    rows_b: int = len(table_b) // cols_b
    i: int = 0
    while i < rows_a:
        jk: int = table_a[i * cols_a + join_col_a]
        build_map[jk] = i
        i = i + 1
    result: list[int] = []
    j: int = 0
    while j < rows_b:
        jk2: int = table_b[j * cols_b + join_col_b]
        if jk2 in build_map:
            a_row: int = build_map[jk2]
            ka: int = 0
            while ka < cols_a:
                rv: int = table_a[a_row * cols_a + ka]
                result.append(rv)
                ka = ka + 1
            kb: int = 0
            while kb < cols_b:
                rv2: int = table_b[j * cols_b + kb]
                result.append(rv2)
                kb = kb + 1
        j = j + 1
    return result


def sort_column(table_data: list[int], num_cols: int, sort_col: int) -> list[int]:
    """Sort table rows by a column (selection sort). Returns sorted copy."""
    num_rows: int = len(table_data) // num_cols
    row_order: list[int] = []
    ri: int = 0
    while ri < num_rows:
        row_order.append(ri)
        ri = ri + 1
    ii: int = 0
    while ii < num_rows:
        min_idx: int = ii
        jj: int = ii + 1
        while jj < num_rows:
            val_j: int = table_data[row_order[jj] * num_cols + sort_col]
            val_m: int = table_data[row_order[min_idx] * num_cols + sort_col]
            if val_j < val_m:
                min_idx = jj
            jj = jj + 1
        if min_idx != ii:
            tmp: int = row_order[ii]
            row_order[ii] = row_order[min_idx]
            row_order[min_idx] = tmp
        ii = ii + 1
    result: list[int] = []
    si: int = 0
    while si < num_rows:
        row_idx: int = row_order[si]
        ci: int = 0
        while ci < num_cols:
            dv: int = table_data[row_idx * num_cols + ci]
            result.append(dv)
            ci = ci + 1
        si = si + 1
    return result


def count_join_results(result: list[int], result_width: int) -> int:
    """Count number of result rows."""
    if result_width == 0:
        return 0
    return len(result) // result_width


def test_module() -> int:
    """Test join algorithms."""
    ok: int = 0
    ta: list[int] = [1, 10, 2, 20, 3, 30]
    tb: list[int] = [2, 200, 3, 300, 4, 400]
    nl: list[int] = nested_loop_join(ta, 2, tb, 2, 0, 0)
    nl_rows: int = count_join_results(nl, 4)
    if nl_rows == 2:
        ok = ok + 1
    hj: list[int] = hash_join(ta, 2, tb, 2, 0, 0)
    hj_rows: int = count_join_results(hj, 4)
    if hj_rows == 2:
        ok = ok + 1
    if len(nl) == len(hj):
        ok = ok + 1
    sorted_a: list[int] = sort_column(ta, 2, 0)
    sv0: int = sorted_a[0]
    if sv0 == 1:
        ok = ok + 1
    ta_empty: list[int] = []
    nl_empty: list[int] = nested_loop_join(ta_empty, 2, tb, 2, 0, 0)
    if len(nl_empty) == 0:
        ok = ok + 1
    return ok
