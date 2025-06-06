# Contributing to Depyler

Thank you for your interest in contributing to Depyler! This document provides guidelines and instructions for contributing to the project.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Philosophy](#development-philosophy)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Testing Guidelines](#testing-guidelines)
- [Code Style](#code-style)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:
- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Respect differing viewpoints and experiences

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/depyler.git
   cd depyler
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/paiml/depyler.git
   ```

## Development Philosophy

We follow the Toyota Production System principles:

### 🔧 Jidoka (自働化) - Build Quality In
- Never merge incomplete features
- All code must have tests
- Quality gates must pass before merge

### 🏭 Genchi Genbutsu (現地現物) - Direct Observation
- Test with real Python code
- Profile actual performance
- Debug at the Rust level

### 📈 Kaizen (改善) - Continuous Improvement
- Fix bugs before adding features
- Improve existing code quality
- Optimize performance iteratively

## How to Contribute

### Reporting Issues
- Check existing issues first
- Use issue templates when available
- Include minimal reproducible examples
- Specify your environment (OS, Rust version, Python version)

### Suggesting Features
- Open a discussion first for major features
- Explain the use case and benefits
- Consider implementation complexity
- Align with project goals

### Contributing Code

#### Priority Areas
1. **Bug Fixes** - Always welcome!
2. **Test Coverage** - Help us reach 95%+ coverage
3. **Documentation** - Improve clarity and completeness
4. **Performance** - Optimize hot paths
5. **Python Feature Support** - Implement missing features from the roadmap

## Development Setup

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python 3.8+
python3 --version  # Verify installation

# Install development tools
cargo install cargo-watch cargo-tarpaulin cargo-criterion

# For WASM development
cargo install wasm-pack
npm install -g http-server
```

### Building the Project
```bash
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run quality checks
cargo run -p depyler-quality -- analyze . --format text

# Build WASM module
cd crates/depyler-wasm
wasm-pack build --target web --out-dir ../../playground/public/wasm
```

## Testing Guidelines

### Test Requirements
- All new features must have tests
- Maintain or improve code coverage
- Include both unit and integration tests
- Test edge cases and error conditions

### Running Tests
```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p depyler-core

# Run with coverage
cargo tarpaulin --out Html

# Run benchmarks
cargo bench
```

### Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_happy_path() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_feature_error_case() {
        // Test error handling
        let invalid_input = create_invalid_input();
        assert!(function_under_test(invalid_input).is_err());
    }
}
```

## Code Style

### Rust Style
- Follow standard Rust formatting: `cargo fmt`
- Pass clippy lints: `cargo clippy -- -D warnings`
- Use meaningful variable names
- Document public APIs with `///` comments
- Keep functions small and focused

### Error Handling
```rust
// Use Result for fallible operations
pub fn transpile(code: &str) -> Result<String, TranspileError> {
    // Implementation
}

// Provide context with errors
return Err(TranspileError::InvalidSyntax {
    line: 42,
    message: "Unexpected token".to_string(),
});
```

### Documentation
```rust
/// Transpiles Python code to Rust.
///
/// # Arguments
/// * `code` - The Python source code to transpile
///
/// # Returns
/// * `Ok(String)` - The generated Rust code
/// * `Err(TranspileError)` - If transpilation fails
///
/// # Example
/// ```
/// let python = "def add(a: int, b: int) -> int: return a + b";
/// let rust_code = transpile(python)?;
/// ```
pub fn transpile(code: &str) -> Result<String, TranspileError> {
    // Implementation
}
```

## Commit Messages

Follow conventional commits format:

```
type(scope): subject

body

footer
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code changes that neither fix bugs nor add features
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples
```
feat(core): add support for f-string expressions

Implements basic f-string support with variable interpolation.
Does not yet support format specifiers or expressions.

Closes #123
```

```
fix(lambda): correct event type inference for nested patterns

The inference engine now correctly handles patterns accessed
through intermediate variables in loops.
```

## Pull Request Process

1. **Update your fork**:
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**:
   - Write code following style guidelines
   - Add tests for new functionality
   - Update documentation as needed
   - Ensure all tests pass
   - Run quality checks

4. **Commit your changes**:
   - Use meaningful commit messages
   - Keep commits focused and atomic
   - Squash WIP commits before PR

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create Pull Request**:
   - Use the PR template
   - Link related issues
   - Describe changes clearly
   - Include test results
   - Add screenshots for UI changes

### PR Checklist
- [ ] Tests pass: `cargo test --workspace`
- [ ] Lints pass: `cargo clippy -- -D warnings`
- [ ] Formatted: `cargo fmt`
- [ ] Documentation updated
- [ ] CHANGELOG.md entry added (for features/breaking changes)
- [ ] Quality gates pass: `cargo run -p depyler-quality -- analyze .`

### Review Process
1. Automated CI checks must pass
2. At least one maintainer review required
3. Address review feedback
4. Maintainer merges when approved

## Questions?

- Open a [Discussion](https://github.com/paiml/depyler/discussions) for general questions
- Join our community channels (coming soon)
- Check the [Documentation](./docs/README.md)

Thank you for contributing to Depyler! Your efforts help make Python-to-Rust transpilation better for everyone. 🚀