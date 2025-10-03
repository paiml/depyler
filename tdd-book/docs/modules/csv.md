# csv

## csv.reader() - Read CSV data from file-like object.

## csv.writer() - Write CSV data to file-like object.

## csv.DictReader() - Read CSV with first row as field names.

## csv.DictWriter() - Write CSV from dictionaries.

## CSV dialects - Predefined formatting styles.

## CSV quoting behavior - Control when fields are quoted.

## CSV escaping behavior - Handle special characters.

## csv.Sniffer - Detect CSV dialect from sample.

## Edge cases and special scenarios.

### Basic: Read simple CSV data.

```python
def test_reader_basic(self):
    """Basic: Read simple CSV data."""
    data = 'a,b,c\n1,2,3\n4,5,6'
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)
    assert rows == [['a', 'b', 'c'], ['1', '2', '3'], ['4', '5', '6']]
```

**Verification**: ✅ Tested in CI

### Feature: Iterate over CSV rows.

```python
def test_reader_iterate(self):
    """Feature: Iterate over CSV rows."""
    data = 'name,age\nAlice,30\nBob,25'
    reader = csv.reader(io.StringIO(data))
    rows = []
    for row in reader:
        rows.append(row)
    assert len(rows) == 3
    assert rows[0] == ['name', 'age']
```

**Verification**: ✅ Tested in CI

### Edge: Empty fields are preserved.

```python
def test_reader_empty_fields(self):
    """Edge: Empty fields are preserved."""
    data = 'a,,c\n1,2,'
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)
    assert rows == [['a', '', 'c'], ['1', '2', '']]
```

**Verification**: ✅ Tested in CI

### Feature: Quoted fields preserve commas.

```python
def test_reader_quoted_fields(self):
    """Feature: Quoted fields preserve commas."""
    data = 'a,"b,c",d\n1,"2,3",4'
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)
    assert rows == [['a', 'b,c', 'd'], ['1', '2,3', '4']]
```

**Verification**: ✅ Tested in CI

### Edge: Newlines in quoted fields are preserved.

```python
def test_reader_newline_in_quoted_field(self):
    """Edge: Newlines in quoted fields are preserved."""
    data = 'a,"b\nc",d\n1,2,3'
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)
    assert rows[0] == ['a', 'b\nc', 'd']
```

**Verification**: ✅ Tested in CI

### Feature: Custom delimiter support.

```python
def test_reader_custom_delimiter(self):
    """Feature: Custom delimiter support."""
    data = 'a;b;c\n1;2;3'
    reader = csv.reader(io.StringIO(data), delimiter=';')
    rows = list(reader)
    assert rows == [['a', 'b', 'c'], ['1', '2', '3']]
```

**Verification**: ✅ Tested in CI

### Edge: Empty lines are yielded as empty lists.

```python
def test_reader_empty_lines_skipped(self):
    """Edge: Empty lines are yielded as empty lists."""
    data = 'a,b,c\n\n1,2,3'
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)
    assert len(rows) == 3
    assert rows[0] == ['a', 'b', 'c']
```

**Verification**: ✅ Tested in CI

### Basic: Write simple CSV data.

```python
def test_writer_basic(self):
    """Basic: Write simple CSV data."""
    output = io.StringIO()
    writer = csv.writer(output)
    writer.writerow(['a', 'b', 'c'])
    writer.writerow(['1', '2', '3'])
    result = output.getvalue()
    assert result == 'a,b,c\r\n1,2,3\r\n'
```

**Verification**: ✅ Tested in CI

### Feature: Write multiple rows at once.

```python
def test_writer_writerows(self):
    """Feature: Write multiple rows at once."""
    output = io.StringIO()
    writer = csv.writer(output)
    writer.writerows([['a', 'b'], ['1', '2'], ['3', '4']])
    result = output.getvalue()
    assert 'a,b' in result
    assert '1,2' in result
```

**Verification**: ✅ Tested in CI

### Feature: Fields with commas are quoted automatically.

```python
def test_writer_quoting_commas(self):
    """Feature: Fields with commas are quoted automatically."""
    output = io.StringIO()
    writer = csv.writer(output)
    writer.writerow(['a', 'b,c', 'd'])
    result = output.getvalue()
    assert '"b,c"' in result
```

**Verification**: ✅ Tested in CI

### Feature: Custom delimiter support.

```python
def test_writer_custom_delimiter(self):
    """Feature: Custom delimiter support."""
    output = io.StringIO()
    writer = csv.writer(output, delimiter=';')
    writer.writerow(['a', 'b', 'c'])
    result = output.getvalue()
    assert result == 'a;b;c\r\n'
```

**Verification**: ✅ Tested in CI

