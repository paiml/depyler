from typing import List, Tuple, Dict

def create_catalog() -> Dict[str, List[str]]:
    return {}

def add_table(catalog: Dict[str, List[str]], name: str, columns: List[str]) -> Dict[str, List[str]]:
    new_cat: Dict[str, List[str]] = {}
    for k in catalog:
        new_cat[k] = catalog[k]
    new_cat[name] = columns
    return new_cat

def get_columns(catalog: Dict[str, List[str]], name: str) -> List[str]:
    if name in catalog:
        return catalog[name]
    return []

def table_exists(catalog: Dict[str, List[str]], name: str) -> bool:
    return name in catalog

def count_tables(catalog: Dict[str, List[str]]) -> int:
    return len(catalog)

def drop_table(catalog: Dict[str, List[str]], name: str) -> Dict[str, List[str]]:
    new_cat: Dict[str, List[str]] = {}
    for k in catalog:
        if k != name:
            new_cat[k] = catalog[k]
    return new_cat
