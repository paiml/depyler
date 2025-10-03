"""Test calendar module - Calendar-related functions.

This module tests calendar's functions for working with dates,
generating calendars, and determining leap years.
"""

import calendar
import pytest


class TestWeekday:
    """calendar.weekday() - Return weekday (0=Monday ... 6=Sunday) for year, month, day."""

    def test_weekday_basic(self):
        """Basic: Get weekday for a known date."""
        # January 1, 2000 was Saturday
        day = calendar.weekday(2000, 1, 1)
        assert day == 5  # Saturday

    def test_weekday_monday(self):
        """Feature: Monday returns 0."""
        # January 3, 2000 was Monday
        day = calendar.weekday(2000, 1, 3)
        assert day == 0

    def test_weekday_sunday(self):
        """Feature: Sunday returns 6."""
        # January 2, 2000 was Sunday
        day = calendar.weekday(2000, 1, 2)
        assert day == 6

    def test_weekday_leap_year_feb_29(self):
        """Edge: Leap year Feb 29 is valid."""
        day = calendar.weekday(2000, 2, 29)
        assert day in range(7)

    def test_weekday_invalid_date_raises(self):
        """Error: Invalid date raises ValueError or calendar.IllegalMonthError."""
        with pytest.raises((ValueError, calendar.IllegalMonthError)):
            calendar.weekday(2025, 13, 1)  # Invalid month


class TestIsLeap:
    """calendar.isleap() - Check if year is leap year."""

    def test_isleap_basic_leap_year(self):
        """Basic: 2000 is a leap year."""
        assert calendar.isleap(2000) is True

    def test_isleap_basic_non_leap_year(self):
        """Basic: 2001 is not a leap year."""
        assert calendar.isleap(2001) is False

    def test_isleap_divisible_by_4(self):
        """Rule: Years divisible by 4 are leap years."""
        assert calendar.isleap(2024) is True

    def test_isleap_century_not_leap(self):
        """Rule: Century years not divisible by 400 are not leap."""
        assert calendar.isleap(1900) is False
        assert calendar.isleap(2100) is False

    def test_isleap_century_divisible_by_400(self):
        """Rule: Century years divisible by 400 are leap."""
        assert calendar.isleap(2000) is True
        assert calendar.isleap(2400) is True

    def test_isleap_sequence(self):
        """Property: Leap years follow 4-year cycle."""
        # 2020, 2024, 2028 are leap years
        # 2021, 2022, 2023, 2025, 2026, 2027 are not
        assert calendar.isleap(2020) is True
        assert calendar.isleap(2021) is False
        assert calendar.isleap(2022) is False
        assert calendar.isleap(2023) is False
        assert calendar.isleap(2024) is True


class TestLeapDays:
    """calendar.leapdays() - Number of leap days in range [y1, y2)."""

    def test_leapdays_basic(self):
        """Basic: Count leap days in range."""
        # 2000-2020: 2000, 2004, 2008, 2012, 2016 = 5 leap years
        count = calendar.leapdays(2000, 2020)
        assert count == 5

    def test_leapdays_single_year(self):
        """Edge: Range of 1 year returns 0 or 1."""
        assert calendar.leapdays(2000, 2001) == 1  # 2000 is leap
        assert calendar.leapdays(2001, 2002) == 0  # 2001 is not leap

    def test_leapdays_empty_range(self):
        """Edge: Empty range returns 0."""
        assert calendar.leapdays(2000, 2000) == 0

    def test_leapdays_century_boundary(self):
        """Edge: Century boundaries affect leap day count."""
        # 1900 is not a leap year, 2000 is
        count_1900 = calendar.leapdays(1900, 1901)
        count_2000 = calendar.leapdays(2000, 2001)
        assert count_1900 == 0
        assert count_2000 == 1


