"""Test re module - Regular expression operations.

This module tests re's functions for pattern matching, searching,
replacing, and text manipulation using regular expressions.
"""

import re
import pytest


class TestCompile:
    """re.compile() - Compile regular expression pattern."""

    def test_compile_basic(self):
        """Basic: Compile simple pattern."""
        pattern = re.compile(r'hello')
        assert pattern.pattern == 'hello'

    def test_compile_with_flags(self):
        """Feature: Compile with flags."""
        pattern = re.compile(r'hello', re.IGNORECASE)
        assert pattern.flags & re.IGNORECASE

    def test_compile_reuse(self):
        """Property: Compiled patterns can be reused."""
        pattern = re.compile(r'\d+')
        assert pattern.search('abc123')
        assert pattern.search('xyz789')

    def test_compile_invalid_pattern_raises(self):
        """Error: Invalid pattern raises re.error."""
        with pytest.raises(re.error):
            re.compile(r'(?P<')  # Unclosed group


class TestSearch:
    """re.search() - Search for pattern anywhere in string."""

    def test_search_basic(self):
        """Basic: Search finds pattern in string."""
        match = re.search(r'world', 'hello world')
        assert match is not None
        assert match.group() == 'world'

    def test_search_no_match_returns_none(self):
        """Basic: No match returns None."""
        match = re.search(r'xyz', 'hello world')
        assert match is None

    def test_search_finds_first_occurrence(self):
        """Feature: Search finds first occurrence."""
        match = re.search(r'\d+', 'abc123def456')
        assert match.group() == '123'

    def test_search_position(self):
        """Property: Match object has position info."""
        match = re.search(r'world', 'hello world')
        assert match.start() == 6
        assert match.end() == 11
        assert match.span() == (6, 11)


class TestMatch:
    """re.match() - Match pattern at beginning of string."""

    def test_match_basic(self):
        """Basic: Match at start of string."""
        match = re.match(r'hello', 'hello world')
        assert match is not None
        assert match.group() == 'hello'

    def test_match_middle_fails(self):
        """Feature: Match requires pattern at start."""
        match = re.match(r'world', 'hello world')
        assert match is None

    def test_match_vs_search(self):
        """Property: match() anchors at start, search() doesn't."""
        text = 'abc123'
        assert re.match(r'\d+', text) is None
        assert re.search(r'\d+', text) is not None


class TestFullmatch:
    """re.fullmatch() - Match entire string."""

    def test_fullmatch_basic(self):
        """Basic: Fullmatch requires complete match."""
        match = re.fullmatch(r'hello', 'hello')
        assert match is not None

    def test_fullmatch_partial_fails(self):
        """Feature: Partial match fails."""
        match = re.fullmatch(r'hello', 'hello world')
        assert match is None

    def test_fullmatch_use_case(self):
        """Feature: Useful for validation."""
        # Validate phone number format
        pattern = r'\d{3}-\d{3}-\d{4}'
        assert re.fullmatch(pattern, '555-123-4567') is not None
        assert re.fullmatch(pattern, '555-123-4567x') is None


class TestFindall:
    """re.findall() - Find all non-overlapping matches."""

    def test_findall_basic(self):
        """Basic: Find all occurrences."""
        matches = re.findall(r'\d+', 'abc123def456ghi789')
        assert matches == ['123', '456', '789']

    def test_findall_no_matches(self):
        """Edge: No matches returns empty list."""
        matches = re.findall(r'\d+', 'abcdef')
        assert matches == []

    def test_findall_with_groups(self):
        """Feature: Groups affect returned values."""
        matches = re.findall(r'(\w+)@(\w+)', 'alice@example bob@test')
        assert matches == [('alice', 'example'), ('bob', 'test')]

    def test_findall_non_overlapping(self):
        """Property: Matches are non-overlapping."""
        matches = re.findall(r'.{2}', 'abcdef')
        assert matches == ['ab', 'cd', 'ef']


