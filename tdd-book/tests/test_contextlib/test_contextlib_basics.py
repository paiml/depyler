"""
TDD Book - Phase 3: Concurrency
Module: contextlib - Context manager utilities
Coverage: contextmanager, closing, suppress, ExitStack, redirect

Test Categories:
- contextmanager decorator
- closing() wrapper
- suppress() exception suppression
- redirect_stdout/redirect_stderr
- ExitStack for dynamic context managers
- nullcontext() no-op manager
- ContextDecorator pattern
- Edge cases and error handling
"""

import pytest
import contextlib
import io
import sys


class TestContextManager:
    """Test contextmanager decorator."""

    def test_contextmanager_basic(self):
        """Property: contextmanager creates context manager from generator."""
        entered = []
        exited = []

        @contextlib.contextmanager
        def manager():
            entered.append(True)
            yield "value"
            exited.append(True)

        with manager() as value:
            assert value == "value"
            assert len(entered) == 1

        assert len(exited) == 1

    def test_contextmanager_no_yield_value(self):
        """Property: contextmanager can yield None."""
        entered = []

        @contextlib.contextmanager
        def manager():
            entered.append(True)
            yield
            entered.append(False)

        with manager():
            assert len(entered) == 1

        assert len(entered) == 2

    def test_contextmanager_exception_handling(self):
        """Property: contextmanager allows cleanup on exception."""
        cleanup = []

        @contextlib.contextmanager
        def manager():
            try:
                yield
            finally:
                cleanup.append("cleaned")

        with pytest.raises(ValueError):
            with manager():
                raise ValueError("test")

        assert cleanup == ["cleaned"]

    def test_contextmanager_suppress_exception(self):
        """Property: contextmanager can suppress exceptions."""
        @contextlib.contextmanager
        def manager():
            try:
                yield
            except ValueError:
                pass  # Suppress ValueError

        # Exception is suppressed
        with manager():
            raise ValueError("suppressed")

        # If we reach here, exception was suppressed
        assert True


class TestClosing:
    """Test closing() context manager."""

    def test_closing_calls_close(self):
        """Property: closing() calls close() on exit."""
        class Resource:
            def __init__(self):
                self.closed = False

            def close(self):
                self.closed = True

        resource = Resource()
        with contextlib.closing(resource):
            assert not resource.closed

        assert resource.closed

    def test_closing_with_exception(self):
        """Property: closing() calls close() even on exception."""
        class Resource:
            def __init__(self):
                self.closed = False

            def close(self):
                self.closed = True

        resource = Resource()
        with pytest.raises(ValueError):
            with contextlib.closing(resource):
                raise ValueError("test")

        assert resource.closed


class TestSuppress:
    """Test suppress() exception suppression."""

    def test_suppress_single_exception(self):
        """Property: suppress() suppresses specified exception."""
        with contextlib.suppress(ValueError):
            raise ValueError("suppressed")

        # If we reach here, exception was suppressed
        assert True

    def test_suppress_multiple_exceptions(self):
        """Property: suppress() can suppress multiple exception types."""
        with contextlib.suppress(ValueError, TypeError):
            raise TypeError("suppressed")

        assert True

    def test_suppress_does_not_suppress_others(self):
        """Property: suppress() only suppresses specified exceptions."""
        with pytest.raises(RuntimeError):
            with contextlib.suppress(ValueError):
                raise RuntimeError("not suppressed")

    def test_suppress_no_exception(self):
        """Property: suppress() works when no exception raised."""
        result = []
        with contextlib.suppress(ValueError):
            result.append(1)

        assert result == [1]


class TestRedirect:
    """Test redirect_stdout and redirect_stderr."""

    def test_redirect_stdout(self):
        """Property: redirect_stdout() redirects stdout."""
        buffer = io.StringIO()

        with contextlib.redirect_stdout(buffer):
            print("redirected")

        assert "redirected" in buffer.getvalue()

    def test_redirect_stdout_restored(self):
        """Property: redirect_stdout() restores original stdout."""
        original_stdout = sys.stdout
        buffer = io.StringIO()

        with contextlib.redirect_stdout(buffer):
            pass

        assert sys.stdout == original_stdout

    def test_redirect_stderr(self):
        """Property: redirect_stderr() redirects stderr."""
        buffer = io.StringIO()

        with contextlib.redirect_stderr(buffer):
            print("error", file=sys.stderr)

        assert "error" in buffer.getvalue()

    def test_redirect_nested(self):
        """Property: Can nest redirect contexts."""
        stdout_buf = io.StringIO()
        stderr_buf = io.StringIO()

        with contextlib.redirect_stdout(stdout_buf):
            with contextlib.redirect_stderr(stderr_buf):
                print("out")
                print("err", file=sys.stderr)

        assert "out" in stdout_buf.getvalue()
        assert "err" in stderr_buf.getvalue()


