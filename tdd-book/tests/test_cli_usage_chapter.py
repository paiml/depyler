"""
TDD Test for CLI Usage Chapter

RED Phase: These tests will fail until we create the chapter.
"""

import pytest
from pathlib import Path


def test_cli_usage_chapter_exists():
    """Test that cli-usage.md exists in the book"""
    chapter_path = Path(__file__).parent.parent / "src" / "cli-usage.md"
    assert chapter_path.exists(), "cli-usage.md chapter must exist"


def test_cli_usage_has_compile_command():
    """Test that the chapter documents the compile command"""
    chapter_path = Path(__file__).parent.parent / "src" / "cli-usage.md"
    content = chapter_path.read_text()

    assert "depyler compile" in content, "Must document 'depyler compile' command"
    assert "--output" in content or "-o" in content, "Must document output flag"
    assert "--profile" in content, "Must document profile flag"


def test_cli_usage_has_examples():
    """Test that the chapter has working examples"""
    chapter_path = Path(__file__).parent.parent / "src" / "cli-usage.md"
    content = chapter_path.read_text()

    # Must have code blocks with examples
    assert "```bash" in content, "Must have bash code examples"
    assert "script.py" in content, "Must show example Python file"


def test_cli_usage_has_all_commands():
    """Test that all major CLI commands are documented"""
    chapter_path = Path(__file__).parent.parent / "src" / "cli-usage.md"
    content = chapter_path.read_text()

    commands = [
        "depyler compile",
        "depyler transpile",
        "depyler analyze",
        "depyler check",
        "depyler interactive",
    ]

    for cmd in commands:
        assert cmd in content, f"Must document '{cmd}' command"


def test_cli_usage_in_summary():
    """Test that cli-usage.md is linked in SUMMARY.md"""
    summary_path = Path(__file__).parent.parent / "src" / "SUMMARY.md"
    content = summary_path.read_text()

    assert "cli-usage.md" in content, "cli-usage.md must be linked in SUMMARY.md"
    assert "CLI Usage" in content or "Getting Started" in content, "Must have proper section"


def test_cli_usage_has_installation():
    """Test that installation instructions are present"""
    chapter_path = Path(__file__).parent.parent / "src" / "cli-usage.md"
    content = chapter_path.read_text()

    assert "cargo install" in content, "Must document installation"
    assert "depyler" in content, "Must mention depyler package"


def test_cli_usage_has_troubleshooting():
    """Test that troubleshooting section exists"""
    chapter_path = Path(__file__).parent.parent / "src" / "cli-usage.md"
    content = chapter_path.read_text()

    assert "troubleshooting" in content.lower() or "common issues" in content.lower(), \
        "Must have troubleshooting section"