class TestFinditer:
    """re.finditer() - Iterator over match objects."""

    def test_finditer_basic(self):
        """Basic: Iterate over matches."""
        matches = list(re.finditer(r'\d+', 'abc123def456'))
        assert len(matches) == 2
        assert matches[0].group() == '123'
        assert matches[1].group() == '456'

    def test_finditer_match_objects(self):
        """Feature: Returns full match objects."""
        matches = list(re.finditer(r'\d+', 'abc123def456'))
        assert matches[0].start() == 3
        assert matches[0].end() == 6
        assert matches[1].start() == 9
        assert matches[1].end() == 12

    def test_finditer_memory_efficient(self):
        """Property: Iterator is memory-efficient."""
        # Can iterate without loading all into memory
        iterator = re.finditer(r'\d+', 'a1b2c3d4e5')
        first = next(iterator)
        assert first.group() == '1'


class TestSub:
    """re.sub() - Replace pattern matches."""

    def test_sub_basic(self):
        """Basic: Replace pattern with string."""
        result = re.sub(r'\d+', 'X', 'abc123def456')
        assert result == 'abcXdefX'

    def test_sub_count(self):
        """Feature: Limit number of replacements."""
        result = re.sub(r'\d+', 'X', 'abc123def456', count=1)
        assert result == 'abcXdef456'

    def test_sub_with_function(self):
        """Feature: Replacement can be a function."""
        def double(match):
            return str(int(match.group()) * 2)

        result = re.sub(r'\d+', double, 'abc5def10')
        assert result == 'abc10def20'

    def test_sub_backreferences(self):
        """Feature: Use backreferences in replacement."""
        result = re.sub(r'(\w+) (\w+)', r'\2 \1', 'hello world')
        assert result == 'world hello'


class TestSubn:
    """re.subn() - Replace and return count."""

    def test_subn_basic(self):
        """Basic: Returns tuple (new_string, count)."""
        result, count = re.subn(r'\d+', 'X', 'abc123def456')
        assert result == 'abcXdefX'
        assert count == 2

    def test_subn_no_matches(self):
        """Edge: No matches returns count of 0."""
        result, count = re.subn(r'\d+', 'X', 'abcdef')
        assert result == 'abcdef'
        assert count == 0


class TestSplit:
    """re.split() - Split string by pattern."""

    def test_split_basic(self):
        """Basic: Split by pattern."""
        parts = re.split(r'\s+', 'hello world  from   python')
        assert parts == ['hello', 'world', 'from', 'python']

    def test_split_maxsplit(self):
        """Feature: Limit number of splits."""
        parts = re.split(r'\s+', 'a b c d', maxsplit=2)
        assert parts == ['a', 'b', 'c d']

    def test_split_with_groups(self):
        """Feature: Capturing groups are included in result."""
        parts = re.split(r'(\s+)', 'hello world')
        assert parts == ['hello', ' ', 'world']

    def test_split_empty_matches(self):
        """Edge: Empty matches at edges."""
        parts = re.split(r'\d+', 'a1b2c')
        assert parts == ['a', 'b', 'c']


class TestGroups:
    """Pattern groups and capturing."""

    def test_groups_basic(self):
        """Basic: Access captured groups."""
        match = re.search(r'(\w+)@(\w+)', 'alice@example')
        assert match.group(0) == 'alice@example'
        assert match.group(1) == 'alice'
        assert match.group(2) == 'example'

    def test_groups_tuple(self):
        """Feature: groups() returns all groups as tuple."""
        match = re.search(r'(\w+)@(\w+)', 'alice@example')
        assert match.groups() == ('alice', 'example')

    def test_named_groups(self):
        """Feature: Named groups with ?P<name>."""
        match = re.search(r'(?P<user>\w+)@(?P<domain>\w+)', 'alice@example')
        assert match.group('user') == 'alice'
        assert match.group('domain') == 'example'

    def test_groupdict(self):
        """Feature: groupdict() returns named groups as dict."""
        match = re.search(r'(?P<user>\w+)@(?P<domain>\w+)', 'alice@example')
        assert match.groupdict() == {'user': 'alice', 'domain': 'example'}

    def test_non_capturing_group(self):
        """Feature: (?:...) is non-capturing."""
        match = re.search(r'(?:\w+)@(\w+)', 'alice@example')
        assert match.groups() == ('example',)


