//! Runtime support type generation for transpiled Rust code.
//!
//! This module generates runtime support types that are injected into transpiled output
//! based on what the Python source code requires:
//!
//! - **PythonIntOps** trait (DEPYLER-1202): Provides Python integer method names
//!   (`bit_length`, `bit_count`) on Rust integer types.
//! - **DepylerDate** struct (DEPYLER-1066): Wrapper for Python `datetime.date` with
//!   `.day()`, `.month()`, `.year()` methods.
//! - **DepylerDateTime** struct (DEPYLER-1067): Wrapper for Python `datetime.datetime`
//!   with full date/time component accessors.
//! - **DepylerTimeDelta** struct (DEPYLER-1068): Wrapper for Python `datetime.timedelta`
//!   with normalized components and arithmetic operators.
//! - **DepylerRegexMatch** struct (DEPYLER-1070): Wrapper for Python `re.Match` object
//!   with `.group()`, `.groups()`, `.start()`, `.end()`, `.span()` methods.

use quote::quote;

use super::context::CodeGenContext;

/// Generate runtime support type tokens based on what the transpiled code needs.
/// Returns token streams for:
/// - PythonIntOps trait (if ctx.needs_python_int_ops || nasa_mode)
/// - DepylerDate struct (if ctx.needs_depyler_date || nasa_mode)
/// - DepylerDateTime struct (if ctx.needs_depyler_datetime || nasa_mode)
/// - DepylerTimeDelta struct (if ctx.needs_depyler_timedelta || nasa_mode)
/// - DepylerRegexMatch struct (if ctx.needs_depyler_regex_match || nasa_mode)
pub(super) fn generate_runtime_type_items(ctx: &CodeGenContext) -> Vec<proc_macro2::TokenStream> {
    let nasa_mode = ctx.type_mapper.nasa_mode;
    let mut items = Vec::new();

    // DEPYLER-1202: Inject PythonIntOps trait if Python int methods were detected
    // This trait provides Python method names (bit_length, bit_count) on Rust integer types
    if ctx.needs_python_int_ops || nasa_mode {
        let python_int_ops_trait = quote! {
            /// DEPYLER-1202: Python integer operations for Rust integer types.
            pub trait PythonIntOps {
                fn bit_length(&self) -> u32;
                fn bit_count(&self) -> u32;
            }

            impl PythonIntOps for i32 {
                fn bit_length(&self) -> u32 {
                    if *self == 0 { 0 }
                    else { (std::mem::size_of::<i32>() as u32 * 8) - self.unsigned_abs().leading_zeros() }
                }
                fn bit_count(&self) -> u32 { self.unsigned_abs().count_ones() }
            }

            impl PythonIntOps for i64 {
                fn bit_length(&self) -> u32 {
                    if *self == 0 { 0 }
                    else { (std::mem::size_of::<i64>() as u32 * 8) - self.unsigned_abs().leading_zeros() }
                }
                fn bit_count(&self) -> u32 { self.unsigned_abs().count_ones() }
            }

            impl PythonIntOps for u32 {
                fn bit_length(&self) -> u32 {
                    if *self == 0 { 0 }
                    else { (std::mem::size_of::<u32>() as u32 * 8) - self.leading_zeros() }
                }
                fn bit_count(&self) -> u32 { self.count_ones() }
            }

            impl PythonIntOps for u64 {
                fn bit_length(&self) -> u32 {
                    if *self == 0 { 0 }
                    else { (std::mem::size_of::<u64>() as u32 * 8) - self.leading_zeros() }
                }
                fn bit_count(&self) -> u32 { self.count_ones() }
            }

            impl PythonIntOps for usize {
                fn bit_length(&self) -> u32 {
                    if *self == 0 { 0 }
                    else { (std::mem::size_of::<usize>() as u32 * 8) - self.leading_zeros() }
                }
                fn bit_count(&self) -> u32 { self.count_ones() }
            }

            impl PythonIntOps for isize {
                fn bit_length(&self) -> u32 {
                    if *self == 0 { 0 }
                    else { (std::mem::size_of::<isize>() as u32 * 8) - self.unsigned_abs().leading_zeros() }
                }
                fn bit_count(&self) -> u32 { self.unsigned_abs().count_ones() }
            }
        };
        items.push(python_int_ops_trait);
    }

    // DEPYLER-1066: Inject DepylerDate struct if date types were detected
    // This wrapper struct provides .day(), .month(), .year() methods
    // that Python's datetime.date has, which raw tuples don't have.
    if ctx.needs_depyler_date || nasa_mode {
        let depyler_date_struct = quote! {
            /// DEPYLER-1066: Wrapper for Python datetime.date
            /// Provides .day(), .month(), .year() methods matching Python's API
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
            pub struct DepylerDate(pub u32, pub u32, pub u32);  // (year, month, day)

            impl DepylerDate {
                /// Create a new date from year, month, day
                pub fn new(year: u32, month: u32, day: u32) -> Self {
                    DepylerDate(year, month, day)
                }

                /// Get today's date (NASA mode: computed from SystemTime)
                pub fn today() -> Self {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let secs = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let days = (secs / 86400) as i64;
                    // Algorithm to convert days since epoch to (year, month, day)
                    let z = days + 719468;
                    let era = if z >= 0 { z } else { z - 146096 } / 146097;
                    let doe = (z - era * 146097) as u32;
                    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
                    let y = yoe as i64 + era * 400;
                    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
                    let mp = (5 * doy + 2) / 153;
                    let d = doy - (153 * mp + 2) / 5 + 1;
                    let m = if mp < 10 { mp + 3 } else { mp - 9 };
                    let y = if m <= 2 { y + 1 } else { y };
                    DepylerDate(y as u32, m, d)
                }

                /// Get the year component
                pub fn year(&self) -> u32 {
                    self.0
                }

                /// Get the month component (1-12)
                pub fn month(&self) -> u32 {
                    self.1
                }

                /// Get the day component (1-31)
                pub fn day(&self) -> u32 {
                    self.2
                }

                /// Convert to tuple (year, month, day) for interop
                pub fn to_tuple(&self) -> (u32, u32, u32) {
                    (self.0, self.1, self.2)
                }

                /// Get weekday (0 = Monday, 6 = Sunday) - Python datetime.date.weekday()
                pub fn weekday(&self) -> u32 {
                    // Zeller's congruence for weekday calculation
                    let (mut y, mut m, d) = (self.0 as i32, self.1 as i32, self.2 as i32);
                    if m < 3 {
                        m += 12;
                        y -= 1;
                    }
                    let q = d;
                    let k = y % 100;
                    let j = y / 100;
                    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
                    // Convert from Zeller (0=Sat) to Python (0=Mon)
                    ((h + 5) % 7) as u32
                }

                /// Get ISO weekday (1 = Monday, 7 = Sunday) - Python datetime.date.isoweekday()
                pub fn isoweekday(&self) -> u32 {
                    self.weekday() + 1
                }

                /// Create date from ordinal (days since year 1, January 1 = ordinal 1)
                /// Python: date.fromordinal(730120) -> date(2000, 1, 1)
                pub fn from_ordinal(ordinal: i64) -> Self {
                    // Convert ordinal to days since epoch (ordinal 1 = Jan 1, year 1)
                    // Python ordinal 730120 = 2000-01-01
                    // Epoch ordinal = 719163 (1970-01-01)
                    let days = ordinal - 719163 - 1;  // Adjust to days since epoch
                    // Use same algorithm as today()
                    let z = days + 719468;
                    let era = if z >= 0 { z } else { z - 146096 } / 146097;
                    let doe = (z - era * 146097) as u32;
                    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
                    let y = yoe as i64 + era * 400;
                    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
                    let mp = (5 * doy + 2) / 153;
                    let d = doy - (153 * mp + 2) / 5 + 1;
                    let m = if mp < 10 { mp + 3 } else { mp - 9 };
                    let y = if m <= 2 { y + 1 } else { y };
                    DepylerDate(y as u32, m, d)
                }
            }

            impl std::fmt::Display for DepylerDate {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:04}-{:02}-{:02}", self.0, self.1, self.2)
                }
            }
        };
        items.push(depyler_date_struct);
    }

    // DEPYLER-1067: Inject DepylerDateTime struct if datetime types were detected
    if ctx.needs_depyler_datetime || nasa_mode {
        let depyler_datetime_struct = quote! {
            /// DEPYLER-1067: Wrapper for Python datetime.datetime
            /// Provides .year(), .month(), .day(), .hour(), .minute(), .second(), .microsecond() methods
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
            pub struct DepylerDateTime {
                pub year: u32,
                pub month: u32,
                pub day: u32,
                pub hour: u32,
                pub minute: u32,
                pub second: u32,
                pub microsecond: u32,
            }

            impl DepylerDateTime {
                /// Create a new datetime from components
                pub fn new(year: u32, month: u32, day: u32, hour: u32, minute: u32, second: u32, microsecond: u32) -> Self {
                    DepylerDateTime { year, month, day, hour, minute, second, microsecond }
                }

                /// Get current datetime (NASA mode: computed from SystemTime)
                pub fn now() -> Self {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let secs = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let nanos = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.subsec_nanos())
                        .unwrap_or(0);
                    let days = (secs / 86400) as i64;
                    let day_secs = (secs % 86400) as u32;
                    // Date from days since epoch
                    let z = days + 719468;
                    let era = if z >= 0 { z } else { z - 146096 } / 146097;
                    let doe = (z - era * 146097) as u32;
                    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
                    let y = yoe as i64 + era * 400;
                    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
                    let mp = (5 * doy + 2) / 153;
                    let d = doy - (153 * mp + 2) / 5 + 1;
                    let m = if mp < 10 { mp + 3 } else { mp - 9 };
                    let y = if m <= 2 { y + 1 } else { y };
                    // Time from seconds within day
                    let hour = day_secs / 3600;
                    let minute = (day_secs % 3600) / 60;
                    let second = day_secs % 60;
                    let microsecond = nanos / 1000;
                    DepylerDateTime { year: y as u32, month: m, day: d, hour, minute, second, microsecond }
                }

                /// Alias for now() - Python datetime.datetime.today()
                pub fn today() -> Self { Self::now() }

                pub fn year(&self) -> u32 { self.year }
                pub fn month(&self) -> u32 { self.month }
                pub fn day(&self) -> u32 { self.day }
                pub fn hour(&self) -> u32 { self.hour }
                pub fn minute(&self) -> u32 { self.minute }
                pub fn second(&self) -> u32 { self.second }
                pub fn microsecond(&self) -> u32 { self.microsecond }

                /// Get weekday (0 = Monday, 6 = Sunday)
                pub fn weekday(&self) -> u32 {
                    DepylerDate::new(self.year, self.month, self.day).weekday()
                }

                /// Get ISO weekday (1 = Monday, 7 = Sunday)
                pub fn isoweekday(&self) -> u32 {
                    self.weekday() + 1
                }

                /// Extract date component
                pub fn date(&self) -> DepylerDate {
                    DepylerDate::new(self.year, self.month, self.day)
                }

                /// Get Unix timestamp
                pub fn timestamp(&self) -> f64 {
                    // Simplified: calculate seconds since epoch
                    let days = self.days_since_epoch();
                    let secs = days as f64 * 86400.0
                        + self.hour as f64 * 3600.0
                        + self.minute as f64 * 60.0
                        + self.second as f64
                        + self.microsecond as f64 / 1_000_000.0;
                    secs
                }

                fn days_since_epoch(&self) -> i64 {
                    // Calculate days from 1970-01-01
                    let (mut y, mut m) = (self.year as i64, self.month as i64);
                    if m <= 2 { y -= 1; m += 12; }
                    let era = if y >= 0 { y } else { y - 399 } / 400;
                    let yoe = (y - era * 400) as u32;
                    let doy = (153 * (m as u32 - 3) + 2) / 5 + self.day - 1;
                    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
                    era * 146097 + doe as i64 - 719468
                }

                /// Create from Unix timestamp
                pub fn fromtimestamp(ts: f64) -> Self {
                    let secs = ts as u64;
                    let microsecond = ((ts - secs as f64) * 1_000_000.0) as u32;
                    let days = (secs / 86400) as i64;
                    let day_secs = (secs % 86400) as u32;
                    let z = days + 719468;
                    let era = if z >= 0 { z } else { z - 146096 } / 146097;
                    let doe = (z - era * 146097) as u32;
                    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
                    let y = yoe as i64 + era * 400;
                    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
                    let mp = (5 * doy + 2) / 153;
                    let d = doy - (153 * mp + 2) / 5 + 1;
                    let m = if mp < 10 { mp + 3 } else { mp - 9 };
                    let y = if m <= 2 { y + 1 } else { y };
                    let hour = day_secs / 3600;
                    let minute = (day_secs % 3600) / 60;
                    let second = day_secs % 60;
                    DepylerDateTime { year: y as u32, month: m, day: d, hour, minute, second, microsecond }
                }

                /// ISO format string
                pub fn isoformat(&self) -> String {
                    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                        self.year, self.month, self.day, self.hour, self.minute, self.second)
                }
            }

            impl std::fmt::Display for DepylerDateTime {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                        self.year, self.month, self.day, self.hour, self.minute, self.second)
                }
            }
        };
        items.push(depyler_datetime_struct);
    }

    // DEPYLER-1068: Inject DepylerTimeDelta struct if timedelta types were detected
    if ctx.needs_depyler_timedelta || nasa_mode {
        let depyler_timedelta_struct = quote! {
            /// DEPYLER-1068: Wrapper for Python datetime.timedelta
            /// Provides .days, .seconds, .microseconds, .total_seconds() methods
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
            pub struct DepylerTimeDelta {
                pub days: i64,
                pub seconds: i64,
                pub microseconds: i64,
            }

            impl DepylerTimeDelta {
                /// Create a new timedelta from components
                pub fn new(days: i64, seconds: i64, microseconds: i64) -> Self {
                    // Normalize: microseconds < 1_000_000, seconds < 86400
                    let total_us = days * 86400 * 1_000_000 + seconds * 1_000_000 + microseconds;
                    let total_secs = total_us / 1_000_000;
                    let us = total_us % 1_000_000;
                    let d = total_secs / 86400;
                    let s = total_secs % 86400;
                    DepylerTimeDelta { days: d, seconds: s, microseconds: us }
                }

                /// Create from keyword-style arguments (hours, minutes, etc.)
                pub fn from_components(
                    days: i64,
                    seconds: i64,
                    microseconds: i64,
                    milliseconds: i64,
                    minutes: i64,
                    hours: i64,
                    weeks: i64,
                ) -> Self {
                    let total_days = days + weeks * 7;
                    let total_secs = seconds + minutes * 60 + hours * 3600;
                    let total_us = microseconds + milliseconds * 1000;
                    Self::new(total_days, total_secs, total_us)
                }

                /// Get total seconds as f64
                pub fn total_seconds(&self) -> f64 {
                    self.days as f64 * 86400.0
                        + self.seconds as f64
                        + self.microseconds as f64 / 1_000_000.0
                }

                /// Get days component
                pub fn days(&self) -> i64 { self.days }

                /// Get seconds component (0-86399)
                pub fn seconds(&self) -> i64 { self.seconds }

                /// Get microseconds component (0-999999)
                pub fn microseconds(&self) -> i64 { self.microseconds }
            }

            impl std::ops::Add for DepylerTimeDelta {
                type Output = Self;
                fn add(self, other: Self) -> Self {
                    Self::new(
                        self.days + other.days,
                        self.seconds + other.seconds,
                        self.microseconds + other.microseconds,
                    )
                }
            }

            impl std::ops::Sub for DepylerTimeDelta {
                type Output = Self;
                fn sub(self, other: Self) -> Self {
                    Self::new(
                        self.days - other.days,
                        self.seconds - other.seconds,
                        self.microseconds - other.microseconds,
                    )
                }
            }

            impl std::fmt::Display for DepylerTimeDelta {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let hours = self.seconds / 3600;
                    let mins = (self.seconds % 3600) / 60;
                    let secs = self.seconds % 60;
                    if self.days != 0 {
                        write!(f, "{} day{}, {:02}:{:02}:{:02}",
                            self.days, if self.days == 1 { "" } else { "s" }, hours, mins, secs)
                    } else {
                        write!(f, "{:02}:{:02}:{:02}", hours, mins, secs)
                    }
                }
            }
        };
        items.push(depyler_timedelta_struct);
    }

    // DEPYLER-1070: Inject DepylerRegexMatch struct if regex patterns were detected
    if ctx.needs_depyler_regex_match || nasa_mode {
        let depyler_regex_match_struct = quote! {
            /// DEPYLER-1070: Wrapper for Python re.Match object
            /// Provides .group(), .groups(), .start(), .end(), .span() methods
            #[derive(Debug, Clone, PartialEq, Eq, Default)]
            pub struct DepylerRegexMatch {
                pub matched: String,
                pub start: usize,
                pub end: usize,
                pub groups: Vec<String>,
            }

            impl DepylerRegexMatch {
                /// Create a new match from a string slice match
                pub fn new(text: &str, start: usize, end: usize) -> Self {
                    DepylerRegexMatch {
                        matched: text[start..end].to_string(),
                        start,
                        end,
                        groups: vec![text[start..end].to_string()],
                    }
                }

                /// Create a match with capture groups
                pub fn with_groups(text: &str, start: usize, end: usize, groups: Vec<String>) -> Self {
                    DepylerRegexMatch {
                        matched: text[start..end].to_string(),
                        start,
                        end,
                        groups,
                    }
                }

                /// Get the matched string (group 0)
                pub fn group(&self, n: usize) -> String {
                    self.groups.get(n).cloned().unwrap_or_default()
                }

                /// Get all capture groups as a tuple-like Vec
                pub fn groups(&self) -> Vec<String> {
                    if self.groups.len() > 1 {
                        self.groups[1..].to_vec()  // Exclude group 0 like Python
                    } else {
                        vec![]
                    }
                }

                /// Get the start position
                pub fn start(&self) -> usize {
                    self.start
                }

                /// Get the end position
                pub fn end(&self) -> usize {
                    self.end
                }

                /// Get (start, end) tuple
                pub fn span(&self) -> (usize, usize) {
                    (self.start, self.end)
                }

                /// Get the matched string (equivalent to group(0))
                pub fn as_str(&self) -> &str {
                    &self.matched
                }

                /// Simple pattern search (NASA mode alternative to regex)
                /// Searches for literal string pattern in text
                pub fn search(pattern: &str, text: &str) -> Option<Self> {
                    text.find(pattern).map(|start| {
                        let end = start + pattern.len();
                        DepylerRegexMatch::new(text, start, end)
                    })
                }

                /// Simple pattern match at start (NASA mode alternative to regex)
                pub fn match_start(pattern: &str, text: &str) -> Option<Self> {
                    if text.starts_with(pattern) {
                        Some(DepylerRegexMatch::new(text, 0, pattern.len()))
                    } else {
                        None
                    }
                }

                /// Find all occurrences (NASA mode alternative to regex findall)
                pub fn findall(pattern: &str, text: &str) -> Vec<String> {
                    let mut results = Vec::new();
                    let mut start = 0;
                    while let Some(pos) = text[start..].find(pattern) {
                        results.push(pattern.to_string());
                        start += pos + pattern.len();
                    }
                    results
                }

                /// Simple string replacement (NASA mode alternative to regex sub)
                pub fn sub(pattern: &str, repl: &str, text: &str) -> String {
                    text.replace(pattern, repl)
                }

                /// Simple string split (NASA mode alternative to regex split)
                pub fn split(pattern: &str, text: &str) -> Vec<String> {
                    text.split(pattern).map(|s| s.to_string()).collect()
                }
            }
        };
        items.push(depyler_regex_match_struct);
    }

    items
}
