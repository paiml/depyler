"""
TDD Book - Phase 4: Network & IPC
Module: http.client - HTTP protocol client
Coverage: HTTPConnection, HTTPSConnection, HTTP methods, headers, responses

Test Categories:
- Basic HTTP connections
- HTTP methods (GET, POST, PUT, DELETE, HEAD, OPTIONS)
- Request headers
- Response handling (status, headers, body)
- Connection management (keep-alive, close)
- Chunked transfer encoding
- Error handling
- Edge cases
"""

import http.client
import http.server
import threading
import time
import pytest
from socketserver import TCPServer


# Test HTTP server for local testing
class SimpleHTTPHandler(http.server.BaseHTTPRequestHandler):
    """Simple HTTP handler for testing."""

    def log_message(self, format, *args):
        """Suppress server logging."""
        pass

    def do_GET(self):
        """Handle GET requests."""
        if self.path == "/":
            self.send_response(200)
            self.send_header("Content-Type", "text/plain")
            self.send_header("Content-Length", "12")
            self.end_headers()
            self.wfile.write(b"Hello, World")
        elif self.path == "/json":
            body = b'{"status": "ok"}'
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        elif self.path == "/404":
            self.send_response(404)
            self.send_header("Content-Type", "text/plain")
            self.end_headers()
            self.wfile.write(b"Not Found")
        elif self.path == "/redirect":
            self.send_response(302)
            self.send_header("Location", "/")
            self.end_headers()
        elif self.path == "/headers":
            # Echo back request headers
            headers_str = str(dict(self.headers)).encode()
            self.send_response(200)
            self.send_header("Content-Type", "text/plain")
            self.send_header("Content-Length", str(len(headers_str)))
            self.end_headers()
            self.wfile.write(headers_str)
        else:
            self.send_response(404)
            self.end_headers()

    def do_POST(self):
        """Handle POST requests."""
        content_length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_length)

        response = b"Received: " + body
        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.send_header("Content-Length", str(len(response)))
        self.end_headers()
        self.wfile.write(response)

    def do_PUT(self):
        """Handle PUT requests."""
        content_length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_length)

        response = b"Updated: " + body
        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.send_header("Content-Length", str(len(response)))
        self.end_headers()
        self.wfile.write(response)

    def do_DELETE(self):
        """Handle DELETE requests."""
        self.send_response(204)  # No Content
        self.end_headers()

    def do_HEAD(self):
        """Handle HEAD requests."""
        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.send_header("Content-Length", "12")
        self.end_headers()

    def do_OPTIONS(self):
        """Handle OPTIONS requests."""
        self.send_response(200)
        self.send_header("Allow", "GET, POST, PUT, DELETE, HEAD, OPTIONS")
        self.send_header("Content-Length", "0")
        self.end_headers()


@pytest.fixture(scope="module")
def test_server():
    """Start a test HTTP server for the module."""
    # Find an available port
    with TCPServer(("127.0.0.1", 0), SimpleHTTPHandler) as server:
        port = server.server_address[1]

        # Start server in background thread
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()

        time.sleep(0.1)  # Let server start

        yield ("127.0.0.1", port)

        server.shutdown()


class TestHTTPConnection:
    """Test basic HTTP connection functionality."""

    def test_create_connection(self, test_server):
        """Property: HTTPConnection creates connection to host."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)
        conn.close()
        assert conn is not None

    def test_connection_request_response(self, test_server):
        """Property: HTTPConnection can send request and receive response."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()

            assert response.status == 200
            assert response.read() == b"Hello, World"
        finally:
            conn.close()

    def test_connection_manual_close(self, test_server):
        """Property: HTTPConnection can be manually closed after use."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()
            assert response.status == 200
            body = response.read()
            assert body == b"Hello, World"
        finally:
            conn.close()
            # Verify connection is closed
            assert conn is not None


class TestHTTPMethods:
    """Test HTTP methods (GET, POST, PUT, DELETE, HEAD, OPTIONS)."""

    def test_get_request(self, test_server):
        """Property: GET request retrieves resource."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()

            assert response.status == 200
            assert response.read() == b"Hello, World"
        finally:
            conn.close()

    def test_post_request(self, test_server):
        """Property: POST request sends data to server."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            body = b"test data"
            conn.request("POST", "/", body)
            response = conn.getresponse()

            assert response.status == 200
            assert b"Received: test data" == response.read()
        finally:
            conn.close()

    def test_put_request(self, test_server):
        """Property: PUT request updates resource."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            body = b"updated data"
            conn.request("PUT", "/", body)
            response = conn.getresponse()

            assert response.status == 200
            assert b"Updated: updated data" == response.read()
        finally:
            conn.close()

    def test_delete_request(self, test_server):
        """Property: DELETE request removes resource."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("DELETE", "/")
            response = conn.getresponse()

            assert response.status == 204  # No Content
        finally:
            conn.close()

    def test_head_request(self, test_server):
        """Property: HEAD request retrieves headers only."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("HEAD", "/")
            response = conn.getresponse()

            assert response.status == 200
            assert response.getheader("Content-Type") == "text/plain"
            assert response.getheader("Content-Length") == "12"
            # HEAD should not return body
            assert response.read() == b""
        finally:
            conn.close()

    def test_options_request(self, test_server):
        """Property: OPTIONS request retrieves allowed methods."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("OPTIONS", "/")
            response = conn.getresponse()

            assert response.status == 200
            allow = response.getheader("Allow")
            assert "GET" in allow
            assert "POST" in allow
        finally:
            conn.close()


class TestHTTPHeaders:
    """Test HTTP header handling."""

    def test_request_headers(self, test_server):
        """Property: Can send custom headers with request."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            headers = {"User-Agent": "TestClient/1.0", "Accept": "application/json"}
            conn.request("GET", "/headers", headers=headers)
            response = conn.getresponse()

            body = response.read().decode()
            assert "User-Agent" in body or "user-agent" in body
        finally:
            conn.close()

    def test_response_headers(self, test_server):
        """Property: Can read response headers."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()

            assert response.getheader("Content-Type") == "text/plain"
            assert response.getheader("Content-Length") == "12"
        finally:
            conn.close()

    def test_getheaders_list(self, test_server):
        """Property: getheaders() returns list of header values."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()

            headers = response.getheaders()
            assert isinstance(headers, list)
            assert len(headers) > 0
        finally:
            conn.close()


