def csv_escape_field(field: str) -> str:
    needs_quote: int = 0
    n: int = len(field)
    i: int = 0
    while i < n:
        ch: str = field[i]
        if ch == "," or ch == "\"" or ch == "\n":
            needs_quote = 1
        i = i + 1
    if needs_quote == 0:
        return field
    result: str = "\""
    j: int = 0
    while j < n:
        ch2: str = field[j]
        if ch2 == "\"":
            result = result + "\"\""
        else:
            result = result + ch2
        j = j + 1
    result = result + "\""
    return result

def csv_row(fields: list[str]) -> str:
    result: str = ""
    n: int = len(fields)
    i: int = 0
    while i < n:
        if i > 0:
            result = result + ","
        result = result + csv_escape_field(fields[i])
        i = i + 1
    return result

def count_fields(line: str) -> int:
    count: int = 1
    in_quote: int = 0
    n: int = len(line)
    i: int = 0
    while i < n:
        ch: str = line[i]
        if ch == "\"":
            if in_quote == 0:
                in_quote = 1
            else:
                in_quote = 0
        if ch == "," and in_quote == 0:
            count = count + 1
        i = i + 1
    return count

def csv_table(rows: list[list[str]]) -> str:
    result: str = ""
    n: int = len(rows)
    i: int = 0
    while i < n:
        row: list[str] = rows[i]
        if i > 0:
            result = result + "\n"
        result = result + csv_row(row)
        i = i + 1
    return result

def field_widths(fields: list[str]) -> list[int]:
    result: list[int] = []
    n: int = len(fields)
    i: int = 0
    while i < n:
        result.append(len(fields[i]))
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    e: str = csv_escape_field("hello")
    if e == "hello":
        passed = passed + 1
    e2: str = csv_escape_field("hello,world")
    if e2 == "\"hello,world\"":
        passed = passed + 1
    fields: list[str] = ["a", "b", "c"]
    r: str = csv_row(fields)
    if r == "a,b,c":
        passed = passed + 1
    cf: int = count_fields("a,b,c")
    if cf == 3:
        passed = passed + 1
    fw: list[int] = field_widths(fields)
    fw0: int = fw[0]
    if fw0 == 1:
        passed = passed + 1
    return passed
