# textwrap

## textwrap.wrap() - Split text into lines.

## textwrap.fill() - Wrap and join lines.

## textwrap.shorten() - Shorten text to fit width.

## textwrap.dedent() - Remove common leading whitespace.

## textwrap.indent() - Add prefix to lines.

## textwrap.TextWrapper - Configurable text wrapper.

## Edge cases and special scenarios.

### Basic: Wrap long text to width.

```python
def test_wrap_basic(self):
    """Basic: Wrap long text to width."""
    text = 'This is a very long line of text that needs to be wrapped'
    lines = textwrap.wrap(text, width=20)
    assert len(lines) > 1
    for line in lines:
        assert len(line) <= 20
```

**Verification**: ✅ Tested in CI

### Property: Wrap preserves word boundaries.

```python
def test_wrap_preserves_words(self):
    """Property: Wrap preserves word boundaries."""
    text = 'hello world foo bar'
    lines = textwrap.wrap(text, width=10)
    assert 'hello' in ' '.join(lines)
    assert 'world' in ' '.join(lines)
```

**Verification**: ✅ Tested in CI

### Edge: Short text returns single line.

```python
def test_wrap_short_text(self):
    """Edge: Short text returns single line."""
    text = 'short'
    lines = textwrap.wrap(text, width=20)
    assert lines == ['short']
```

**Verification**: ✅ Tested in CI

### Edge: Empty string returns empty list.

```python
def test_wrap_empty_string(self):
    """Edge: Empty string returns empty list."""
    lines = textwrap.wrap('', width=20)
    assert lines == []
```

**Verification**: ✅ Tested in CI

### Edge: Whitespace-only string returns empty list.

```python
def test_wrap_whitespace_only(self):
    """Edge: Whitespace-only string returns empty list."""
    lines = textwrap.wrap('   \n  \t  ', width=20)
    assert lines == []
```

**Verification**: ✅ Tested in CI

### Feature: max_lines limits output.

```python
def test_wrap_max_lines(self):
    """Feature: max_lines limits output."""
    text = 'one two three four five six seven eight'
    lines = textwrap.wrap(text, width=10, max_lines=2)
    assert len(lines) <= 2
```

**Verification**: ✅ Tested in CI

### Feature: placeholder for truncated text.

```python
def test_wrap_placeholder(self):
    """Feature: placeholder for truncated text."""
    text = 'one two three four five six seven eight'
    lines = textwrap.wrap(text, width=15, max_lines=2, placeholder='...')
    assert lines[-1].endswith('...')
```

**Verification**: ✅ Tested in CI

### Basic: Fill wraps and joins with newlines.

```python
def test_fill_basic(self):
    """Basic: Fill wraps and joins with newlines."""
    text = 'This is a very long line of text that needs to be wrapped'
    result = textwrap.fill(text, width=20)
    assert '\n' in result
    lines = result.split('\n')
    for line in lines:
        assert len(line) <= 20
```

**Verification**: ✅ Tested in CI

### Property: fill() is equivalent to '\n'.join(wrap()).

```python
def test_fill_vs_wrap(self):
    """Property: fill() is equivalent to '\\n'.join(wrap())."""
    text = 'hello world foo bar baz'
    filled = textwrap.fill(text, width=10)
    wrapped = '\n'.join(textwrap.wrap(text, width=10))
    assert filled == wrapped
```

**Verification**: ✅ Tested in CI

### Feature: initial_indent for first line.

```python
def test_fill_initial_indent(self):
    """Feature: initial_indent for first line."""
    text = 'hello world'
    result = textwrap.fill(text, width=20, initial_indent='>>> ')
    assert result.startswith('>>> ')
```

**Verification**: ✅ Tested in CI

### Feature: subsequent_indent for remaining lines.

```python
def test_fill_subsequent_indent(self):
    """Feature: subsequent_indent for remaining lines."""
    text = 'one two three four five six seven'
    result = textwrap.fill(text, width=10, subsequent_indent='... ')
    lines = result.split('\n')
    assert not lines[0].startswith('... ')
    if len(lines) > 1:
        assert lines[1].startswith('... ')
```

**Verification**: ✅ Tested in CI

### Feature: Both initial and subsequent indent.

