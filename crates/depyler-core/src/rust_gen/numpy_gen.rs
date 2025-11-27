//! NumPy to Trueno codegen mapping.
//!
//! Maps NumPy API calls to trueno (SIMD-accelerated tensor library).
//!
//! # Mappings
//!
//! | NumPy | trueno |
//! |-------|--------|
//! | `np.array([1.0, 2.0])` | `Vector::from_slice(&[1.0f32, 2.0])` |
//! | `np.dot(a, b)` | `a.dot(&b)?` |
//! | `np.sum(a)` | `a.sum()?` |
//! | `np.mean(a)` | `a.mean()?` |
//! | `np.sqrt(a)` | `a.sqrt()?` |
//!
//! Created: 2025-11-27 (Phase 3 - NumPy→Trueno)

use proc_macro2::TokenStream;
use quote::quote;

/// Represents a recognized numpy function call
#[derive(Debug, Clone)]
pub enum NumpyCall {
    /// np.array([...]) - create vector from list
    Array { elements: Vec<TokenStream> },
    /// np.dot(a, b) - dot product
    Dot { a: TokenStream, b: TokenStream },
    /// np.sum(a) - sum all elements
    Sum { arr: TokenStream },
    /// np.mean(a) - mean of all elements
    Mean { arr: TokenStream },
    /// np.sqrt(a) - element-wise sqrt
    Sqrt { arr: TokenStream },
    /// np.abs(a) - element-wise abs
    Abs { arr: TokenStream },
    /// np.min(a) - minimum element
    Min { arr: TokenStream },
    /// np.max(a) - maximum element
    Max { arr: TokenStream },
    /// np.exp(a) - element-wise exp
    Exp { arr: TokenStream },
    /// np.log(a) - element-wise log
    Log { arr: TokenStream },
    /// np.sin(a) - element-wise sin
    Sin { arr: TokenStream },
    /// np.cos(a) - element-wise cos
    Cos { arr: TokenStream },
    /// np.clip(a, min, max) - clip values
    Clip {
        arr: TokenStream,
        min: TokenStream,
        max: TokenStream,
    },
    /// np.argmax(a) - index of max element
    ArgMax { arr: TokenStream },
    /// np.argmin(a) - index of min element
    ArgMin { arr: TokenStream },
    /// np.std(a) - standard deviation
    Std { arr: TokenStream },
    /// np.var(a) - variance
    Var { arr: TokenStream },
    /// np.zeros(n) - array of zeros
    Zeros { size: TokenStream },
    /// np.ones(n) - array of ones
    Ones { size: TokenStream },
    /// np.norm(a) or np.linalg.norm(a) - L2 norm
    Norm { arr: TokenStream },
}

