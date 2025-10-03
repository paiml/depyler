"""Test io module - In-memory streams for text and binary data.

This module tests io's StringIO and BytesIO classes for working with
in-memory streams.
"""

import io
import pytest


class TestStringIO:
    """StringIO - In-memory text stream."""

    def test_stringio_basic_write(self):
        """Basic: Write text to StringIO."""
        sio = io.StringIO()
        sio.write('Hello')
        assert sio.getvalue() == 'Hello'

    def test_stringio_read_after_write(self):
        """Feature: Read after writing requires seek(0)."""
        sio = io.StringIO()
        sio.write('Hello')
        sio.seek(0)
        content = sio.read()
        assert content == 'Hello'

    def test_stringio_multiple_writes(self):
        """Feature: Multiple writes append."""
        sio = io.StringIO()
        sio.write('Hello ')
        sio.write('World')
        assert sio.getvalue() == 'Hello World'

    def test_stringio_readline(self):
        """Feature: Read line by line."""
        sio = io.StringIO('Line 1\nLine 2\nLine 3')
        assert sio.readline() == 'Line 1\n'
        assert sio.readline() == 'Line 2\n'
        assert sio.readline() == 'Line 3'

    def test_stringio_readlines(self):
        """Feature: Read all lines at once."""
        sio = io.StringIO('Line 1\nLine 2\nLine 3')
        lines = sio.readlines()
        assert lines == ['Line 1\n', 'Line 2\n', 'Line 3']

    def test_stringio_tell_position(self):
        """Feature: tell() returns current position."""
        sio = io.StringIO()
        assert sio.tell() == 0
        sio.write('Hello')
        assert sio.tell() == 5

    def test_stringio_seek(self):
        """Feature: Seek to specific position."""
        sio = io.StringIO('Hello World')
        sio.seek(6)
        assert sio.read() == 'World'

    def test_stringio_seek_from_end(self):
        """Feature: Seek from end with whence=2."""
        sio = io.StringIO('Hello World')
        sio.seek(0, 2)  # Seek to end
        assert sio.tell() == 11

    def test_stringio_seek_relative_unsupported(self):
        """Edge: StringIO doesn't support cur-relative seeks (whence=1 with offset != 0)."""
        sio = io.StringIO('Hello World')
        with pytest.raises(OSError):
            sio.seek(-5, 1)  # Can't do nonzero cur-relative seeks

    def test_stringio_initial_value(self):
        """Feature: Initialize with existing string."""
        sio = io.StringIO('Initial')
        assert sio.read() == 'Initial'

    def test_stringio_truncate(self):
        """Feature: Truncate at current position."""
        sio = io.StringIO('Hello World')
        sio.seek(5)
        sio.truncate()
        assert sio.getvalue() == 'Hello'

    def test_stringio_truncate_size(self):
        """Feature: Truncate to specific size."""
        sio = io.StringIO('Hello World')
        sio.truncate(5)
        assert sio.getvalue() == 'Hello'

    def test_stringio_close(self):
        """Feature: Close stream and verify closed property."""
        sio = io.StringIO('Hello')
        assert sio.closed is False
        sio.close()
        assert sio.closed is True

    def test_stringio_context_manager(self):
        """Feature: Use StringIO as context manager."""
        with io.StringIO() as sio:
            sio.write('Hello')
            assert sio.getvalue() == 'Hello'
        assert sio.closed is True

    def test_stringio_writable(self):
        """Property: StringIO is writable."""
        sio = io.StringIO()
        assert sio.writable() is True

    def test_stringio_readable(self):
        """Property: StringIO is readable."""
        sio = io.StringIO()
        assert sio.readable() is True

    def test_stringio_seekable(self):
        """Property: StringIO is seekable."""
        sio = io.StringIO()
        assert sio.seekable() is True

    def test_stringio_empty_read(self):
        """Edge: Reading from empty StringIO returns empty string."""
        sio = io.StringIO()
        assert sio.read() == ''

    def test_stringio_read_after_close_raises(self):
        """Error: Reading after close raises ValueError."""
        sio = io.StringIO('Hello')
        sio.close()
        with pytest.raises(ValueError):
            sio.read()

    def test_stringio_write_after_close_raises(self):
        """Error: Writing after close raises ValueError."""
        sio = io.StringIO()
        sio.close()
        with pytest.raises(ValueError):
            sio.write('Hello')

    def test_stringio_negative_seek_raises(self):
        """Error: Seeking to negative position raises ValueError."""
        sio = io.StringIO('Hello')
        with pytest.raises(ValueError):
            sio.seek(-1)