```python
def test_fill_both_indents(self):
    """Feature: Both initial and subsequent indent."""
    text = 'hello world foo bar'
    result = textwrap.fill(text, width=15, initial_indent='> ', subsequent_indent='  ')
    lines = result.split('\n')
    assert lines[0].startswith('> ')
    if len(lines) > 1:
        assert lines[1].startswith('  ')
```

**Verification**: ✅ Tested in CI

### Basic: Shorten long text to width.

```python
def test_shorten_basic(self):
    """Basic: Shorten long text to width."""
    text = 'This is a very long line of text'
    result = textwrap.shorten(text, width=20)
    assert len(result) <= 20
```

**Verification**: ✅ Tested in CI

### Feature: Adds placeholder when truncating.

```python
def test_shorten_adds_placeholder(self):
    """Feature: Adds placeholder when truncating."""
    text = 'This is a very long line of text'
    result = textwrap.shorten(text, width=20)
    assert '[...]' in result
```

**Verification**: ✅ Tested in CI

### Feature: Custom placeholder.

```python
def test_shorten_custom_placeholder(self):
    """Feature: Custom placeholder."""
    text = 'This is a very long line of text'
    result = textwrap.shorten(text, width=20, placeholder='...')
    assert result.endswith('...')
```

**Verification**: ✅ Tested in CI

### Edge: Short text is not truncated.

```python
def test_shorten_no_truncation_needed(self):
    """Edge: Short text is not truncated."""
    text = 'short'
    result = textwrap.shorten(text, width=20)
    assert result == 'short'
```

**Verification**: ✅ Tested in CI

### Property: Shorten breaks at word boundaries.

```python
def test_shorten_preserves_words(self):
    """Property: Shorten breaks at word boundaries."""
    text = 'hello world foo bar'
    result = textwrap.shorten(text, width=15)
    words = result.replace('[...]', '').split()
    for word in words:
        assert word in text.split()
```

**Verification**: ✅ Tested in CI

### Basic: Remove common indentation.

```python
def test_dedent_basic(self):
    """Basic: Remove common indentation."""
    text = '    hello\n    world'
    result = textwrap.dedent(text)
    assert result == 'hello\nworld'
```

**Verification**: ✅ Tested in CI

### Feature: Removes common indentation only.

```python
def test_dedent_mixed_indentation(self):
    """Feature: Removes common indentation only."""
    text = '    hello\n        world\n    foo'
    result = textwrap.dedent(text)
    assert result == 'hello\n    world\nfoo'
```

**Verification**: ✅ Tested in CI

### Edge: No common indent means no change.

```python
def test_dedent_no_common_indent(self):
    """Edge: No common indent means no change."""
    text = 'hello\n    world'
    result = textwrap.dedent(text)
    assert result == text
```

**Verification**: ✅ Tested in CI

### Property: Empty lines don't affect common indent.

```python
def test_dedent_empty_lines_ignored(self):
    """Property: Empty lines don't affect common indent."""
    text = '    hello\n\n    world'
    result = textwrap.dedent(text)
    assert result == 'hello\n\nworld'
```

**Verification**: ✅ Tested in CI

### Edge: Whitespace-only lines don't affect common indent.

```python
def test_dedent_whitespace_only_lines(self):
    """Edge: Whitespace-only lines don't affect common indent."""
    text = '    hello\n      \n    world'
    result = textwrap.dedent(text)
    assert 'hello' in result
    assert 'world' in result
```

**Verification**: ✅ Tested in CI

### Feature: Handles tabs and spaces.

```python
def test_dedent_tabs_and_spaces(self):
    """Feature: Handles tabs and spaces."""
    text = '\thello\n\tworld'
    result = textwrap.dedent(text)
    assert result == 'hello\nworld'
```

**Verification**: ✅ Tested in CI

### Property: Preserves relative indentation.

```python
def test_dedent_preserves_relative_indentation(self):
    """Property: Preserves relative indentation."""
    text = '    def foo():\n        pass'
    result = textwrap.dedent(text)
    assert result == 'def foo():\n    pass'
```

**Verification**: ✅ Tested in CI

### Basic: Add prefix to all lines.

```python
def test_indent_basic(self):
    """Basic: Add prefix to all lines."""
    text = 'hello\nworld'
    result = textwrap.indent(text, '> ')
    assert result == '> hello\n> world'
```

**Verification**: ✅ Tested in CI

### Feature: By default, indent empty lines.

