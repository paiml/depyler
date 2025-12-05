# Minimal repro: json.loads returns Result but function returns HashMap
import json
from typing import Dict, Any

def parse_data(text: str) -> Dict[str, Any]:
    return json.loads(text)
