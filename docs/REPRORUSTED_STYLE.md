
## Type Annotations Required

All Python examples MUST have complete type annotations for ty compatibility:

```python
# ✅ GOOD - ty can infer concrete types
def process(data: list[int]) -> dict[str, int]:
    return {"sum": sum(data)}

# ❌ BAD - ty cannot help, depyler defaults to Value
def process(data):
    return {"sum": sum(data)}
```

## Pre-flight Checks

Before transpilation:
```bash
# Type check with ty
ty check script.py

# Then transpile
depyler compile script.py
```