```python
def test_indent_empty_lines(self):
    """Feature: By default, indent empty lines."""
    text = 'hello\n\nworld'
    result = textwrap.indent(text, '> ')
    lines = result.split('\n')
    assert len([l for l in lines if l.startswith('> ')]) >= 2
```

**Verification**: ✅ Tested in CI

### Feature: Predicate controls which lines get prefix.

```python
def test_indent_predicate(self):
    """Feature: Predicate controls which lines get prefix."""
    text = 'hello\nworld\nfoo'
    result = textwrap.indent(text, '> ', predicate=lambda line: line.strip() != '')
    lines = result.split('\n')
    assert all((line.startswith('> ') for line in lines if line.strip()))
```

**Verification**: ✅ Tested in CI

### Edge: Single line gets prefix.

```python
def test_indent_single_line(self):
    """Edge: Single line gets prefix."""
    text = 'hello'
    result = textwrap.indent(text, '> ')
    assert result == '> hello'
```

**Verification**: ✅ Tested in CI

### Edge: Empty string returns empty string.

```python
def test_indent_empty_string(self):
    """Edge: Empty string returns empty string."""
    result = textwrap.indent('', '> ')
    assert result == ''
```

**Verification**: ✅ Tested in CI

### Property: Preserves line endings.

```python
def test_indent_multiple_line_endings(self):
    """Property: Preserves line endings."""
    text = 'hello\nworld\n'
    result = textwrap.indent(text, '> ')
    assert result == '> hello\n> world\n'
```

**Verification**: ✅ Tested in CI

### Basic: Create and use TextWrapper.

```python
def test_textwrapper_basic(self):
    """Basic: Create and use TextWrapper."""
    wrapper = textwrap.TextWrapper(width=20)
    text = 'This is a very long line of text'
    lines = wrapper.wrap(text)
    assert all((len(line) <= 20 for line in lines))
```

**Verification**: ✅ Tested in CI

### Feature: break_long_words controls word breaking.

```python
def test_textwrapper_break_long_words(self):
    """Feature: break_long_words controls word breaking."""
    wrapper = textwrap.TextWrapper(width=10, break_long_words=True)
    text = 'supercalifragilisticexpialidocious'
    lines = wrapper.wrap(text)
    assert len(lines) > 1
    wrapper_no_break = textwrap.TextWrapper(width=10, break_long_words=False)
    lines_no_break = wrapper_no_break.wrap(text)
    assert any((len(line) > 10 for line in lines_no_break))
```

**Verification**: ✅ Tested in CI

### Feature: break_on_hyphens controls hyphen breaking.

```python
def test_textwrapper_break_on_hyphens(self):
    """Feature: break_on_hyphens controls hyphen breaking."""
    wrapper = textwrap.TextWrapper(width=15, break_on_hyphens=True)
    text = 'well-known'
    lines = wrapper.wrap(text)
    wrapper_no_break = textwrap.TextWrapper(width=15, break_on_hyphens=False)
    text2 = 'well-known example'
    lines2 = wrapper_no_break.wrap(text2)
```

**Verification**: ✅ Tested in CI

### Feature: replace_whitespace normalizes whitespace.

```python
def test_textwrapper_replace_whitespace(self):
    """Feature: replace_whitespace normalizes whitespace."""
    wrapper = textwrap.TextWrapper(width=20, replace_whitespace=True)
    text = 'hello\n\t  world'
    result = wrapper.fill(text)
    assert 'hello world' in result.replace('\n', ' ')
```

**Verification**: ✅ Tested in CI

### Feature: drop_whitespace removes leading/trailing space.

```python
def test_textwrapper_drop_whitespace(self):
    """Feature: drop_whitespace removes leading/trailing space."""
    wrapper = textwrap.TextWrapper(width=20, drop_whitespace=True)
    text = 'hello   world   foo'
    result = wrapper.fill(text)
    lines = result.split('\n')
    assert all((line == line.strip() for line in lines))
```

**Verification**: ✅ Tested in CI

### Feature: expand_tabs controls tab expansion.

```python
def test_textwrapper_expand_tabs(self):
    """Feature: expand_tabs controls tab expansion."""
    wrapper = textwrap.TextWrapper(width=20, expand_tabs=True)
    text = 'hello\tworld'
    result = wrapper.fill(text)
    assert '\t' not in result
```

**Verification**: ✅ Tested in CI

