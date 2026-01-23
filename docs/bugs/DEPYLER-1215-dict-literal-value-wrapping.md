# DEPYLER-1215: Dict Literal Values Not Wrapped in DepylerValue at Call Sites

## Problem Statement

When a dict literal is passed as an argument to a function expecting `HashMap<String, DepylerValue>`, the values are not wrapped in `DepylerValue`.

## Example

```python
def from_dict(data: dict) -> str:
    return data["name"]

def main():
    print(from_dict({"name": "test"}))  # Dict literal at call site
```

## Current Generated Code (Broken)

```rust
// Call site generates:
from_dict(&{
    let mut map = HashMap::new();
    map.insert("name".to_string(), "test".to_string());  // String, not DepylerValue!
    map
})
```

## Expected Generated Code

```rust
from_dict(&{
    let mut map = HashMap::new();
    map.insert("name".to_string(), DepylerValue::Str("test".to_string()));
    map
})
```

## Root Cause

The dict literal generator doesn't have access to the target function's parameter type. The context has `current_assign_type` for assignments, but not for function call arguments.

## Proposed Solution

Add `function_param_types: HashMap<String, Vec<Type>>` to `CodegenContext` that maps function names to their parameter types. When generating function call arguments:

1. Look up the called function's parameter types
2. For each argument, set a context flag indicating the expected type
3. Dict literal generator checks this flag and wraps values accordingly

## Files to Modify

- `crates/depyler-core/src/rust_gen/context.rs` - Add `function_param_types`
- `crates/depyler-core/src/rust_gen/func_gen.rs` - Populate `function_param_types`
- `crates/depyler-core/src/rust_gen/expr_gen.rs` - Set context when processing call args
- `crates/depyler-core/src/rust_gen/expr_gen_instance_methods.rs` - Check context in dict literal gen

## Related Tickets

- DEPYLER-1214: Fixed dict ACCESS to use String keys (completed)
- DEPYLER-1213: Fixed dict CREATION to use String keys (completed)

## Priority

Medium - Affects dict literals passed directly to functions with `dict` parameter type.

## Workaround

Assign dict to a typed variable first:
```python
d: dict = {"name": "test"}
print(from_dict(d))
```