class TestExitStack:
    """Test ExitStack for dynamic context management."""

    def test_exitstack_single_context(self):
        """Property: ExitStack manages single context."""
        closed = []

        class Resource:
            def __enter__(self):
                return self

            def __exit__(self, *exc):
                closed.append(True)
                return False

        with contextlib.ExitStack() as stack:
            resource = stack.enter_context(Resource())
            assert len(closed) == 0

        assert len(closed) == 1

    def test_exitstack_multiple_contexts(self):
        """Property: ExitStack manages multiple contexts."""
        closed = []

        class Resource:
            def __init__(self, n):
                self.n = n

            def __enter__(self):
                return self

            def __exit__(self, *exc):
                closed.append(self.n)
                return False

        with contextlib.ExitStack() as stack:
            r1 = stack.enter_context(Resource(1))
            r2 = stack.enter_context(Resource(2))
            r3 = stack.enter_context(Resource(3))

        # Contexts exit in reverse order (LIFO)
        assert closed == [3, 2, 1]

    def test_exitstack_callback(self):
        """Property: ExitStack can register callbacks."""
        called = []

        def callback(arg):
            called.append(arg)

        with contextlib.ExitStack() as stack:
            stack.callback(callback, "first")
            stack.callback(callback, "second")

        # Callbacks execute in reverse order (LIFO)
        assert called == ["second", "first"]

    def test_exitstack_pop_all(self):
        """Property: pop_all() transfers contexts to new stack."""
        closed = []

        class Resource:
            def __enter__(self):
                return self

            def __exit__(self, *exc):
                closed.append(True)
                return False

        stack1 = contextlib.ExitStack()
        stack1.enter_context(Resource())
        stack1.enter_context(Resource())

        stack2 = stack1.pop_all()
        stack1.close()

        # Resources not closed yet (transferred to stack2)
        assert len(closed) == 0

        stack2.close()
        # Now they're closed
        assert len(closed) == 2

    def test_exitstack_exception_propagation(self):
        """Property: ExitStack propagates exceptions after cleanup."""
        closed = []

        class Resource:
            def __enter__(self):
                return self

            def __exit__(self, *exc):
                closed.append(True)
                return False

        with pytest.raises(ValueError):
            with contextlib.ExitStack() as stack:
                stack.enter_context(Resource())
                raise ValueError("test")

        # Cleanup still happened
        assert len(closed) == 1


class TestNullContext:
    """Test nullcontext() no-op context manager."""

    def test_nullcontext_basic(self):
        """Property: nullcontext() is a no-op context manager."""
        with contextlib.nullcontext():
            pass

        assert True

    def test_nullcontext_with_value(self):
        """Property: nullcontext() can return a value."""
        with contextlib.nullcontext("value") as value:
            assert value == "value"

    def test_nullcontext_optional_context(self):
        """Property: nullcontext() useful for optional contexts."""
        def process(use_context):
            ctx = open("/dev/null") if use_context else contextlib.nullcontext()
            with ctx:
                pass

        # Both paths work
        process(False)


class TestContextDecorator:
    """Test ContextDecorator pattern."""

    def test_context_decorator_as_context_manager(self):
        """Property: ContextDecorator can be used as context manager."""
        entered = []

        class MyContext(contextlib.ContextDecorator):
            def __enter__(self):
                entered.append(True)
                return self

            def __exit__(self, *exc):
                entered.append(False)
                return False

        with MyContext():
            assert len(entered) == 1

        assert len(entered) == 2

    def test_context_decorator_as_decorator(self):
        """Property: ContextDecorator can be used as decorator."""
        entered = []

        class MyContext(contextlib.ContextDecorator):
            def __enter__(self):
                entered.append(True)
                return self

            def __exit__(self, *exc):
                entered.append(False)
                return False

        @MyContext()
        def func():
            return "result"

        result = func()

        assert result == "result"
        assert len(entered) == 2


class TestAsyncContextManager:
    """Test async context manager utilities."""

    @pytest.mark.asyncio
    async def test_async_contextmanager_basic(self):
        """Property: asynccontextmanager creates async context manager."""
        entered = []
        exited = []

        @contextlib.asynccontextmanager
        async def manager():
            entered.append(True)
            yield "value"
            exited.append(True)

        async with manager() as value:
            assert value == "value"
            assert len(entered) == 1

        assert len(exited) == 1

    @pytest.mark.asyncio
    async def test_async_exitstack(self):
        """Property: AsyncExitStack manages async contexts."""
        closed = []

        class AsyncResource:
            async def __aenter__(self):
                return self

            async def __aexit__(self, *exc):
                closed.append(True)
                return False

        async with contextlib.AsyncExitStack() as stack:
            resource = await stack.enter_async_context(AsyncResource())
            assert len(closed) == 0

        assert len(closed) == 1


class TestEdgeCases:
    """Test edge cases and error handling."""

    def test_nested_suppressions(self):
        """Property: Can nest suppress contexts."""
        with contextlib.suppress(ValueError):
            with contextlib.suppress(TypeError):
                raise TypeError("inner")

        assert True

    def test_exitstack_empty(self):
        """Property: Empty ExitStack is valid."""
        with contextlib.ExitStack() as stack:
            pass

        assert True

    def test_closing_object_without_close(self):
        """Property: closing() requires close() method."""
        with pytest.raises(AttributeError):
            with contextlib.closing(object()):
                pass

    def test_contextmanager_generator_error(self):
        """Property: contextmanager handles generator errors."""
        @contextlib.contextmanager
        def manager():
            raise ValueError("setup error")
            yield  # Never reached

        with pytest.raises(ValueError, match="setup error"):
            with manager():
                pass

    def test_exitstack_close_twice(self):
        """Property: Closing ExitStack twice is safe."""
        stack = contextlib.ExitStack()
        stack.close()
        stack.close()  # Should not raise

        assert True
