"""
TDD Book - Phase 3: Concurrency
Module: time - Time access and conversions
Coverage: time(), sleep(), perf_counter(), monotonic(), strftime()

Test Categories:
- Current time functions (time, time_ns)
- Sleep and delays
- Performance counters (perf_counter, monotonic)
- Process time measurement
- Time structures (gmtime, localtime)
- Time formatting (strftime, strptime)
- Timezone handling
- Edge cases and precision
"""

import pytest
import time


class TestCurrentTime:
    """Test current time functions."""

    def test_time_returns_float(self):
        """Property: time() returns float timestamp."""
        t = time.time()
        assert isinstance(t, float)
        assert t > 0

    def test_time_increases(self):
        """Property: time() increases over time."""
        t1 = time.time()
        time.sleep(0.01)
        t2 = time.time()

        assert t2 > t1

    def test_time_ns_returns_int(self):
        """Property: time_ns() returns nanosecond timestamp."""
        t = time.time_ns()
        assert isinstance(t, int)
        assert t > 0

    def test_time_ns_precision(self):
        """Property: time_ns() has nanosecond precision."""
        t1 = time.time_ns()
        t2 = time.time_ns()

        # Should differ even with very quick calls
        assert t1 <= t2


class TestSleep:
    """Test sleep() for delays."""

    def test_sleep_basic(self):
        """Property: sleep() delays execution."""
        start = time.time()
        time.sleep(0.01)
        elapsed = time.time() - start

        # Sleep should take at least 0.01 seconds
        assert elapsed >= 0.01

    def test_sleep_zero(self):
        """Property: sleep(0) yields control."""
        start = time.time()
        time.sleep(0)
        elapsed = time.time() - start

        # Should complete very quickly
        assert elapsed < 0.1

    def test_sleep_fractional(self):
        """Property: sleep() accepts fractional seconds."""
        start = time.time()
        time.sleep(0.05)
        elapsed = time.time() - start

        assert elapsed >= 0.05


class TestPerfCounter:
    """Test perf_counter() high-resolution timer."""

    def test_perf_counter_returns_float(self):
        """Property: perf_counter() returns float."""
        t = time.perf_counter()
        assert isinstance(t, float)

    def test_perf_counter_increases(self):
        """Property: perf_counter() increases monotonically."""
        t1 = time.perf_counter()
        time.sleep(0.01)
        t2 = time.perf_counter()

        assert t2 > t1

    def test_perf_counter_timing(self):
        """Property: perf_counter() measures elapsed time."""
        start = time.perf_counter()
        time.sleep(0.05)
        elapsed = time.perf_counter() - start

        assert 0.04 < elapsed < 0.10

    def test_perf_counter_ns(self):
        """Property: perf_counter_ns() returns nanoseconds."""
        t = time.perf_counter_ns()
        assert isinstance(t, int)
        assert t > 0


class TestMonotonic:
    """Test monotonic() for monotonic clock."""

    def test_monotonic_returns_float(self):
        """Property: monotonic() returns float."""
        t = time.monotonic()
        assert isinstance(t, float)

    def test_monotonic_never_decreases(self):
        """Property: monotonic() never decreases."""
        t1 = time.monotonic()
        time.sleep(0.01)
        t2 = time.monotonic()

        assert t2 >= t1

    def test_monotonic_ns(self):
        """Property: monotonic_ns() returns nanoseconds."""
        t = time.monotonic_ns()
        assert isinstance(t, int)


class TestProcessTime:
    """Test process_time() for CPU time."""

    def test_process_time_returns_float(self):
        """Property: process_time() returns float."""
        t = time.process_time()
        assert isinstance(t, float)
        assert t >= 0

    def test_process_time_excludes_sleep(self):
        """Property: process_time() excludes sleep time."""
        start = time.process_time()
        time.sleep(0.05)  # Sleep doesn't count as CPU time
        elapsed = time.process_time() - start

        # Should be much less than sleep duration
        assert elapsed < 0.01

    def test_process_time_measures_cpu(self):
        """Property: process_time() measures CPU work."""
        start = time.process_time()

        # Do some CPU work
        total = 0
        for i in range(100000):
            total += i

        elapsed = time.process_time() - start

        # Should have some elapsed CPU time
        assert elapsed > 0


class TestTimeStructures:
    """Test time structure functions."""

    def test_gmtime_returns_struct(self):
        """Property: gmtime() returns time structure."""
        t = time.gmtime()
        assert hasattr(t, "tm_year")
        assert hasattr(t, "tm_mon")
        assert hasattr(t, "tm_mday")

    def test_gmtime_with_timestamp(self):
        """Property: gmtime() accepts timestamp."""
        timestamp = 1000000000.0
        t = time.gmtime(timestamp)

        assert t.tm_year == 2001
        assert t.tm_mon == 9
        assert t.tm_mday == 9

    def test_localtime_returns_struct(self):
        """Property: localtime() returns local time structure."""
        t = time.localtime()
        assert hasattr(t, "tm_year")
        assert t.tm_year >= 2024

    def test_mktime_inverse_of_localtime(self):
        """Property: mktime() converts struct to timestamp."""
        timestamp = time.time()
        struct = time.localtime(timestamp)
        recovered = time.mktime(struct)

        # Should be very close (within 1 second)
        assert abs(recovered - timestamp) < 1.0


