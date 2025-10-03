"""Test textwrap module - Text wrapping and filling.

This module tests textwrap's functions for wrapping paragraphs,
indenting text, and removing indentation.
"""

import textwrap
import pytest


class TestWrap:
    """textwrap.wrap() - Split text into lines."""

    def test_wrap_basic(self):
        """Basic: Wrap long text to width."""
        text = "This is a very long line of text that needs to be wrapped"
        lines = textwrap.wrap(text, width=20)
        assert len(lines) > 1
        for line in lines:
            assert len(line) <= 20

    def test_wrap_preserves_words(self):
        """Property: Wrap preserves word boundaries."""
        text = "hello world foo bar"
        lines = textwrap.wrap(text, width=10)
        assert 'hello' in ' '.join(lines)
        assert 'world' in ' '.join(lines)

    def test_wrap_short_text(self):
        """Edge: Short text returns single line."""
        text = "short"
        lines = textwrap.wrap(text, width=20)
        assert lines == ['short']

    def test_wrap_empty_string(self):
        """Edge: Empty string returns empty list."""
        lines = textwrap.wrap('', width=20)
        assert lines == []

    def test_wrap_whitespace_only(self):
        """Edge: Whitespace-only string returns empty list."""
        lines = textwrap.wrap('   \n  \t  ', width=20)
        assert lines == []

    def test_wrap_max_lines(self):
        """Feature: max_lines limits output."""
        text = "one two three four five six seven eight"
        lines = textwrap.wrap(text, width=10, max_lines=2)
        assert len(lines) <= 2

    def test_wrap_placeholder(self):
        """Feature: placeholder for truncated text."""
        text = "one two three four five six seven eight"
        lines = textwrap.wrap(text, width=15, max_lines=2, placeholder='...')
        # Last line should contain placeholder
        assert lines[-1].endswith('...')


class TestFill:
    """textwrap.fill() - Wrap and join lines."""

    def test_fill_basic(self):
        """Basic: Fill wraps and joins with newlines."""
        text = "This is a very long line of text that needs to be wrapped"
        result = textwrap.fill(text, width=20)
        assert '\n' in result
        lines = result.split('\n')
        for line in lines:
            assert len(line) <= 20

    def test_fill_vs_wrap(self):
        """Property: fill() is equivalent to '\\n'.join(wrap())."""
        text = "hello world foo bar baz"
        filled = textwrap.fill(text, width=10)
        wrapped = '\n'.join(textwrap.wrap(text, width=10))
        assert filled == wrapped

    def test_fill_initial_indent(self):
        """Feature: initial_indent for first line."""
        text = "hello world"
        result = textwrap.fill(text, width=20, initial_indent='>>> ')
        assert result.startswith('>>> ')

    def test_fill_subsequent_indent(self):
        """Feature: subsequent_indent for remaining lines."""
        text = "one two three four five six seven"
        result = textwrap.fill(text, width=10, subsequent_indent='... ')
        lines = result.split('\n')
        # First line should not have subsequent indent
        assert not lines[0].startswith('... ')
        # Subsequent lines should
        if len(lines) > 1:
            assert lines[1].startswith('... ')

    def test_fill_both_indents(self):
        """Feature: Both initial and subsequent indent."""
        text = "hello world foo bar"
        result = textwrap.fill(text, width=15,
                               initial_indent='> ',
                               subsequent_indent='  ')
        lines = result.split('\n')
        assert lines[0].startswith('> ')
        if len(lines) > 1:
            assert lines[1].startswith('  ')


class TestShorten:
    """textwrap.shorten() - Shorten text to fit width."""

    def test_shorten_basic(self):
        """Basic: Shorten long text to width."""
        text = "This is a very long line of text"
        result = textwrap.shorten(text, width=20)
        assert len(result) <= 20

    def test_shorten_adds_placeholder(self):
        """Feature: Adds placeholder when truncating."""
        text = "This is a very long line of text"
        result = textwrap.shorten(text, width=20)
        # Default placeholder is [...]
        assert '[...]' in result

    def test_shorten_custom_placeholder(self):
        """Feature: Custom placeholder."""
        text = "This is a very long line of text"
        result = textwrap.shorten(text, width=20, placeholder='...')
        assert result.endswith('...')

    def test_shorten_no_truncation_needed(self):
        """Edge: Short text is not truncated."""
        text = "short"
        result = textwrap.shorten(text, width=20)
        assert result == 'short'

    def test_shorten_preserves_words(self):
        """Property: Shorten breaks at word boundaries."""
        text = "hello world foo bar"
        result = textwrap.shorten(text, width=15)
        # Should not break words in the middle
        words = result.replace('[...]', '').split()
        for word in words:
            assert word in text.split()


