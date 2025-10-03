#!/usr/bin/env python3
"""
Extract examples from pytest tests and generate documentation.

This script parses test files and converts them to markdown documentation.
"""
import ast
import argparse
from pathlib import Path
from typing import Dict, List


class ExampleExtractor:
    """Extract test cases and convert to documentation."""

    def __init__(self, tests_dir: Path, docs_dir: Path):
        self.tests_dir = tests_dir
        self.docs_dir = docs_dir

    def extract_from_test_file(self, test_path: Path) -> Dict[str, List[Dict]]:
        """Parse test file and extract example code."""
        with open(test_path) as f:
            content = f.read()
            tree = ast.parse(content)

        examples = {}
        current_class = None

        for node in ast.walk(tree):
            if isinstance(node, ast.ClassDef):
                current_class = node.name
                class_docstring = ast.get_docstring(node)
                examples[current_class] = {
                    "description": class_docstring or "",
                    "tests": [],
                }

            elif isinstance(node, ast.FunctionDef) and node.name.startswith("test_"):
                if current_class and current_class in examples:
                    docstring = ast.get_docstring(node)
                    source = ast.unparse(node)

                    examples[current_class]["tests"].append(
                        {"name": node.name, "description": docstring or "", "code": source}
                    )

        return examples

    def generate_markdown(self, examples: Dict, module_name: str) -> str:
        """Convert examples to markdown format."""
        md_lines = [f"# {module_name}\n"]

        for class_name, class_data in examples.items():
            md_lines.append(f"## {class_data['description']}\n")

            for test in class_data["tests"]:
                md_lines.append(f"### {test['description']}\n")
                md_lines.append("```python")
                md_lines.append(test["code"])
                md_lines.append("```\n")
                md_lines.append("**Verification**: ✅ Tested in CI\n")

        return "\n".join(md_lines)

    def process_module(self, module_dir: Path):
        """Process all test files in a module directory."""
        module_name = module_dir.name.replace("test_", "")
        all_examples = {}

        for test_file in module_dir.glob("test_*.py"):
            examples = self.extract_from_test_file(test_file)
            all_examples.update(examples)

        if all_examples:
            markdown = self.generate_markdown(all_examples, module_name)
            output_file = self.docs_dir / "modules" / f"{module_name}.md"
            output_file.parent.mkdir(parents=True, exist_ok=True)
            output_file.write_text(markdown)
            print(f"✅ Generated: {output_file}")

    def process_all(self):
        """Process all test modules."""
        for module_dir in self.tests_dir.iterdir():
            if module_dir.is_dir() and module_dir.name.startswith("test_"):
                self.process_module(module_dir)


def main():
    parser = argparse.ArgumentParser(
        description="Extract examples from tests and generate documentation"
    )
    parser.add_argument(
        "--module", help="Specific module to process (e.g., 'os')", default=None
    )
    parser.add_argument("--all", action="store_true", help="Process all modules")

    args = parser.parse_args()

    script_dir = Path(__file__).parent
    tests_dir = script_dir.parent / "tests"
    docs_dir = script_dir.parent / "docs"

    extractor = ExampleExtractor(tests_dir, docs_dir)

    if args.all:
        extractor.process_all()
        print("✅ All modules processed")
    elif args.module:
        module_dir = tests_dir / f"test_{args.module}"
        if module_dir.exists():
            extractor.process_module(module_dir)
        else:
            print(f"❌ Module directory not found: {module_dir}")
    else:
        print("Please specify --all or --module <name>")


if __name__ == "__main__":
    main()
