# Minimal repro: loop reassigns str variable with concatenated result
def build_path(base: str, parts: list) -> str:
    result = base
    for p in parts:
        result = result + "/" + p
    return result