class TestTimeFormatting:
    """Test time formatting functions."""

    def test_strftime_basic(self):
        """Property: strftime() formats time."""
        t = time.localtime()
        formatted = time.strftime("%Y-%m-%d", t)

        assert isinstance(formatted, str)
        assert "-" in formatted

    def test_strftime_various_formats(self):
        """Property: strftime() supports various format codes."""
        t = time.localtime()

        year = time.strftime("%Y", t)
        assert len(year) == 4
        assert year.isdigit()

        month = time.strftime("%m", t)
        assert 1 <= int(month) <= 12

    def test_strptime_basic(self):
        """Property: strptime() parses time string."""
        time_str = "2024-01-15"
        parsed = time.strptime(time_str, "%Y-%m-%d")

        assert parsed.tm_year == 2024
        assert parsed.tm_mon == 1
        assert parsed.tm_mday == 15

    def test_strptime_strftime_roundtrip(self):
        """Property: strptime() inverts strftime()."""
        original = time.localtime(1000000000.0)
        formatted = time.strftime("%Y-%m-%d %H:%M:%S", original)
        parsed = time.strptime(formatted, "%Y-%m-%d %H:%M:%S")

        assert parsed.tm_year == original.tm_year
        assert parsed.tm_mon == original.tm_mon
        assert parsed.tm_mday == original.tm_mday


class TestTimeConstants:
    """Test time module constants."""

    def test_timezone_constant(self):
        """Property: timezone is defined."""
        assert hasattr(time, "timezone")
        assert isinstance(time.timezone, int)

    def test_daylight_constant(self):
        """Property: daylight is defined."""
        assert hasattr(time, "daylight")
        assert isinstance(time.daylight, int)

    def test_tzname_constant(self):
        """Property: tzname contains timezone names."""
        assert hasattr(time, "tzname")
        assert isinstance(time.tzname, tuple)
        assert len(time.tzname) >= 1


class TestClock:
    """Test deprecated clock functions (if available)."""

    def test_get_clock_info(self):
        """Property: get_clock_info() returns clock information."""
        info = time.get_clock_info("time")

        assert hasattr(info, "implementation")
        assert hasattr(info, "monotonic")
        assert hasattr(info, "resolution")

    def test_clock_info_monotonic(self):
        """Property: get_clock_info('monotonic') shows monotonic."""
        info = time.get_clock_info("monotonic")

        assert info.monotonic is True


class TestThreadTime:
    """Test thread_time() if available."""

    def test_thread_time_returns_float(self):
        """Property: thread_time() returns float."""
        t = time.thread_time()
        assert isinstance(t, float)
        assert t >= 0

    def test_thread_time_measures_thread_cpu(self):
        """Property: thread_time() measures thread CPU time."""
        start = time.thread_time()

        # Do some work
        total = 0
        for i in range(10000):
            total += i

        elapsed = time.thread_time() - start

        assert elapsed >= 0


class TestCtime:
    """Test ctime() and asctime()."""

    def test_ctime_returns_string(self):
        """Property: ctime() returns readable time string."""
        result = time.ctime()
        assert isinstance(result, str)
        assert len(result) > 0

    def test_ctime_with_timestamp(self):
        """Property: ctime() accepts timestamp."""
        result = time.ctime(1000000000.0)
        assert isinstance(result, str)
        assert "2001" in result

    def test_asctime_returns_string(self):
        """Property: asctime() formats time struct."""
        t = time.gmtime(1000000000.0)
        result = time.asctime(t)

        assert isinstance(result, str)
        assert "2001" in result


class TestEdgeCases:
    """Test edge cases and special scenarios."""

    def test_sleep_negative_raises(self):
        """Property: sleep() with negative value raises."""
        with pytest.raises(ValueError):
            time.sleep(-1.0)

    def test_very_short_sleep(self):
        """Property: Very short sleep() works."""
        start = time.perf_counter()
        time.sleep(0.0001)
        elapsed = time.perf_counter() - start

        # Should complete quickly
        assert elapsed < 1.0

    def test_time_consistency(self):
        """Property: time() and time_ns() are consistent."""
        t_sec = time.time()
        t_ns = time.time_ns()

        # Convert ns to seconds and compare
        t_ns_sec = t_ns / 1_000_000_000

        # Should be very close (within 0.1 second)
        assert abs(t_sec - t_ns_sec) < 0.1

    def test_struct_time_attributes(self):
        """Property: struct_time has all required attributes."""
        t = time.gmtime()

        assert hasattr(t, "tm_year")
        assert hasattr(t, "tm_mon")
        assert hasattr(t, "tm_mday")
        assert hasattr(t, "tm_hour")
        assert hasattr(t, "tm_min")
        assert hasattr(t, "tm_sec")
        assert hasattr(t, "tm_wday")
        assert hasattr(t, "tm_yday")
        assert hasattr(t, "tm_isdst")

    def test_zero_timestamp(self):
        """Property: Epoch timestamp 0 is handled."""
        t = time.gmtime(0)

        assert t.tm_year == 1970
        assert t.tm_mon == 1
        assert t.tm_mday == 1
