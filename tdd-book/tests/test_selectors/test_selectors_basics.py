"""
TDD Book - Phase 3: Concurrency
Module: selectors - High-level I/O multiplexing
Coverage: DefaultSelector, register, select, event masks

Test Categories:
- Selector creation (DefaultSelector)
- File descriptor registration (register, unregister, modify)
- Event selection (select with timeout)
- Event masks (READ, WRITE)
- SelectorKey objects
- Multiple file descriptor monitoring
- Context manager usage
- Edge cases and error handling
"""

import pytest
import selectors
import socket
import os
import sys


class TestSelectorCreation:
    """Test selector creation and basic properties."""

    def test_default_selector(self):
        """Property: DefaultSelector creates appropriate selector."""
        selector = selectors.DefaultSelector()
        assert selector is not None
        selector.close()

    def test_selector_close(self):
        """Property: close() releases selector resources."""
        selector = selectors.DefaultSelector()
        selector.close()

        # After close, operations should fail
        with pytest.raises((ValueError, OSError)):
            selector.select(timeout=0)

    def test_selector_context_manager(self):
        """Property: Selector works as context manager."""
        with selectors.DefaultSelector() as selector:
            assert selector is not None

        # After context exit, selector is closed
        with pytest.raises((ValueError, OSError)):
            selector.select(timeout=0)


class TestFileDescriptorRegistration:
    """Test file descriptor registration."""

    def test_register_file_descriptor(self):
        """Property: register() adds file descriptor."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                key = selector.register(r, selectors.EVENT_READ)

                assert key.fileobj == r
                assert key.events == selectors.EVENT_READ
        finally:
            os.close(r)
            os.close(w)

    def test_register_with_data(self):
        """Property: register() can attach data to key."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                data = {"name": "test"}
                key = selector.register(r, selectors.EVENT_READ, data=data)

                assert key.data == data
        finally:
            os.close(r)
            os.close(w)

    def test_unregister_file_descriptor(self):
        """Property: unregister() removes file descriptor."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(r, selectors.EVENT_READ)
                key = selector.unregister(r)

                assert key.fileobj == r

                # After unregister, fd should not be monitored
                ready = selector.select(timeout=0)
                assert len(ready) == 0
        finally:
            os.close(r)
            os.close(w)

    def test_modify_registration(self):
        """Property: modify() changes event mask."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(r, selectors.EVENT_READ)
                new_key = selector.modify(r, selectors.EVENT_WRITE)

                assert new_key.events == selectors.EVENT_WRITE
        finally:
            os.close(r)
            os.close(w)

    def test_modify_with_new_data(self):
        """Property: modify() can update data."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(r, selectors.EVENT_READ, data="old")
                new_key = selector.modify(r, selectors.EVENT_READ, data="new")

                assert new_key.data == "new"
        finally:
            os.close(r)
            os.close(w)


class TestEventSelection:
    """Test event selection with select()."""

    def test_select_no_events(self):
        """Property: select() returns empty when no events ready."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(r, selectors.EVENT_READ)
                ready = selector.select(timeout=0.01)

                assert len(ready) == 0
        finally:
            os.close(r)
            os.close(w)

    def test_select_read_ready(self):
        """Property: select() detects readable file descriptor."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(r, selectors.EVENT_READ)

                # Write data to make pipe readable
                os.write(w, b"test")

                ready = selector.select(timeout=0.1)

                assert len(ready) == 1
                key, events = ready[0]
                assert key.fileobj == r
                assert events & selectors.EVENT_READ
        finally:
            os.close(r)
            os.close(w)

    def test_select_write_ready(self):
        """Property: select() detects writable file descriptor."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(w, selectors.EVENT_WRITE)

                # Pipe write end is immediately writable
                ready = selector.select(timeout=0.1)

                assert len(ready) == 1
                key, events = ready[0]
                assert key.fileobj == w
                assert events & selectors.EVENT_WRITE
        finally:
            os.close(r)
            os.close(w)

    def test_select_timeout_none_blocks(self):
        """Property: select(timeout=None) blocks until event."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(w, selectors.EVENT_WRITE)

                # Should return immediately (pipe writable)
                ready = selector.select(timeout=None)

                assert len(ready) == 1
        finally:
            os.close(r)
            os.close(w)

    def test_select_timeout_zero_nonblocking(self):
        """Property: select(timeout=0) returns immediately."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(r, selectors.EVENT_READ)

                # Should return immediately with no events
                ready = selector.select(timeout=0)

                assert len(ready) == 0
        finally:
            os.close(r)
            os.close(w)