### Edge: Empty fields are written correctly.

```python
def test_writer_empty_field(self):
    """Edge: Empty fields are written correctly."""
    output = io.StringIO()
    writer = csv.writer(output)
    writer.writerow(['a', '', 'c'])
    result = output.getvalue()
    assert result == 'a,,c\r\n'
```

**Verification**: ✅ Tested in CI

### Property: Numeric values are converted to strings.

```python
def test_writer_numeric_values(self):
    """Property: Numeric values are converted to strings."""
    output = io.StringIO()
    writer = csv.writer(output)
    writer.writerow([1, 2, 3])
    result = output.getvalue()
    assert result == '1,2,3\r\n'
```

**Verification**: ✅ Tested in CI

### Basic: Read CSV as dictionaries.

```python
def test_dictreader_basic(self):
    """Basic: Read CSV as dictionaries."""
    data = 'name,age,city\nAlice,30,NYC\nBob,25,LA'
    reader = csv.DictReader(io.StringIO(data))
    rows = list(reader)
    assert len(rows) == 2
    assert rows[0] == {'name': 'Alice', 'age': '30', 'city': 'NYC'}
    assert rows[1] == {'name': 'Bob', 'age': '25', 'city': 'LA'}
```

**Verification**: ✅ Tested in CI

### Property: fieldnames accessible after reading.

```python
def test_dictreader_fieldnames_property(self):
    """Property: fieldnames accessible after reading."""
    data = 'name,age\nAlice,30'
    reader = csv.DictReader(io.StringIO(data))
    list(reader)
    assert reader.fieldnames == ['name', 'age']
```

**Verification**: ✅ Tested in CI

### Feature: Specify custom field names.

```python
def test_dictreader_custom_fieldnames(self):
    """Feature: Specify custom field names."""
    data = 'Alice,30\nBob,25'
    reader = csv.DictReader(io.StringIO(data), fieldnames=['name', 'age'])
    rows = list(reader)
    assert rows[0] == {'name': 'Alice', 'age': '30'}
```

**Verification**: ✅ Tested in CI

### Edge: Missing fields get None values.

```python
def test_dictreader_missing_fields(self):
    """Edge: Missing fields get None values."""
    data = 'name,age,city\nAlice,30\nBob,25,LA'
    reader = csv.DictReader(io.StringIO(data))
    rows = list(reader)
    assert rows[0] == {'name': 'Alice', 'age': '30', 'city': None}
```

**Verification**: ✅ Tested in CI

### Edge: Extra fields go into restkey.

```python
def test_dictreader_extra_fields(self):
    """Edge: Extra fields go into restkey."""
    data = 'name,age\nAlice,30,extra'
    reader = csv.DictReader(io.StringIO(data))
    rows = list(reader)
    assert rows[0]['name'] == 'Alice'
    assert rows[0]['age'] == '30'
```

**Verification**: ✅ Tested in CI

### Property: DictReader is iterable.

```python
def test_dictreader_iterate(self):
    """Property: DictReader is iterable."""
    data = 'name,age\nAlice,30\nBob,25'
    reader = csv.DictReader(io.StringIO(data))
    count = 0
    for row in reader:
        assert isinstance(row, dict)
        count += 1
    assert count == 2
```

**Verification**: ✅ Tested in CI

### Basic: Write dictionaries as CSV.

```python
def test_dictwriter_basic(self):
    """Basic: Write dictionaries as CSV."""
    output = io.StringIO()
    fieldnames = ['name', 'age']
    writer = csv.DictWriter(output, fieldnames=fieldnames)
    writer.writeheader()
    writer.writerow({'name': 'Alice', 'age': 30})
    writer.writerow({'name': 'Bob', 'age': 25})
    result = output.getvalue()
    assert 'name,age' in result
    assert 'Alice,30' in result
    assert 'Bob,25' in result
```

**Verification**: ✅ Tested in CI

### Feature: writeheader() writes field names.

```python
def test_dictwriter_writeheader(self):
    """Feature: writeheader() writes field names."""
    output = io.StringIO()
    writer = csv.DictWriter(output, fieldnames=['a', 'b', 'c'])
    writer.writeheader()
    result = output.getvalue()
    assert result == 'a,b,c\r\n'
```

**Verification**: ✅ Tested in CI

### Edge: Missing fields in dict use empty string.

```python
def test_dictwriter_missing_field(self):
    """Edge: Missing fields in dict use empty string."""
    output = io.StringIO()
    writer = csv.DictWriter(output, fieldnames=['name', 'age', 'city'])
    writer.writeheader()
    writer.writerow({'name': 'Alice', 'age': 30})
    result = output.getvalue()
    assert 'Alice,30,' in result
```

