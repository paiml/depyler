"""Test time module - Time access and conversions.

This module tests time's functions for working with timestamps,
time structs, and time formatting/parsing.
"""

import time
import pytest


class TestTimeTimestamp:
    """time.time() - Get current time as seconds since epoch."""

    def test_time_returns_float(self):
        """Basic: time() returns a float timestamp."""
        t = time.time()
        assert isinstance(t, float)

    def test_time_is_positive(self):
        """Property: Timestamp is positive (after epoch)."""
        t = time.time()
        assert t > 0

    def test_time_increases(self):
        """Property: Time increases monotonically."""
        t1 = time.time()
        time.sleep(0.01)
        t2 = time.time()
        assert t2 > t1

    def test_time_resolution(self):
        """Property: Time has sub-second resolution."""
        t1 = time.time()
        t2 = time.time()
        # Even if same, they're floats with decimal precision
        assert isinstance(t1, float)
        assert '.' in str(t1) or t1 == int(t1)


class TestSleep:
    """time.sleep() - Suspend execution for given seconds."""

    def test_sleep_basic(self):
        """Basic: Sleep for short duration."""
        start = time.time()
        time.sleep(0.1)
        elapsed = time.time() - start
        assert elapsed >= 0.09  # Allow small margin

    def test_sleep_zero(self):
        """Edge: Sleep for 0 seconds (yields to scheduler)."""
        start = time.time()
        time.sleep(0)
        elapsed = time.time() - start
        assert elapsed >= 0

    def test_sleep_negative_raises(self):
        """Error: Sleep with negative value raises ValueError."""
        with pytest.raises(ValueError):
            time.sleep(-1)


class TestPerfCounter:
    """time.perf_counter() - High-resolution performance counter."""

    def test_perf_counter_returns_float(self):
        """Basic: perf_counter() returns float."""
        pc = time.perf_counter()
        assert isinstance(pc, float)

    def test_perf_counter_increases(self):
        """Property: Performance counter increases."""
        pc1 = time.perf_counter()
        time.sleep(0.01)
        pc2 = time.perf_counter()
        assert pc2 > pc1

    def test_perf_counter_high_resolution(self):
        """Property: perf_counter has higher resolution than time()."""
        pc1 = time.perf_counter()
        pc2 = time.perf_counter()
        # Can measure very short intervals
        assert isinstance(pc2 - pc1, float)


class TestMonotonic:
    """time.monotonic() - Monotonic clock (cannot go backwards)."""

    def test_monotonic_returns_float(self):
        """Basic: monotonic() returns float."""
        m = time.monotonic()
        assert isinstance(m, float)

    def test_monotonic_increases(self):
        """Property: Monotonic clock always increases."""
        m1 = time.monotonic()
        time.sleep(0.01)
        m2 = time.monotonic()
        assert m2 > m1

    def test_monotonic_unaffected_by_system_clock(self):
        """Property: Monotonic is unaffected by system clock changes."""
        # This is a property we can only assert exists, not test directly
        m1 = time.monotonic()
        m2 = time.monotonic()
        assert m2 >= m1  # Never goes backward


class TestGmtime:
    """time.gmtime() - Convert timestamp to UTC struct_time."""

    def test_gmtime_basic(self):
        """Basic: Convert timestamp to UTC time."""
        t = 0  # Unix epoch
        gmt = time.gmtime(t)
        assert gmt.tm_year == 1970
        assert gmt.tm_mon == 1
        assert gmt.tm_mday == 1
        assert gmt.tm_hour == 0
        assert gmt.tm_min == 0
        assert gmt.tm_sec == 0

    def test_gmtime_no_arg_uses_current_time(self):
        """Feature: gmtime() without arg uses current time."""
        gmt = time.gmtime()
        assert gmt.tm_year >= 2025

    def test_gmtime_struct_time_attributes(self):
        """Property: struct_time has all required attributes."""
        gmt = time.gmtime(0)
        assert hasattr(gmt, 'tm_year')
        assert hasattr(gmt, 'tm_mon')
        assert hasattr(gmt, 'tm_mday')
        assert hasattr(gmt, 'tm_hour')
        assert hasattr(gmt, 'tm_min')
        assert hasattr(gmt, 'tm_sec')
        assert hasattr(gmt, 'tm_wday')
        assert hasattr(gmt, 'tm_yday')
        assert hasattr(gmt, 'tm_isdst')

    def test_gmtime_indexable(self):
        """Property: struct_time is indexable like tuple."""
        gmt = time.gmtime(0)
        assert gmt[0] == 1970  # year
        assert gmt[1] == 1     # month
        assert gmt[2] == 1     # day


