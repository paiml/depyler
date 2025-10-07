"""
TDD Book - Phase 4: Network & IPC
Module: urllib - URL handling
Coverage: urllib.parse (URL parsing), urllib.request (URL opening), urllib.error

Test Categories:
- URL parsing (urlparse, urlunparse)
- URL encoding/decoding (quote, unquote)
- Query string parsing (parse_qs, parse_qsl, urlencode)
- URL joining (urljoin)
- URL components (ParseResult, SplitResult)
- urllib.request (urlopen, Request)
- urllib.error (URLError, HTTPError)
- Edge cases
"""

import pytest
import urllib.parse
import urllib.request
import urllib.error
from http.server import BaseHTTPRequestHandler
from socketserver import TCPServer
import threading
import time


class SimpleURLHandler(BaseHTTPRequestHandler):
    """Simple HTTP handler for urllib testing."""

    def log_message(self, format, *args):
        """Suppress server logging."""
        pass

    def do_GET(self):
        """Handle GET requests."""
        if self.path == "/":
            self.send_response(200)
            self.send_header("Content-Type", "text/plain")
            body = b"Hello from urllib test"
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        elif self.path.startswith("/echo"):
            # Echo back the query string
            query = self.path.split("?", 1)[1] if "?" in self.path else ""
            body = f"Query: {query}".encode()
            self.send_response(200)
            self.send_header("Content-Type", "text/plain")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        elif self.path == "/404":
            self.send_response(404)
            self.send_header("Content-Type", "text/plain")
            self.end_headers()
            self.wfile.write(b"Not Found")
        else:
            self.send_response(404)
            self.end_headers()


@pytest.fixture(scope="module")
def url_test_server():
    """Start a test HTTP server for urllib tests."""
    with TCPServer(("127.0.0.1", 0), SimpleURLHandler) as server:
        port = server.server_address[1]

        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()

        time.sleep(0.1)

        yield ("127.0.0.1", port)

        server.shutdown()


class TestURLParse:
    """Test urllib.parse.urlparse() and URL parsing."""

    def test_urlparse_basic(self):
        """Property: urlparse() parses URL into components."""
        result = urllib.parse.urlparse("http://example.com/path?query=value#fragment")

        assert result.scheme == "http"
        assert result.netloc == "example.com"
        assert result.path == "/path"
        assert result.query == "query=value"
        assert result.fragment == "fragment"

    def test_urlparse_with_port(self):
        """Property: urlparse() handles port numbers."""
        result = urllib.parse.urlparse("http://example.com:8080/path")

        assert result.netloc == "example.com:8080"
        assert result.hostname == "example.com"
        assert result.port == 8080

    def test_urlparse_https(self):
        """Property: urlparse() recognizes HTTPS scheme."""
        result = urllib.parse.urlparse("https://secure.example.com/")

        assert result.scheme == "https"
        assert result.netloc == "secure.example.com"

    def test_urlparse_no_scheme(self):
        """Property: urlparse() handles URLs without scheme."""
        result = urllib.parse.urlparse("//example.com/path")

        assert result.scheme == ""
        assert result.netloc == "example.com"
        assert result.path == "/path"

    def test_urlparse_relative(self):
        """Property: urlparse() handles relative URLs."""
        result = urllib.parse.urlparse("/path/to/resource")

        assert result.scheme == ""
        assert result.netloc == ""
        assert result.path == "/path/to/resource"

    def test_urlunparse(self):
        """Property: urlunparse() reconstructs URL from components."""
        parts = ("http", "example.com", "/path", "", "query=value", "fragment")
        url = urllib.parse.urlunparse(parts)

        assert url == "http://example.com/path?query=value#fragment"

    def test_urlparse_urlunparse_roundtrip(self):
        """Property: urlparse() and urlunparse() are inverses."""
        original = "http://example.com:8080/path?key=value#section"
        parsed = urllib.parse.urlparse(original)
        reconstructed = urllib.parse.urlunparse(parsed)

        # Reconstruct components
        assert urllib.parse.urlparse(reconstructed).scheme == "http"
        assert urllib.parse.urlparse(reconstructed).netloc == "example.com:8080"


class TestURLSplit:
    """Test urllib.parse.urlsplit() - variant of urlparse."""

    def test_urlsplit_basic(self):
        """Property: urlsplit() splits URL into 5 components."""
        result = urllib.parse.urlsplit("http://example.com/path?query#frag")

        assert result.scheme == "http"
        assert result.netloc == "example.com"
        assert result.path == "/path"
        assert result.query == "query"
        assert result.fragment == "frag"

    def test_urlunsplit(self):
        """Property: urlunsplit() joins URL components."""
        parts = ("http", "example.com", "/path", "query", "frag")
        url = urllib.parse.urlunsplit(parts)

        assert "http://example.com/path" in url
        assert "query" in url


