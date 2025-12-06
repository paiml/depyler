# Repro case: E0599 - dict.keys() method not found on HashMap<String, Value>
# Target: DEPYLER-204 (Hunt Mode single-shot compilation)
# Pattern: Dict[str, Any].keys() -> HashMap<String, Value> has no `.keys()` that returns iter

from typing import Dict, Any

def count_keys(data: Dict[str, Any]) -> int:
    """Count number of keys in dict."""
    return len(data.keys())

def main() -> None:
    d: Dict[str, Any] = {"a": 1, "b": 2}
    count = count_keys(d)
    print(f"Keys: {count}")

if __name__ == "__main__":
    main()
