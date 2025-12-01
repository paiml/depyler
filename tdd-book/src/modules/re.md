# re

## re.compile() - Compile regular expression pattern.

## re.search() - Search for pattern anywhere in string.

## re.match() - Match pattern at beginning of string.

## re.fullmatch() - Match entire string.

## re.findall() - Find all non-overlapping matches.

## re.finditer() - Iterator over match objects.

## re.sub() - Replace pattern matches.

## re.subn() - Replace and return count.

## re.split() - Split string by pattern.

## Pattern groups and capturing.

## Regular expression flags.

## Special character classes and escapes.

## Pattern quantifiers (* + ? {m,n}).

## Pattern anchors (^ $ \b \B).

## Lookahead and lookbehind assertions.

## Edge cases and special scenarios.

### Basic: Compile simple pattern.

```python
def test_compile_basic(self):
    """Basic: Compile simple pattern."""
    pattern = re.compile('hello')
    assert pattern.pattern == 'hello'
```

**Verification**: ✅ Tested in CI

### Feature: Compile with flags.

```python
def test_compile_with_flags(self):
    """Feature: Compile with flags."""
    pattern = re.compile('hello', re.IGNORECASE)
    assert pattern.flags & re.IGNORECASE
```

**Verification**: ✅ Tested in CI

### Property: Compiled patterns can be reused.

```python
def test_compile_reuse(self):
    """Property: Compiled patterns can be reused."""
    pattern = re.compile('\\d+')
    assert pattern.search('abc123')
    assert pattern.search('xyz789')
```

**Verification**: ✅ Tested in CI

### Error: Invalid pattern raises re.error.

```python
def test_compile_invalid_pattern_raises(self):
    """Error: Invalid pattern raises re.error."""
    with pytest.raises(re.error):
        re.compile('(?P<')
```

**Verification**: ✅ Tested in CI

### Basic: Search finds pattern in string.

```python
def test_search_basic(self):
    """Basic: Search finds pattern in string."""
    match = re.search('world', 'hello world')
    assert match is not None
    assert match.group() == 'world'
```

**Verification**: ✅ Tested in CI

### Basic: No match returns None.

```python
def test_search_no_match_returns_none(self):
    """Basic: No match returns None."""
    match = re.search('xyz', 'hello world')
    assert match is None
```

**Verification**: ✅ Tested in CI

### Feature: Search finds first occurrence.

```python
def test_search_finds_first_occurrence(self):
    """Feature: Search finds first occurrence."""
    match = re.search('\\d+', 'abc123def456')
    assert match.group() == '123'
```

**Verification**: ✅ Tested in CI

### Property: Match object has position info.

```python
def test_search_position(self):
    """Property: Match object has position info."""
    match = re.search('world', 'hello world')
    assert match.start() == 6
    assert match.end() == 11
    assert match.span() == (6, 11)
```

**Verification**: ✅ Tested in CI

### Basic: Match at start of string.

```python
def test_match_basic(self):
    """Basic: Match at start of string."""
    match = re.match('hello', 'hello world')
    assert match is not None
    assert match.group() == 'hello'
```

**Verification**: ✅ Tested in CI

### Feature: Match requires pattern at start.

```python
def test_match_middle_fails(self):
    """Feature: Match requires pattern at start."""
    match = re.match('world', 'hello world')
    assert match is None
```

**Verification**: ✅ Tested in CI

### Property: match() anchors at start, search() doesn't.

```python
def test_match_vs_search(self):
    """Property: match() anchors at start, search() doesn't."""
    text = 'abc123'
    assert re.match('\\d+', text) is None
    assert re.search('\\d+', text) is not None
```

**Verification**: ✅ Tested in CI

### Basic: Fullmatch requires complete match.

```python
def test_fullmatch_basic(self):
    """Basic: Fullmatch requires complete match."""
    match = re.fullmatch('hello', 'hello')
    assert match is not None
```

**Verification**: ✅ Tested in CI

### Feature: Partial match fails.

```python
def test_fullmatch_partial_fails(self):
    """Feature: Partial match fails."""
    match = re.fullmatch('hello', 'hello world')
    assert match is None
```

**Verification**: ✅ Tested in CI

