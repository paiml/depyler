"""Real-world simple line-based diff algorithm.

Mimics: difflib, git diff, unified diff format.
Computes longest common subsequence for line diffing.
"""


def diff_split_lines(text: str) -> list[str]:
    """Split text into lines on newline character."""
    result: list[str] = []
    current: str = ""
    idx: int = 0
    while idx < len(text):
        if text[idx] == "\n":
            result.append(current)
            current = ""
        else:
            current = current + text[idx]
        idx = idx + 1
    if len(current) > 0:
        result.append(current)
    return result


def diff_lcs_table(lines_a: list[str], lines_b: list[str]) -> list[list[int]]:
    """Build LCS (longest common subsequence) table for line lists."""
    m: int = len(lines_a)
    n: int = len(lines_b)
    table: list[list[int]] = []
    ri: int = 0
    while ri <= m:
        row: list[int] = []
        ci: int = 0
        while ci <= n:
            row.append(0)
            ci = ci + 1
        table.append(row)
        ri = ri + 1
    i: int = 1
    while i <= m:
        j: int = 1
        while j <= n:
            if lines_a[i - 1] == lines_b[j - 1]:
                table[i][j] = table[i - 1][j - 1] + 1
            else:
                up: int = table[i - 1][j]
                left: int = table[i][j - 1]
                if up > left:
                    table[i][j] = up
                else:
                    table[i][j] = left
            j = j + 1
        i = i + 1
    return table


def compute_diff_ops(lines_a: list[str], lines_b: list[str]) -> list[list[str]]:
    """Compute diff operations. Returns [[op, line], ...].
    ops: 'same', 'del', 'add'."""
    table: list[list[int]] = diff_lcs_table(lines_a, lines_b)
    result: list[list[str]] = []
    i: int = len(lines_a)
    j: int = len(lines_b)
    while i > 0 or j > 0:
        if i > 0 and j > 0 and lines_a[i - 1] == lines_b[j - 1]:
            result.append(["same", lines_a[i - 1]])
            i = i - 1
            j = j - 1
        elif j > 0 and (i == 0 or table[i][j - 1] >= table[i - 1][j]):
            result.append(["add", lines_b[j - 1]])
            j = j - 1
        else:
            result.append(["del", lines_a[i - 1]])
            i = i - 1
    # Reverse
    reversed_result: list[list[str]] = []
    ri: int = len(result) - 1
    while ri >= 0:
        reversed_result.append(result[ri])
        ri = ri - 1
    return reversed_result


def diff_count_op(diff: list[list[str]], op_type: str) -> int:
    """Count operations of a given type in diff."""
    count: int = 0
    idx: int = 0
    while idx < len(diff):
        if diff[idx][0] == op_type:
            count = count + 1
        idx = idx + 1
    return count


def diff_format_entry(entry: list[str]) -> str:
    """Format a diff entry as a display string."""
    if entry[0] == "same":
        return "  " + entry[1]
    if entry[0] == "add":
        return "+ " + entry[1]
    if entry[0] == "del":
        return "- " + entry[1]
    return "? " + entry[1]


def diff_summary(diff: list[list[str]]) -> list[int]:
    """Return [additions, deletions, unchanged] counts."""
    adds: int = diff_count_op(diff, "add")
    dels: int = diff_count_op(diff, "del")
    same: int = diff_count_op(diff, "same")
    return [adds, dels, same]


def diff_similarity_pct(lines_a: list[str], lines_b: list[str]) -> int:
    """Compute similarity ratio as percentage (0-100)."""
    total: int = len(lines_a) + len(lines_b)
    if total == 0:
        return 100
    table: list[list[int]] = diff_lcs_table(lines_a, lines_b)
    lcs_len: int = table[len(lines_a)][len(lines_b)]
    return (2 * lcs_len * 100) // total


def diff_has_changes(diff: list[list[str]]) -> int:
    """Check if diff contains any changes. Returns 1 if changes exist."""
    idx: int = 0
    while idx < len(diff):
        if diff[idx][0] != "same":
            return 1
        idx = idx + 1
    return 0


def test_module() -> int:
    """Test diff module."""
    passed: int = 0

    # Test 1: split lines
    lines: list[str] = diff_split_lines("a\nb\nc")
    if len(lines) == 3 and lines[0] == "a":
        passed = passed + 1

    # Test 2: identical files
    a: list[str] = ["hello", "world"]
    b: list[str] = ["hello", "world"]
    diff: list[list[str]] = compute_diff_ops(a, b)
    same_count: int = diff_count_op(diff, "same")
    add_count: int = diff_count_op(diff, "add")
    if same_count == 2 and add_count == 0:
        passed = passed + 1

    # Test 3: addition
    a2: list[str] = ["line1", "line3"]
    b2: list[str] = ["line1", "line2", "line3"]
    diff2: list[list[str]] = compute_diff_ops(a2, b2)
    adds2: int = diff_count_op(diff2, "add")
    if adds2 == 1:
        passed = passed + 1

    # Test 4: deletion
    a3: list[str] = ["line1", "line2", "line3"]
    b3: list[str] = ["line1", "line3"]
    diff3: list[list[str]] = compute_diff_ops(a3, b3)
    dels3: int = diff_count_op(diff3, "del")
    if dels3 == 1:
        passed = passed + 1

    # Test 5: diff summary
    summary: list[int] = diff_summary(diff2)
    if summary[0] == 1 and summary[2] == 2:
        passed = passed + 1

    # Test 6: format diff line
    formatted: str = diff_format_entry(["add", "new line"])
    if formatted == "+ new line":
        passed = passed + 1

    # Test 7: similarity - identical
    ratio: int = diff_similarity_pct(a, b)
    if ratio == 100:
        passed = passed + 1

    # Test 8: has changes
    if diff_has_changes(diff) == 0 and diff_has_changes(diff2) == 1:
        passed = passed + 1

    return passed
