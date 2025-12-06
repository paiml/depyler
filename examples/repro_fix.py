# Repro case: E0308 - Dict literal values not wrapped in serde_json::Value
# Target: DEPYLER-204 (Hunt Mode single-shot compilation)
# Pattern: {"a": 1} with Dict[str, Any] -> insert(1) instead of insert(Value::from(1))

from typing import Dict, Any

def create_dict() -> Dict[str, Any]:
    """Create a simple dict."""
    return {"a": 1, "b": 2}

def main() -> None:
    d = create_dict()
    print(f"Dict: {d:?}")

if __name__ == "__main__":
    main()
