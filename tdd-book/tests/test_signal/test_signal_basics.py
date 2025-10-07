"""
TDD Book - Phase 3: Concurrency
Module: signal - Signal handling for asynchronous events
Coverage: signal handlers, alarm, pause, raise_signal

Test Categories:
- Signal handler registration (signal.signal)
- Handler retrieval (signal.getsignal)
- Alarm scheduling (signal.alarm)
- Signal raising (signal.raise_signal, os.kill)
- Default and ignore handlers (SIG_DFL, SIG_IGN)
- Signal constants (SIGTERM, SIGINT, SIGALRM, etc.)
- Edge cases and platform differences

Note: Tests must be safe and not interfere with test runner.
"""

import pytest
import signal
import os
import sys
import time


# Skip all signal tests on Windows (limited signal support)
pytestmark = pytest.mark.skipif(
    sys.platform == "win32",
    reason="Signal handling differs significantly on Windows"
)


class TestSignalHandlers:
    """Test signal handler registration and retrieval."""

    def test_signal_register_handler(self):
        """Property: signal.signal() registers custom handler."""
        handled = []

        def handler(signum, frame):
            handled.append(signum)

        # Save original handler
        old_handler = signal.signal(signal.SIGUSR1, handler)

        try:
            # Raise signal
            signal.raise_signal(signal.SIGUSR1)
            time.sleep(0.01)  # Let handler execute

            assert len(handled) == 1
            assert handled[0] == signal.SIGUSR1
        finally:
            # Restore original handler
            signal.signal(signal.SIGUSR1, old_handler)

    def test_getsignal_returns_handler(self):
        """Property: getsignal() returns current handler."""
        def custom_handler(signum, frame):
            pass

        old_handler = signal.signal(signal.SIGUSR1, custom_handler)

        try:
            current = signal.getsignal(signal.SIGUSR1)
            assert current == custom_handler
        finally:
            signal.signal(signal.SIGUSR1, old_handler)

    def test_signal_returns_previous_handler(self):
        """Property: signal.signal() returns previous handler."""
        def handler1(signum, frame):
            pass

        def handler2(signum, frame):
            pass

        old = signal.signal(signal.SIGUSR1, handler1)
        prev = signal.signal(signal.SIGUSR1, handler2)

        try:
            assert prev == handler1
        finally:
            signal.signal(signal.SIGUSR1, old)

    def test_handler_with_arguments(self):
        """Property: Handler receives signum and frame."""
        received = []

        def handler(signum, frame):
            received.append((signum, frame))

        old_handler = signal.signal(signal.SIGUSR2, handler)

        try:
            signal.raise_signal(signal.SIGUSR2)
            time.sleep(0.01)

            assert len(received) == 1
            assert received[0][0] == signal.SIGUSR2
            assert received[0][1] is not None  # Frame object
        finally:
            signal.signal(signal.SIGUSR2, old_handler)


class TestSignalConstants:
    """Test signal constant availability."""

    def test_sigusr1_constant(self):
        """Property: SIGUSR1 is available."""
        assert hasattr(signal, "SIGUSR1")
        assert isinstance(signal.SIGUSR1, int)

    def test_sigusr2_constant(self):
        """Property: SIGUSR2 is available."""
        assert hasattr(signal, "SIGUSR2")
        assert isinstance(signal.SIGUSR2, int)

    def test_sigterm_constant(self):
        """Property: SIGTERM is available."""
        assert hasattr(signal, "SIGTERM")
        assert isinstance(signal.SIGTERM, int)

    def test_sigint_constant(self):
        """Property: SIGINT is available."""
        assert hasattr(signal, "SIGINT")
        assert isinstance(signal.SIGINT, int)

    def test_sigalrm_constant(self):
        """Property: SIGALRM is available."""
        assert hasattr(signal, "SIGALRM")
        assert isinstance(signal.SIGALRM, int)

    def test_signal_constants_unique(self):
        """Property: Signal constants have unique values."""
        signals = [signal.SIGUSR1, signal.SIGUSR2, signal.SIGTERM, signal.SIGALRM]
        assert len(signals) == len(set(signals))


