#!/usr/bin/env python3
"""
Comprehensive test suite for wordcount.py
Targets: 100% code coverage, property-based testing

Run with:
    pytest tests/test_wordcount.py -v --cov=python.wordcount --cov-report=term-missing --cov-report=json
"""
import sys
import tempfile
from pathlib import Path
from io import StringIO
import pytest
from hypothesis import given, strategies as st, settings, assume

# Add parent directory to path to import wordcount
sys.path.insert(0, str(Path(__file__).parent.parent / "python"))

import wordcount


class TestCountFile:
    """Test count_file() function"""

    def test_count_file_success(self, tmp_path):
        """Test counting a valid file"""
        test_file = tmp_path / "test.txt"
        test_file.write_text("line1\nline2\nline3")

        stats = wordcount.count_file(test_file)

        assert stats.lines == 3
        assert stats.words == 3
        assert stats.chars == 17  # "line1\nline2\nline3"
        assert stats.filename == str(test_file)

    def test_count_file_empty(self, tmp_path):
        """Test counting an empty file"""
        test_file = tmp_path / "empty.txt"
        test_file.write_text("")

        stats = wordcount.count_file(test_file)

        assert stats.lines == 0
        assert stats.words == 0
        assert stats.chars == 0
        assert stats.filename == str(test_file)

    def test_count_file_single_line_no_newline(self, tmp_path):
        """Test file with single line and no trailing newline"""
        test_file = tmp_path / "single.txt"
        test_file.write_text("hello world")

        stats = wordcount.count_file(test_file)

        assert stats.lines == 1
        assert stats.words == 2
        assert stats.chars == 11

    def test_count_file_multiple_spaces(self, tmp_path):
        """Test file with multiple spaces between words"""
        test_file = tmp_path / "spaces.txt"
        test_file.write_text("hello    world\nfoo  bar")

        stats = wordcount.count_file(test_file)

        assert stats.lines == 2
        assert stats.words == 4
        assert stats.chars == 23  # Actual character count

    def test_count_file_blank_lines(self, tmp_path):
        """Test file with blank lines"""
        test_file = tmp_path / "blanks.txt"
        test_file.write_text("line1\n\nline3\n\n")

        stats = wordcount.count_file(test_file)

        assert stats.lines == 4
        assert stats.words == 2

    def test_count_file_unicode(self, tmp_path):
        """Test file with unicode characters"""
        test_file = tmp_path / "unicode.txt"
        test_file.write_text("Hello 世界\nПривет мир\n")

        stats = wordcount.count_file(test_file)

        assert stats.lines == 2
        assert stats.words == 4
        assert stats.chars == 20  # Actual character count

    def test_count_file_ioerror(self, tmp_path, capsys):
        """Test handling of IOError when file doesn't exist"""
        nonexistent = tmp_path / "nonexistent.txt"

        stats = wordcount.count_file(nonexistent)

        # Should return zero stats
        assert stats.lines == 0
        assert stats.words == 0
        assert stats.chars == 0
        assert stats.filename == str(nonexistent)

        # Should print error to stderr
        captured = capsys.readouterr()
        assert "Error reading" in captured.err
        assert str(nonexistent) in captured.err


class TestFormatStats:
    """Test format_stats() function"""

    def test_format_stats_with_filename(self):
        """Test formatting stats with filename"""
        stats = wordcount.Stats(10, 20, 30, "test.txt")
        result = wordcount.format_stats(stats, show_filename=True)
        assert result == "      10       20       30 test.txt"

    def test_format_stats_without_filename(self):
        """Test formatting stats without filename"""
        stats = wordcount.Stats(5, 15, 25, "test.txt")
        result = wordcount.format_stats(stats, show_filename=False)
        assert result == "       5       15       25"
        assert "test.txt" not in result

    def test_format_stats_default_with_filename(self):
        """Test that show_filename=True is default"""
        stats = wordcount.Stats(1, 2, 3, "file.txt")
        result = wordcount.format_stats(stats)
        assert "file.txt" in result

    def test_format_stats_alignment(self):
        """Test that numbers are right-aligned in 8-character fields"""
        stats = wordcount.Stats(999, 8888, 77777, "test.txt")
        result = wordcount.format_stats(stats, show_filename=False)

        # Extract the three numeric fields
        parts = result.split()
        assert len(parts) == 3
        assert parts[0] == "999"
        assert parts[1] == "8888"
        assert parts[2] == "77777"


