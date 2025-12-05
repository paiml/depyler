# Minimal repro: from X.Y import Z generates invalid Rust import
from os.path import join as path_join

def combine_paths(base: str, suffix: str) -> str:
    return path_join(base, suffix)
