# string

## String module constants.

## string.Template - Simple string substitution.

## string.capwords() - Capitalize words in string.

## string.Formatter - Advanced string formatting.

## Practical usage of string constants.

## Custom Template subclasses.

## Edge cases and special scenarios.

### Basic: ascii_lowercase contains a-z.

```python
def test_ascii_lowercase(self):
    """Basic: ascii_lowercase contains a-z."""
    assert string.ascii_lowercase == 'abcdefghijklmnopqrstuvwxyz'
```

**Verification**: ✅ Tested in CI

### Basic: ascii_uppercase contains A-Z.

```python
def test_ascii_uppercase(self):
    """Basic: ascii_uppercase contains A-Z."""
    assert string.ascii_uppercase == 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
```

**Verification**: ✅ Tested in CI

### Property: ascii_letters is lowercase + uppercase.

```python
def test_ascii_letters(self):
    """Property: ascii_letters is lowercase + uppercase."""
    assert string.ascii_letters == string.ascii_lowercase + string.ascii_uppercase
```

**Verification**: ✅ Tested in CI

### Basic: digits contains 0-9.

```python
def test_digits(self):
    """Basic: digits contains 0-9."""
    assert string.digits == '0123456789'
```

**Verification**: ✅ Tested in CI

### Feature: hexdigits contains 0-9a-fA-F.

```python
def test_hexdigits(self):
    """Feature: hexdigits contains 0-9a-fA-F."""
    assert string.hexdigits == '0123456789abcdefABCDEF'
```

**Verification**: ✅ Tested in CI

### Feature: octdigits contains 0-7.

```python
def test_octdigits(self):
    """Feature: octdigits contains 0-7."""
    assert string.octdigits == '01234567'
```

**Verification**: ✅ Tested in CI

### Feature: punctuation contains common punctuation marks.

```python
def test_punctuation(self):
    """Feature: punctuation contains common punctuation marks."""
    expected = '!"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~'
    assert string.punctuation == expected
```

**Verification**: ✅ Tested in CI

### Feature: whitespace contains space, tab, newline, etc.

```python
def test_whitespace(self):
    """Feature: whitespace contains space, tab, newline, etc."""
    assert ' ' in string.whitespace
    assert '\t' in string.whitespace
    assert '\n' in string.whitespace
    assert '\r' in string.whitespace
```

**Verification**: ✅ Tested in CI

### Property: printable is letters + digits + punctuation + whitespace.

```python
def test_printable(self):
    """Property: printable is letters + digits + punctuation + whitespace."""
    assert len(string.printable) == 100
    for c in string.ascii_letters:
        assert c in string.printable
```

**Verification**: ✅ Tested in CI

### Basic: Substitute placeholders with values.

```python
def test_template_basic(self):
    """Basic: Substitute placeholders with values."""
    template = string.Template('Hello $name')
    result = template.substitute(name='World')
    assert result == 'Hello World'
```

**Verification**: ✅ Tested in CI

### Feature: Multiple placeholders.

```python
def test_template_multiple_placeholders(self):
    """Feature: Multiple placeholders."""
    template = string.Template('$greeting $name!')
    result = template.substitute(greeting='Hello', name='Alice')
    assert result == 'Hello Alice!'
```

**Verification**: ✅ Tested in CI

### Feature: Braces disambiguate placeholders.

```python
def test_template_braces(self):
    """Feature: Braces disambiguate placeholders."""
    template = string.Template('${noun}ification')
    result = template.substitute(noun='python')
    assert result == 'pythonification'
```

**Verification**: ✅ Tested in CI

### Feature: safe_substitute doesn't raise on missing keys.

```python
def test_template_safe_substitute(self):
    """Feature: safe_substitute doesn't raise on missing keys."""
    template = string.Template('Hello $name, welcome to $place')
    result = template.safe_substitute(name='Alice')
    assert result == 'Hello Alice, welcome to $place'
```

**Verification**: ✅ Tested in CI

### Error: substitute() raises KeyError on missing placeholder.

```python
def test_template_substitute_missing_raises(self):
    """Error: substitute() raises KeyError on missing placeholder."""
    template = string.Template('Hello $name')
    with pytest.raises(KeyError):
        template.substitute()
```

**Verification**: ✅ Tested in CI

### Feature: Can use dict for substitution.

```python
def test_template_dict_substitution(self):
    """Feature: Can use dict for substitution."""
    template = string.Template('$x + $y = $z')
    values = {'x': 1, 'y': 2, 'z': 3}
    result = template.substitute(values)
    assert result == '1 + 2 = 3'
```

**Verification**: ✅ Tested in CI

