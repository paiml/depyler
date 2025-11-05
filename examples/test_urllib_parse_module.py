"""
Comprehensive test suite for urllib.parse module.
Following TDD Book methodology: minimal examples, incremental complexity.

Tests urllib.parse core features:
- URL parsing and components
- URL encoding/decoding
- Query string handling
- URL joining and splitting
"""

from urllib.parse import (
    urlparse, urlunparse, parse_qs, parse_qsl,
    urlencode, quote, unquote, urljoin, urlsplit
)


def test_urlparse_basic():
    """Test basic URL parsing."""
    url = "https://example.com/path/page.html"
    result = urlparse(url)

    assert result.scheme == "https"
    assert result.netloc == "example.com"
    assert result.path == "/path/page.html"
    print("PASS: test_urlparse_basic")


def test_urlparse_with_query():
    """Test URL parsing with query string."""
    url = "https://example.com/search?q=python&lang=en"
    result = urlparse(url)

    assert result.scheme == "https"
    assert result.netloc == "example.com"
    assert result.path == "/search"
    assert result.query == "q=python&lang=en"
    print("PASS: test_urlparse_with_query")


def test_urlparse_with_fragment():
    """Test URL parsing with fragment."""
    url = "https://example.com/page#section1"
    result = urlparse(url)

    assert result.scheme == "https"
    assert result.path == "/page"
    assert result.fragment == "section1"
    print("PASS: test_urlparse_with_fragment")


def test_urlparse_full():
    """Test full URL parsing with all components."""
    url = "https://user:pass@example.com:8080/path?query=value#fragment"
    result = urlparse(url)

    assert result.scheme == "https"
    assert result.netloc == "user:pass@example.com:8080"
    assert result.path == "/path"
    assert result.query == "query=value"
    assert result.fragment == "fragment"
    print("PASS: test_urlparse_full")


def test_parse_qs_basic():
    """Test query string parsing."""
    query = "name=John&age=30&city=NYC"
    result = parse_qs(query)

    assert result["name"] == ["John"]
    assert result["age"] == ["30"]
    assert result["city"] == ["NYC"]
    print("PASS: test_parse_qs_basic")


def test_parse_qs_multiple_values():
    """Test query string with multiple values."""
    query = "tag=python&tag=rust&tag=programming"
    result = parse_qs(query)

    assert len(result["tag"]) == 3
    assert "python" in result["tag"]
    assert "rust" in result["tag"]
    print("PASS: test_parse_qs_multiple_values")


def test_parse_qsl_tuples():
    """Test query string parsing as list of tuples."""
    query = "a=1&b=2&c=3"
    result = parse_qsl(query)

    assert len(result) == 3
    assert ("a", "1") in result
    assert ("b", "2") in result
    assert ("c", "3") in result
    print("PASS: test_parse_qsl_tuples")


def test_urlencode_basic():
    """Test URL encoding from dict."""
    params = {"name": "John Doe", "age": "30"}
    result = urlencode(params)

    # Result may vary in order, so check both
    assert "name=John+Doe" in result or "name=John%20Doe" in result
    assert "age=30" in result
    print("PASS: test_urlencode_basic")


def test_quote_string():
    """Test URL quoting/encoding."""
    text = "Hello World!"
    result = quote(text)

    assert result == "Hello%20World%21"
    print("PASS: test_quote_string")


def test_unquote_string():
    """Test URL unquoting/decoding."""
    encoded = "Hello%20World%21"
    result = unquote(encoded)

    assert result == "Hello World!"
    print("PASS: test_unquote_string")


def test_quote_safe_chars():
    """Test URL quoting with safe characters."""
    # Forward slashes are typically safe in paths
    path = "/path/to/file"
    result = quote(path, safe='/')

    assert result == "/path/to/file"
    print("PASS: test_quote_safe_chars")


def test_urljoin_basic():
    """Test joining URLs."""
    base = "https://example.com/dir/"
    relative = "page.html"
    result = urljoin(base, relative)

    assert result == "https://example.com/dir/page.html"
    print("PASS: test_urljoin_basic")


def test_urljoin_absolute():
    """Test joining with absolute URL."""
    base = "https://example.com/dir/"
    absolute = "https://other.com/page.html"
    result = urljoin(base, absolute)

    # Absolute URL should replace base
    assert result == "https://other.com/page.html"
    print("PASS: test_urljoin_absolute")


def test_urlsplit_basic():
    """Test URL splitting (similar to urlparse)."""
    url = "https://example.com/path?query=value#fragment"
    result = urlsplit(url)

    assert result.scheme == "https"
    assert result.netloc == "example.com"
    assert result.path == "/path"
    assert result.query == "query=value"
    assert result.fragment == "fragment"
    print("PASS: test_urlsplit_basic")


def main():
    """Run all urllib.parse tests."""
    print("=" * 60)
    print("URLLIB.PARSE MODULE TESTS")
    print("=" * 60)

    test_urlparse_basic()
    test_urlparse_with_query()
    test_urlparse_with_fragment()
    test_urlparse_full()
    test_parse_qs_basic()
    test_parse_qs_multiple_values()
    test_parse_qsl_tuples()
    test_urlencode_basic()
    test_quote_string()
    test_unquote_string()
    test_quote_safe_chars()
    test_urljoin_basic()
    test_urljoin_absolute()
    test_urlsplit_basic()

    print("=" * 60)
    print("ALL URLLIB.PARSE TESTS PASSED!")
    print("Total tests: 14")
    print("=" * 60)


if __name__ == "__main__":
    main()