class TestURLQuoting:
    """Test URL encoding/decoding (quote, unquote)."""

    def test_quote_basic(self):
        """Property: quote() encodes special characters."""
        result = urllib.parse.quote("hello world")
        assert result == "hello%20world"

    def test_quote_special_chars(self):
        """Property: quote() encodes URL-unsafe characters."""
        result = urllib.parse.quote("hello/world?key=value")
        # Forward slashes are preserved by default
        assert "%3F" in result  # ? encoded

    def test_quote_plus(self):
        """Property: quote_plus() uses + for spaces."""
        result = urllib.parse.quote_plus("hello world")
        assert result == "hello+world"

    def test_unquote_basic(self):
        """Property: unquote() decodes percent-encoded strings."""
        result = urllib.parse.unquote("hello%20world")
        assert result == "hello world"

    def test_unquote_plus(self):
        """Property: unquote_plus() decodes + as space."""
        result = urllib.parse.unquote_plus("hello+world")
        assert result == "hello world"

    def test_quote_unquote_roundtrip(self):
        """Property: quote() and unquote() are inverses."""
        original = "Hello, World! 你好"
        encoded = urllib.parse.quote(original)
        decoded = urllib.parse.unquote(encoded)

        assert decoded == original

    def test_quote_safe_chars(self):
        """Property: quote() preserves safe characters."""
        result = urllib.parse.quote("hello-world_123.txt", safe="")
        # All special chars should be encoded when safe=""
        assert "hello" in result
        assert "world" in result


class TestQueryString:
    """Test query string parsing and encoding."""

    def test_parse_qs_basic(self):
        """Property: parse_qs() parses query string into dict."""
        result = urllib.parse.parse_qs("key1=value1&key2=value2")

        assert result == {"key1": ["value1"], "key2": ["value2"]}

    def test_parse_qs_multiple_values(self):
        """Property: parse_qs() handles multiple values for same key."""
        result = urllib.parse.parse_qs("color=red&color=blue&color=green")

        assert result == {"color": ["red", "blue", "green"]}

    def test_parse_qsl(self):
        """Property: parse_qsl() returns list of tuples."""
        result = urllib.parse.parse_qsl("key1=value1&key2=value2")

        assert result == [("key1", "value1"), ("key2", "value2")]

    def test_urlencode_basic(self):
        """Property: urlencode() creates query string from dict."""
        data = {"key1": "value1", "key2": "value2"}
        result = urllib.parse.urlencode(data)

        # Order may vary
        assert "key1=value1" in result
        assert "key2=value2" in result
        assert "&" in result

    def test_urlencode_list(self):
        """Property: urlencode() handles list of tuples."""
        data = [("key", "value1"), ("key", "value2")]
        result = urllib.parse.urlencode(data)

        assert result == "key=value1&key=value2"

    def test_urlencode_with_spaces(self):
        """Property: urlencode() encodes spaces."""
        data = {"message": "hello world"}
        result = urllib.parse.urlencode(data)

        assert "message=hello+world" in result or "message=hello%20world" in result

    def test_parse_qs_urlencode_roundtrip(self):
        """Property: parse_qs() and urlencode() roundtrip."""
        data = {"key1": "value1", "key2": "value2"}
        encoded = urllib.parse.urlencode(data)
        decoded = urllib.parse.parse_qs(encoded, keep_blank_values=True)

        # Convert list values back to strings for comparison
        decoded_simple = {k: v[0] for k, v in decoded.items()}
        assert decoded_simple == data


class TestURLJoin:
    """Test URL joining (urljoin)."""

    def test_urljoin_absolute_path(self):
        """Property: urljoin() joins base URL with absolute path."""
        result = urllib.parse.urljoin("http://example.com/path/", "/newpath")
        assert result == "http://example.com/newpath"

    def test_urljoin_relative_path(self):
        """Property: urljoin() joins base URL with relative path."""
        result = urllib.parse.urljoin("http://example.com/path/", "subpath")
        assert result == "http://example.com/path/subpath"

    def test_urljoin_parent_directory(self):
        """Property: urljoin() handles parent directory (..)."""
        result = urllib.parse.urljoin("http://example.com/path/sub/", "../other")
        assert result == "http://example.com/path/other"

    def test_urljoin_absolute_url(self):
        """Property: urljoin() with absolute URL returns the new URL."""
        result = urllib.parse.urljoin(
            "http://example.com/", "http://other.com/path"
        )
        assert result == "http://other.com/path"

    def test_urljoin_query_string(self):
        """Property: urljoin() handles query strings."""
        result = urllib.parse.urljoin("http://example.com/", "path?query=value")
        assert result == "http://example.com/path?query=value"

    def test_urljoin_fragment(self):
        """Property: urljoin() handles fragments."""
        result = urllib.parse.urljoin("http://example.com/path", "#section")
        assert result == "http://example.com/path#section"