class TestBytesIO:
    """BytesIO - In-memory binary stream."""

    def test_bytesio_basic_write(self):
        """Basic: Write bytes to BytesIO."""
        bio = io.BytesIO()
        bio.write(b'Hello')
        assert bio.getvalue() == b'Hello'

    def test_bytesio_read_after_write(self):
        """Feature: Read after writing requires seek(0)."""
        bio = io.BytesIO()
        bio.write(b'Hello')
        bio.seek(0)
        content = bio.read()
        assert content == b'Hello'

    def test_bytesio_multiple_writes(self):
        """Feature: Multiple writes append."""
        bio = io.BytesIO()
        bio.write(b'Hello ')
        bio.write(b'World')
        assert bio.getvalue() == b'Hello World'

    def test_bytesio_readline(self):
        """Feature: Read line by line from binary stream."""
        bio = io.BytesIO(b'Line 1\nLine 2\nLine 3')
        assert bio.readline() == b'Line 1\n'
        assert bio.readline() == b'Line 2\n'
        assert bio.readline() == b'Line 3'

    def test_bytesio_readlines(self):
        """Feature: Read all lines at once."""
        bio = io.BytesIO(b'Line 1\nLine 2\nLine 3')
        lines = bio.readlines()
        assert lines == [b'Line 1\n', b'Line 2\n', b'Line 3']

    def test_bytesio_tell_position(self):
        """Feature: tell() returns current position."""
        bio = io.BytesIO()
        assert bio.tell() == 0
        bio.write(b'Hello')
        assert bio.tell() == 5

    def test_bytesio_seek(self):
        """Feature: Seek to specific position."""
        bio = io.BytesIO(b'Hello World')
        bio.seek(6)
        assert bio.read() == b'World'

    def test_bytesio_initial_value(self):
        """Feature: Initialize with existing bytes."""
        bio = io.BytesIO(b'Initial')
        assert bio.read() == b'Initial'

    def test_bytesio_truncate(self):
        """Feature: Truncate at current position."""
        bio = io.BytesIO(b'Hello World')
        bio.seek(5)
        bio.truncate()
        assert bio.getvalue() == b'Hello'

    def test_bytesio_close(self):
        """Feature: Close stream and verify closed property."""
        bio = io.BytesIO(b'Hello')
        assert bio.closed is False
        bio.close()
        assert bio.closed is True

    def test_bytesio_context_manager(self):
        """Feature: Use BytesIO as context manager."""
        with io.BytesIO() as bio:
            bio.write(b'Hello')
            assert bio.getvalue() == b'Hello'
        assert bio.closed is True

    def test_bytesio_writable(self):
        """Property: BytesIO is writable."""
        bio = io.BytesIO()
        assert bio.writable() is True

    def test_bytesio_readable(self):
        """Property: BytesIO is readable."""
        bio = io.BytesIO()
        assert bio.readable() is True

    def test_bytesio_seekable(self):
        """Property: BytesIO is seekable."""
        bio = io.BytesIO()
        assert bio.seekable() is True

    def test_bytesio_binary_data(self):
        """Feature: Handle arbitrary binary data."""
        data = b'\x00\x01\x02\x03\xff\xfe\xfd'
        bio = io.BytesIO(data)
        assert bio.read() == data

    def test_bytesio_read_size(self):
        """Feature: Read specific number of bytes."""
        bio = io.BytesIO(b'Hello World')
        assert bio.read(5) == b'Hello'
        assert bio.read(1) == b' '
        assert bio.read(5) == b'World'

    def test_bytesio_empty_read(self):
        """Edge: Reading from empty BytesIO returns empty bytes."""
        bio = io.BytesIO()
        assert bio.read() == b''

    def test_bytesio_read_beyond_end(self):
        """Edge: Reading beyond end returns what's available."""
        bio = io.BytesIO(b'Hello')
        assert bio.read(100) == b'Hello'

    def test_bytesio_write_string_raises(self):
        """Error: Writing string to BytesIO raises TypeError."""
        bio = io.BytesIO()
        with pytest.raises(TypeError):
            bio.write('Hello')

    def test_bytesio_read_after_close_raises(self):
        """Error: Reading after close raises ValueError."""
        bio = io.BytesIO(b'Hello')
        bio.close()
        with pytest.raises(ValueError):
            bio.read()


class TestTextIOWrapper:
    """TextIOWrapper - Text wrapper for binary streams."""

    def test_textiowrapper_basic(self):
        """Basic: Wrap BytesIO with TextIOWrapper."""
        bio = io.BytesIO()
        text = io.TextIOWrapper(bio, encoding='utf-8')
        text.write('Hello')
        text.flush()
        assert bio.getvalue() == b'Hello'

    def test_textiowrapper_encoding(self):
        """Feature: Specify encoding for text wrapper."""
        bio = io.BytesIO()
        text = io.TextIOWrapper(bio, encoding='utf-8')
        text.write('Hello 世界')
        text.flush()
        bio.seek(0)
        assert bio.read() == 'Hello 世界'.encode('utf-8')

    def test_textiowrapper_newline_handling(self):
        """Feature: Handle newline translations."""
        bio = io.BytesIO()
        text = io.TextIOWrapper(bio, encoding='utf-8', newline='\n')
        text.write('Line 1\nLine 2\n')
        text.flush()
        assert bio.getvalue() == b'Line 1\nLine 2\n'

    def test_textiowrapper_close_underlying(self):
        """Edge: Closing wrapper closes underlying stream."""
        bio = io.BytesIO()
        text = io.TextIOWrapper(bio, encoding='utf-8')
        text.close()
        assert bio.closed is True


class TestStringIOIterator:
    """StringIO iteration - Iterate over lines."""

    def test_stringio_iterate_lines(self):
        """Feature: Iterate over StringIO lines."""
        sio = io.StringIO('Line 1\nLine 2\nLine 3')
        lines = list(sio)
        assert lines == ['Line 1\n', 'Line 2\n', 'Line 3']

    def test_stringio_for_loop(self):
        """Feature: Use StringIO in for loop."""
        sio = io.StringIO('A\nB\nC')
        result = []
        for line in sio:
            result.append(line.strip())
        assert result == ['A', 'B', 'C']


class TestBytesIOIterator:
    """BytesIO iteration - Iterate over binary lines."""

    def test_bytesio_iterate_lines(self):
        """Feature: Iterate over BytesIO lines."""
        bio = io.BytesIO(b'Line 1\nLine 2\nLine 3')
        lines = list(bio)
        assert lines == [b'Line 1\n', b'Line 2\n', b'Line 3']

    def test_bytesio_for_loop(self):
        """Feature: Use BytesIO in for loop."""
        bio = io.BytesIO(b'A\nB\nC')
        result = []
        for line in bio:
            result.append(line.strip())
        assert result == [b'A', b'B', b'C']
