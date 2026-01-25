//! Python-compatible operations for Rust types.
//!
//! This module provides extension traits that implement Python method semantics
//! on Rust types. When the transpiler cannot directly translate a method call,
//! these traits provide a fallback that maintains Python behavior.
//!
//! # Design Philosophy (DEPYLER-1202)
//!
//! Instead of requiring the transpiler to handle every possible method translation,
//! we provide trait-based extensions that:
//! 1. Bridge Python semantics to idiomatic Rust
//! 2. Maintain type safety at compile time
//! 3. Allow transpiled code to call Python methods directly on Rust types
//!
//! # Example
//!
//! ```rust
//! use depyler::prelude::*;
//!
//! let s = String::from("HELLO");
//! assert_eq!(s.lower(), "hello");
//!
//! let n: i32 = 42;
//! assert_eq!(n.bit_length(), 6);
//! ```

/// Python string operations for Rust String and &str types.
///
/// This trait provides Python's string methods that don't have direct
/// equivalents in Rust's standard library.
pub trait PythonStringOps {
    /// Convert to lowercase (Python's `str.lower()`).
    fn lower(&self) -> String;

    /// Convert to uppercase (Python's `str.upper()`).
    fn upper(&self) -> String;

    /// Remove leading and trailing whitespace (Python's `str.strip()`).
    fn strip(&self) -> String;

    /// Remove leading whitespace (Python's `str.lstrip()`).
    fn lstrip(&self) -> String;

    /// Remove trailing whitespace (Python's `str.rstrip()`).
    fn rstrip(&self) -> String;

    /// Split on whitespace (Python's `str.split()` with no args).
    fn split_py(&self) -> Vec<String>;

    /// Split on delimiter (Python's `str.split(sep)`).
    fn split_on(&self, sep: &str) -> Vec<String>;

    /// Check if string starts with prefix (Python's `str.startswith()`).
    fn startswith(&self, prefix: &str) -> bool;

    /// Check if string ends with suffix (Python's `str.endswith()`).
    fn endswith(&self, suffix: &str) -> bool;

    /// Replace occurrences (Python's `str.replace()`).
    fn replace_py(&self, old: &str, new: &str) -> String;

    /// Find substring index (Python's `str.find()`).
    /// Returns -1 if not found (Python semantics).
    fn find(&self, sub: &str) -> i64;

    /// Count non-overlapping occurrences (Python's `str.count()`).
    fn count_py(&self, sub: &str) -> usize;

    /// Check if all chars are alphabetic (Python's `str.isalpha()`).
    fn isalpha(&self) -> bool;

    /// Check if all chars are digits (Python's `str.isdigit()`).
    fn isdigit(&self) -> bool;

    /// Check if all chars are alphanumeric (Python's `str.isalnum()`).
    fn isalnum(&self) -> bool;

    /// Check if all chars are whitespace (Python's `str.isspace()`).
    fn isspace(&self) -> bool;

    /// Check if string is lowercase (Python's `str.islower()`).
    fn islower(&self) -> bool;

    /// Check if string is uppercase (Python's `str.isupper()`).
    fn isupper(&self) -> bool;

    /// Capitalize first char (Python's `str.capitalize()`).
    fn capitalize(&self) -> String;

    /// Title case (Python's `str.title()`).
    fn title(&self) -> String;

    /// Swap case (Python's `str.swapcase()`).
    fn swapcase(&self) -> String;

    /// Center string in width (Python's `str.center()`).
    fn center(&self, width: usize) -> String;

    /// Left justify (Python's `str.ljust()`).
    fn ljust(&self, width: usize) -> String;

    /// Right justify (Python's `str.rjust()`).
    fn rjust(&self, width: usize) -> String;

    /// Zero fill (Python's `str.zfill()`).
    fn zfill(&self, width: usize) -> String;

    /// Join iterable (Python's `str.join()`).
    fn join_py<I, S>(&self, iterable: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;

    /// Encode to bytes (Python's `str.encode()`).
    fn encode(&self) -> Vec<u8>;
}

impl PythonStringOps for String {
    fn lower(&self) -> String {
        self.to_lowercase()
    }

    fn upper(&self) -> String {
        self.to_uppercase()
    }

    fn strip(&self) -> String {
        self.trim().to_string()
    }

    fn lstrip(&self) -> String {
        self.trim_start().to_string()
    }