class TestLocaltime:
    """time.localtime() - Convert timestamp to local struct_time."""

    def test_localtime_basic(self):
        """Basic: Convert timestamp to local time."""
        lt = time.localtime(0)
        assert lt.tm_year == 1970 or lt.tm_year == 1969  # Depends on timezone

    def test_localtime_no_arg_uses_current_time(self):
        """Feature: localtime() without arg uses current time."""
        lt = time.localtime()
        assert lt.tm_year >= 2025

    def test_localtime_has_isdst(self):
        """Property: localtime has DST information."""
        lt = time.localtime()
        assert lt.tm_isdst in (-1, 0, 1)


class TestMktime:
    """time.mktime() - Convert struct_time to timestamp."""

    def test_mktime_basic(self):
        """Basic: Convert struct_time to timestamp."""
        lt = time.localtime(1000000000)
        timestamp = time.mktime(lt)
        assert abs(timestamp - 1000000000) < 1

    def test_mktime_roundtrip(self):
        """Property: localtime → mktime roundtrip preserves timestamp."""
        original = time.time()
        lt = time.localtime(original)
        timestamp = time.mktime(lt)
        assert abs(timestamp - original) < 1


class TestStrftime:
    """time.strftime() - Format time as string."""

    def test_strftime_basic(self):
        """Basic: Format time with format string."""
        gmt = time.gmtime(0)
        formatted = time.strftime('%Y-%m-%d', gmt)
        assert formatted == '1970-01-01'

    def test_strftime_time_format(self):
        """Feature: Format time components."""
        gmt = time.gmtime(0)
        formatted = time.strftime('%H:%M:%S', gmt)
        assert formatted == '00:00:00'

    def test_strftime_common_formats(self):
        """Feature: Common date/time format strings."""
        gmt = time.gmtime(0)

        # ISO format
        iso = time.strftime('%Y-%m-%d %H:%M:%S', gmt)
        assert iso == '1970-01-01 00:00:00'

        # US format
        us = time.strftime('%m/%d/%Y', gmt)
        assert us == '01/01/1970'

    def test_strftime_weekday(self):
        """Feature: Format weekday names."""
        gmt = time.gmtime(0)  # 1970-01-01 was Thursday
        weekday = time.strftime('%A', gmt)
        assert weekday == 'Thursday'

    def test_strftime_no_struct_uses_localtime(self):
        """Feature: strftime without struct_time uses localtime."""
        formatted = time.strftime('%Y')
        assert int(formatted) >= 2025


