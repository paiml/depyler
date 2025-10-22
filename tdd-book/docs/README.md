# Depyler TDD Book

**Test-Driven Documentation for Python to Rust Transpilation**

Welcome to the Depyler TDD Book - a comprehensive, test-driven guide to Python standard library transpilation using Depyler.

## Philosophy: TDD-First Documentation

Every example in this book is:

✅ **Tested** - All code examples have corresponding tests in `tdd-book/tests/`
✅ **Verified** - Tests run on every commit via GitHub Actions
✅ **Compilable** - Generated Rust code compiles with zero warnings
✅ **Idiomatic** - Follows Rust best practices and safety guarantees

## What You'll Learn

This book demonstrates how Depyler transpiles Python standard library modules to safe, idiomatic Rust code:

- **Type-safe transpilation** - How Python types map to Rust
- **Memory safety** - Ownership and borrowing in generated code
- **Performance** - Energy-efficient Rust vs. Python
- **Stdlib coverage** - 25+ Python stdlib modules

## Structure

Each module chapter follows this pattern:

1. **Python Example** - Idiomatic Python code
2. **Transpiled Rust** - Generated Rust code
3. **Test Coverage** - Property tests and edge cases
4. **Performance Notes** - Energy and speed comparisons

## Quality Standards

All examples meet these requirements:

- **Complexity**: ≤10 cyclomatic complexity
- **Coverage**: 80%+ test coverage
- **Safety**: Zero unsafe Rust
- **Clippy**: Zero warnings with `-D warnings`

## Quick Start

```bash
# Clone the repository
git clone https://github.com/noahshinn/depyler
cd depyler

# Run book tests
cd tdd-book
uv run pytest tests/ -v

# Serve book locally
mdbook serve
```

## Contributing

Found an issue? Want to add examples?

1. Write failing tests first (RED)
2. Implement the feature (GREEN)
3. Meet quality standards (REFACTOR)
4. Update book documentation

See [Contributing Guide](./contributing.md) for details.

## Toyota Way Principles

This book follows Toyota Manufacturing principles:

- **自働化 (Jidoka)** - Quality built in, not bolted on
- **現地現物 (Genchi Genbutsu)** - Verify with real tests
- **改善 (Kaizen)** - Continuous improvement via TDD

## License

Licensed under MIT OR Apache-2.0