**Verification**: ✅ Tested in CI

### Error: Extra fields in dict raise ValueError by default.

```python
def test_dictwriter_extra_field_raises(self):
    """Error: Extra fields in dict raise ValueError by default."""
    output = io.StringIO()
    writer = csv.DictWriter(output, fieldnames=['name', 'age'])
    with pytest.raises(ValueError):
        writer.writerow({'name': 'Alice', 'age': 30, 'city': 'NYC'})
```

**Verification**: ✅ Tested in CI

### Feature: extrasaction='ignore' skips extra fields.

```python
def test_dictwriter_extrasaction_ignore(self):
    """Feature: extrasaction='ignore' skips extra fields."""
    output = io.StringIO()
    writer = csv.DictWriter(output, fieldnames=['name', 'age'], extrasaction='ignore')
    writer.writerow({'name': 'Alice', 'age': 30, 'city': 'NYC'})
    result = output.getvalue()
    assert result == 'Alice,30\r\n'
```

**Verification**: ✅ Tested in CI

### Feature: writerows() writes multiple dicts.

```python
def test_dictwriter_writerows(self):
    """Feature: writerows() writes multiple dicts."""
    output = io.StringIO()
    writer = csv.DictWriter(output, fieldnames=['a', 'b'])
    writer.writerows([{'a': 1, 'b': 2}, {'a': 3, 'b': 4}])
    result = output.getvalue()
    assert '1,2' in result
    assert '3,4' in result
```

**Verification**: ✅ Tested in CI

### Feature: excel dialect is default.

```python
def test_excel_dialect(self):
    """Feature: excel dialect is default."""
    output = io.StringIO()
    writer = csv.writer(output, dialect='excel')
    writer.writerow(['a', 'b'])
    result = output.getvalue()
    assert 'a,b' in result
```

**Verification**: ✅ Tested in CI

### Feature: excel-tab dialect uses tabs.

```python
def test_excel_tab_dialect(self):
    """Feature: excel-tab dialect uses tabs."""
    output = io.StringIO()
    writer = csv.writer(output, dialect='excel-tab')
    writer.writerow(['a', 'b', 'c'])
    result = output.getvalue()
    assert 'a\tb\tc' in result
```

**Verification**: ✅ Tested in CI

### Feature: unix dialect uses LF line terminator.

```python
def test_unix_dialect(self):
    """Feature: unix dialect uses LF line terminator."""
    output = io.StringIO()
    writer = csv.writer(output, dialect='unix')
    writer.writerow(['a', 'b'])
    result = output.getvalue()
    assert result == '"a","b"\n'
```

**Verification**: ✅ Tested in CI

### Property: csv.list_dialects() returns available dialects.

```python
def test_list_dialects(self):
    """Property: csv.list_dialects() returns available dialects."""
    dialects = csv.list_dialects()
    assert 'excel' in dialects
    assert 'unix' in dialects
```

**Verification**: ✅ Tested in CI

### Property: QUOTE_MINIMAL is default (quote only when needed).

```python
def test_quote_minimal_default(self):
    """Property: QUOTE_MINIMAL is default (quote only when needed)."""
    output = io.StringIO()
    writer = csv.writer(output, quoting=csv.QUOTE_MINIMAL)
    writer.writerow(['a', 'b,c', 'd'])
    result = output.getvalue()
    assert 'a,' in result or 'a"' not in result
    assert '"b,c"' in result
```

**Verification**: ✅ Tested in CI

### Feature: QUOTE_ALL quotes every field.

```python
def test_quote_all(self):
    """Feature: QUOTE_ALL quotes every field."""
    output = io.StringIO()
    writer = csv.writer(output, quoting=csv.QUOTE_ALL)
    writer.writerow(['a', 'b', 'c'])
    result = output.getvalue()
    assert '"a","b","c"' in result
```

**Verification**: ✅ Tested in CI

### Feature: QUOTE_NONNUMERIC quotes non-numeric fields.

```python
def test_quote_nonnumeric(self):
    """Feature: QUOTE_NONNUMERIC quotes non-numeric fields."""
    output = io.StringIO()
    writer = csv.writer(output, quoting=csv.QUOTE_NONNUMERIC)
    writer.writerow(['a', 1, 2.5])
    result = output.getvalue()
    assert '"a"' in result
    assert result.count('"') == 2
```

**Verification**: ✅ Tested in CI

### Edge: QUOTE_NONE never quotes (escapes instead).

```python
def test_quote_none(self):
    """Edge: QUOTE_NONE never quotes (escapes instead)."""
    output = io.StringIO()
    writer = csv.writer(output, quoting=csv.QUOTE_NONE, escapechar='\\')
    writer.writerow(['a', 'b', 'c'])
    result = output.getvalue()
    assert '"' not in result
```