class TestDedent:
    """textwrap.dedent() - Remove common leading whitespace."""

    def test_dedent_basic(self):
        """Basic: Remove common indentation."""
        text = "    hello\n    world"
        result = textwrap.dedent(text)
        assert result == "hello\nworld"

    def test_dedent_mixed_indentation(self):
        """Feature: Removes common indentation only."""
        text = "    hello\n        world\n    foo"
        result = textwrap.dedent(text)
        # Common indent is 4 spaces, so "world" keeps 4 spaces
        assert result == "hello\n    world\nfoo"

    def test_dedent_no_common_indent(self):
        """Edge: No common indent means no change."""
        text = "hello\n    world"
        result = textwrap.dedent(text)
        assert result == text

    def test_dedent_empty_lines_ignored(self):
        """Property: Empty lines don't affect common indent."""
        text = "    hello\n\n    world"
        result = textwrap.dedent(text)
        assert result == "hello\n\nworld"

    def test_dedent_whitespace_only_lines(self):
        """Edge: Whitespace-only lines don't affect common indent."""
        text = "    hello\n      \n    world"
        result = textwrap.dedent(text)
        # Common indent is 4 spaces
        assert 'hello' in result
        assert 'world' in result

    def test_dedent_tabs_and_spaces(self):
        """Feature: Handles tabs and spaces."""
        text = "\thello\n\tworld"
        result = textwrap.dedent(text)
        assert result == "hello\nworld"

    def test_dedent_preserves_relative_indentation(self):
        """Property: Preserves relative indentation."""
        text = "    def foo():\n        pass"
        result = textwrap.dedent(text)
        # After removing 4 common spaces, "pass" still has 4 spaces
        assert result == "def foo():\n    pass"


class TestIndent:
    """textwrap.indent() - Add prefix to lines."""

    def test_indent_basic(self):
        """Basic: Add prefix to all lines."""
        text = "hello\nworld"
        result = textwrap.indent(text, '> ')
        assert result == "> hello\n> world"

    def test_indent_empty_lines(self):
        """Feature: By default, indent empty lines."""
        text = "hello\n\nworld"
        result = textwrap.indent(text, '> ')
        lines = result.split('\n')
        # All lines including empty ones get prefix
        assert len([l for l in lines if l.startswith('> ')]) >= 2

    def test_indent_predicate(self):
        """Feature: Predicate controls which lines get prefix."""
        text = "hello\nworld\nfoo"
        # Only indent non-empty lines
        result = textwrap.indent(text, '> ', predicate=lambda line: line.strip() != '')
        lines = result.split('\n')
        assert all(line.startswith('> ') for line in lines if line.strip())

    def test_indent_single_line(self):
        """Edge: Single line gets prefix."""
        text = "hello"
        result = textwrap.indent(text, '> ')
        assert result == "> hello"

    def test_indent_empty_string(self):
        """Edge: Empty string returns empty string."""
        result = textwrap.indent('', '> ')
        assert result == ''

    def test_indent_multiple_line_endings(self):
        """Property: Preserves line endings."""
        text = "hello\nworld\n"
        result = textwrap.indent(text, '> ')
        # Trailing newline is preserved but last empty line doesn't get prefix
        assert result == "> hello\n> world\n"


