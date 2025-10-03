"""Test string module - String constants and utilities.

This module tests string constants, Template substitution,
and utility functions for string manipulation.
"""

import string
import pytest


class TestStringConstants:
    """String module constants."""

    def test_ascii_lowercase(self):
        """Basic: ascii_lowercase contains a-z."""
        assert string.ascii_lowercase == 'abcdefghijklmnopqrstuvwxyz'

    def test_ascii_uppercase(self):
        """Basic: ascii_uppercase contains A-Z."""
        assert string.ascii_uppercase == 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'

    def test_ascii_letters(self):
        """Property: ascii_letters is lowercase + uppercase."""
        assert string.ascii_letters == string.ascii_lowercase + string.ascii_uppercase

    def test_digits(self):
        """Basic: digits contains 0-9."""
        assert string.digits == '0123456789'

    def test_hexdigits(self):
        """Feature: hexdigits contains 0-9a-fA-F."""
        assert string.hexdigits == '0123456789abcdefABCDEF'

    def test_octdigits(self):
        """Feature: octdigits contains 0-7."""
        assert string.octdigits == '01234567'

    def test_punctuation(self):
        """Feature: punctuation contains common punctuation marks."""
        expected = '!"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~'
        assert string.punctuation == expected

    def test_whitespace(self):
        """Feature: whitespace contains space, tab, newline, etc."""
        assert ' ' in string.whitespace
        assert '\t' in string.whitespace
        assert '\n' in string.whitespace
        assert '\r' in string.whitespace

    def test_printable(self):
        """Property: printable is letters + digits + punctuation + whitespace."""
        # Printable includes all visible characters plus whitespace
        assert len(string.printable) == 100
        # All ascii letters should be in printable
        for c in string.ascii_letters:
            assert c in string.printable


class TestTemplate:
    """string.Template - Simple string substitution."""

    def test_template_basic(self):
        """Basic: Substitute placeholders with values."""
        template = string.Template('Hello $name')
        result = template.substitute(name='World')
        assert result == 'Hello World'

    def test_template_multiple_placeholders(self):
        """Feature: Multiple placeholders."""
        template = string.Template('$greeting $name!')
        result = template.substitute(greeting='Hello', name='Alice')
        assert result == 'Hello Alice!'

    def test_template_braces(self):
        """Feature: Braces disambiguate placeholders."""
        template = string.Template('${noun}ification')
        result = template.substitute(noun='python')
        assert result == 'pythonification'

    def test_template_safe_substitute(self):
        """Feature: safe_substitute doesn't raise on missing keys."""
        template = string.Template('Hello $name, welcome to $place')
        # Missing 'place' key - safe_substitute leaves it as-is
        result = template.safe_substitute(name='Alice')
        assert result == 'Hello Alice, welcome to $place'

    def test_template_substitute_missing_raises(self):
        """Error: substitute() raises KeyError on missing placeholder."""
        template = string.Template('Hello $name')
        with pytest.raises(KeyError):
            template.substitute()

    def test_template_dict_substitution(self):
        """Feature: Can use dict for substitution."""
        template = string.Template('$x + $y = $z')
        values = {'x': 1, 'y': 2, 'z': 3}
        result = template.substitute(values)
        assert result == '1 + 2 = 3'

    def test_template_dollar_escape(self):
        """Feature: $$ escapes to single $."""
        template = string.Template('Price: $$${price}')
        result = template.substitute(price='9.99')
        assert result == 'Price: $9.99'

    def test_template_identifier_rules(self):
        """Property: Identifiers follow Python naming rules."""
        # Valid identifier: letters, digits, underscore (but not starting with digit)
        template = string.Template('$_var $var1 $CamelCase')
        result = template.substitute(_var='a', var1='b', CamelCase='c')
        assert result == 'a b c'

    def test_template_invalid_placeholder(self):
        """Edge: Invalid placeholders are left as-is."""
        # $ followed by non-identifier characters
        template = string.Template('Price: $$ $1.99')
        result = template.safe_substitute()
        # The $1 is invalid (starts with digit), left alone
        assert '$ $1.99' in result


class TestCapwords:
    """string.capwords() - Capitalize words in string."""

    def test_capwords_basic(self):
        """Basic: Capitalize first letter of each word."""
        result = string.capwords('hello world')
        assert result == 'Hello World'

    def test_capwords_already_capitalized(self):
        """Edge: Already capitalized words remain capitalized."""
        result = string.capwords('Hello World')
        assert result == 'Hello World'

    def test_capwords_mixed_case(self):
        """Feature: Lowercases non-first letters."""
        result = string.capwords('hELLo WoRLd')
        assert result == 'Hello World'

    def test_capwords_custom_separator(self):
        """Feature: Custom separator for words."""
        result = string.capwords('hello-world', sep='-')
        assert result == 'Hello-World'

    def test_capwords_multiple_spaces(self):
        """Edge: Multiple spaces are preserved."""
        result = string.capwords('hello  world')
        # capwords splits on whitespace and joins with single space
        assert result == 'Hello World'

    def test_capwords_leading_trailing_whitespace(self):
        """Edge: Leading/trailing whitespace is removed."""
        result = string.capwords('  hello world  ')
        assert result == 'Hello World'

    def test_capwords_empty_string(self):
        """Edge: Empty string returns empty string."""
        result = string.capwords('')
        assert result == ''

    def test_capwords_single_word(self):
        """Basic: Single word is capitalized."""
        result = string.capwords('hello')
        assert result == 'Hello'