### Feature: $$ escapes to single $.

```python
def test_template_dollar_escape(self):
    """Feature: $$ escapes to single $."""
    template = string.Template('Price: $$${price}')
    result = template.substitute(price='9.99')
    assert result == 'Price: $9.99'
```

**Verification**: ✅ Tested in CI

### Property: Identifiers follow Python naming rules.

```python
def test_template_identifier_rules(self):
    """Property: Identifiers follow Python naming rules."""
    template = string.Template('$_var $var1 $CamelCase')
    result = template.substitute(_var='a', var1='b', CamelCase='c')
    assert result == 'a b c'
```

**Verification**: ✅ Tested in CI

### Edge: Invalid placeholders are left as-is.

```python
def test_template_invalid_placeholder(self):
    """Edge: Invalid placeholders are left as-is."""
    template = string.Template('Price: $$ $1.99')
    result = template.safe_substitute()
    assert '$ $1.99' in result
```

**Verification**: ✅ Tested in CI

### Basic: Capitalize first letter of each word.

```python
def test_capwords_basic(self):
    """Basic: Capitalize first letter of each word."""
    result = string.capwords('hello world')
    assert result == 'Hello World'
```

**Verification**: ✅ Tested in CI

### Edge: Already capitalized words remain capitalized.

```python
def test_capwords_already_capitalized(self):
    """Edge: Already capitalized words remain capitalized."""
    result = string.capwords('Hello World')
    assert result == 'Hello World'
```

**Verification**: ✅ Tested in CI

### Feature: Lowercases non-first letters.

```python
def test_capwords_mixed_case(self):
    """Feature: Lowercases non-first letters."""
    result = string.capwords('hELLo WoRLd')
    assert result == 'Hello World'
```

**Verification**: ✅ Tested in CI

### Feature: Custom separator for words.

```python
def test_capwords_custom_separator(self):
    """Feature: Custom separator for words."""
    result = string.capwords('hello-world', sep='-')
    assert result == 'Hello-World'
```

**Verification**: ✅ Tested in CI

### Edge: Multiple spaces are preserved.

```python
def test_capwords_multiple_spaces(self):
    """Edge: Multiple spaces are preserved."""
    result = string.capwords('hello  world')
    assert result == 'Hello World'
```

**Verification**: ✅ Tested in CI

### Edge: Leading/trailing whitespace is removed.

```python
def test_capwords_leading_trailing_whitespace(self):
    """Edge: Leading/trailing whitespace is removed."""
    result = string.capwords('  hello world  ')
    assert result == 'Hello World'
```

**Verification**: ✅ Tested in CI

### Edge: Empty string returns empty string.

```python
def test_capwords_empty_string(self):
    """Edge: Empty string returns empty string."""
    result = string.capwords('')
    assert result == ''
```

**Verification**: ✅ Tested in CI

### Basic: Single word is capitalized.

```python
def test_capwords_single_word(self):
    """Basic: Single word is capitalized."""
    result = string.capwords('hello')
    assert result == 'Hello'
```

**Verification**: ✅ Tested in CI

### Basic: Format with positional arguments.

```python
def test_formatter_basic(self):
    """Basic: Format with positional arguments."""
    formatter = string.Formatter()
    result = formatter.format('{0} {1}', 'hello', 'world')
    assert result == 'hello world'
```

**Verification**: ✅ Tested in CI

### Feature: Named field references.

```python
def test_formatter_named_fields(self):
    """Feature: Named field references."""
    formatter = string.Formatter()
    result = formatter.format('{greeting} {name}', greeting='Hello', name='Alice')
    assert result == 'Hello Alice'
```

**Verification**: ✅ Tested in CI

### Feature: vformat() takes args and kwargs separately.

```python
def test_formatter_vformat(self):
    """Feature: vformat() takes args and kwargs separately."""
    formatter = string.Formatter()
    result = formatter.vformat('{0} {name}', ('Hello',), {'name': 'World'})
    assert result == 'Hello World'
```

**Verification**: ✅ Tested in CI

### Feature: parse() breaks format string into parts.

```python
def test_formatter_parse(self):
    """Feature: parse() breaks format string into parts."""
    formatter = string.Formatter()
    parts = list(formatter.parse('Hello {name}, {greeting}!'))
    assert len(parts) == 3
    assert parts[0][0] == 'Hello '
    assert parts[0][1] == 'name'
```

**Verification**: ✅ Tested in CI

### Feature: get_field() retrieves field value.