class TestFlags:
    """Regular expression flags."""

    def test_ignorecase_flag(self):
        """Feature: IGNORECASE makes pattern case-insensitive."""
        pattern = re.compile(r'hello', re.IGNORECASE)
        assert pattern.search('HELLO') is not None
        assert pattern.search('Hello') is not None

    def test_multiline_flag(self):
        """Feature: MULTILINE makes ^ and $ match line boundaries."""
        text = 'first line\nsecond line'
        # Without MULTILINE, ^ only matches start of string
        assert len(re.findall(r'^second', text)) == 0
        # With MULTILINE, ^ matches start of any line
        assert len(re.findall(r'^second', text, re.MULTILINE)) == 1

    def test_dotall_flag(self):
        """Feature: DOTALL makes . match newlines."""
        text = 'hello\nworld'
        # Without DOTALL, . doesn't match newline
        assert re.search(r'hello.world', text) is None
        # With DOTALL, . matches newline
        assert re.search(r'hello.world', text, re.DOTALL) is not None

    def test_verbose_flag(self):
        """Feature: VERBOSE allows whitespace and comments in pattern."""
        pattern = re.compile(r'''
            (\d{3})  # Area code
            -        # Separator
            (\d{4})  # Number
        ''', re.VERBOSE)
        match = pattern.search('555-1234')
        assert match.groups() == ('555', '1234')

    def test_combined_flags(self):
        """Feature: Multiple flags can be combined."""
        pattern = re.compile(r'hello.world', re.IGNORECASE | re.DOTALL)
        assert pattern.search('HELLO\nWORLD') is not None


class TestCharacterClasses:
    """Special character classes and escapes."""

    def test_digit_class(self):
        """Feature: \\d matches digits."""
        assert re.findall(r'\d', 'abc123def') == ['1', '2', '3']

    def test_word_class(self):
        """Feature: \\w matches word characters."""
        assert re.findall(r'\w+', 'hello world_123') == ['hello', 'world_123']

    def test_whitespace_class(self):
        """Feature: \\s matches whitespace."""
        assert re.findall(r'\S+', 'hello world') == ['hello', 'world']

    def test_negated_classes(self):
        """Feature: \\D, \\W, \\S match non-digits, non-word, non-whitespace."""
        assert re.findall(r'\D+', 'abc123def') == ['abc', 'def']

    def test_custom_character_class(self):
        """Feature: [abc] matches any of a, b, c."""
        assert re.findall(r'[aeiou]', 'hello world') == ['e', 'o', 'o']

    def test_range_character_class(self):
        """Feature: [a-z] matches range."""
        assert re.findall(r'[0-9]+', 'abc123def456') == ['123', '456']


class TestQuantifiers:
    """Pattern quantifiers (* + ? {m,n})."""

    def test_star_quantifier(self):
        """Feature: * matches 0 or more."""
        assert re.findall(r'a*b', 'b ab aab aaab') == ['b', 'ab', 'aab', 'aaab']

    def test_plus_quantifier(self):
        """Feature: + matches 1 or more."""
        assert re.findall(r'a+b', 'b ab aab aaab') == ['ab', 'aab', 'aaab']

    def test_question_quantifier(self):
        """Feature: ? matches 0 or 1."""
        assert re.findall(r'a?b', 'b ab aab') == ['b', 'ab', 'ab']

    def test_exact_quantifier(self):
        """Feature: {n} matches exactly n."""
        assert re.findall(r'a{3}', 'a aa aaa aaaa') == ['aaa', 'aaa']

    def test_range_quantifier(self):
        """Feature: {m,n} matches m to n."""
        assert re.findall(r'a{2,3}', 'a aa aaa aaaa') == ['aa', 'aaa', 'aaa']

    def test_greedy_vs_non_greedy(self):
        """Property: Quantifiers are greedy by default."""
        # Greedy: matches as much as possible
        assert re.findall(r'<.*>', '<a>text</a>') == ['<a>text</a>']
        # Non-greedy: matches as little as possible
        assert re.findall(r'<.*?>', '<a>text</a>') == ['<a>', '</a>']