class TestURLRequest:
    """Test urllib.request.urlopen() and URL fetching."""

    def test_urlopen_basic(self, url_test_server):
        """Property: urlopen() retrieves URL content."""
        host, port = url_test_server
        url = f"http://{host}:{port}/"

        with urllib.request.urlopen(url, timeout=5) as response:
            content = response.read()
            assert content == b"Hello from urllib test"

    def test_urlopen_status(self, url_test_server):
        """Property: urlopen() response has status code."""
        host, port = url_test_server
        url = f"http://{host}:{port}/"

        with urllib.request.urlopen(url, timeout=5) as response:
            assert response.status == 200
            assert response.reason == "OK"

    def test_urlopen_headers(self, url_test_server):
        """Property: urlopen() response has headers."""
        host, port = url_test_server
        url = f"http://{host}:{port}/"

        with urllib.request.urlopen(url, timeout=5) as response:
            content_type = response.getheader("Content-Type")
            assert content_type == "text/plain"

    def test_urlopen_read_partial(self, url_test_server):
        """Property: urlopen() response supports partial reads."""
        host, port = url_test_server
        url = f"http://{host}:{port}/"

        with urllib.request.urlopen(url, timeout=5) as response:
            partial = response.read(5)
            assert partial == b"Hello"

    def test_request_with_headers(self, url_test_server):
        """Property: Request object allows custom headers."""
        host, port = url_test_server
        url = f"http://{host}:{port}/"

        req = urllib.request.Request(url, headers={"User-Agent": "TestClient/1.0"})
        assert req.get_header("User-agent") == "TestClient/1.0"


class TestURLError:
    """Test urllib.error exceptions."""

    def test_http_error_404(self, url_test_server):
        """Property: 404 responses raise HTTPError."""
        host, port = url_test_server
        url = f"http://{host}:{port}/404"

        with pytest.raises(urllib.error.HTTPError) as exc_info:
            urllib.request.urlopen(url, timeout=5)

        assert exc_info.value.code == 404

    def test_url_error_invalid_host(self):
        """Property: Invalid host raises URLError."""
        url = "http://invalid.host.example.invalid/"

        with pytest.raises(urllib.error.URLError):
            urllib.request.urlopen(url, timeout=1)

    def test_http_error_has_info(self, url_test_server):
        """Property: HTTPError contains response information."""
        host, port = url_test_server
        url = f"http://{host}:{port}/404"

        try:
            urllib.request.urlopen(url, timeout=5)
        except urllib.error.HTTPError as e:
            assert e.code == 404
            assert e.reason == "Not Found"
            # HTTPError also has .read() method
            body = e.read()
            assert b"Not Found" in body


class TestParseResult:
    """Test ParseResult and SplitResult objects."""

    def test_parse_result_attributes(self):
        """Property: ParseResult has named tuple attributes."""
        result = urllib.parse.urlparse("http://example.com:80/path?q=v#frag")

        assert isinstance(result, urllib.parse.ParseResult)
        assert result.scheme == "http"
        assert result.netloc == "example.com:80"
        assert result.path == "/path"
        assert result.params == ""
        assert result.query == "q=v"
        assert result.fragment == "frag"

    def test_parse_result_geturl(self):
        """Property: ParseResult.geturl() reconstructs URL."""
        original = "http://example.com/path?query#fragment"
        result = urllib.parse.urlparse(original)

        reconstructed = result.geturl()
        assert reconstructed == original

    def test_split_result_attributes(self):
        """Property: SplitResult has 5 components."""
        result = urllib.parse.urlsplit("http://example.com/path?query#frag")

        assert isinstance(result, urllib.parse.SplitResult)
        assert result.scheme == "http"
        assert result.netloc == "example.com"
        assert result.path == "/path"
        assert result.query == "query"
        assert result.fragment == "frag"


class TestEdgeCases:
    """Test edge cases and special scenarios."""

    def test_quote_empty_string(self):
        """Property: quote() handles empty string."""
        result = urllib.parse.quote("")
        assert result == ""

    def test_unquote_empty_string(self):
        """Property: unquote() handles empty string."""
        result = urllib.parse.unquote("")
        assert result == ""

    def test_parse_qs_empty(self):
        """Property: parse_qs() handles empty query string."""
        result = urllib.parse.parse_qs("")
        assert result == {}

    def test_urlencode_empty(self):
        """Property: urlencode() handles empty dict."""
        result = urllib.parse.urlencode({})
        assert result == ""

    def test_urlparse_malformed(self):
        """Property: urlparse() handles malformed URLs gracefully."""
        result = urllib.parse.urlparse("http:///path")
        # Should not raise, even if malformed
        assert result.scheme == "http"

    def test_quote_non_ascii(self):
        """Property: quote() handles non-ASCII characters."""
        result = urllib.parse.quote("café")
        assert "caf" in result
        assert "%" in result  # Some characters should be encoded

    def test_parse_qs_no_value(self):
        """Property: parse_qs() handles keys without values."""
        result = urllib.parse.parse_qs("key1&key2=value2", keep_blank_values=True)
        assert "key1" in result
        assert result["key1"] == [""]

    def test_urljoin_empty_base(self):
        """Property: urljoin() with empty base uses relative URL."""
        result = urllib.parse.urljoin("", "/path")
        assert result == "/path"

    def test_urlopen_timeout(self, url_test_server):
        """Property: urlopen() accepts timeout parameter."""
        host, port = url_test_server
        url = f"http://{host}:{port}/"

        # Should complete within timeout
        with urllib.request.urlopen(url, timeout=10) as response:
            assert response.status == 200