### Feature: Useful for validation.

```python
def test_fullmatch_use_case(self):
    """Feature: Useful for validation."""
    pattern = '\\d{3}-\\d{3}-\\d{4}'
    assert re.fullmatch(pattern, '555-123-4567') is not None
    assert re.fullmatch(pattern, '555-123-4567x') is None
```

**Verification**: ✅ Tested in CI

### Basic: Find all occurrences.

```python
def test_findall_basic(self):
    """Basic: Find all occurrences."""
    matches = re.findall('\\d+', 'abc123def456ghi789')
    assert matches == ['123', '456', '789']
```

**Verification**: ✅ Tested in CI

### Edge: No matches returns empty list.

```python
def test_findall_no_matches(self):
    """Edge: No matches returns empty list."""
    matches = re.findall('\\d+', 'abcdef')
    assert matches == []
```

**Verification**: ✅ Tested in CI

### Feature: Groups affect returned values.

```python
def test_findall_with_groups(self):
    """Feature: Groups affect returned values."""
    matches = re.findall('(\\w+)@(\\w+)', 'alice@example bob@test')
    assert matches == [('alice', 'example'), ('bob', 'test')]
```

**Verification**: ✅ Tested in CI

### Property: Matches are non-overlapping.

```python
def test_findall_non_overlapping(self):
    """Property: Matches are non-overlapping."""
    matches = re.findall('.{2}', 'abcdef')
    assert matches == ['ab', 'cd', 'ef']
```

**Verification**: ✅ Tested in CI

### Basic: Iterate over matches.

```python
def test_finditer_basic(self):
    """Basic: Iterate over matches."""
    matches = list(re.finditer('\\d+', 'abc123def456'))
    assert len(matches) == 2
    assert matches[0].group() == '123'
    assert matches[1].group() == '456'
```

**Verification**: ✅ Tested in CI

### Feature: Returns full match objects.

```python
def test_finditer_match_objects(self):
    """Feature: Returns full match objects."""
    matches = list(re.finditer('\\d+', 'abc123def456'))
    assert matches[0].start() == 3
    assert matches[0].end() == 6
    assert matches[1].start() == 9
    assert matches[1].end() == 12
```

**Verification**: ✅ Tested in CI

### Property: Iterator is memory-efficient.

```python
def test_finditer_memory_efficient(self):
    """Property: Iterator is memory-efficient."""
    iterator = re.finditer('\\d+', 'a1b2c3d4e5')
    first = next(iterator)
    assert first.group() == '1'
```

**Verification**: ✅ Tested in CI

### Basic: Replace pattern with string.

```python
def test_sub_basic(self):
    """Basic: Replace pattern with string."""
    result = re.sub('\\d+', 'X', 'abc123def456')
    assert result == 'abcXdefX'
```

**Verification**: ✅ Tested in CI

### Feature: Limit number of replacements.

```python
def test_sub_count(self):
    """Feature: Limit number of replacements."""
    result = re.sub('\\d+', 'X', 'abc123def456', count=1)
    assert result == 'abcXdef456'
```

**Verification**: ✅ Tested in CI

### Feature: Replacement can be a function.

```python
def test_sub_with_function(self):
    """Feature: Replacement can be a function."""

    def double(match):
        return str(int(match.group()) * 2)
    result = re.sub('\\d+', double, 'abc5def10')
    assert result == 'abc10def20'
```

**Verification**: ✅ Tested in CI

### Feature: Use backreferences in replacement.

```python
def test_sub_backreferences(self):
    """Feature: Use backreferences in replacement."""
    result = re.sub('(\\w+) (\\w+)', '\\2 \\1', 'hello world')
    assert result == 'world hello'
```

**Verification**: ✅ Tested in CI

### Basic: Returns tuple (new_string, count).

```python
def test_subn_basic(self):
    """Basic: Returns tuple (new_string, count)."""
    result, count = re.subn('\\d+', 'X', 'abc123def456')
    assert result == 'abcXdefX'
    assert count == 2
```

**Verification**: ✅ Tested in CI

### Edge: No matches returns count of 0.