    fn rstrip(&self) -> String {
        self.trim_end().to_string()
    }

    fn split_py(&self) -> Vec<String> {
        self.split_whitespace().map(|s| s.to_string()).collect()
    }

    fn split_on(&self, sep: &str) -> Vec<String> {
        self.split(sep).map(|s| s.to_string()).collect()
    }

    fn startswith(&self, prefix: &str) -> bool {
        self.starts_with(prefix)
    }

    fn endswith(&self, suffix: &str) -> bool {
        self.ends_with(suffix)
    }

    fn replace_py(&self, old: &str, new: &str) -> String {
        self.replace(old, new)
    }

    fn find(&self, sub: &str) -> i64 {
        self.as_str().find(sub).map(|i| i as i64).unwrap_or(-1)
    }

    fn count_py(&self, sub: &str) -> usize {
        self.matches(sub).count()
    }

    fn isalpha(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_alphabetic())
    }

    fn isdigit(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_ascii_digit())
    }

    fn isalnum(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_alphanumeric())
    }

    fn isspace(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_whitespace())
    }

    fn islower(&self) -> bool {
        let has_cased = self.chars().any(|c| c.is_alphabetic());
        has_cased
            && self
                .chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_lowercase())
    }

    fn isupper(&self) -> bool {
        let has_cased = self.chars().any(|c| c.is_alphabetic());
        has_cased
            && self
                .chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_uppercase())
    }

    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => {
                first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
            }
        }
    }

    fn title(&self) -> String {
        let mut result = String::with_capacity(self.len());
        let mut capitalize_next = true;
        for c in self.chars() {
            if c.is_whitespace() {
                result.push(c);
                capitalize_next = true;
            } else if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(c.to_lowercase());
            }
        }
        result
    }

    fn swapcase(&self) -> String {
        self.chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().collect::<String>()
                } else {
                    c.to_uppercase().collect::<String>()
                }
            })
            .collect()
    }

    fn center(&self, width: usize) -> String {
        let len = self.chars().count();
        if len >= width {
            return self.clone();
        }
        let total_padding = width - len;
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;
        format!(
            "{}{}{}",
            " ".repeat(left_padding),
            self,
            " ".repeat(right_padding)
        )
    }

    fn ljust(&self, width: usize) -> String {
        let len = self.chars().count();
        if len >= width {
            return self.clone();
        }
        format!("{}{}", self, " ".repeat(width - len))
    }

    fn rjust(&self, width: usize) -> String {
        let len = self.chars().count();
        if len >= width {
            return self.clone();
        }
        format!("{}{}", " ".repeat(width - len), self)
    }

    fn zfill(&self, width: usize) -> String {
        let len = self.chars().count();
        if len >= width {
            return self.clone();
        }
        // Handle negative numbers - keep the sign at the front
        if self.starts_with('-') || self.starts_with('+') {
            let sign = &self[..1];
            let rest = &self[1..];
            format!("{}{}{}", sign, "0".repeat(width - len), rest)
        } else {
            format!("{}{}", "0".repeat(width - len), self)
        }
    }

    fn join_py<I, S>(&self, iterable: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        iterable
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(self)
    }

    fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl PythonStringOps for str {
    fn lower(&self) -> String {
        self.to_lowercase()
    }

    fn upper(&self) -> String {
        self.to_uppercase()
    }

    fn strip(&self) -> String {
        self.trim().to_string()
    }

    fn lstrip(&self) -> String {
        self.trim_start().to_string()
    }

    fn rstrip(&self) -> String {
        self.trim_end().to_string()
    }

    fn split_py(&self) -> Vec<String> {
        self.split_whitespace().map(|s| s.to_string()).collect()
    }

    fn split_on(&self, sep: &str) -> Vec<String> {
        self.split(sep).map(|s| s.to_string()).collect()
    }

    fn startswith(&self, prefix: &str) -> bool {
        self.starts_with(prefix)
    }

    fn endswith(&self, suffix: &str) -> bool {
        self.ends_with(suffix)
    }

    fn replace_py(&self, old: &str, new: &str) -> String {
        self.replace(old, new)
    }

    fn find(&self, sub: &str) -> i64 {
        std::str::from_utf8(self.as_bytes())
            .ok()
            .and_then(|s| s.find(sub))
            .map(|i| i as i64)
            .unwrap_or(-1)
    }

    fn count_py(&self, sub: &str) -> usize {
        self.matches(sub).count()
    }

    fn isalpha(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_alphabetic())
    }

    fn isdigit(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_ascii_digit())
    }

    fn isalnum(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_alphanumeric())
    }

    fn isspace(&self) -> bool {
        !self.is_empty() && self.chars().all(|c| c.is_whitespace())
    }

    fn islower(&self) -> bool {
        let has_cased = self.chars().any(|c| c.is_alphabetic());
        has_cased
            && self
                .chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_lowercase())
    }

    fn isupper(&self) -> bool {
        let has_cased = self.chars().any(|c| c.is_alphabetic());
        has_cased
            && self
                .chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_uppercase())
    }

    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => {
                first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
            }
        }
    }

    fn title(&self) -> String {
        let mut result = String::with_capacity(self.len());
        let mut capitalize_next = true;
        for c in self.chars() {
            if c.is_whitespace() {
                result.push(c);
                capitalize_next = true;
            } else if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(c.to_lowercase());
            }
        }
        result
    }

    fn swapcase(&self) -> String {
        self.chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().collect::<String>()
                } else {
                    c.to_uppercase().collect::<String>()
                }
            })
            .collect()
    }

    fn center(&self, width: usize) -> String {
        let len = self.chars().count();
        if len >= width {
            return self.to_string();
        }
        let total_padding = width - len;
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;
        format!(
            "{}{}{}",
            " ".repeat(left_padding),
            self,
            " ".repeat(right_padding)
        )
    }

    fn ljust(&self, width: usize) -> String {
        let len = self.chars().count();
        if len >= width {
            return self.to_string();
        }
        format!("{}{}", self, " ".repeat(width - len))
    }

    fn rjust(&self, width: usize) -> String {
        let len = self.chars().count();
        if len >= width {
            return self.to_string();
        }
        format!("{}{}", " ".repeat(width - len), self)
    }

    fn zfill(&self, width: usize) -> String {
        let len = self.chars().count();
        if len >= width {
            return self.to_string();
        }
        if self.starts_with('-') || self.starts_with('+') {
            let sign = &self[..1];
            let rest = &self[1..];
            format!("{}{}{}", sign, "0".repeat(width - len), rest)
        } else {
            format!("{}{}", "0".repeat(width - len), self)
        }
    }

    fn join_py<I, S>(&self, iterable: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        iterable
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(self)
    }

    fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

