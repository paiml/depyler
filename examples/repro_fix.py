# Repro case: DEPYLER-0725 - Any type maps to serde_json::Value
# Target: DEPYLER-204 (Hunt Mode single-shot compilation)
# Verifies: Dict[str, Any] -> HashMap<String, serde_json::Value> (not generic T: Clone)

from typing import Dict, Any

def get_keys(data: Dict[str, Any]) -> int:
    """Get number of keys in a dict."""
    return len(data)

def main() -> None:
    count = get_keys({})
    print(f"Keys: {count}")

if __name__ == "__main__":
    main()