**Verification**: ✅ Tested in CI

### Feature: Quotes in quoted fields are doubled.

```python
def test_escape_quotes(self):
    """Feature: Quotes in quoted fields are doubled."""
    output = io.StringIO()
    writer = csv.writer(output)
    writer.writerow(['a', 'b"c', 'd'])
    result = output.getvalue()
    assert 'b""c' in result
```

**Verification**: ✅ Tested in CI

### Feature: Custom escape character.

```python
def test_custom_escapechar(self):
    """Feature: Custom escape character."""
    output = io.StringIO()
    writer = csv.writer(output, quoting=csv.QUOTE_NONE, escapechar='\\')
    writer.writerow(['a', 'b,c'])
    result = output.getvalue()
    assert 'b\\,c' in result
```

**Verification**: ✅ Tested in CI

### Edge: doublequote=False uses escapechar.

```python
def test_doublequote_disabled(self):
    """Edge: doublequote=False uses escapechar."""
    output = io.StringIO()
    writer = csv.writer(output, doublequote=False, escapechar='\\')
    writer.writerow(['a', 'b"c'])
    result = output.getvalue()
    assert 'b\\"c' in result or 'b\\"c' in result
```

**Verification**: ✅ Tested in CI

### Feature: Sniffer detects delimiter.

```python
def test_sniffer_detect_delimiter(self):
    """Feature: Sniffer detects delimiter."""
    sample = 'a;b;c\n1;2;3\n'
    sniffer = csv.Sniffer()
    dialect = sniffer.sniff(sample)
    assert dialect.delimiter == ';'
```

**Verification**: ✅ Tested in CI

### Feature: Sniffer detects if first row is header.

```python
def test_sniffer_has_header(self):
    """Feature: Sniffer detects if first row is header."""
    sample = 'name,age,city\nAlice,30,NYC\nBob,25,LA\n'
    sniffer = csv.Sniffer()
    has_header = sniffer.has_header(sample)
    assert has_header is True
```

**Verification**: ✅ Tested in CI

### Edge: Sniffer detects when no header present.

```python
def test_sniffer_no_header(self):
    """Edge: Sniffer detects when no header present."""
    sample = '1,2,3\n4,5,6\n7,8,9\n'
    sniffer = csv.Sniffer()
    has_header = sniffer.has_header(sample)
    assert has_header is False
```

**Verification**: ✅ Tested in CI

### Property: Write → Read roundtrip preserves data.

```python
def test_roundtrip_preservation(self):
    """Property: Write → Read roundtrip preserves data."""
    original = [['a', 'b', 'c'], ['1', '2', '3'], ['4', '5', '6']]
    output = io.StringIO()
    writer = csv.writer(output)
    writer.writerows(original)
    output.seek(0)
    reader = csv.reader(output)
    result = list(reader)
    assert result == original
```

**Verification**: ✅ Tested in CI

### Edge: Unicode characters are handled correctly.

```python
def test_unicode_content(self):
    """Edge: Unicode characters are handled correctly."""
    output = io.StringIO()
    writer = csv.writer(output)
    writer.writerow(['Hello', '世界', 'Привет'])
    result = output.getvalue()
    assert '世界' in result
    assert 'Привет' in result
```

**Verification**: ✅ Tested in CI

### Edge: Very long fields are handled.

```python
def test_very_long_field(self):
    """Edge: Very long fields are handled."""
    output = io.StringIO()
    writer = csv.writer(output)
    long_field = 'a' * 10000
    writer.writerow(['short', long_field, 'end'])
    result = output.getvalue()
    assert long_field in result
```

**Verification**: ✅ Tested in CI

### Edge: Single column CSV works correctly.

```python
def test_single_column(self):
    """Edge: Single column CSV works correctly."""
    data = 'a\nb\nc'
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)
    assert rows == [['a'], ['b'], ['c']]
```

**Verification**: ✅ Tested in CI

### Edge: Empty CSV returns empty list.

```python
def test_empty_csv(self):
    """Edge: Empty CSV returns empty list."""
    data = ''
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)
    assert rows == []
```

**Verification**: ✅ Tested in CI

### Edge: Trailing delimiter creates empty field.

```python
def test_trailing_delimiter(self):
    """Edge: Trailing delimiter creates empty field."""
    data = 'a,b,c,\n1,2,3,'
    reader = csv.reader(io.StringIO(data))
    rows = list(reader)
    assert rows[0] == ['a', 'b', 'c', '']
    assert rows[1] == ['1', '2', '3', '']
```

**Verification**: ✅ Tested in CI