/// Python integer operations for Rust integer types.
///
/// This trait provides Python's int methods that don't have direct
/// equivalents in Rust's standard library.
pub trait PythonIntOps {
    /// Number of bits needed to represent this number (Python's `int.bit_length()`).
    fn bit_length(&self) -> u32;

    /// Number of 1 bits in binary representation (Python's `int.bit_count()`).
    fn bit_count(&self) -> u32;

    /// Convert to bytes (Python's `int.to_bytes()`).
    fn to_bytes_py(&self, length: usize, byteorder: &str) -> Vec<u8>;
}

macro_rules! impl_python_int_ops_signed {
    ($($t:ty),*) => {
        $(
            impl PythonIntOps for $t {
                fn bit_length(&self) -> u32 {
                    if *self == 0 {
                        0
                    } else {
                        // For signed integers, we need the absolute value
                        let abs_val = self.unsigned_abs();
                        (std::mem::size_of::<$t>() as u32 * 8) - abs_val.leading_zeros()
                    }
                }

                fn bit_count(&self) -> u32 {
                    // Python's bit_count() works on absolute value for signed
                    self.unsigned_abs().count_ones()
                }

                fn to_bytes_py(&self, length: usize, byteorder: &str) -> Vec<u8> {
                    let bytes = match byteorder {
                        "big" => self.to_be_bytes(),
                        "little" | _ => self.to_le_bytes(),
                    };
                    let mut result: Vec<u8> = bytes.to_vec();
                    if result.len() < length {
                        // Extend with sign bit
                        let fill = if *self < 0 { 0xFF } else { 0x00 };
                        if byteorder == "big" {
                            let mut padded = vec![fill; length - result.len()];
                            padded.extend(result);
                            result = padded;
                        } else {
                            result.resize(length, fill);
                        }
                    } else if result.len() > length {
                        if byteorder == "big" {
                            result = result[result.len() - length..].to_vec();
                        } else {
                            result.truncate(length);
                        }
                    }
                    result
                }
            }
        )*
    };
}