class TestDefaultHandlers:
    """Test SIG_DFL and SIG_IGN special handlers."""

    def test_sig_dfl_available(self):
        """Property: SIG_DFL is available."""
        assert hasattr(signal, "SIG_DFL")
        assert signal.SIG_DFL is not None

    def test_sig_ign_available(self):
        """Property: SIG_IGN is available."""
        assert hasattr(signal, "SIG_IGN")
        assert signal.SIG_IGN is not None

    def test_restore_default_handler(self):
        """Property: SIG_DFL restores default behavior."""
        def custom_handler(signum, frame):
            pass

        old = signal.signal(signal.SIGUSR1, custom_handler)

        try:
            # Restore default
            signal.signal(signal.SIGUSR1, signal.SIG_DFL)
            current = signal.getsignal(signal.SIGUSR1)
            assert current == signal.SIG_DFL
        finally:
            signal.signal(signal.SIGUSR1, old)

    def test_ignore_signal(self):
        """Property: SIG_IGN ignores signals."""
        old = signal.signal(signal.SIGUSR1, signal.SIG_IGN)

        try:
            # Signal should be ignored
            signal.raise_signal(signal.SIGUSR1)
            time.sleep(0.01)
            # If we reach here, signal was ignored (no crash)
            assert True
        finally:
            signal.signal(signal.SIGUSR1, old)


class TestAlarmSignal:
    """Test alarm scheduling with SIGALRM."""

    def test_alarm_schedules_sigalrm(self):
        """Property: alarm() schedules SIGALRM delivery."""
        handled = []

        def handler(signum, frame):
            handled.append(signum)

        old_handler = signal.signal(signal.SIGALRM, handler)

        try:
            signal.alarm(1)  # Schedule in 1 second
            time.sleep(1.1)  # Wait for signal

            assert len(handled) == 1
            assert handled[0] == signal.SIGALRM
        finally:
            signal.alarm(0)  # Cancel any pending alarm
            signal.signal(signal.SIGALRM, old_handler)

    def test_alarm_cancel(self):
        """Property: alarm(0) cancels pending alarm."""
        handled = []

        def handler(signum, frame):
            handled.append(signum)

        old_handler = signal.signal(signal.SIGALRM, handler)

        try:
            signal.alarm(1)
            signal.alarm(0)  # Cancel
            time.sleep(1.1)

            assert len(handled) == 0
        finally:
            signal.alarm(0)
            signal.signal(signal.SIGALRM, old_handler)

    def test_alarm_returns_remaining(self):
        """Property: alarm() returns remaining seconds."""
        old_handler = signal.signal(signal.SIGALRM, signal.SIG_IGN)

        try:
            signal.alarm(10)
            remaining = signal.alarm(5)

            # Should return ~10 (or slightly less)
            assert 9 <= remaining <= 10
        finally:
            signal.alarm(0)
            signal.signal(signal.SIGALRM, old_handler)


class TestRaiseSignal:
    """Test signal raising."""

    def test_raise_signal_to_self(self):
        """Property: raise_signal() sends signal to current process."""
        handled = []

        def handler(signum, frame):
            handled.append(signum)

        old = signal.signal(signal.SIGUSR1, handler)

        try:
            signal.raise_signal(signal.SIGUSR1)
            time.sleep(0.01)

            assert len(handled) == 1
        finally:
            signal.signal(signal.SIGUSR1, old)

    def test_os_kill_self(self):
        """Property: os.kill() can send signal to self."""
        handled = []

        def handler(signum, frame):
            handled.append(signum)

        old = signal.signal(signal.SIGUSR2, handler)

        try:
            os.kill(os.getpid(), signal.SIGUSR2)
            time.sleep(0.01)

            assert len(handled) == 1
        finally:
            signal.signal(signal.SIGUSR2, old)