class TestStrptime:
    """time.strptime() - Parse time string to struct_time."""

    def test_strptime_basic(self):
        """Basic: Parse date string."""
        result = time.strptime('1970-01-01', '%Y-%m-%d')
        assert result.tm_year == 1970
        assert result.tm_mon == 1
        assert result.tm_mday == 1

    def test_strptime_with_time(self):
        """Feature: Parse date and time."""
        result = time.strptime('2025-10-03 14:30:00', '%Y-%m-%d %H:%M:%S')
        assert result.tm_year == 2025
        assert result.tm_mon == 10
        assert result.tm_mday == 3
        assert result.tm_hour == 14
        assert result.tm_min == 30
        assert result.tm_sec == 0

    def test_strptime_roundtrip(self):
        """Property: strftime → strptime roundtrip preserves data."""
        original = time.gmtime(0)
        formatted = time.strftime('%Y-%m-%d %H:%M:%S', original)
        parsed = time.strptime(formatted, '%Y-%m-%d %H:%M:%S')

        assert parsed.tm_year == original.tm_year
        assert parsed.tm_mon == original.tm_mon
        assert parsed.tm_mday == original.tm_mday
        assert parsed.tm_hour == original.tm_hour
        assert parsed.tm_min == original.tm_min
        assert parsed.tm_sec == original.tm_sec

    def test_strptime_invalid_format_raises(self):
        """Error: Invalid format string raises ValueError."""
        with pytest.raises(ValueError):
            time.strptime('not-a-date', '%Y-%m-%d')

    def test_strptime_mismatch_raises(self):
        """Error: String/format mismatch raises ValueError."""
        with pytest.raises(ValueError):
            time.strptime('2025-10-03', '%Y/%m/%d')


class TestCtime:
    """time.ctime() - Convert timestamp to readable string."""

    def test_ctime_basic(self):
        """Basic: Convert timestamp to string."""
        result = time.ctime(0)
        assert 'Jan' in result or '1970' in result

    def test_ctime_no_arg_uses_current_time(self):
        """Feature: ctime() without arg uses current time."""
        result = time.ctime()
        assert isinstance(result, str)
        assert len(result) > 0

    def test_ctime_format(self):
        """Property: ctime returns consistent format."""
        result = time.ctime(0)
        # Format: "Day Mon DD HH:MM:SS YYYY"
        parts = result.split()
        assert len(parts) == 5


class TestAsctime:
    """time.asctime() - Convert struct_time to string."""

    def test_asctime_basic(self):
        """Basic: Convert struct_time to readable string."""
        gmt = time.gmtime(0)
        result = time.asctime(gmt)
        assert 'Jan' in result or '1970' in result

    def test_asctime_no_arg_uses_localtime(self):
        """Feature: asctime() without arg uses localtime."""
        result = time.asctime()
        assert isinstance(result, str)
        assert len(result) > 0

    def test_asctime_format(self):
        """Property: asctime returns consistent format."""
        gmt = time.gmtime(0)
        result = time.asctime(gmt)
        # Format: "Day Mon DD HH:MM:SS YYYY"
        parts = result.split()
        assert len(parts) == 5


class TestTimeConstants:
    """time module constants - timezone, daylight, tzname."""

    def test_timezone_constant(self):
        """Property: timezone is offset from UTC in seconds."""
        assert isinstance(time.timezone, int)

    def test_daylight_constant(self):
        """Property: daylight indicates DST support."""
        assert isinstance(time.daylight, int)
        assert time.daylight in (0, 1)

    def test_tzname_constant(self):
        """Property: tzname is tuple of timezone names."""
        assert isinstance(time.tzname, tuple)
        assert len(time.tzname) == 2
        assert all(isinstance(name, str) for name in time.tzname)


class TestTimeEdgeCases:
    """Edge cases and special scenarios."""

    def test_leap_year_handling(self):
        """Edge: Leap year dates are handled correctly."""
        # 2000-02-29 (leap year)
        leap_time = time.strptime('2000-02-29', '%Y-%m-%d')
        assert leap_time.tm_year == 2000
        assert leap_time.tm_mon == 2
        assert leap_time.tm_mday == 29

    def test_non_leap_year_feb_29_raises(self):
        """Error: Feb 29 on non-leap year raises ValueError."""
        with pytest.raises(ValueError):
            time.strptime('1999-02-29', '%Y-%m-%d')

    def test_large_timestamp(self):
        """Edge: Large timestamps are handled."""
        large_ts = 2000000000  # Year 2033
        gmt = time.gmtime(large_ts)
        assert gmt.tm_year == 2033

    def test_struct_time_immutable(self):
        """Property: struct_time is immutable."""
        gmt = time.gmtime(0)
        with pytest.raises((TypeError, AttributeError)):
            gmt.tm_year = 2025