macro_rules! impl_python_int_ops_unsigned {
    ($($t:ty),*) => {
        $(
            impl PythonIntOps for $t {
                fn bit_length(&self) -> u32 {
                    if *self == 0 {
                        0
                    } else {
                        (std::mem::size_of::<$t>() as u32 * 8) - self.leading_zeros()
                    }
                }

                fn bit_count(&self) -> u32 {
                    self.count_ones()
                }

                fn to_bytes_py(&self, length: usize, byteorder: &str) -> Vec<u8> {
                    let bytes = match byteorder {
                        "big" => self.to_be_bytes(),
                        "little" | _ => self.to_le_bytes(),
                    };
                    let mut result: Vec<u8> = bytes.to_vec();
                    if result.len() < length {
                        if byteorder == "big" {
                            let mut padded = vec![0u8; length - result.len()];
                            padded.extend(result);
                            result = padded;
                        } else {
                            result.resize(length, 0);
                        }
                    } else if result.len() > length {
                        if byteorder == "big" {
                            result = result[result.len() - length..].to_vec();
                        } else {
                            result.truncate(length);
                        }
                    }
                    result
                }
            }
        )*
    };
}

impl_python_int_ops_signed!(i8, i16, i32, i64, i128, isize);
impl_python_int_ops_unsigned!(u8, u16, u32, u64, u128, usize);

// ============================================================================
// PyOps Arithmetic Traits - DEPYLER-1307
// These traits provide Python-style arithmetic operations on Rust types.
// When Python allows `list - list` or `array + array`, these traits enable it.
// ============================================================================

/// Python-style addition for types (DEPYLER-1307).
/// - For `Vec<T>`: concatenation (`list + list`)
/// - For numeric `Vec`: element-wise addition (NumPy semantics)
/// - For `HashSet<T>`: union
pub trait PyAdd<Rhs = Self> {
    type Output;
    fn py_add(self, rhs: Rhs) -> Self::Output;
}

/// Python-style subtraction for types (DEPYLER-1307).
/// - For numeric `Vec`: element-wise subtraction (NumPy semantics)
/// - For `HashSet<T>`: set difference
pub trait PySub<Rhs = Self> {
    type Output;
    fn py_sub(self, rhs: Rhs) -> Self::Output;
}

/// Python-style multiplication for types (DEPYLER-1307).
/// - For `Vec<T>` with integer: repetition (`list * n`)
/// - For numeric `Vec`: element-wise multiplication (NumPy semantics)
pub trait PyMul<Rhs = Self> {
    type Output;
    fn py_mul(self, rhs: Rhs) -> Self::Output;
}

/// Python-style division for types (DEPYLER-1307).
/// - For numeric `Vec`: element-wise division (NumPy semantics)
pub trait PyDiv<Rhs = Self> {
    type Output;
    fn py_div(self, rhs: Rhs) -> Self::Output;
}

/// Python-style floor division for types (DEPYLER-1307).
pub trait PyFloorDiv<Rhs = Self> {
    type Output;
    fn py_floordiv(self, rhs: Rhs) -> Self::Output;
}

/// Python-style modulo for types (DEPYLER-1307).
pub trait PyMod<Rhs = Self> {
    type Output;
    fn py_mod(self, rhs: Rhs) -> Self::Output;
}

/// Python-style power for types (DEPYLER-1307).
pub trait PyPow<Rhs = Self> {
    type Output;
    fn py_pow(self, rhs: Rhs) -> Self::Output;
}

// ============================================================================
// PyOps implementations for primitive numeric types
// ============================================================================

macro_rules! impl_pyops_numeric {
    ($($t:ty),*) => {
        $(
            impl PyAdd for $t {
                type Output = $t;
                fn py_add(self, rhs: $t) -> $t { self + rhs }
            }

            impl PySub for $t {
                type Output = $t;
                fn py_sub(self, rhs: $t) -> $t { self - rhs }
            }

            impl PyMul for $t {
                type Output = $t;
                fn py_mul(self, rhs: $t) -> $t { self * rhs }
            }

            impl PyDiv for $t {
                type Output = $t;
                fn py_div(self, rhs: $t) -> $t { self / rhs }
            }

            impl PyMod for $t {
                type Output = $t;
                fn py_mod(self, rhs: $t) -> $t { self % rhs }
            }
        )*
    };
}