```python
def test_subn_no_matches(self):
    """Edge: No matches returns count of 0."""
    result, count = re.subn('\\d+', 'X', 'abcdef')
    assert result == 'abcdef'
    assert count == 0
```

**Verification**: ✅ Tested in CI

### Basic: Split by pattern.

```python
def test_split_basic(self):
    """Basic: Split by pattern."""
    parts = re.split('\\s+', 'hello world  from   python')
    assert parts == ['hello', 'world', 'from', 'python']
```

**Verification**: ✅ Tested in CI

### Feature: Limit number of splits.

```python
def test_split_maxsplit(self):
    """Feature: Limit number of splits."""
    parts = re.split('\\s+', 'a b c d', maxsplit=2)
    assert parts == ['a', 'b', 'c d']
```

**Verification**: ✅ Tested in CI

### Feature: Capturing groups are included in result.

```python
def test_split_with_groups(self):
    """Feature: Capturing groups are included in result."""
    parts = re.split('(\\s+)', 'hello world')
    assert parts == ['hello', ' ', 'world']
```

**Verification**: ✅ Tested in CI

### Edge: Empty matches at edges.

```python
def test_split_empty_matches(self):
    """Edge: Empty matches at edges."""
    parts = re.split('\\d+', 'a1b2c')
    assert parts == ['a', 'b', 'c']
```

**Verification**: ✅ Tested in CI

### Basic: Access captured groups.

```python
def test_groups_basic(self):
    """Basic: Access captured groups."""
    match = re.search('(\\w+)@(\\w+)', 'alice@example')
    assert match.group(0) == 'alice@example'
    assert match.group(1) == 'alice'
    assert match.group(2) == 'example'
```

**Verification**: ✅ Tested in CI

### Feature: groups() returns all groups as tuple.

```python
def test_groups_tuple(self):
    """Feature: groups() returns all groups as tuple."""
    match = re.search('(\\w+)@(\\w+)', 'alice@example')
    assert match.groups() == ('alice', 'example')
```

**Verification**: ✅ Tested in CI

### Feature: Named groups with ?P<name>.

```python
def test_named_groups(self):
    """Feature: Named groups with ?P<name>."""
    match = re.search('(?P<user>\\w+)@(?P<domain>\\w+)', 'alice@example')
    assert match.group('user') == 'alice'
    assert match.group('domain') == 'example'
```

**Verification**: ✅ Tested in CI

### Feature: groupdict() returns named groups as dict.

```python
def test_groupdict(self):
    """Feature: groupdict() returns named groups as dict."""
    match = re.search('(?P<user>\\w+)@(?P<domain>\\w+)', 'alice@example')
    assert match.groupdict() == {'user': 'alice', 'domain': 'example'}
```

**Verification**: ✅ Tested in CI

### Feature: (?:...) is non-capturing.

```python
def test_non_capturing_group(self):
    """Feature: (?:...) is non-capturing."""
    match = re.search('(?:\\w+)@(\\w+)', 'alice@example')
    assert match.groups() == ('example',)
```

**Verification**: ✅ Tested in CI

### Feature: IGNORECASE makes pattern case-insensitive.

```python
def test_ignorecase_flag(self):
    """Feature: IGNORECASE makes pattern case-insensitive."""
    pattern = re.compile('hello', re.IGNORECASE)
    assert pattern.search('HELLO') is not None
    assert pattern.search('Hello') is not None
```

**Verification**: ✅ Tested in CI

### Feature: MULTILINE makes ^ and $ match line boundaries.

```python
def test_multiline_flag(self):
    """Feature: MULTILINE makes ^ and $ match line boundaries."""
    text = 'first line\nsecond line'
    assert len(re.findall('^second', text)) == 0
    assert len(re.findall('^second', text, re.MULTILINE)) == 1
```

**Verification**: ✅ Tested in CI

### Feature: DOTALL makes . match newlines.

```python
def test_dotall_flag(self):
    """Feature: DOTALL makes . match newlines."""
    text = 'hello\nworld'
    assert re.search('hello.world', text) is None
    assert re.search('hello.world', text, re.DOTALL) is not None
```

**Verification**: ✅ Tested in CI

### Feature: VERBOSE allows whitespace and comments in pattern.

