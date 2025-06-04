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
                        _ => format!("key_{}", i),
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

pub fn shrink_value(value: &serde_json::Value) -> Vec<serde_json::Value> {
    match value {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                let mut shrunk = vec![];
                if i != 0 {
                    shrunk.push(serde_json::json!(0));
                    shrunk.push(serde_json::json!(i / 2));
                    if i > 0 {
                        shrunk.push(serde_json::json!(i - 1));
                    } else {
                        shrunk.push(serde_json::json!(i + 1));
                    }
                }
                shrunk
            } else if let Some(f) = n.as_f64() {
                let mut shrunk = vec![];
                if f != 0.0 {
                    shrunk.push(serde_json::json!(0.0));
                    shrunk.push(serde_json::json!(f / 2.0));
                }
                shrunk
            } else {
                vec![]
            }
        }
        serde_json::Value::String(s) => {
            let mut shrunk = vec![];
            if !s.is_empty() {
                shrunk.push(serde_json::json!(""));
                if s.len() > 1 {
                    shrunk.push(serde_json::json!(&s[..s.len() / 2]));
                    shrunk.push(serde_json::json!(&s[1..]));
                }
            }
            shrunk
        }
        serde_json::Value::Array(arr) => {
            let mut shrunk = vec![];
            if !arr.is_empty() {
                shrunk.push(serde_json::json!([]));
                if arr.len() > 1 {
                    shrunk.push(serde_json::Value::Array(arr[..arr.len() / 2].to_vec()));
                    shrunk.push(serde_json::Value::Array(arr[1..].to_vec()));
                }
            }
            shrunk
        }
        _ => vec![],
    }
}