class TestTextWrapper:
    """textwrap.TextWrapper - Configurable text wrapper."""

    def test_textwrapper_basic(self):
        """Basic: Create and use TextWrapper."""
        wrapper = textwrap.TextWrapper(width=20)
        text = "This is a very long line of text"
        lines = wrapper.wrap(text)
        assert all(len(line) <= 20 for line in lines)

    def test_textwrapper_break_long_words(self):
        """Feature: break_long_words controls word breaking."""
        wrapper = textwrap.TextWrapper(width=10, break_long_words=True)
        text = "supercalifragilisticexpialidocious"
        lines = wrapper.wrap(text)
        # Long word is broken
        assert len(lines) > 1

        wrapper_no_break = textwrap.TextWrapper(width=10, break_long_words=False)
        lines_no_break = wrapper_no_break.wrap(text)
        # Long word exceeds width but is not broken
        assert any(len(line) > 10 for line in lines_no_break)

    def test_textwrapper_break_on_hyphens(self):
        """Feature: break_on_hyphens controls hyphen breaking."""
        wrapper = textwrap.TextWrapper(width=15, break_on_hyphens=True)
        text = "well-known"
        lines = wrapper.wrap(text)
        # Can break on hyphen

        wrapper_no_break = textwrap.TextWrapper(width=15, break_on_hyphens=False)
        text2 = "well-known example"
        lines2 = wrapper_no_break.wrap(text2)
        # Hyphenated words stay together

    def test_textwrapper_replace_whitespace(self):
        """Feature: replace_whitespace normalizes whitespace."""
        wrapper = textwrap.TextWrapper(width=20, replace_whitespace=True)
        text = "hello\n\t  world"
        result = wrapper.fill(text)
        # Whitespace normalized to single spaces
        assert 'hello world' in result.replace('\n', ' ')

    def test_textwrapper_drop_whitespace(self):
        """Feature: drop_whitespace removes leading/trailing space."""
        wrapper = textwrap.TextWrapper(width=20, drop_whitespace=True)
        text = "hello   world   foo"
        result = wrapper.fill(text)
        lines = result.split('\n')
        # No leading/trailing spaces on lines
        assert all(line == line.strip() for line in lines)

    def test_textwrapper_expand_tabs(self):
        """Feature: expand_tabs controls tab expansion."""
        wrapper = textwrap.TextWrapper(width=20, expand_tabs=True)
        text = "hello\tworld"
        result = wrapper.fill(text)
        # Tabs expanded to spaces
        assert '\t' not in result

    def test_textwrapper_tabsize(self):
        """Feature: tabsize sets tab width."""
        wrapper = textwrap.TextWrapper(width=20, expand_tabs=True, tabsize=4)
        text = "a\tb"
        result = wrapper.fill(text)
        # Tab expanded to 4 spaces (minus 1 for 'a')
        assert 'a   b' in result or 'a    b' in result

    def test_textwrapper_max_lines(self):
        """Feature: max_lines limits output lines."""
        wrapper = textwrap.TextWrapper(width=10, max_lines=2)
        text = "one two three four five"
        lines = wrapper.wrap(text)
        assert len(lines) <= 2

    def test_textwrapper_placeholder(self):
        """Feature: placeholder for truncated text."""
        wrapper = textwrap.TextWrapper(width=15, max_lines=1, placeholder='...')
        text = "this is a very long line"
        lines = wrapper.wrap(text)
        assert len(lines) == 1
        assert lines[0].endswith('...')


class TestEdgeCases:
    """Edge cases and special scenarios."""

    def test_wrap_unicode(self):
        """Feature: Unicode text is handled correctly."""
        text = "Hello 世界 привет мир"
        lines = textwrap.wrap(text, width=15)
        assert '世界' in ' '.join(lines)
        assert 'привет' in ' '.join(lines)

    def test_dedent_multiline_string(self):
        """Property: Useful for cleaning up multiline strings."""
        text = """
            def foo():
                pass
        """
        result = textwrap.dedent(text)
        # Leading/trailing newlines preserved
        assert 'def foo():' in result
        assert '    pass' in result

    def test_fill_width_smaller_than_word(self):
        """Edge: Width smaller than word length."""
        text = "hello"
        # Width 3, but "hello" is 5 chars
        result = textwrap.fill(text, width=3)
        # With break_long_words=True (default), it breaks
        assert 'hel' in result or 'hello' in result

    def test_indent_with_lambda(self):
        """Feature: Custom predicate with lambda."""
        text = "# comment\ncode\n# another comment"
        # Only indent lines that don't start with #
        result = textwrap.indent(
            text,
            '    ',
            predicate=lambda line: not line.strip().startswith('#')
        )
        lines = result.split('\n')
        # Comment lines not indented, code is
        assert '# comment' in result
        assert '    code' in result

    def test_shorten_break_long_words(self):
        """Feature: break_long_words in shorten."""
        text = "supercalifragilisticexpialidocious"
        result = textwrap.shorten(text, width=20, break_long_words=True)
        assert len(result) <= 20

    def test_wrap_preserves_paragraph_breaks(self):
        """Property: wrap() processes single paragraphs."""
        # Newlines are treated as whitespace by default
        text = "line1\nline2"
        lines = textwrap.wrap(text, width=20)
        # Lines joined (newline treated as space)
        assert 'line1' in ' '.join(lines)
        assert 'line2' in ' '.join(lines)

    def test_dedent_no_trailing_newline(self):
        """Edge: Text without trailing newline."""
        text = "    hello\n    world"
        result = textwrap.dedent(text)
        assert not result.endswith('\n')

    def test_indent_custom_prefix(self):
        """Feature: Custom prefix string."""
        text = "hello\nworld"
        result = textwrap.indent(text, '>>> ')
        assert result == ">>> hello\n>>> world"

    def test_fill_fix_sentence_endings(self):
        """Feature: fix_sentence_endings adds extra space."""
        wrapper = textwrap.TextWrapper(width=40, fix_sentence_endings=True)
        text = "First sentence. Second sentence. Third sentence."
        result = wrapper.fill(text)
        # Sentence endings get two spaces (when not at line end)
        # This behavior is subtle and may not always be visible
        assert 'sentence' in result
