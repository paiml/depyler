"""
TDD Book - Phase 3: Concurrency
Module: socket - Low-level networking
Coverage: socket creation, TCP/UDP, client/server patterns, blocking modes

Test Categories:
- Socket creation and families
- TCP client/server (SOCK_STREAM)
- UDP sockets (SOCK_DGRAM)
- Socket options
- Blocking and non-blocking modes
- Host/port resolution
- Socket pairs
- Context manager usage
- Edge cases and error handling
"""

import pytest
import socket
import threading
import time


class TestSocketCreation:
    """Test socket creation and basic properties."""

    def test_create_tcp_socket(self):
        """Property: Create TCP socket with AF_INET and SOCK_STREAM."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        assert sock is not None
        sock.close()

    def test_create_udp_socket(self):
        """Property: Create UDP socket with SOCK_DGRAM."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        assert sock is not None
        sock.close()

    def test_socket_family(self):
        """Property: Socket has family attribute."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        assert sock.family == socket.AF_INET
        sock.close()

    def test_socket_type(self):
        """Property: Socket has type attribute."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        assert sock.type == socket.SOCK_STREAM
        sock.close()

    def test_socket_context_manager(self):
        """Property: Socket works as context manager."""
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
            assert sock is not None

        # Socket should be closed after context


class TestTCPClientServer:
    """Test TCP client/server communication."""

    def test_tcp_bind_listen_accept(self):
        """Property: TCP server can bind, listen, and accept."""
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

        try:
            server.bind(("127.0.0.1", 0))  # Bind to any available port
            port = server.getsockname()[1]
            server.listen(1)

            # Connect from another thread
            def client_connect():
                client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                client.connect(("127.0.0.1", port))
                client.close()

            thread = threading.Thread(target=client_connect)
            thread.start()

            conn, addr = server.accept()
            conn.close()
            thread.join()

            assert True
        finally:
            server.close()

    def test_tcp_send_recv(self):
        """Property: TCP sockets can send and receive data."""
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

        try:
            server.bind(("127.0.0.1", 0))
            port = server.getsockname()[1]
            server.listen(1)

            received = []

            def server_accept():
                conn, addr = server.accept()
                data = conn.recv(1024)
                received.append(data)
                conn.close()

            thread = threading.Thread(target=server_accept)
            thread.start()

            time.sleep(0.01)  # Let server start

            client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            client.connect(("127.0.0.1", port))
            client.sendall(b"Hello, server!")
            client.close()

            thread.join()

            assert received[0] == b"Hello, server!"
        finally:
            server.close()

    def test_tcp_getsockname(self):
        """Property: getsockname() returns local address."""
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

        try:
            server.bind(("127.0.0.1", 0))
            addr = server.getsockname()

            assert addr[0] == "127.0.0.1"
            assert isinstance(addr[1], int)
            assert addr[1] > 0
        finally:
            server.close()


class TestUDPSockets:
    """Test UDP socket communication."""

    def test_udp_sendto_recvfrom(self):
        """Property: UDP sockets can send and receive datagrams."""
        server = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

        try:
            server.bind(("127.0.0.1", 0))
            port = server.getsockname()[1]

            client = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

            try:
                client.sendto(b"UDP message", ("127.0.0.1", port))

                server.settimeout(1.0)
                data, addr = server.recvfrom(1024)

                assert data == b"UDP message"
            finally:
                client.close()
        finally:
            server.close()

    def test_udp_multiple_messages(self):
        """Property: UDP can handle multiple messages."""
        server = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

        try:
            server.bind(("127.0.0.1", 0))
            port = server.getsockname()[1]
            server.settimeout(1.0)

            client = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

            try:
                for i in range(3):
                    client.sendto(f"message-{i}".encode(), ("127.0.0.1", port))

                messages = []
                for _ in range(3):
                    data, addr = server.recvfrom(1024)
                    messages.append(data.decode())

                assert "message-0" in messages
                assert "message-1" in messages
                assert "message-2" in messages
            finally:
                client.close()
        finally:
            server.close()


class TestSocketOptions:
    """Test socket options (setsockopt, getsockopt)."""

    def test_setsockopt_reuseaddr(self):
        """Property: Can set SO_REUSEADDR option."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        try:
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            value = sock.getsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR)

            assert value == 1
        finally:
            sock.close()

    def test_getsockopt_default(self):
        """Property: getsockopt() returns option values."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        try:
            # Get default SO_REUSEADDR value
            value = sock.getsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR)
            assert isinstance(value, int)
        finally:
            sock.close()


