"""Real-world CSV parsing without imports.

Mimics: pandas read_csv, csv.reader patterns found in data engineering.
Handles comma splitting, quoted fields, header extraction, column selection.
"""


def split_csv_line(line: str, delimiter: str) -> list[str]:
    """Split a CSV line on delimiter, respecting double-quoted fields."""
    fields: list[str] = []
    current: str = ""
    in_quotes: bool = False
    idx: int = 0
    while idx < len(line):
        ch: str = line[idx]
        if ch == '"':
            in_quotes = not in_quotes
        elif ch == delimiter and not in_quotes:
            fields.append(current)
            current = ""
        else:
            current = current + ch
        idx = idx + 1
    fields.append(current)
    return fields


def parse_csv_text(text: str) -> list[list[str]]:
    """Parse multi-line CSV text into rows of fields."""
    rows: list[list[str]] = []
    current_line: str = ""
    idx: int = 0
    while idx < len(text):
        ch: str = text[idx]
        if ch == "\n":
            if len(current_line) > 0:
                row: list[str] = split_csv_line(current_line, ",")
                rows.append(row)
            current_line = ""
        else:
            current_line = current_line + ch
        idx = idx + 1
    if len(current_line) > 0:
        last_row: list[str] = split_csv_line(current_line, ",")
        rows.append(last_row)
    return rows


def get_column_by_index(rows: list[list[str]], col_idx: int) -> list[str]:
    """Extract a single column from parsed CSV rows."""
    result: list[str] = []
    row_idx: int = 0
    while row_idx < len(rows):
        row: list[str] = rows[row_idx]
        if col_idx < len(row):
            result.append(row[col_idx])
        else:
            result.append("")
        row_idx = row_idx + 1
    return result


def find_header_index(headers: list[str], name: str) -> int:
    """Find column index by header name. Returns -1 if not found."""
    idx: int = 0
    while idx < len(headers):
        if headers[idx] == name:
            return idx
        idx = idx + 1
    return -1


def count_non_empty(column: list[str]) -> int:
    """Count non-empty values in a column."""
    total: int = 0
    idx: int = 0
    while idx < len(column):
        if len(column[idx]) > 0:
            total = total + 1
        idx = idx + 1
    return total


def column_to_ints(column: list[str]) -> list[int]:
    """Convert string column to integers, skipping non-numeric."""
    result: list[int] = []
    idx: int = 0
    while idx < len(column):
        val: str = column[idx]
        if len(val) > 0:
            num: int = 0
            is_neg: bool = False
            char_idx: int = 0
            valid: bool = True
            if len(val) > 0 and val[0] == "-":
                is_neg = True
                char_idx = 1
            while char_idx < len(val):
                c: str = val[char_idx]
                if c >= "0" and c <= "9":
                    digit: int = ord(c) - ord("0")
                    num = num * 10 + digit
                else:
                    valid = False
                char_idx = char_idx + 1
            if valid:
                if is_neg:
                    num = 0 - num
                result.append(num)
        idx = idx + 1
    return result


def sum_int_column(values: list[int]) -> int:
    """Sum all integers in a column."""
    total: int = 0
    idx: int = 0
    while idx < len(values):
        total = total + values[idx]
        idx = idx + 1
    return total


def csv_row_count(rows: list[list[str]]) -> int:
    """Count number of data rows (excluding potential header)."""
    return len(rows)


def filter_rows_by_value(rows: list[list[str]], col_idx: int, match_val: str) -> list[list[str]]:
    """Filter rows where column matches a value."""
    result: list[list[str]] = []
    row_idx: int = 0
    while row_idx < len(rows):
        row: list[str] = rows[row_idx]
        if col_idx < len(row) and row[col_idx] == match_val:
            result.append(row)
        row_idx = row_idx + 1
    return result


def test_module() -> int:
    """Test CSV parsing module."""
    passed: int = 0

    # Test 1: basic line splitting
    fields: list[str] = split_csv_line("hello,world,test", ",")
    if len(fields) == 3 and fields[0] == "hello" and fields[2] == "test":
        passed = passed + 1

    # Test 2: quoted field handling
    qfields: list[str] = split_csv_line('a,"b,c",d', ",")
    if len(qfields) == 3 and qfields[1] == "b,c":
        passed = passed + 1

    # Test 3: parse multi-line CSV
    csv_text: str = "name,age,city\nalice,30,nyc\nbob,25,sf"
    rows: list[list[str]] = parse_csv_text(csv_text)
    if len(rows) == 3 and rows[0][0] == "name":
        passed = passed + 1

    # Test 4: column extraction
    ages: list[str] = get_column_by_index(rows, 1)
    if len(ages) == 3 and ages[0] == "age" and ages[1] == "30":
        passed = passed + 1

    # Test 5: header index lookup
    headers: list[str] = rows[0]
    city_idx: int = find_header_index(headers, "city")
    if city_idx == 2:
        passed = passed + 1

    # Test 6: count non-empty
    col: list[str] = ["a", "", "b", "c", ""]
    if count_non_empty(col) == 3:
        passed = passed + 1

    # Test 7: column to ints
    str_nums: list[str] = ["10", "20", "30"]
    nums: list[int] = column_to_ints(str_nums)
    if sum_int_column(nums) == 60:
        passed = passed + 1

    # Test 8: filter rows
    data_rows: list[list[str]] = [["alice", "nyc"], ["bob", "sf"], ["carol", "nyc"]]
    nyc_rows: list[list[str]] = filter_rows_by_value(data_rows, 1, "nyc")
    if len(nyc_rows) == 2:
        passed = passed + 1

    return passed