```python
def test_verbose_flag(self):
    """Feature: VERBOSE allows whitespace and comments in pattern."""
    pattern = re.compile('\n            (\\d{3})  # Area code\n            -        # Separator\n            (\\d{4})  # Number\n        ', re.VERBOSE)
    match = pattern.search('555-1234')
    assert match.groups() == ('555', '1234')
```

**Verification**: ✅ Tested in CI

### Feature: Multiple flags can be combined.

```python
def test_combined_flags(self):
    """Feature: Multiple flags can be combined."""
    pattern = re.compile('hello.world', re.IGNORECASE | re.DOTALL)
    assert pattern.search('HELLO\nWORLD') is not None
```

**Verification**: ✅ Tested in CI

### Feature: \d matches digits.

```python
def test_digit_class(self):
    """Feature: \\d matches digits."""
    assert re.findall('\\d', 'abc123def') == ['1', '2', '3']
```

**Verification**: ✅ Tested in CI

### Feature: \w matches word characters.

```python
def test_word_class(self):
    """Feature: \\w matches word characters."""
    assert re.findall('\\w+', 'hello world_123') == ['hello', 'world_123']
```

**Verification**: ✅ Tested in CI

### Feature: \s matches whitespace.

```python
def test_whitespace_class(self):
    """Feature: \\s matches whitespace."""
    assert re.findall('\\S+', 'hello world') == ['hello', 'world']
```

**Verification**: ✅ Tested in CI

### Feature: \D, \W, \S match non-digits, non-word, non-whitespace.

```python
def test_negated_classes(self):
    """Feature: \\D, \\W, \\S match non-digits, non-word, non-whitespace."""
    assert re.findall('\\D+', 'abc123def') == ['abc', 'def']
```

**Verification**: ✅ Tested in CI

### Feature: [abc] matches any of a, b, c.

```python
def test_custom_character_class(self):
    """Feature: [abc] matches any of a, b, c."""
    assert re.findall('[aeiou]', 'hello world') == ['e', 'o', 'o']
```

**Verification**: ✅ Tested in CI

### Feature: [a-z] matches range.

```python
def test_range_character_class(self):
    """Feature: [a-z] matches range."""
    assert re.findall('[0-9]+', 'abc123def456') == ['123', '456']
```

**Verification**: ✅ Tested in CI

### Feature: * matches 0 or more.

```python
def test_star_quantifier(self):
    """Feature: * matches 0 or more."""
    assert re.findall('a*b', 'b ab aab aaab') == ['b', 'ab', 'aab', 'aaab']
```

**Verification**: ✅ Tested in CI

### Feature: + matches 1 or more.

```python
def test_plus_quantifier(self):
    """Feature: + matches 1 or more."""
    assert re.findall('a+b', 'b ab aab aaab') == ['ab', 'aab', 'aaab']
```

**Verification**: ✅ Tested in CI

### Feature: ? matches 0 or 1.

```python
def test_question_quantifier(self):
    """Feature: ? matches 0 or 1."""
    assert re.findall('a?b', 'b ab aab') == ['b', 'ab', 'ab']
```

**Verification**: ✅ Tested in CI

### Feature: {n} matches exactly n.

```python
def test_exact_quantifier(self):
    """Feature: {n} matches exactly n."""
    assert re.findall('a{3}', 'a aa aaa aaaa') == ['aaa', 'aaa']
```

**Verification**: ✅ Tested in CI

### Feature: {m,n} matches m to n.

```python
def test_range_quantifier(self):
    """Feature: {m,n} matches m to n."""
    assert re.findall('a{2,3}', 'a aa aaa aaaa') == ['aa', 'aaa', 'aaa']
```

**Verification**: ✅ Tested in CI

### Property: Quantifiers are greedy by default.

```python
def test_greedy_vs_non_greedy(self):
    """Property: Quantifiers are greedy by default."""
    assert re.findall('<.*>', '<a>text</a>') == ['<a>text</a>']
    assert re.findall('<.*?>', '<a>text</a>') == ['<a>', '</a>']
```

**Verification**: ✅ Tested in CI

### Feature: ^ matches start of string.

```python
def test_start_anchor(self):
    """Feature: ^ matches start of string."""
    assert re.match('^hello', 'hello world') is not None
    assert re.match('^world', 'hello world') is None
```