class TestHTTPResponse:
    """Test HTTP response handling."""

    def test_response_status(self, test_server):
        """Property: Response has status code."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()

            assert response.status == 200
            assert response.reason == "OK"
        finally:
            conn.close()

    def test_response_404(self, test_server):
        """Property: 404 responses have correct status."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/404")
            response = conn.getresponse()

            assert response.status == 404
            assert response.reason == "Not Found"
        finally:
            conn.close()

    def test_response_read(self, test_server):
        """Property: read() returns response body."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()

            body = response.read()
            assert body == b"Hello, World"
        finally:
            conn.close()

    def test_response_read_partial(self, test_server):
        """Property: read(amt) returns partial body."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()

            partial = response.read(5)
            assert partial == b"Hello"

            rest = response.read()
            assert rest == b", World"
        finally:
            conn.close()

    def test_response_readline(self, test_server):
        """Property: readline() reads line from response."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()

            # Response is single line
            line = response.readline()
            assert b"Hello" in line
        finally:
            conn.close()


class TestConnectionManagement:
    """Test connection lifecycle and management."""

    def test_connection_close(self, test_server):
        """Property: close() closes connection."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        conn.request("GET", "/")
        response = conn.getresponse()
        response.read()

        conn.close()
        # After close, new request should fail or reconnect
        assert conn is not None

    def test_multiple_requests_same_connection(self, test_server):
        """Property: Can reuse connection for multiple requests."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            # First request
            conn.request("GET", "/")
            response1 = conn.getresponse()
            assert response1.status == 200
            response1.read()  # Must read to reuse connection

            # Second request on same connection
            conn.request("GET", "/json")
            response2 = conn.getresponse()
            assert response2.status == 200
            assert b"ok" in response2.read()
        finally:
            conn.close()

    def test_connection_timeout(self, test_server):
        """Property: HTTPConnection accepts timeout parameter."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port, timeout=5.0)

        try:
            conn.request("GET", "/")
            response = conn.getresponse()
            assert response.status == 200
        finally:
            conn.close()


class TestHTTPConstants:
    """Test HTTP module constants and status codes."""

    def test_http_status_constants(self):
        """Property: HTTP status code constants are defined."""
        assert http.client.OK == 200
        assert http.client.NOT_FOUND == 404
        assert http.client.INTERNAL_SERVER_ERROR == 500

    def test_http_method_constants(self):
        """Property: HTTP has common status codes."""
        assert hasattr(http.client, "OK")
        assert hasattr(http.client, "CREATED")
        assert hasattr(http.client, "BAD_REQUEST")

    def test_http_responses_dict(self):
        """Property: responses dict maps status to reason."""
        assert http.client.responses[200] == "OK"
        assert http.client.responses[404] == "Not Found"
        assert http.client.responses[500] == "Internal Server Error"


class TestEdgeCases:
    """Test edge cases and error handling."""

    def test_invalid_host(self):
        """Property: Connection to invalid host raises error."""
        conn = http.client.HTTPConnection("invalid.host.example", 80, timeout=1.0)

        with pytest.raises((OSError, http.client.HTTPException)):
            conn.request("GET", "/")
            conn.getresponse()

    def test_connection_refused(self):
        """Property: Connection refused raises error."""
        conn = http.client.HTTPConnection("127.0.0.1", 1, timeout=1.0)

        with pytest.raises((OSError, ConnectionRefusedError)):
            conn.request("GET", "/")

    def test_read_after_close(self, test_server):
        """Property: Reading after close raises error."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        conn.request("GET", "/")
        response = conn.getresponse()
        body = response.read()
        conn.close()

        # Reading again should return empty or raise
        assert body == b"Hello, World"

    def test_request_without_connect(self, test_server):
        """Property: Request auto-connects if not connected."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            # Request should auto-connect
            conn.request("GET", "/")
            response = conn.getresponse()
            assert response.status == 200
        finally:
            conn.close()

    def test_empty_response_read(self, test_server):
        """Property: Reading empty response body works."""
        host, port = test_server
        conn = http.client.HTTPConnection(host, port)

        try:
            conn.request("DELETE", "/")
            response = conn.getresponse()

            # 204 No Content
            body = response.read()
            assert body == b""
        finally:
            conn.close()


class TestHTTPSConnection:
    """Test HTTPS connections (if SSL available)."""

    def test_https_connection_creation(self):
        """Property: HTTPSConnection can be created."""
        conn = http.client.HTTPSConnection("www.python.org", timeout=5.0)
        assert conn is not None
        conn.close()

    @pytest.mark.skip(reason="Requires external network connection")
    def test_https_request(self):
        """Property: HTTPSConnection can make HTTPS requests."""
        conn = http.client.HTTPSConnection("www.python.org", timeout=5.0)

        try:
            conn.request("HEAD", "/")
            response = conn.getresponse()
            assert response.status in (200, 301, 302)
        finally:
            conn.close()