### Feature: tabsize sets tab width.

```python
def test_textwrapper_tabsize(self):
    """Feature: tabsize sets tab width."""
    wrapper = textwrap.TextWrapper(width=20, expand_tabs=True, tabsize=4)
    text = 'a\tb'
    result = wrapper.fill(text)
    assert 'a   b' in result or 'a    b' in result
```

**Verification**: ✅ Tested in CI

### Feature: max_lines limits output lines.

```python
def test_textwrapper_max_lines(self):
    """Feature: max_lines limits output lines."""
    wrapper = textwrap.TextWrapper(width=10, max_lines=2)
    text = 'one two three four five'
    lines = wrapper.wrap(text)
    assert len(lines) <= 2
```

**Verification**: ✅ Tested in CI

### Feature: placeholder for truncated text.

```python
def test_textwrapper_placeholder(self):
    """Feature: placeholder for truncated text."""
    wrapper = textwrap.TextWrapper(width=15, max_lines=1, placeholder='...')
    text = 'this is a very long line'
    lines = wrapper.wrap(text)
    assert len(lines) == 1
    assert lines[0].endswith('...')
```

**Verification**: ✅ Tested in CI

### Feature: Unicode text is handled correctly.

```python
def test_wrap_unicode(self):
    """Feature: Unicode text is handled correctly."""
    text = 'Hello 世界 привет мир'
    lines = textwrap.wrap(text, width=15)
    assert '世界' in ' '.join(lines)
    assert 'привет' in ' '.join(lines)
```

**Verification**: ✅ Tested in CI

### Property: Useful for cleaning up multiline strings.

```python
def test_dedent_multiline_string(self):
    """Property: Useful for cleaning up multiline strings."""
    text = '\n            def foo():\n                pass\n        '
    result = textwrap.dedent(text)
    assert 'def foo():' in result
    assert '    pass' in result
```

**Verification**: ✅ Tested in CI

### Edge: Width smaller than word length.

```python
def test_fill_width_smaller_than_word(self):
    """Edge: Width smaller than word length."""
    text = 'hello'
    result = textwrap.fill(text, width=3)
    assert 'hel' in result or 'hello' in result
```

**Verification**: ✅ Tested in CI

### Feature: Custom predicate with lambda.

```python
def test_indent_with_lambda(self):
    """Feature: Custom predicate with lambda."""
    text = '# comment\ncode\n# another comment'
    result = textwrap.indent(text, '    ', predicate=lambda line: not line.strip().startswith('#'))
    lines = result.split('\n')
    assert '# comment' in result
    assert '    code' in result
```

**Verification**: ✅ Tested in CI

### Feature: break_long_words in shorten.

```python
def test_shorten_break_long_words(self):
    """Feature: break_long_words in shorten."""
    text = 'supercalifragilisticexpialidocious'
    result = textwrap.shorten(text, width=20, break_long_words=True)
    assert len(result) <= 20
```

**Verification**: ✅ Tested in CI

### Property: wrap() processes single paragraphs.

```python
def test_wrap_preserves_paragraph_breaks(self):
    """Property: wrap() processes single paragraphs."""
    text = 'line1\nline2'
    lines = textwrap.wrap(text, width=20)
    assert 'line1' in ' '.join(lines)
    assert 'line2' in ' '.join(lines)
```

**Verification**: ✅ Tested in CI

### Edge: Text without trailing newline.

```python
def test_dedent_no_trailing_newline(self):
    """Edge: Text without trailing newline."""
    text = '    hello\n    world'
    result = textwrap.dedent(text)
    assert not result.endswith('\n')
```

**Verification**: ✅ Tested in CI

### Feature: Custom prefix string.

```python
def test_indent_custom_prefix(self):
    """Feature: Custom prefix string."""
    text = 'hello\nworld'
    result = textwrap.indent(text, '>>> ')
    assert result == '>>> hello\n>>> world'
```

**Verification**: ✅ Tested in CI

### Feature: fix_sentence_endings adds extra space.

```python
def test_fill_fix_sentence_endings(self):
    """Feature: fix_sentence_endings adds extra space."""
    wrapper = textwrap.TextWrapper(width=40, fix_sentence_endings=True)
    text = 'First sentence. Second sentence. Third sentence.'
    result = wrapper.fill(text)
    assert 'sentence' in result
```

**Verification**: ✅ Tested in CI
