from typing import Dict, Optional

def process_config(config: Dict[str, str]) -> Optional[str]:
    """Process configuration dictionary and return debug value if present."""
    if "debug" in config:
        return config["debug"]
    return None