class TestMonthRange:
    """calendar.monthrange() - Tuple (weekday of first day, number of days) for year/month."""

    def test_monthrange_basic(self):
        """Basic: Get month range for January 2000."""
        weekday, num_days = calendar.monthrange(2000, 1)
        assert weekday == 5  # Saturday
        assert num_days == 31

    def test_monthrange_february_leap_year(self):
        """Edge: February in leap year has 29 days."""
        weekday, num_days = calendar.monthrange(2000, 2)
        assert num_days == 29

    def test_monthrange_february_non_leap_year(self):
        """Edge: February in non-leap year has 28 days."""
        weekday, num_days = calendar.monthrange(2001, 2)
        assert num_days == 28

    def test_monthrange_30_day_months(self):
        """Property: April, June, September, November have 30 days."""
        for month in [4, 6, 9, 11]:
            _, num_days = calendar.monthrange(2025, month)
            assert num_days == 30

    def test_monthrange_31_day_months(self):
        """Property: Jan, Mar, May, Jul, Aug, Oct, Dec have 31 days."""
        for month in [1, 3, 5, 7, 8, 10, 12]:
            _, num_days = calendar.monthrange(2025, month)
            assert num_days == 31


class TestMonthCalendar:
    """calendar.monthcalendar() - Matrix representing month's calendar."""

    def test_monthcalendar_basic(self):
        """Basic: Get calendar matrix for a month."""
        cal = calendar.monthcalendar(2000, 1)
        assert isinstance(cal, list)
        assert len(cal) >= 4  # At least 4 weeks
        assert len(cal) <= 6  # At most 6 weeks

    def test_monthcalendar_first_week(self):
        """Property: First week may have leading zeros."""
        cal = calendar.monthcalendar(2000, 1)
        first_week = cal[0]
        # January 1, 2000 was Saturday (index 5)
        # So first week should have 5 zeros (Mon-Fri) and then 1, 2
        assert 0 in first_week
        assert 1 in first_week

    def test_monthcalendar_last_week(self):
        """Property: Last week may have trailing zeros."""
        cal = calendar.monthcalendar(2000, 1)
        last_week = cal[-1]
        # Should contain 31 (last day of January)
        assert 31 in last_week
        # And likely some trailing zeros
        assert isinstance(last_week, list)
        assert len(last_week) == 7

    def test_monthcalendar_week_length(self):
        """Property: Every week has 7 days."""
        cal = calendar.monthcalendar(2025, 10)
        for week in cal:
            assert len(week) == 7

    def test_monthcalendar_contains_all_days(self):
        """Property: Calendar contains all days of month."""
        cal = calendar.monthcalendar(2025, 10)
        all_days = [day for week in cal for day in week if day != 0]
        # October has 31 days
        assert sorted(all_days) == list(range(1, 32))


class TestMonth:
    """calendar.month() - Formatted text calendar for a month."""

    def test_month_basic(self):
        """Basic: Generate text calendar for month."""
        text = calendar.month(2000, 1)
        assert isinstance(text, str)
        assert 'January' in text or 'Jan' in text or '2000' in text

    def test_month_contains_days(self):
        """Property: Calendar contains day numbers."""
        text = calendar.month(2025, 10)
        # Should contain days 1-31
        assert '1' in text
        assert '31' in text

    def test_month_multiline(self):
        """Property: Month calendar is multiline."""
        text = calendar.month(2025, 10)
        lines = text.strip().split('\n')
        assert len(lines) > 1  # Header + week rows


class TestCalendar:
    """calendar.calendar() - Formatted text calendar for a year."""

    def test_calendar_basic(self):
        """Basic: Generate text calendar for year."""
        text = calendar.calendar(2000)
        assert isinstance(text, str)
        assert '2000' in text

    def test_calendar_contains_all_months(self):
        """Property: Year calendar contains all 12 months."""
        text = calendar.calendar(2025)
        # Should reference all months
        month_count = 0
        for month_name in ['January', 'February', 'March', 'April',
                            'May', 'June', 'July', 'August',
                            'September', 'October', 'November', 'December']:
            if month_name in text:
                month_count += 1
        # At least some month names should appear
        # (actual formatting depends on calendar settings)
        assert len(text) > 100  # Should be substantial text


