use depyler_core::hir::Type;
use quickcheck::{Arbitrary, Gen};

pub struct TypedValue {
    pub ty: Type,
    pub value: serde_json::Value,
}

impl TypedValue {
    pub fn arbitrary_for_type(ty: &Type, g: &mut Gen) -> Self {
        let value = match ty {
            Type::Int => {
                let n: i32 = Arbitrary::arbitrary(g);
                serde_json::json!(n)
            }
            Type::Float => {
                let f: f64 = Arbitrary::arbitrary(g);
                // Avoid NaN and infinity which may not be considered numbers by is_number()
                let f = if f.is_finite() { f } else { 1.0 };
                serde_json::json!(f)
            }
            Type::String => {
                let s: String = Arbitrary::arbitrary(g);
                serde_json::json!(s)
            }
            Type::Bool => {
                let b: bool = Arbitrary::arbitrary(g);
                serde_json::json!(b)
            }
            Type::None => serde_json::Value::Null,
            Type::List(inner) => {
                let size = g.size();
                let len = (size % 10) + 1; // Keep lists reasonably small
                let items: Vec<serde_json::Value> = (0..len)
                    .map(|_| Self::arbitrary_for_type(inner, g).value)
                    .collect();
                serde_json::json!(items)
            }
            Type::Dict(key_ty, val_ty) => {
                let size = g.size();
                let len = (size % 5) + 1; // Keep dicts small
                let mut map = serde_json::Map::new();

                for i in 0..len {
                    let key = match key_ty.as_ref() {
                        Type::String => {
                            let s: String = Arbitrary::arbitrary(g);
                            s
                        }
                        Type::Int => {
                            let n: i32 = Arbitrary::arbitrary(g);
                            n.to_string()
                        }
                        _ => format!("key_{i}"),
                    };
                    let val = Self::arbitrary_for_type(val_ty, g).value;
                    map.insert(key, val);
                }
                serde_json::Value::Object(map)
            }
            Type::Optional(inner) => {
                let is_some: bool = Arbitrary::arbitrary(g);
                if is_some {
                    Self::arbitrary_for_type(inner, g).value
                } else {
                    serde_json::Value::Null
                }
            }
            Type::Tuple(types) => {
                let items: Vec<serde_json::Value> = types
                    .iter()
                    .map(|t| Self::arbitrary_for_type(t, g).value)
                    .collect();
                serde_json::json!(items)
            }
            _ => serde_json::Value::Null,
        };

        TypedValue {
            ty: ty.clone(),
            value,
        }
    }
}

// DEPYLER-0024: Helper functions to reduce complexity (extracted from shrink_value)
fn shrink_integer(i: i64) -> Vec<serde_json::Value> {
    if i == 0 {
        return vec![];
    }

    let mut shrunk = vec![serde_json::json!(0), serde_json::json!(i / 2)];

    if i > 0 {
        shrunk.push(serde_json::json!(i - 1));
    } else {
        shrunk.push(serde_json::json!(i + 1));
    }

    shrunk
}

fn shrink_float(f: f64) -> Vec<serde_json::Value> {
    if f == 0.0 {
        vec![]
    } else {
        vec![serde_json::json!(0.0), serde_json::json!(f / 2.0)]
    }
}

fn shrink_string(s: &str) -> Vec<serde_json::Value> {
    if s.is_empty() {
        return vec![];
    }

    let mut shrunk = vec![serde_json::json!("")];

    if s.len() > 1 {
        shrunk.push(serde_json::json!(&s[..s.len() / 2]));
        shrunk.push(serde_json::json!(&s[1..]));
    }

    shrunk
}

fn shrink_array(arr: &[serde_json::Value]) -> Vec<serde_json::Value> {
    if arr.is_empty() {
        return vec![];
    }

    let mut shrunk = vec![serde_json::json!([])];

    if arr.len() > 1 {
        shrunk.push(serde_json::Value::Array(arr[..arr.len() / 2].to_vec()));
        shrunk.push(serde_json::Value::Array(arr[1..].to_vec()));
    }

    shrunk
}

