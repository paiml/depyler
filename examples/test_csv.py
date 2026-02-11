"""Manual CSV parsing without stdlib csv module."""


def split_csv_line(line: str) -> list[str]:
    """Split a CSV line by comma into fields."""
    fields: list[str] = []
    current: str = ""
    i: int = 0
    length: int = len(line)
    while i < length:
        ch: str = line[i]
        if ch == ",":
            fields.append(current)
            current = ""
        else:
            current = current + ch
        i = i + 1
    fields.append(current)
    return fields


def join_csv_fields(fields: list[str]) -> str:
    """Join fields into a CSV line with commas."""
    result: str = ""
    i: int = 0
    length: int = len(fields)
    while i < length:
        if i > 0:
            result = result + ","
        result = result + fields[i]
        i = i + 1
    return result


def parse_csv_rows(text: str) -> list[list[str]]:
    """Parse multi-line CSV text into rows of fields."""
    rows: list[list[str]] = []
    current_line: str = ""
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        if ch == "\n":
            if len(current_line) > 0:
                row: list[str] = split_csv_line(current_line)
                rows.append(row)
            current_line = ""
        else:
            current_line = current_line + ch
        i = i + 1
    if len(current_line) > 0:
        last_row: list[str] = split_csv_line(current_line)
        rows.append(last_row)
    return rows


def count_csv_columns(text: str) -> int:
    """Count columns in first row of CSV text."""
    rows: list[list[str]] = parse_csv_rows(text)
    if len(rows) == 0:
        return 0
    first: list[str] = rows[0]
    return len(first)


def count_csv_rows(text: str) -> int:
    """Count number of data rows in CSV text."""
    rows: list[list[str]] = parse_csv_rows(text)
    return len(rows)


def test_module() -> int:
    """Test CSV parsing operations."""
    passed: int = 0

    fields: list[str] = split_csv_line("alice,30,engineer")
    if len(fields) == 3:
        passed = passed + 1

    if fields[0] == "alice":
        passed = passed + 1

    joined: str = join_csv_fields(fields)
    if joined == "alice,30,engineer":
        passed = passed + 1

    csv_text: str = "name,age\nalice,30\nbob,25"
    rows: list[list[str]] = parse_csv_rows(csv_text)
    if len(rows) == 3:
        passed = passed + 1

    if count_csv_columns(csv_text) == 2:
        passed = passed + 1

    if count_csv_rows(csv_text) == 3:
        passed = passed + 1

    return passed