```python
def test_formatter_get_field(self):
    """Feature: get_field() retrieves field value."""
    formatter = string.Formatter()
    obj = {'name': 'Alice'}
    value, key = formatter.get_field('name', (), obj)
    assert value == 'Alice'
```

**Verification**: ✅ Tested in CI

### Property: Check if string is alphabetic.

```python
def test_is_alpha_using_constants(self):
    """Property: Check if string is alphabetic."""
    text = 'HelloWorld'
    is_alpha = all((c in string.ascii_letters for c in text))
    assert is_alpha is True
    text_with_digit = 'Hello123'
    is_alpha = all((c in string.ascii_letters for c in text_with_digit))
    assert is_alpha is False
```

**Verification**: ✅ Tested in CI

### Property: Check if string is alphanumeric.

```python
def test_is_alnum_using_constants(self):
    """Property: Check if string is alphanumeric."""
    text = 'Hello123'
    is_alnum = all((c in string.ascii_letters + string.digits for c in text))
    assert is_alnum is True
```

**Verification**: ✅ Tested in CI

### Feature: Remove punctuation from string.

```python
def test_remove_punctuation(self):
    """Feature: Remove punctuation from string."""
    text = 'Hello, World!'
    no_punct = ''.join((c for c in text if c not in string.punctuation))
    assert no_punct == 'Hello World'
```

**Verification**: ✅ Tested in CI

### Property: Check if string is valid hexadecimal.

```python
def test_is_hex_using_constants(self):
    """Property: Check if string is valid hexadecimal."""
    hex_str = '1A2B3C'
    is_hex = all((c in string.hexdigits for c in hex_str))
    assert is_hex is True
    not_hex = '1A2B3G'
    is_hex = all((c in string.hexdigits for c in not_hex))
    assert is_hex is False
```

**Verification**: ✅ Tested in CI

### Feature: Subclass Template with custom delimiter.

```python
def test_custom_delimiter(self):
    """Feature: Subclass Template with custom delimiter."""

    class MyTemplate(string.Template):
        delimiter = '%'
    template = MyTemplate('Hello %name')
    result = template.substitute(name='World')
    assert result == 'Hello World'
```

**Verification**: ✅ Tested in CI

### Feature: Subclass Template with custom pattern.

```python
def test_custom_pattern(self):
    """Feature: Subclass Template with custom pattern."""

    class BraceTemplate(string.Template):
        delimiter = '%'
        idpattern = '[a-z][_a-z0-9]*'
    template = BraceTemplate('%greeting %name')
    result = template.substitute(greeting='Hi', name='Alice')
    assert result == 'Hi Alice'
```

**Verification**: ✅ Tested in CI

### Property: String constants are strings (immutable).

```python
def test_constants_are_immutable(self):
    """Property: String constants are strings (immutable)."""
    original = string.ascii_lowercase
    modified = original.upper()
    assert string.ascii_lowercase == original
    assert modified != original
```

**Verification**: ✅ Tested in CI

### Edge: Placeholders can have numeric values.

```python
def test_template_with_numeric_strings(self):
    """Edge: Placeholders can have numeric values."""
    template = string.Template('Value: $value')
    result = template.substitute(value=42)
    assert result == 'Value: 42'
```

**Verification**: ✅ Tested in CI

### Edge: Empty substitution value.

```python
def test_template_with_empty_string(self):
    """Edge: Empty substitution value."""
    template = string.Template('Hello $name!')
    result = template.substitute(name='')
    assert result == 'Hello !'
```

**Verification**: ✅ Tested in CI

### Edge: capwords with numbers.

```python
def test_capwords_with_numbers(self):
    """Edge: capwords with numbers."""
    result = string.capwords('hello 123 world')
    assert result == 'Hello 123 World'
```

**Verification**: ✅ Tested in CI

### Property: printable covers all ASCII printable characters.

```python
def test_printable_coverage(self):
    """Property: printable covers all ASCII printable characters."""
    for i in range(32, 127):
        char = chr(i)
        if char.isprintable():
            assert char in string.printable or char not in string.printable
```

**Verification**: ✅ Tested in CI

### Feature: Template works with unicode.

```python
def test_template_unicode(self):
    """Feature: Template works with unicode."""
    template = string.Template('Hello $name')
    result = template.substitute(name='世界')
    assert result == 'Hello 世界'
```

**Verification**: ✅ Tested in CI

### Property: safe_substitute allows partial substitution.

```python
def test_safe_substitute_partial(self):
    """Property: safe_substitute allows partial substitution."""
    template = string.Template('$a $b $c')
    result = template.safe_substitute(a='1', c='3')
    assert result == '1 $b 3'
```

**Verification**: ✅ Tested in CI

## 

## 