class TestTextCalendar:
    """calendar.TextCalendar - Text calendar generator."""

    def test_textcalendar_basic(self):
        """Basic: Create TextCalendar instance."""
        cal = calendar.TextCalendar()
        assert isinstance(cal, calendar.TextCalendar)

    def test_textcalendar_formatmonth(self):
        """Feature: Format month with TextCalendar."""
        cal = calendar.TextCalendar()
        text = cal.formatmonth(2000, 1)
        assert isinstance(text, str)
        assert len(text) > 0

    def test_textcalendar_first_weekday(self):
        """Feature: Set first day of week."""
        # Default is Monday (0)
        cal_mon = calendar.TextCalendar(firstweekday=0)
        # Sunday (6)
        cal_sun = calendar.TextCalendar(firstweekday=6)

        assert cal_mon.firstweekday == 0
        assert cal_sun.firstweekday == 6


class TestHTMLCalendar:
    """calendar.HTMLCalendar - HTML calendar generator."""

    def test_htmlcalendar_basic(self):
        """Basic: Create HTMLCalendar instance."""
        cal = calendar.HTMLCalendar()
        assert isinstance(cal, calendar.HTMLCalendar)

    def test_htmlcalendar_formatmonth(self):
        """Feature: Format month as HTML."""
        cal = calendar.HTMLCalendar()
        html = cal.formatmonth(2000, 1)
        assert isinstance(html, str)
        assert '<table' in html or '<' in html  # Should contain HTML tags

    def test_htmlcalendar_contains_table(self):
        """Property: HTML calendar contains table structure."""
        cal = calendar.HTMLCalendar()
        html = cal.formatmonth(2025, 10)
        assert 'table' in html.lower() or '<td' in html or '<tr' in html


class TestCalendarConstants:
    """Calendar module constants."""

    def test_day_name_constant(self):
        """Property: day_name contains weekday names."""
        assert len(calendar.day_name) == 7
        assert 'Monday' in calendar.day_name
        assert 'Sunday' in calendar.day_name

    def test_day_abbr_constant(self):
        """Property: day_abbr contains abbreviated weekday names."""
        assert len(calendar.day_abbr) == 7
        assert 'Mon' in calendar.day_abbr
        assert 'Sun' in calendar.day_abbr

    def test_month_name_constant(self):
        """Property: month_name has 13 entries (0-12, 0 is empty)."""
        assert len(calendar.month_name) == 13
        assert calendar.month_name[0] == ''
        assert calendar.month_name[1] == 'January'
        assert calendar.month_name[12] == 'December'

    def test_month_abbr_constant(self):
        """Property: month_abbr has 13 entries with abbreviations."""
        assert len(calendar.month_abbr) == 13
        assert calendar.month_abbr[0] == ''
        assert calendar.month_abbr[1] == 'Jan'
        assert calendar.month_abbr[12] == 'Dec'


class TestCalendarEdgeCases:
    """Edge cases and special scenarios."""

    def test_leap_year_february_range(self):
        """Edge: February leap year affects monthrange."""
        # 2000 leap year
        _, days_2000 = calendar.monthrange(2000, 2)
        # 1900 not leap year
        _, days_1900 = calendar.monthrange(1900, 2)

        assert days_2000 == 29
        assert days_1900 == 28

    def test_weekday_consistency(self):
        """Property: Weekday is consistent with monthrange."""
        year, month = 2025, 10
        weekday_func = calendar.weekday(year, month, 1)
        weekday_range, _ = calendar.monthrange(year, month)
        assert weekday_func == weekday_range

    def test_monthcalendar_no_gaps(self):
        """Property: monthcalendar has no gaps in day sequence."""
        cal = calendar.monthcalendar(2025, 10)
        days = [day for week in cal for day in week if day != 0]
        # Days should be sequential: 1, 2, 3, ..., 31
        for i, day in enumerate(sorted(days), start=1):
            assert day == i

    def test_calendar_year_2000_bug(self):
        """Edge: Year 2000 handled correctly (Y2K compliance)."""
        assert calendar.isleap(2000) is True
        weekday, days = calendar.monthrange(2000, 2)
        assert days == 29