/// Generate trueno code for a numpy call.
///
/// # Returns
///
/// TokenStream containing the trueno equivalent code.
pub fn generate_trueno_code(call: &NumpyCall) -> TokenStream {
    match call {
        NumpyCall::Array { elements } => {
            quote! {
                Vector::from_slice(&[#(#elements as f32),*])
            }
        }
        NumpyCall::Dot { a, b } => {
            quote! {
                #a.dot(&#b).unwrap()
            }
        }
        NumpyCall::Sum { arr } => {
            quote! {
                #arr.sum().unwrap()
            }
        }
        NumpyCall::Mean { arr } => {
            quote! {
                #arr.mean().unwrap()
            }
        }
        NumpyCall::Sqrt { arr } => {
            quote! {
                #arr.sqrt().unwrap()
            }
        }
        NumpyCall::Abs { arr } => {
            quote! {
                #arr.abs().unwrap()
            }
        }
        NumpyCall::Min { arr } => {
            quote! {
                #arr.min().unwrap()
            }
        }
        NumpyCall::Max { arr } => {
            quote! {
                #arr.max().unwrap()
            }
        }
        NumpyCall::Exp { arr } => {
            quote! {
                #arr.exp().unwrap()
            }
        }
        NumpyCall::Log { arr } => {
            quote! {
                #arr.ln().unwrap()
            }
        }
        NumpyCall::Sin { arr } => {
            quote! {
                #arr.sin().unwrap()
            }
        }
        NumpyCall::Cos { arr } => {
            quote! {
                #arr.cos().unwrap()
            }
        }
        NumpyCall::Clip { arr, min, max } => {
            quote! {
                #arr.clamp(#min, #max).unwrap()
            }
        }
        NumpyCall::ArgMax { arr } => {
            quote! {
                #arr.argmax().unwrap()
            }
        }
        NumpyCall::ArgMin { arr } => {
            quote! {
                #arr.argmin().unwrap()
            }
        }
        NumpyCall::Std { arr } => {
            quote! {
                #arr.std().unwrap()
            }
        }
        NumpyCall::Var { arr } => {
            quote! {
                #arr.variance().unwrap()
            }
        }
        NumpyCall::Zeros { size } => {
            quote! {
                Vector::zeros(#size)
            }
        }
        NumpyCall::Ones { size } => {
            quote! {
                Vector::ones(#size)
            }
        }
        NumpyCall::Norm { arr } => {
            quote! {
                #arr.norm().unwrap()
            }
        }
    }
}

/// Check if a module name is numpy or np alias.
pub fn is_numpy_module(module: &str) -> bool {
    module == "numpy" || module == "np"
}

/// Parse a numpy function name and return the corresponding NumpyCall variant name.
///
/// Returns None if not a recognized numpy function.
pub fn parse_numpy_function(func_name: &str) -> Option<&'static str> {
    match func_name {
        "array" => Some("Array"),
        "dot" => Some("Dot"),
        "sum" => Some("Sum"),
        "mean" => Some("Mean"),
        "sqrt" => Some("Sqrt"),
        "abs" => Some("Abs"),
        "min" | "amin" => Some("Min"),
        "max" | "amax" => Some("Max"),
        "exp" => Some("Exp"),
        "log" => Some("Log"),
        "sin" => Some("Sin"),
        "cos" => Some("Cos"),
        "clip" => Some("Clip"),
        "argmax" => Some("ArgMax"),
        "argmin" => Some("ArgMin"),
        "std" => Some("Std"),
        "var" => Some("Var"),
        "zeros" => Some("Zeros"),
        "ones" => Some("Ones"),
        "norm" => Some("Norm"),
        _ => None,
    }
}

/// Get the trueno use statement needed for numpy code.
pub fn trueno_use_statement() -> TokenStream {
    quote! {
        use trueno::Vector;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_array() {
        let elements = vec![quote!(1.0), quote!(2.0), quote!(3.0)];
        let call = NumpyCall::Array { elements };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("Vector :: from_slice"),
            "Should generate Vector::from_slice: {}",
            code_str
        );
        assert!(
            code_str.contains("1.0") && code_str.contains("2.0") && code_str.contains("3.0"),
            "Should contain elements: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_dot() {
        let call = NumpyCall::Dot {
            a: quote!(a),
            b: quote!(b),
        };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("dot"),
            "Should generate dot call: {}",
            code_str
        );
        assert!(
            code_str.contains("unwrap"),
            "Should unwrap result: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_sum() {
        let call = NumpyCall::Sum { arr: quote!(arr) };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("sum"),
            "Should generate sum call: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_mean() {
        let call = NumpyCall::Mean { arr: quote!(arr) };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("mean"),
            "Should generate mean call: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_sqrt() {
        let call = NumpyCall::Sqrt { arr: quote!(arr) };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("sqrt"),
            "Should generate sqrt call: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_zeros() {
        let call = NumpyCall::Zeros { size: quote!(10) };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("zeros"),
            "Should generate zeros call: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_ones() {
        let call = NumpyCall::Ones { size: quote!(10) };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("ones"),
            "Should generate ones call: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_clip() {
        let call = NumpyCall::Clip {
            arr: quote!(arr),
            min: quote!(0.0),
            max: quote!(1.0),
        };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("clamp"),
            "Should generate clamp call: {}",
            code_str
        );
    }

    #[test]
    fn test_is_numpy_module() {
        assert!(is_numpy_module("numpy"));
        assert!(is_numpy_module("np"));
        assert!(!is_numpy_module("math"));
        assert!(!is_numpy_module("random"));
    }

    #[test]
    fn test_parse_numpy_function() {
        assert_eq!(parse_numpy_function("array"), Some("Array"));
        assert_eq!(parse_numpy_function("dot"), Some("Dot"));
        assert_eq!(parse_numpy_function("sum"), Some("Sum"));
        assert_eq!(parse_numpy_function("mean"), Some("Mean"));
        assert_eq!(parse_numpy_function("sqrt"), Some("Sqrt"));
        assert_eq!(parse_numpy_function("min"), Some("Min"));
        assert_eq!(parse_numpy_function("amin"), Some("Min"));
        assert_eq!(parse_numpy_function("max"), Some("Max"));
        assert_eq!(parse_numpy_function("amax"), Some("Max"));
        assert_eq!(parse_numpy_function("unknown"), None);
    }

    #[test]
    fn test_trueno_use_statement() {
        let stmt = trueno_use_statement();
        let stmt_str = stmt.to_string();

        assert!(
            stmt_str.contains("trueno"),
            "Should use trueno: {}",
            stmt_str
        );
        assert!(
            stmt_str.contains("Vector"),
            "Should import Vector: {}",
            stmt_str
        );
    }

    #[test]
    fn test_generate_norm() {
        let call = NumpyCall::Norm { arr: quote!(v) };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("norm"),
            "Should generate norm call: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_argmax() {
        let call = NumpyCall::ArgMax { arr: quote!(arr) };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("argmax"),
            "Should generate argmax call: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_std() {
        let call = NumpyCall::Std { arr: quote!(arr) };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("std"),
            "Should generate std call: {}",
            code_str
        );
    }

    #[test]
    fn test_generate_var() {
        let call = NumpyCall::Var { arr: quote!(arr) };
        let code = generate_trueno_code(&call);
        let code_str = code.to_string();

        assert!(
            code_str.contains("variance"),
            "Should generate variance call: {}",
            code_str
        );
    }
}
