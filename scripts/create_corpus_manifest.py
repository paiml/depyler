#!/usr/bin/env python3
import os
import json
import argparse
import ast
from pathlib import Path

def is_parsable(filepath):
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            source = f.read()
        ast.parse(source)
        return True
    except Exception:
        return False

EXCLUDE_DIRS = {'.venv', 'venv', '__pycache__', '.git', 'node_modules', '.tox', '.eggs', 'build', 'dist'}

def scan_directory(root_dir, check_parse=False):
    files = []
    root_path = Path(root_dir)
    if not root_path.exists():
        print(f"Warning: Directory {root_dir} does not exist.")
        return []

    for path in root_path.rglob("*.py"):
        # Skip excluded directories
        if any(excluded in path.parts for excluded in EXCLUDE_DIRS):
            continue
        if check_parse:
            if is_parsable(path):
                files.append(str(path))
            else:
                print(f"Skipping unparsable: {path}")
        else:
            files.append(str(path))
    return sorted(files)

def main():
    parser = argparse.ArgumentParser(description="Generate corpus_manifest.json")
    parser.add_argument("--output", default="corpus_manifest.json", help="Output JSON file")
    parser.add_argument("--include", action="append", help="Directories to include")
    parser.add_argument("--check-parse", action="store_true", help="Only include parsable files")
    
    args = parser.parse_args()
    
    manifest = {
        "version": "1.0",
        "corpora": {}
    }
    
    if args.include:
        for folder in args.include:
            name = os.path.basename(folder.rstrip("/\\"))
            print(f"Scanning {folder}...")
            files = scan_directory(folder, args.check_parse)
            manifest["corpora"][name] = {
                "root": folder,
                "count": len(files),
                "files": files
            }
            
    with open(args.output, "w") as f:
        json.dump(manifest, f, indent=2)
    
    print(f"Manifest written to {args.output}")

if __name__ == "__main__":
    main()