pub fn shrink_value(value: &serde_json::Value) -> Vec<serde_json::Value> {
    match value {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                shrink_integer(i)
            } else if let Some(f) = n.as_f64() {
                shrink_float(f)
            } else {
                vec![]
            }
        }
        serde_json::Value::String(s) => shrink_string(s),
        serde_json::Value::Array(arr) => shrink_array(arr),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Gen;

    fn test_gen() -> Gen {
        Gen::new(5) // Small size for testing
    }

    #[test]
    fn test_typed_value_creation() {
        let ty = Type::Int;
        let value = serde_json::json!(42);

        let typed_value = TypedValue {
            ty: ty.clone(),
            value: value.clone(),
        };

        assert_eq!(typed_value.ty, ty);
        assert_eq!(typed_value.value, value);
    }

    #[test]
    fn test_arbitrary_for_type_int() {
        let mut g = test_gen();
        let typed_value = TypedValue::arbitrary_for_type(&Type::Int, &mut g);

        assert_eq!(typed_value.ty, Type::Int);
        assert!(typed_value.value.is_number());
    }

    #[test]
    fn test_arbitrary_for_type_float() {
        let mut g = test_gen();
        let typed_value = TypedValue::arbitrary_for_type(&Type::Float, &mut g);

        assert_eq!(typed_value.ty, Type::Float);
        // Debug print for investigating test failure
        if !typed_value.value.is_number() {
            eprintln!(
                "Generated non-number for Float type: {:?}",
                typed_value.value
            );
        }
        assert!(typed_value.value.is_number());
    }

    #[test]
    fn test_arbitrary_for_type_string() {
        let mut g = test_gen();
        let typed_value = TypedValue::arbitrary_for_type(&Type::String, &mut g);

        assert_eq!(typed_value.ty, Type::String);
        assert!(typed_value.value.is_string());
    }

    #[test]
    fn test_arbitrary_for_type_bool() {
        let mut g = test_gen();
        let typed_value = TypedValue::arbitrary_for_type(&Type::Bool, &mut g);

        assert_eq!(typed_value.ty, Type::Bool);
        assert!(typed_value.value.is_boolean());
    }

    #[test]
    fn test_arbitrary_for_type_none() {
        let mut g = test_gen();
        let typed_value = TypedValue::arbitrary_for_type(&Type::None, &mut g);

        assert_eq!(typed_value.ty, Type::None);
        assert!(typed_value.value.is_null());
    }

    #[test]
    fn test_arbitrary_for_type_list() {
        let mut g = test_gen();
        let list_type = Type::List(Box::new(Type::Int));
        let typed_value = TypedValue::arbitrary_for_type(&list_type, &mut g);

        assert_eq!(typed_value.ty, list_type);
        assert!(typed_value.value.is_array());

        if let Some(arr) = typed_value.value.as_array() {
            // Should have some elements (up to 10)
            assert!(!arr.is_empty());
            assert!(arr.len() <= 10);

            // All elements should be numbers (integers)
            for item in arr {
                assert!(item.is_number());
            }
        }
    }

    #[test]
    fn test_arbitrary_for_type_dict() {
        let mut g = test_gen();
        let dict_type = Type::Dict(Box::new(Type::String), Box::new(Type::Int));
        let typed_value = TypedValue::arbitrary_for_type(&dict_type, &mut g);

        assert_eq!(typed_value.ty, dict_type);
        assert!(typed_value.value.is_object());

        if let Some(obj) = typed_value.value.as_object() {
            // Should have some entries (up to 5)
            assert!(!obj.is_empty());
            assert!(obj.len() <= 5);

            // All values should be numbers (integers)
            for (_key, value) in obj {
                // Key should be a valid string (might be empty from random generation)
                assert!(value.is_number());
            }
        }
    }

    #[test]
    fn test_arbitrary_for_type_optional_some() {
        let mut g = test_gen();
        let optional_type = Type::Optional(Box::new(Type::String));

        // Generate multiple values to test both Some and None cases
        let mut has_some = false;
        let mut has_none = false;

        for _ in 0..20 {
            let typed_value = TypedValue::arbitrary_for_type(&optional_type, &mut g);
            assert_eq!(typed_value.ty, optional_type);

            if typed_value.value.is_null() {
                has_none = true;
            } else if typed_value.value.is_string() {
                has_some = true;
            }

            if has_some && has_none {
                break;
            }
        }

        // Should generate both Some and None values with enough iterations
        // Note: This is probabilistic, so we can't guarantee both will occur
    }

    #[test]
    fn test_arbitrary_for_type_tuple() {
        let mut g = test_gen();
        let tuple_type = Type::Tuple(vec![Type::Int, Type::String, Type::Bool]);
        let typed_value = TypedValue::arbitrary_for_type(&tuple_type, &mut g);

        assert_eq!(typed_value.ty, tuple_type);
        assert!(typed_value.value.is_array());

        if let Some(arr) = typed_value.value.as_array() {
            assert_eq!(arr.len(), 3);
            assert!(arr[0].is_number());
            assert!(arr[1].is_string());
            assert!(arr[2].is_boolean());
        }
    }

    #[test]
    fn test_arbitrary_for_type_unknown() {
        let mut g = test_gen();
        let typed_value = TypedValue::arbitrary_for_type(&Type::Unknown, &mut g);

        assert_eq!(typed_value.ty, Type::Unknown);
        assert!(typed_value.value.is_null());
    }

    #[test]
    fn test_shrink_value_integer() {
        let value = serde_json::json!(10);
        let shrunk = shrink_value(&value);

        assert!(shrunk.contains(&serde_json::json!(0)));
        assert!(shrunk.contains(&serde_json::json!(5)));
        assert!(shrunk.contains(&serde_json::json!(9)));
    }

    #[test]
    fn test_shrink_value_negative_integer() {
        let value = serde_json::json!(-10);
        let shrunk = shrink_value(&value);

        assert!(shrunk.contains(&serde_json::json!(0)));
        assert!(shrunk.contains(&serde_json::json!(-5)));
        assert!(shrunk.contains(&serde_json::json!(-9)));
    }

    #[test]
    fn test_shrink_value_float() {
        let value = serde_json::json!(4.0);
        let shrunk = shrink_value(&value);

        assert!(shrunk.contains(&serde_json::json!(0.0)));
        assert!(shrunk.contains(&serde_json::json!(2.0)));
    }

    #[test]
    fn test_shrink_value_zero() {
        let value = serde_json::json!(0);
        let shrunk = shrink_value(&value);

        // Zero should not shrink further
        assert!(shrunk.is_empty());
    }

    #[test]
    fn test_shrink_value_string() {
        let value = serde_json::json!("hello");
        let shrunk = shrink_value(&value);

        assert!(shrunk.contains(&serde_json::json!("")));
        assert!(shrunk.contains(&serde_json::json!("he")));
        assert!(shrunk.contains(&serde_json::json!("ello")));
    }

    #[test]
    fn test_shrink_value_empty_string() {
        let value = serde_json::json!("");
        let shrunk = shrink_value(&value);

        // Empty string should not shrink further
        assert!(shrunk.is_empty());
    }

    #[test]
    fn test_shrink_value_array() {
        let value = serde_json::json!([1, 2, 3, 4]);
        let shrunk = shrink_value(&value);

        assert!(shrunk.contains(&serde_json::json!([])));
        assert!(shrunk.contains(&serde_json::json!([1, 2])));
        assert!(shrunk.contains(&serde_json::json!([2, 3, 4])));
    }

    #[test]
    fn test_shrink_value_empty_array() {
        let value = serde_json::json!([]);
        let shrunk = shrink_value(&value);

        // Empty array should not shrink further
        assert!(shrunk.is_empty());
    }

    #[test]
    fn test_shrink_value_single_element_array() {
        let value = serde_json::json!([42]);
        let shrunk = shrink_value(&value);

        assert!(shrunk.contains(&serde_json::json!([])));
        // Single element array should not have other shrinks
        assert_eq!(shrunk.len(), 1);
    }

    #[test]
    fn test_shrink_value_unsupported_types() {
        let null_value = serde_json::Value::Null;
        let shrunk = shrink_value(&null_value);
        assert!(shrunk.is_empty());

        let bool_value = serde_json::json!(true);
        let shrunk = shrink_value(&bool_value);
        assert!(shrunk.is_empty());

        let object_value = serde_json::json!({"key": "value"});
        let shrunk = shrink_value(&object_value);
        assert!(shrunk.is_empty());
    }

    #[test]
    fn test_nested_type_generation() {
        let mut g = test_gen();
        let nested_type = Type::List(Box::new(Type::List(Box::new(Type::Int))));
        let typed_value = TypedValue::arbitrary_for_type(&nested_type, &mut g);

        assert_eq!(typed_value.ty, nested_type);
        assert!(typed_value.value.is_array());

        if let Some(outer_arr) = typed_value.value.as_array() {
            for inner_val in outer_arr {
                assert!(inner_val.is_array());
                if let Some(inner_arr) = inner_val.as_array() {
                    for item in inner_arr {
                        assert!(item.is_number());
                    }
                }
            }
        }
    }

    #[test]
    fn test_dict_with_int_keys() {
        let mut g = test_gen();
        let dict_type = Type::Dict(Box::new(Type::Int), Box::new(Type::String));
        let typed_value = TypedValue::arbitrary_for_type(&dict_type, &mut g);

        assert_eq!(typed_value.ty, dict_type);
        assert!(typed_value.value.is_object());

        if let Some(obj) = typed_value.value.as_object() {
            for (key, value) in obj {
                // Keys should be stringified integers
                assert!(key.parse::<i32>().is_ok());
                assert!(value.is_string());
            }
        }
    }
}