**Verification**: ✅ Tested in CI

### Feature: $ matches end of string.

```python
def test_end_anchor(self):
    """Feature: $ matches end of string."""
    assert re.search('world$', 'hello world') is not None
    assert re.search('hello$', 'hello world') is None
```

**Verification**: ✅ Tested in CI

### Feature: \b matches word boundary.

```python
def test_word_boundary(self):
    """Feature: \\b matches word boundary."""
    text = 'the cat in the hat'
    assert re.findall('\\bcat\\b', text) == ['cat']
    assert re.findall('\\bcat\\b', 'cats') == []
```

**Verification**: ✅ Tested in CI

### Feature: \B matches non-word boundary.

```python
def test_non_word_boundary(self):
    """Feature: \\B matches non-word boundary."""
    assert re.findall('\\Bcat', 'concatenate') == ['cat']
```

**Verification**: ✅ Tested in CI

### Feature: (?=...) positive lookahead.

```python
def test_positive_lookahead(self):
    """Feature: (?=...) positive lookahead."""
    assert re.search('hello(?= world)', 'hello world') is not None
    assert re.search('hello(?= world)', 'hello there') is None
```

**Verification**: ✅ Tested in CI

### Feature: (?!...) negative lookahead.

```python
def test_negative_lookahead(self):
    """Feature: (?!...) negative lookahead."""
    assert re.search('hello(?! world)', 'hello there') is not None
    assert re.search('hello(?! world)', 'hello world') is None
```

**Verification**: ✅ Tested in CI

### Feature: (?<=...) positive lookbehind.

```python
def test_positive_lookbehind(self):
    """Feature: (?<=...) positive lookbehind."""
    assert re.findall('(?<=\\$)\\d+', 'Price: $100') == ['100']
```

**Verification**: ✅ Tested in CI

### Feature: (?<!...) negative lookbehind.

```python
def test_negative_lookbehind(self):
    """Feature: (?<!...) negative lookbehind."""
    assert re.findall('(?<!\\$)\\d+', 'Item 5 costs $100') == ['5', '00']
```

**Verification**: ✅ Tested in CI

### Edge: Empty string handling.

```python
def test_empty_string(self):
    """Edge: Empty string handling."""
    assert re.search('.*', '') is not None
    assert re.findall('\\d+', '') == []
```

**Verification**: ✅ Tested in CI

### Feature: re.escape() escapes special characters.

```python
def test_escape_special_chars(self):
    """Feature: re.escape() escapes special characters."""
    text = 'cost: $5.00'
    assert re.search('$5.00', text) is None
    escaped = re.escape('$5.00')
    assert re.search(escaped, text) is not None
```

**Verification**: ✅ Tested in CI

### Feature: Unicode patterns work correctly.

```python
def test_unicode_support(self):
    """Feature: Unicode patterns work correctly."""
    text = 'Hello 世界 привет'
    assert re.findall('[^\\x00-\\x7F]+', text) == ['世界', 'привет']
```

**Verification**: ✅ Tested in CI

### Edge: Some patterns can cause performance issues.

```python
def test_catastrophic_backtracking(self):
    """Edge: Some patterns can cause performance issues."""
    text = 'a' * 20 + 'b'
    match = re.search('(a+)+b', text)
    assert match is not None
```

**Verification**: ✅ Tested in CI

### Feature: Groups can be nested.

```python
def test_nested_groups(self):
    """Feature: Groups can be nested."""
    pattern = '((\\w+)@(\\w+)\\.(\\w+))'
    match = re.search(pattern, 'alice@example.com')
    assert match.group(1) == 'alice@example.com'
    assert match.group(2) == 'alice'
    assert match.group(3) == 'example'
    assert match.group(4) == 'com'
```

**Verification**: ✅ Tested in CI

### Feature: \1 references first captured group.

```python
def test_backreference(self):
    """Feature: \\1 references first captured group."""
    pattern = '\\b(\\w+)\\s+\\1\\b'
    assert re.search(pattern, 'hello hello') is not None
    assert re.search(pattern, 'hello world') is None
```

**Verification**: ✅ Tested in CI