class TestSignalEdgeCases:
    """Test edge cases and error conditions."""

    def test_invalid_signal_number(self):
        """Property: Invalid signal number raises ValueError."""
        with pytest.raises((ValueError, OSError)):
            signal.signal(99999, signal.SIG_DFL)

    def test_handler_exception_propagates(self):
        """Property: Handler exceptions propagate to caller."""
        def failing_handler(signum, frame):
            raise RuntimeError("handler error")

        old = signal.signal(signal.SIGUSR1, failing_handler)

        try:
            # Handler exceptions propagate
            with pytest.raises(RuntimeError, match="handler error"):
                signal.raise_signal(signal.SIGUSR1)
        finally:
            signal.signal(signal.SIGUSR1, old)

    def test_nested_signals(self):
        """Property: Signals can be handled while in handler."""
        handled = []

        def handler(signum, frame):
            handled.append(signum)
            if len(handled) < 2:
                signal.raise_signal(signal.SIGUSR1)

        old = signal.signal(signal.SIGUSR1, handler)

        try:
            signal.raise_signal(signal.SIGUSR1)
            time.sleep(0.05)  # Let nested signals complete

            # Should handle at least 2 signals
            assert len(handled) >= 2
        finally:
            signal.signal(signal.SIGUSR1, old)

    def test_multiple_signals_queued(self):
        """Property: Multiple signals are processed."""
        handled = []

        def handler(signum, frame):
            handled.append(signum)

        old = signal.signal(signal.SIGUSR1, handler)

        try:
            # Send multiple signals quickly
            for _ in range(5):
                signal.raise_signal(signal.SIGUSR1)

            time.sleep(0.05)

            # Should handle all or most signals
            assert len(handled) >= 1
        finally:
            signal.signal(signal.SIGUSR1, old)


class TestSignalValidSignals:
    """Test signal.valid_signals() if available."""

    def test_valid_signals_available(self):
        """Property: valid_signals() returns set of valid signals."""
        if hasattr(signal, "valid_signals"):
            valid = signal.valid_signals()
            assert isinstance(valid, set)
            assert signal.SIGUSR1 in valid
            assert signal.SIGTERM in valid

    def test_strsignal_available(self):
        """Property: strsignal() returns signal name."""
        if hasattr(signal, "strsignal"):
            name = signal.strsignal(signal.SIGTERM)
            assert isinstance(name, str)
            assert len(name) > 0


class TestSignalItimer:
    """Test interval timer (setitimer/getitimer) if available."""

    def test_setitimer_available(self):
        """Property: setitimer() is available on Unix."""
        if hasattr(signal, "setitimer"):
            handled = []

            def handler(signum, frame):
                handled.append(signum)

            old = signal.signal(signal.SIGALRM, handler)

            try:
                # Set interval timer (0.1s initial, 0 repeat)
                signal.setitimer(signal.ITIMER_REAL, 0.1, 0)
                time.sleep(0.15)

                assert len(handled) >= 1
            finally:
                signal.setitimer(signal.ITIMER_REAL, 0, 0)  # Cancel
                signal.signal(signal.SIGALRM, old)

    def test_getitimer_available(self):
        """Property: getitimer() returns timer values."""
        if hasattr(signal, "getitimer"):
            old = signal.signal(signal.SIGALRM, signal.SIG_IGN)

            try:
                signal.setitimer(signal.ITIMER_REAL, 1.0, 0)
                current, interval = signal.getitimer(signal.ITIMER_REAL)

                assert 0 < current <= 1.0
                assert interval == 0.0
            finally:
                signal.setitimer(signal.ITIMER_REAL, 0, 0)
                signal.signal(signal.SIGALRM, old)


class TestSignalNames:
    """Test signal name utilities."""

    def test_signals_dict(self):
        """Property: Signals module defines signal constants."""
        assert signal.SIGUSR1 > 0
        assert signal.SIGUSR2 > 0
        assert signal.SIGTERM > 0

    def test_nsig_constant(self):
        """Property: NSIG is available (number of signals)."""
        if hasattr(signal, "NSIG"):
            assert isinstance(signal.NSIG, int)
            assert signal.NSIG > 0