impl_pyops_numeric!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

// Floor division for integers
macro_rules! impl_pyfloordiv_int {
    ($($t:ty),*) => {
        $(
            impl PyFloorDiv for $t {
                type Output = $t;
                fn py_floordiv(self, rhs: $t) -> $t {
                    // Python floor division: always rounds toward negative infinity
                    let q = self / rhs;
                    let r = self % rhs;
                    if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
                        q - 1
                    } else {
                        q
                    }
                }
            }
        )*
    };
}

impl_pyfloordiv_int!(i8, i16, i32, i64, i128, isize);

// Floor division for floats
impl PyFloorDiv for f32 {
    type Output = f32;
    fn py_floordiv(self, rhs: f32) -> f32 {
        (self / rhs).floor()
    }
}

impl PyFloorDiv for f64 {
    type Output = f64;
    fn py_floordiv(self, rhs: f64) -> f64 {
        (self / rhs).floor()
    }
}

// Power for integers (returns same type)
macro_rules! impl_pypow_int {
    ($($t:ty),*) => {
        $(
            impl PyPow<u32> for $t {
                type Output = $t;
                fn py_pow(self, rhs: u32) -> $t { self.pow(rhs) }
            }
        )*
    };
}

impl_pypow_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

// Power for floats
impl PyPow for f32 {
    type Output = f32;
    fn py_pow(self, rhs: f32) -> f32 {
        self.powf(rhs)
    }
}

impl PyPow for f64 {
    type Output = f64;
    fn py_pow(self, rhs: f64) -> f64 {
        self.powf(rhs)
    }
}

// ============================================================================
// PyOps implementations for Vec (NumPy-style element-wise operations)
// ============================================================================

macro_rules! impl_pyops_vec_elementwise {
    ($($t:ty),*) => {
        $(
            impl PyAdd for Vec<$t> {
                type Output = Vec<$t>;
                fn py_add(self, rhs: Vec<$t>) -> Vec<$t> {
                    self.into_iter().zip(rhs).map(|(a, b)| a + b).collect()
                }
            }

            impl PyAdd<$t> for Vec<$t> {
                type Output = Vec<$t>;
                fn py_add(self, rhs: $t) -> Vec<$t> {
                    self.into_iter().map(|a| a + rhs).collect()
                }
            }

            impl PySub for Vec<$t> {
                type Output = Vec<$t>;
                fn py_sub(self, rhs: Vec<$t>) -> Vec<$t> {
                    self.into_iter().zip(rhs).map(|(a, b)| a - b).collect()
                }
            }

            impl PySub<$t> for Vec<$t> {
                type Output = Vec<$t>;
                fn py_sub(self, rhs: $t) -> Vec<$t> {
                    self.into_iter().map(|a| a - rhs).collect()
                }
            }

            impl PyMul for Vec<$t> {
                type Output = Vec<$t>;
                fn py_mul(self, rhs: Vec<$t>) -> Vec<$t> {
                    self.into_iter().zip(rhs).map(|(a, b)| a * b).collect()
                }
            }

            impl PyMul<$t> for Vec<$t> {
                type Output = Vec<$t>;
                fn py_mul(self, rhs: $t) -> Vec<$t> {
                    self.into_iter().map(|a| a * rhs).collect()
                }
            }

            impl PyDiv for Vec<$t> {
                type Output = Vec<$t>;
                fn py_div(self, rhs: Vec<$t>) -> Vec<$t> {
                    self.into_iter().zip(rhs).map(|(a, b)| a / b).collect()
                }
            }

            impl PyDiv<$t> for Vec<$t> {
                type Output = Vec<$t>;
                fn py_div(self, rhs: $t) -> Vec<$t> {
                    self.into_iter().map(|a| a / rhs).collect()
                }
            }
        )*
    };
}

impl_pyops_vec_elementwise!(i32, i64, f32, f64);

// String concatenation - Python's + operator
impl PyAdd for String {
    type Output = String;
    fn py_add(mut self, rhs: String) -> String {
        self.push_str(&rhs);
        self
    }
}

impl PyAdd<&str> for String {
    type Output = String;
    fn py_add(mut self, rhs: &str) -> String {
        self.push_str(rhs);
        self
    }
}

