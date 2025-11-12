#!/usr/bin/env python3
"""Test heterogeneous dict transpilation"""
import json

def main():
    # Heterogeneous dict with mixed types
    data = {
        "name": "test",
        "count": 42,
        "enabled": True,
        "rate": 3.14,
        "items": [1, 2, 3],
    }

    print(json.dumps(data))

if __name__ == "__main__":
    main()