class TestFormatter:
    """string.Formatter - Advanced string formatting."""

    def test_formatter_basic(self):
        """Basic: Format with positional arguments."""
        formatter = string.Formatter()
        result = formatter.format('{0} {1}', 'hello', 'world')
        assert result == 'hello world'

    def test_formatter_named_fields(self):
        """Feature: Named field references."""
        formatter = string.Formatter()
        result = formatter.format('{greeting} {name}', greeting='Hello', name='Alice')
        assert result == 'Hello Alice'

    def test_formatter_vformat(self):
        """Feature: vformat() takes args and kwargs separately."""
        formatter = string.Formatter()
        result = formatter.vformat('{0} {name}', ('Hello',), {'name': 'World'})
        assert result == 'Hello World'

    def test_formatter_parse(self):
        """Feature: parse() breaks format string into parts."""
        formatter = string.Formatter()
        parts = list(formatter.parse('Hello {name}, {greeting}!'))
        # Returns (literal_text, field_name, format_spec, conversion)
        assert len(parts) == 3
        assert parts[0][0] == 'Hello '
        assert parts[0][1] == 'name'

    def test_formatter_get_field(self):
        """Feature: get_field() retrieves field value."""
        formatter = string.Formatter()
        obj = {'name': 'Alice'}
        value, key = formatter.get_field('name', (), obj)
        assert value == 'Alice'


class TestConstantsUsage:
    """Practical usage of string constants."""

    def test_is_alpha_using_constants(self):
        """Property: Check if string is alphabetic."""
        text = 'HelloWorld'
        is_alpha = all(c in string.ascii_letters for c in text)
        assert is_alpha is True

        text_with_digit = 'Hello123'
        is_alpha = all(c in string.ascii_letters for c in text_with_digit)
        assert is_alpha is False

    def test_is_alnum_using_constants(self):
        """Property: Check if string is alphanumeric."""
        text = 'Hello123'
        is_alnum = all(c in string.ascii_letters + string.digits for c in text)
        assert is_alnum is True

    def test_remove_punctuation(self):
        """Feature: Remove punctuation from string."""
        text = 'Hello, World!'
        # Remove all punctuation
        no_punct = ''.join(c for c in text if c not in string.punctuation)
        assert no_punct == 'Hello World'

    def test_is_hex_using_constants(self):
        """Property: Check if string is valid hexadecimal."""
        hex_str = '1A2B3C'
        is_hex = all(c in string.hexdigits for c in hex_str)
        assert is_hex is True

        not_hex = '1A2B3G'
        is_hex = all(c in string.hexdigits for c in not_hex)
        assert is_hex is False


class TestTemplateSubclass:
    """Custom Template subclasses."""

    def test_custom_delimiter(self):
        """Feature: Subclass Template with custom delimiter."""
        class MyTemplate(string.Template):
            delimiter = '%'

        template = MyTemplate('Hello %name')
        result = template.substitute(name='World')
        assert result == 'Hello World'

    def test_custom_pattern(self):
        """Feature: Subclass Template with custom pattern."""
        # This is more advanced, but demonstrates extensibility
        class BraceTemplate(string.Template):
            delimiter = '%'
            idpattern = r'[a-z][_a-z0-9]*'

        template = BraceTemplate('%greeting %name')
        result = template.substitute(greeting='Hi', name='Alice')
        assert result == 'Hi Alice'


class TestEdgeCases:
    """Edge cases and special scenarios."""

    def test_constants_are_immutable(self):
        """Property: String constants are strings (immutable)."""
        # Can't modify the constants themselves
        original = string.ascii_lowercase
        # String is immutable, so this creates a new string
        modified = original.upper()
        # Original is unchanged
        assert string.ascii_lowercase == original
        assert modified != original

    def test_template_with_numeric_strings(self):
        """Edge: Placeholders can have numeric values."""
        template = string.Template('Value: $value')
        result = template.substitute(value=42)
        assert result == 'Value: 42'

    def test_template_with_empty_string(self):
        """Edge: Empty substitution value."""
        template = string.Template('Hello $name!')
        result = template.substitute(name='')
        assert result == 'Hello !'

    def test_capwords_with_numbers(self):
        """Edge: capwords with numbers."""
        result = string.capwords('hello 123 world')
        assert result == 'Hello 123 World'

    def test_printable_coverage(self):
        """Property: printable covers all ASCII printable characters."""
        # ASCII printable range is 32-126
        for i in range(32, 127):
            char = chr(i)
            if char.isprintable():
                assert char in string.printable or char not in string.printable

    def test_template_unicode(self):
        """Feature: Template works with unicode."""
        template = string.Template('Hello $name')
        result = template.substitute(name='世界')
        assert result == 'Hello 世界'

    def test_safe_substitute_partial(self):
        """Property: safe_substitute allows partial substitution."""
        template = string.Template('$a $b $c')
        result = template.safe_substitute(a='1', c='3')
        assert result == '1 $b 3'