class TestAnchors:
    """Pattern anchors (^ $ \\b \\B)."""

    def test_start_anchor(self):
        """Feature: ^ matches start of string."""
        assert re.match(r'^hello', 'hello world') is not None
        assert re.match(r'^world', 'hello world') is None

    def test_end_anchor(self):
        """Feature: $ matches end of string."""
        assert re.search(r'world$', 'hello world') is not None
        assert re.search(r'hello$', 'hello world') is None

    def test_word_boundary(self):
        """Feature: \\b matches word boundary."""
        text = 'the cat in the hat'
        assert re.findall(r'\bcat\b', text) == ['cat']
        assert re.findall(r'\bcat\b', 'cats') == []

    def test_non_word_boundary(self):
        """Feature: \\B matches non-word boundary."""
        assert re.findall(r'\Bcat', 'concatenate') == ['cat']


class TestLookaround:
    """Lookahead and lookbehind assertions."""

    def test_positive_lookahead(self):
        """Feature: (?=...) positive lookahead."""
        # Match 'hello' only if followed by 'world'
        assert re.search(r'hello(?= world)', 'hello world') is not None
        assert re.search(r'hello(?= world)', 'hello there') is None

    def test_negative_lookahead(self):
        """Feature: (?!...) negative lookahead."""
        # Match 'hello' only if NOT followed by 'world'
        assert re.search(r'hello(?! world)', 'hello there') is not None
        assert re.search(r'hello(?! world)', 'hello world') is None

    def test_positive_lookbehind(self):
        """Feature: (?<=...) positive lookbehind."""
        # Match number preceded by $
        assert re.findall(r'(?<=\$)\d+', 'Price: $100') == ['100']

    def test_negative_lookbehind(self):
        """Feature: (?<!...) negative lookbehind."""
        # Match number NOT preceded by $
        assert re.findall(r'(?<!\$)\d+', 'Item 5 costs $100') == ['5', '00']


class TestEdgeCases:
    """Edge cases and special scenarios."""

    def test_empty_string(self):
        """Edge: Empty string handling."""
        assert re.search(r'.*', '') is not None
        assert re.findall(r'\d+', '') == []

    def test_escape_special_chars(self):
        """Feature: re.escape() escapes special characters."""
        text = 'cost: $5.00'
        # Without escape, . matches any character
        assert re.search(r'$5.00', text) is None
        # With escape, special chars are literal
        escaped = re.escape('$5.00')
        assert re.search(escaped, text) is not None

    def test_unicode_support(self):
        """Feature: Unicode patterns work correctly."""
        text = 'Hello 世界 привет'
        # Match non-ASCII characters
        assert re.findall(r'[^\x00-\x7F]+', text) == ['世界', 'привет']

    def test_catastrophic_backtracking(self):
        """Edge: Some patterns can cause performance issues."""
        # This is a well-known pathological case
        # Just verify it completes (might be slow)
        text = 'a' * 20 + 'b'
        match = re.search(r'(a+)+b', text)
        assert match is not None

    def test_nested_groups(self):
        """Feature: Groups can be nested."""
        pattern = r'((\w+)@(\w+)\.(\w+))'
        match = re.search(pattern, 'alice@example.com')
        assert match.group(1) == 'alice@example.com'
        assert match.group(2) == 'alice'
        assert match.group(3) == 'example'
        assert match.group(4) == 'com'

    def test_backreference(self):
        """Feature: \\1 references first captured group."""
        # Match repeated words
        pattern = r'\b(\w+)\s+\1\b'
        assert re.search(pattern, 'hello hello') is not None
        assert re.search(pattern, 'hello world') is None