// List repetition: list * n (requires Clone for the elements)
impl<T: Clone> PyMul<usize> for Vec<T> {
    type Output = Vec<T>;
    fn py_mul(self, n: usize) -> Vec<T> {
        // Manual repeat implementation that only requires Clone
        let mut result = Vec::with_capacity(self.len() * n);
        for _ in 0..n {
            result.extend(self.iter().cloned());
        }
        result
    }
}

// ============================================================================
// PyOps implementations for HashSet (set operations)
// ============================================================================

use std::collections::HashSet;
use std::hash::Hash;

impl<T: Eq + Hash + Clone> PyAdd for HashSet<T> {
    type Output = HashSet<T>;
    fn py_add(self, rhs: HashSet<T>) -> HashSet<T> {
        self.union(&rhs).cloned().collect()
    }
}

impl<T: Eq + Hash + Clone> PySub for HashSet<T> {
    type Output = HashSet<T>;
    fn py_sub(self, rhs: HashSet<T>) -> HashSet<T> {
        self.difference(&rhs).cloned().collect()
    }
}

/// Python list operations for Rust Vec types.
pub trait PythonListOps<T> {
    /// Append item (Python's `list.append()`). Named append_py to avoid conflict with Vec::append.
    fn append_py(&mut self, item: T);

    /// Extend with iterable (Python's `list.extend()`).
    fn extend_py<I: IntoIterator<Item = T>>(&mut self, iterable: I);

    /// Insert at index (Python's `list.insert()`).
    fn insert_py(&mut self, index: usize, item: T);

    /// Remove first occurrence (Python's `list.remove()`).
    fn remove_py(&mut self, item: &T) -> bool
    where
        T: PartialEq;

    /// Pop from end (Python's `list.pop()`).
    fn pop_py(&mut self) -> Option<T>;

    /// Pop at index (Python's `list.pop(i)`).
    fn pop_at(&mut self, index: usize) -> Option<T>;

    /// Clear list (Python's `list.clear()`).
    fn clear_py(&mut self);

    /// Count occurrences (Python's `list.count()`).
    fn count_py(&self, item: &T) -> usize
    where
        T: PartialEq;

    /// Find index (Python's `list.index()`).
    fn index_py(&self, item: &T) -> Option<usize>
    where
        T: PartialEq;

    /// Reverse in place (Python's `list.reverse()`).
    fn reverse_py(&mut self);

    /// Copy list (Python's `list.copy()`).
    fn copy_py(&self) -> Vec<T>
    where
        T: Clone;
}

impl<T> PythonListOps<T> for Vec<T> {
    fn append_py(&mut self, item: T) {
        self.push(item);
    }

    fn extend_py<I: IntoIterator<Item = T>>(&mut self, iterable: I) {
        self.extend(iterable);
    }

    fn insert_py(&mut self, index: usize, item: T) {
        self.insert(index, item);
    }

    fn remove_py(&mut self, item: &T) -> bool
    where
        T: PartialEq,
    {
        if let Some(pos) = self.iter().position(|x| x == item) {
            self.remove(pos);
            true
        } else {
            false
        }
    }

    fn pop_py(&mut self) -> Option<T> {
        self.pop()
    }