class TestBlockingModes:
    """Test blocking and non-blocking socket modes."""

    def test_setblocking_true(self):
        """Property: setblocking(True) enables blocking mode."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        try:
            sock.setblocking(True)
            # In blocking mode, getblocking() returns True
            assert sock.getblocking() is True
        finally:
            sock.close()

    def test_setblocking_false(self):
        """Property: setblocking(False) enables non-blocking mode."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        try:
            sock.setblocking(False)
            assert sock.getblocking() is False
        finally:
            sock.close()

    def test_settimeout(self):
        """Property: settimeout() sets socket timeout."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        try:
            sock.settimeout(2.0)
            timeout = sock.gettimeout()

            assert timeout == 2.0
        finally:
            sock.close()

    def test_settimeout_none_blocking(self):
        """Property: settimeout(None) enables blocking mode."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        try:
            sock.settimeout(None)
            assert sock.gettimeout() is None
            assert sock.getblocking() is True
        finally:
            sock.close()

    def test_recv_timeout(self):
        """Property: recv() with timeout raises on timeout."""
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

        try:
            server.bind(("127.0.0.1", 0))
            port = server.getsockname()[1]
            server.listen(1)

            client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

            try:
                client.connect(("127.0.0.1", port))
                client.settimeout(0.01)

                # No data to receive, should timeout
                with pytest.raises(socket.timeout):
                    client.recv(1024)
            finally:
                client.close()
        finally:
            server.close()


class TestHostnameResolution:
    """Test hostname and address resolution."""

    def test_gethostname(self):
        """Property: gethostname() returns hostname."""
        hostname = socket.gethostname()
        assert isinstance(hostname, str)
        assert len(hostname) > 0

    def test_gethostbyname_localhost(self):
        """Property: gethostbyname() resolves localhost."""
        addr = socket.gethostbyname("localhost")
        assert addr == "127.0.0.1"

    def test_getaddrinfo_localhost(self):
        """Property: getaddrinfo() returns address info."""
        results = socket.getaddrinfo("localhost", 80, socket.AF_INET, socket.SOCK_STREAM)

        assert len(results) > 0
        family, socktype, proto, canonname, sockaddr = results[0]
        assert family == socket.AF_INET
        assert socktype == socket.SOCK_STREAM

    def test_inet_aton_inet_ntoa(self):
        """Property: inet_aton/inet_ntoa convert addresses."""
        packed = socket.inet_aton("192.168.1.1")
        assert isinstance(packed, bytes)

        addr = socket.inet_ntoa(packed)
        assert addr == "192.168.1.1"


class TestSocketPair:
    """Test socketpair() for local IPC."""

    def test_socketpair_communication(self):
        """Property: socketpair() creates connected socket pair."""
        if not hasattr(socket, "socketpair"):
            pytest.skip("socketpair not available on this platform")

        sock1, sock2 = socket.socketpair()

        try:
            sock1.send(b"Hello from sock1")
            data = sock2.recv(1024)

            assert data == b"Hello from sock1"

            sock2.send(b"Hello from sock2")
            data = sock1.recv(1024)

            assert data == b"Hello from sock2"
        finally:
            sock1.close()
            sock2.close()


class TestSocketConstants:
    """Test socket module constants."""

    def test_af_inet_constant(self):
        """Property: AF_INET is available."""
        assert hasattr(socket, "AF_INET")
        assert isinstance(socket.AF_INET, int)

    def test_sock_stream_constant(self):
        """Property: SOCK_STREAM is available."""
        assert hasattr(socket, "SOCK_STREAM")
        assert isinstance(socket.SOCK_STREAM, int)

    def test_sock_dgram_constant(self):
        """Property: SOCK_DGRAM is available."""
        assert hasattr(socket, "SOCK_DGRAM")
        assert isinstance(socket.SOCK_DGRAM, int)

    def test_sol_socket_constant(self):
        """Property: SOL_SOCKET is available."""
        assert hasattr(socket, "SOL_SOCKET")
        assert isinstance(socket.SOL_SOCKET, int)


class TestShutdown:
    """Test socket shutdown."""

    def test_shutdown_read(self):
        """Property: shutdown(SHUT_RD) closes read half."""
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

        try:
            server.bind(("127.0.0.1", 0))
            port = server.getsockname()[1]
            server.listen(1)

            def accept_conn():
                conn, addr = server.accept()
                conn.shutdown(socket.SHUT_RD)
                conn.close()

            thread = threading.Thread(target=accept_conn)
            thread.start()

            client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            client.connect(("127.0.0.1", port))

            time.sleep(0.01)
            client.close()
            thread.join()

            assert True
        finally:
            server.close()


class TestEdgeCases:
    """Test edge cases and error conditions."""

    def test_connect_refused(self):
        """Property: connect() raises on connection refused."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        try:
            with pytest.raises(OSError):
                sock.connect(("127.0.0.1", 1))  # Port 1 likely closed
        finally:
            sock.close()

    def test_bind_address_in_use(self):
        """Property: bind() raises when address in use."""
        sock1 = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        try:
            sock1.bind(("127.0.0.1", 0))
            port = sock1.getsockname()[1]

            sock2 = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

            try:
                with pytest.raises(OSError):
                    sock2.bind(("127.0.0.1", port))
            finally:
                sock2.close()
        finally:
            sock1.close()

    def test_recv_on_closed_socket(self):
        """Property: recv() on closed socket raises."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.close()

        with pytest.raises(OSError):
            sock.recv(1024)

    def test_send_on_unconnected_socket(self):
        """Property: send() on unconnected TCP socket raises."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        try:
            with pytest.raises(OSError):
                sock.send(b"data")
        finally:
            sock.close()

    def test_double_close(self):
        """Property: Closing socket twice is safe."""
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.close()
        sock.close()  # Should not raise

        assert True