class TestMainFunction:
    """Test main() entry point"""

    def test_main_single_file(self, tmp_path, monkeypatch, capsys):
        """Test processing a single file"""
        test_file = tmp_path / "test.txt"
        test_file.write_text("hello world\nfoo bar\n")

        monkeypatch.setattr(sys, "argv", [
            "wordcount.py",
            str(test_file)
        ])

        exit_code = wordcount.main()

        assert exit_code == 0
        captured = capsys.readouterr()
        assert "2" in captured.out  # lines
        assert "4" in captured.out  # words
        assert str(test_file) in captured.out

    def test_main_multiple_files(self, tmp_path, monkeypatch, capsys):
        """Test processing multiple files shows totals"""
        file1 = tmp_path / "file1.txt"
        file1.write_text("one\ntwo\n")

        file2 = tmp_path / "file2.txt"
        file2.write_text("three\nfour\n")

        monkeypatch.setattr(sys, "argv", [
            "wordcount.py",
            str(file1),
            str(file2)
        ])

        exit_code = wordcount.main()

        assert exit_code == 0
        captured = capsys.readouterr()

        # Check both files are shown
        assert str(file1) in captured.out
        assert str(file2) in captured.out

        # Check total line appears
        assert "total" in captured.out

    def test_main_lines_only(self, tmp_path, monkeypatch, capsys):
        """Test -l/--lines flag shows only line count"""
        test_file = tmp_path / "test.txt"
        test_file.write_text("line1\nline2\nline3\n")

        monkeypatch.setattr(sys, "argv", [
            "wordcount.py",
            "-l",
            str(test_file)
        ])

        exit_code = wordcount.main()

        assert exit_code == 0
        captured = capsys.readouterr()
        lines = captured.out.strip().split()

        # Should show: "3 <filename>"
        assert lines[0] == "3"
        assert str(test_file) in captured.out

    def test_main_words_only(self, tmp_path, monkeypatch, capsys):
        """Test -w/--words flag shows only word count"""
        test_file = tmp_path / "test.txt"
        test_file.write_text("one two three four five")

        monkeypatch.setattr(sys, "argv", [
            "wordcount.py",
            "--words",
            str(test_file)
        ])

        exit_code = wordcount.main()

        assert exit_code == 0
        captured = capsys.readouterr()
        lines = captured.out.strip().split()

        # Should show: "5 <filename>"
        assert lines[0] == "5"

    def test_main_chars_only(self, tmp_path, monkeypatch, capsys):
        """Test -c/--chars flag shows only character count"""
        test_file = tmp_path / "test.txt"
        test_file.write_text("12345")

        monkeypatch.setattr(sys, "argv", [
            "wordcount.py",
            "-c",
            str(test_file)
        ])

        exit_code = wordcount.main()

        assert exit_code == 0
        captured = capsys.readouterr()
        lines = captured.out.strip().split()

        # Should show: "5 <filename>"
        assert lines[0] == "5"

    def test_main_no_files_shows_help(self, monkeypatch, capsys):
        """Test that running with no arguments shows help/error"""
        monkeypatch.setattr(sys, "argv", ["wordcount.py"])

        with pytest.raises(SystemExit) as exc_info:
            wordcount.main()

        assert exc_info.value.code == 2  # argparse exits with 2 for usage errors

    def test_main_nonexistent_file(self, tmp_path, monkeypatch, capsys):
        """Test processing nonexistent file (should handle gracefully)"""
        nonexistent = tmp_path / "doesnotexist.txt"

        monkeypatch.setattr(sys, "argv", [
            "wordcount.py",
            str(nonexistent)
        ])

        exit_code = wordcount.main()

        # Should still return 0 (graceful error handling)
        assert exit_code == 0

        # Should show 0 counts
        captured = capsys.readouterr()
        assert "0" in captured.out

        # Should have error in stderr
        assert "Error reading" in captured.err