class TestEventMasks:
    """Test event mask constants."""

    def test_event_read_constant(self):
        """Property: EVENT_READ is available."""
        assert hasattr(selectors, "EVENT_READ")
        assert isinstance(selectors.EVENT_READ, int)

    def test_event_write_constant(self):
        """Property: EVENT_WRITE is available."""
        assert hasattr(selectors, "EVENT_WRITE")
        assert isinstance(selectors.EVENT_WRITE, int)

    def test_combined_events(self):
        """Property: Events can be combined with OR."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                combined = selectors.EVENT_READ | selectors.EVENT_WRITE
                key = selector.register(r, combined)

                assert key.events == combined
        finally:
            os.close(r)
            os.close(w)


class TestSelectorKey:
    """Test SelectorKey properties."""

    def test_selector_key_fileobj(self):
        """Property: SelectorKey has fileobj attribute."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                key = selector.register(r, selectors.EVENT_READ)

                assert hasattr(key, "fileobj")
                assert key.fileobj == r
        finally:
            os.close(r)
            os.close(w)

    def test_selector_key_events(self):
        """Property: SelectorKey has events attribute."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                key = selector.register(r, selectors.EVENT_READ)

                assert hasattr(key, "events")
                assert key.events == selectors.EVENT_READ
        finally:
            os.close(r)
            os.close(w)

    def test_selector_key_data(self):
        """Property: SelectorKey has data attribute."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                key = selector.register(r, selectors.EVENT_READ, data="test")

                assert hasattr(key, "data")
                assert key.data == "test"
        finally:
            os.close(r)
            os.close(w)


class TestMultipleFileDescriptors:
    """Test monitoring multiple file descriptors."""

    def test_multiple_registrations(self):
        """Property: Can register multiple file descriptors."""
        pipes = [(os.pipe()) for _ in range(3)]

        try:
            with selectors.DefaultSelector() as selector:
                for r, w in pipes:
                    selector.register(r, selectors.EVENT_READ)

                # No events ready
                ready = selector.select(timeout=0)
                assert len(ready) == 0
        finally:
            for r, w in pipes:
                os.close(r)
                os.close(w)

    def test_multiple_events_ready(self):
        """Property: select() returns all ready events."""
        pipes = [(os.pipe()) for _ in range(3)]

        try:
            with selectors.DefaultSelector() as selector:
                for r, w in pipes:
                    selector.register(r, selectors.EVENT_READ)

                # Write to all pipes
                for r, w in pipes:
                    os.write(w, b"x")

                ready = selector.select(timeout=0.1)

                assert len(ready) == 3
        finally:
            for r, w in pipes:
                os.close(r)
                os.close(w)

    def test_partial_events_ready(self):
        """Property: select() returns only ready events."""
        pipes = [(os.pipe()) for _ in range(3)]

        try:
            with selectors.DefaultSelector() as selector:
                for r, w in pipes:
                    selector.register(r, selectors.EVENT_READ)

                # Write to only first pipe
                os.write(pipes[0][1], b"x")

                ready = selector.select(timeout=0.1)

                assert len(ready) == 1
                key, events = ready[0]
                assert key.fileobj == pipes[0][0]
        finally:
            for r, w in pipes:
                os.close(r)
                os.close(w)


class TestGetKey:
    """Test get_key() method."""

    def test_get_key_returns_selector_key(self):
        """Property: get_key() returns SelectorKey for registered fd."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(r, selectors.EVENT_READ, data="test")
                key = selector.get_key(r)

                assert key.fileobj == r
                assert key.data == "test"
        finally:
            os.close(r)
            os.close(w)

    def test_get_key_unregistered_raises(self):
        """Property: get_key() raises KeyError for unregistered fd."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                with pytest.raises(KeyError):
                    selector.get_key(r)
        finally:
            os.close(r)
            os.close(w)


class TestGetMap:
    """Test get_map() method."""

    def test_get_map_returns_mapping(self):
        """Property: get_map() returns mapping of registered fds."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(r, selectors.EVENT_READ)
                mapping = selector.get_map()

                assert r in mapping
                assert mapping[r].fileobj == r
        finally:
            os.close(r)
            os.close(w)

    def test_get_map_empty(self):
        """Property: get_map() is empty when no registrations."""
        with selectors.DefaultSelector() as selector:
            mapping = selector.get_map()
            assert len(mapping) == 0


class TestEdgeCases:
    """Test edge cases and error conditions."""

    def test_register_twice_raises(self):
        """Property: Registering same fd twice raises KeyError."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                selector.register(r, selectors.EVENT_READ)

                with pytest.raises(KeyError):
                    selector.register(r, selectors.EVENT_READ)
        finally:
            os.close(r)
            os.close(w)

    def test_unregister_unregistered_raises(self):
        """Property: Unregistering unregistered fd raises KeyError."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                with pytest.raises(KeyError):
                    selector.unregister(r)
        finally:
            os.close(r)
            os.close(w)

    def test_modify_unregistered_raises(self):
        """Property: Modifying unregistered fd raises KeyError."""
        r, w = os.pipe()

        try:
            with selectors.DefaultSelector() as selector:
                with pytest.raises(KeyError):
                    selector.modify(r, selectors.EVENT_READ)
        finally:
            os.close(r)
            os.close(w)

    def test_select_after_close_raises(self):
        """Property: select() after close raises ValueError."""
        selector = selectors.DefaultSelector()
        selector.close()

        with pytest.raises((ValueError, OSError)):
            selector.select(timeout=0)

    def test_closed_fd_handling(self):
        """Property: Closing fd before unregister is safe."""
        r, w = os.pipe()

        with selectors.DefaultSelector() as selector:
            selector.register(r, selectors.EVENT_READ)

            # Close fd first
            os.close(r)
            os.close(w)

            # Unregister should work (or raise acceptable error)
            try:
                selector.unregister(r)
            except (KeyError, OSError):
                pass  # Acceptable