    fn pop_at(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self.remove(index))
        } else {
            None
        }
    }

    fn clear_py(&mut self) {
        self.clear();
    }

    fn count_py(&self, item: &T) -> usize
    where
        T: PartialEq,
    {
        self.iter().filter(|x| *x == item).count()
    }

    fn index_py(&self, item: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.iter().position(|x| x == item)
    }

    fn reverse_py(&mut self) {
        self.reverse();
    }

    fn copy_py(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_lower() {
        assert_eq!("HELLO".lower(), "hello");
        assert_eq!(String::from("WORLD").lower(), "world");
    }

    #[test]
    fn test_string_upper() {
        assert_eq!("hello".upper(), "HELLO");
        assert_eq!(String::from("world").upper(), "WORLD");
    }

    #[test]
    fn test_string_strip() {
        assert_eq!("  hello  ".strip(), "hello");
        assert_eq!(String::from("\t\ntest\n\t").strip(), "test");
    }

    #[test]
    fn test_string_lstrip() {
        assert_eq!("  hello  ".lstrip(), "hello  ");
    }

    #[test]
    fn test_string_rstrip() {
        assert_eq!("  hello  ".rstrip(), "  hello");
    }

    #[test]
    fn test_string_split() {
        assert_eq!("a b c".split_py(), vec!["a", "b", "c"]);
        assert_eq!("a,b,c".split_on(","), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_string_startswith_endswith() {
        assert!("hello".startswith("hel"));
        assert!("hello".endswith("llo"));
        assert!(!String::from("hello").startswith("bye"));
    }

    #[test]
    fn test_string_find() {
        assert_eq!(PythonStringOps::find("hello", "ll"), 2);
        assert_eq!(PythonStringOps::find("hello", "x"), -1);
    }

    #[test]
    fn test_string_count() {
        assert_eq!("hello".count_py("l"), 2);
        assert_eq!("hello".count_py("x"), 0);
    }

    #[test]
    fn test_string_is_methods() {
        assert!("abc".isalpha());
        assert!(!"ab1".isalpha());
        assert!("123".isdigit());
        assert!(!"12a".isdigit());
        assert!("abc123".isalnum());
        assert!("   ".isspace());
        assert!("hello".islower());
        assert!("HELLO".isupper());
    }

    #[test]
    fn test_string_capitalize() {
        assert_eq!("hello world".capitalize(), "Hello world");
        assert_eq!("HELLO".capitalize(), "Hello");
    }

    #[test]
    fn test_string_title() {
        assert_eq!("hello world".title(), "Hello World");
    }

    #[test]
    fn test_string_swapcase() {
        assert_eq!("Hello World".swapcase(), "hELLO wORLD");
    }

    #[test]
    fn test_string_center_ljust_rjust() {
        assert_eq!("hi".center(6), "  hi  ");
        assert_eq!("hi".ljust(5), "hi   ");
        assert_eq!("hi".rjust(5), "   hi");
    }

    #[test]
    fn test_string_zfill() {
        assert_eq!("42".zfill(5), "00042");
        assert_eq!("-42".zfill(5), "-0042");
    }

    #[test]
    fn test_string_join() {
        assert_eq!(",".join_py(vec!["a", "b", "c"]), "a,b,c");
    }

    #[test]
    fn test_string_encode() {
        assert_eq!("hello".encode(), vec![104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_int_bit_length() {
        assert_eq!(0i32.bit_length(), 0);
        assert_eq!(1i32.bit_length(), 1);
        assert_eq!(7i32.bit_length(), 3);
        assert_eq!(8i32.bit_length(), 4);
        assert_eq!(255i32.bit_length(), 8);
        assert_eq!((-1i32).bit_length(), 1);
        assert_eq!((-255i32).bit_length(), 8);
    }

    #[test]
    fn test_int_bit_count() {
        assert_eq!(0i32.bit_count(), 0);
        assert_eq!(1i32.bit_count(), 1);
        assert_eq!(7i32.bit_count(), 3);
        assert_eq!(255i32.bit_count(), 8);
    }

    #[test]
    fn test_int_to_bytes() {
        assert_eq!(1024i32.to_bytes_py(2, "big"), vec![4, 0]);
        assert_eq!(1024i32.to_bytes_py(2, "little"), vec![0, 4]);
    }

    #[test]
    fn test_list_append() {
        let mut v = vec![1, 2];
        v.append_py(3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_list_extend() {
        let mut v = vec![1, 2];
        v.extend_py(vec![3, 4]);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_list_insert() {
        let mut v = vec![1, 3];
        v.insert_py(1, 2);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_list_remove() {
        let mut v = vec![1, 2, 3];
        assert!(v.remove_py(&2));
        assert_eq!(v, vec![1, 3]);
        assert!(!v.remove_py(&5));
    }

    #[test]
    fn test_list_pop() {
        let mut v = vec![1, 2, 3];
        assert_eq!(v.pop_py(), Some(3));
        assert_eq!(v.pop_at(0), Some(1));
        assert_eq!(v, vec![2]);
    }

    #[test]
    fn test_list_count() {
        let v = vec![1, 2, 2, 3];
        assert_eq!(v.count_py(&2), 2);
        assert_eq!(v.count_py(&5), 0);
    }

    #[test]
    fn test_list_index() {
        let v = vec![1, 2, 3];
        assert_eq!(v.index_py(&2), Some(1));
        assert_eq!(v.index_py(&5), None);
    }

    #[test]
    fn test_list_reverse() {
        let mut v = vec![1, 2, 3];
        v.reverse_py();
        assert_eq!(v, vec![3, 2, 1]);
    }

    #[test]
    fn test_list_copy() {
        let v = vec![1, 2, 3];
        let v2 = v.copy_py();
        assert_eq!(v, v2);
    }

    // ============================================================================
    // PyOps arithmetic tests - DEPYLER-1307
    // ============================================================================

    #[test]
    fn test_pyops_numeric_add() {
        assert_eq!(5i32.py_add(3i32), 8);
        assert_eq!(5.0f64.py_add(3.0f64), 8.0);
    }

    #[test]
    fn test_pyops_numeric_sub() {
        assert_eq!(5i32.py_sub(3i32), 2);
        assert_eq!(5.0f64.py_sub(3.0f64), 2.0);
    }

    #[test]
    fn test_pyops_numeric_mul() {
        assert_eq!(5i32.py_mul(3i32), 15);
        assert_eq!(5.0f64.py_mul(3.0f64), 15.0);
    }

    #[test]
    fn test_pyops_numeric_div() {
        assert_eq!(10i32.py_div(3i32), 3);
        assert_eq!(10.0f64.py_div(4.0f64), 2.5);
    }

    #[test]
    fn test_pyops_floordiv() {
        // Python: -7 // 3 = -3 (rounds toward negative infinity)
        assert_eq!((-7i32).py_floordiv(3i32), -3);
        assert_eq!(7i32.py_floordiv(3i32), 2);
        assert_eq!(7.0f64.py_floordiv(3.0f64), 2.0);
    }

    #[test]
    fn test_pyops_mod() {
        assert_eq!(10i32.py_mod(3i32), 1);
        assert_eq!(10.0f64.py_mod(3.0f64), 1.0);
    }

    #[test]
    fn test_pyops_pow() {
        assert_eq!(2i32.py_pow(10u32), 1024);
        assert_eq!(2.0f64.py_pow(10.0f64), 1024.0);
    }

    #[test]
    fn test_pyops_vec_elementwise_add() {
        let a = vec![1.0f64, 2.0, 3.0];
        let b = vec![4.0f64, 5.0, 6.0];
        assert_eq!(a.py_add(b), vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_pyops_vec_elementwise_sub() {
        let a = vec![5.0f64, 7.0, 9.0];
        let b = vec![1.0f64, 2.0, 3.0];
        assert_eq!(a.py_sub(b), vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_pyops_vec_elementwise_mul() {
        let a = vec![1.0f64, 2.0, 3.0];
        let b = vec![4.0f64, 5.0, 6.0];
        assert_eq!(a.py_mul(b), vec![4.0, 10.0, 18.0]);
    }

    #[test]
    fn test_pyops_vec_elementwise_div() {
        let a = vec![4.0f64, 10.0, 18.0];
        let b = vec![2.0f64, 5.0, 6.0];
        assert_eq!(a.py_div(b), vec![2.0, 2.0, 3.0]);
    }

    #[test]
    fn test_pyops_vec_scalar_add() {
        let a = vec![1.0f64, 2.0, 3.0];
        assert_eq!(a.py_add(10.0), vec![11.0, 12.0, 13.0]);
    }

    #[test]
    fn test_pyops_vec_scalar_sub() {
        let a = vec![10.0f64, 20.0, 30.0];
        assert_eq!(a.py_sub(5.0), vec![5.0, 15.0, 25.0]);
    }

    #[test]
    fn test_pyops_vec_scalar_mul() {
        let a = vec![1.0f64, 2.0, 3.0];
        assert_eq!(a.py_mul(2.0), vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_pyops_vec_scalar_div() {
        let a = vec![10.0f64, 20.0, 30.0];
        assert_eq!(a.py_div(5.0), vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_pyops_list_repetition() {
        let a = vec![1, 2, 3];
        assert_eq!(a.py_mul(3usize), vec![1, 2, 3, 1, 2, 3, 1, 2, 3]);
    }

    #[test]
    fn test_pyops_set_union() {
        let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
        let result = a.py_add(b);
        assert_eq!(result.len(), 5);
        assert!(result.contains(&1));
        assert!(result.contains(&5));
    }

    #[test]
    fn test_pyops_set_difference() {
        let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let b: HashSet<i32> = [2, 3, 4].into_iter().collect();
        let result = a.py_sub(b);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&1));
    }
}