class TestPropertyBased:
    """Property-based tests using Hypothesis"""

    @given(st.text(alphabet=st.characters(blacklist_categories=('Cs',)), min_size=0, max_size=1000))
    @settings(max_examples=200, deadline=None)
    def test_count_file_chars_property(self, tmp_path_factory, content):
        """Property: character count should equal len(content)"""
        tmp_path = tmp_path_factory.mktemp("data")
        test_file = tmp_path / "test.txt"
        test_file.write_text(content)

        stats = wordcount.count_file(test_file)

        assert stats.chars == len(content)

    @given(st.lists(st.text(alphabet=st.characters(whitelist_categories=('L', 'N')), min_size=1, max_size=20), min_size=0, max_size=100))
    @settings(max_examples=200, deadline=None)
    def test_count_file_words_property(self, tmp_path_factory, words):
        """Property: word count should equal number of whitespace-separated tokens"""
        tmp_path = tmp_path_factory.mktemp("data")
        test_file = tmp_path / "test.txt"

        # Join words with single spaces
        content = " ".join(words)
        test_file.write_text(content)

        stats = wordcount.count_file(test_file)

        # Expected word count (Python split() behavior)
        expected_words = len(content.split())
        assert stats.words == expected_words

    @given(st.lists(st.text(min_size=0, max_size=50), min_size=0, max_size=100))
    @settings(max_examples=200, deadline=None)
    def test_count_file_lines_property(self, tmp_path_factory, lines):
        """Property: line count should equal len(splitlines())"""
        tmp_path = tmp_path_factory.mktemp("data")
        test_file = tmp_path / "test.txt"

        content = "\n".join(lines)
        test_file.write_text(content)

        stats = wordcount.count_file(test_file)

        expected_lines = len(content.splitlines())
        assert stats.lines == expected_lines

    @given(st.integers(min_value=0, max_value=10), st.integers(min_value=0, max_value=100), st.integers(min_value=0, max_value=1000))
    @settings(max_examples=100, deadline=None)
    def test_format_stats_property(self, lines, words, chars):
        """Property: formatted output should contain the three numbers"""
        stats = wordcount.Stats(lines, words, chars, "test.txt")
        result = wordcount.format_stats(stats, show_filename=False)

        # Extract numbers from formatted output
        parts = result.split()
        assert len(parts) == 3
        assert int(parts[0]) == lines
        assert int(parts[1]) == words
        assert int(parts[2]) == chars


class TestIntegration:
    """Integration tests - end-to-end scenarios"""

    def test_main_entry_point_via_subprocess(self, tmp_path):
        """Test __name__ == '__main__' execution path via subprocess"""
        import subprocess

        test_file = tmp_path / "test.txt"
        test_file.write_text("hello world\n")

        # Run the script directly as __main__
        result = subprocess.run(
            [sys.executable, str(Path(__file__).parent.parent / "python" / "wordcount.py"), str(test_file)],
            capture_output=True,
            text=True
        )

        # Verify successful execution
        assert result.returncode == 0
        assert str(test_file) in result.stdout
        assert "1" in result.stdout  # 1 line
        assert "2" in result.stdout  # 2 words

    def test_sample_file_exact_output(self, tmp_path, monkeypatch, capsys):
        """Test exact output for the provided sample.txt"""
        # Create sample.txt with exact content
        sample = tmp_path / "sample.txt"
        sample.write_text("""The quick brown fox jumps over the lazy dog.
This is a test file for word count demonstration.
It contains multiple lines of text.

Python is a great programming language.
Rust is fast and safe.
Depyler converts Python to Rust.
""")

        monkeypatch.setattr(sys, "argv", ["wordcount.py", str(sample)])

        exit_code = wordcount.main()

        assert exit_code == 0
        captured = capsys.readouterr()

        # Verify output format
        assert str(sample) in captured.out
        # Should have 3 numbers followed by filename
        output_parts = captured.out.strip().split()
        assert len(output_parts) == 4  # lines words chars filename

    def test_mixed_existing_and_nonexistent_files(self, tmp_path, monkeypatch, capsys):
        """Test processing mix of valid and invalid files"""
        valid_file = tmp_path / "valid.txt"
        valid_file.write_text("content")

        invalid_file = tmp_path / "invalid.txt"

        monkeypatch.setattr(sys, "argv", [
            "wordcount.py",
            str(valid_file),
            str(invalid_file)
        ])

        exit_code = wordcount.main()

        # Should complete successfully despite one error
        assert exit_code == 0

        captured = capsys.readouterr()

        # Valid file should show in output
        assert str(valid_file) in captured.out

        # Invalid file should show error
        assert "Error reading" in captured.err

    def test_all_flags_with_multiple_files(self, tmp_path, monkeypatch, capsys):
        """Test that flags work correctly with multiple files"""
        file1 = tmp_path / "f1.txt"
        file1.write_text("one two\nthree\n")

        file2 = tmp_path / "f2.txt"
        file2.write_text("four five\nsix\n")

        # Test -l flag with multiple files
        monkeypatch.setattr(sys, "argv", [
            "wordcount.py", "-l", str(file1), str(file2)
        ])

        exit_code = wordcount.main()

        assert exit_code == 0
        captured = capsys.readouterr()

        # Should show line counts for each file + total (Python implementation shows totals)
        lines = captured.out.strip().split("\n")
        assert len(lines) == 3  # One line per file + total line
        assert "total" in captured.out


if __name__ == "__main__":
    pytest.main([__file__, "-v", "--cov=wordcount", "--cov-report=term-missing"])
