# Depyler: Python to Rust Transpiler

This document provides a high-level overview of the Depyler project, intended for new contributors and users.

## Project Overview

Depyler is a sophisticated transpiler that converts Python code into safe, idiomatic, and energy-efficient Rust code. It leverages Python's type annotations to generate high-quality Rust code, and includes a semantic verification engine to ensure that the transpiled code is behaviorally equivalent to the original Python source.

The project is designed with a focus on correctness, performance, and developer productivity. It provides tools for not only transpilation but also for analyzing the complexity of migrating Python codebases to Rust.

## Key Features

*   **Python to Rust Transpilation:** Converts a significant subset of the Python language to Rust, including functions, classes, collections, control flow, and async/await.
*   **Semantic Verification:** Uses property-based testing to verify the correctness of the transpiled code.
*   **Migration Analysis:** Provides a tool to analyze Python code and estimate the effort required for migration to Rust.
*   **Modular Architecture:** The project is organized as a Cargo workspace with several specialized crates, promoting separation of concerns and maintainability.
*   **AI Assistant Integration:** Includes an MCP (Model Context Protocol) server for integration with AI assistants, enabling code transformation and analysis through a conversational interface.
*   **High-Quality Standards:** The project adheres to strict quality gates, including high test coverage, low cyclomatic complexity, and comprehensive documentation.

## Getting Started

### Installation

To install the Depyler CLI, use the following command:

```bash
cargo install depyler
```

### Basic Usage

**Transpile a Python file:**

```bash
depyler transpile my_script.py
```

**Analyze a Python file for migration complexity:**

```bash
depyler analyze my_script.py
```

**Transpile and verify the output:**

```bash
depyler transpile my_script.py --verify
```

## Project Structure

The Depyler project is a Cargo workspace with the following key components:

*   `crates/`: This directory contains the individual crates that make up the Depyler project.
    *   `depyler-core/`: The core transpilation logic.
    *   `depyler-analyzer/`: The migration complexity analysis tool.
    *   `depyler-verify/`: The semantic verification engine.
    *   `depyler-mcp/`: The MCP server for AI assistant integration.
    *   `depyler/`: The main CLI application.
*   `docs/`: Contains detailed documentation about the project's architecture, features, and usage.
*   `examples/`: A collection of Python scripts that can be used to test and demonstrate Depyler's capabilities.
*   `tests/`: Contains the integration and unit tests for the project.
*   `Cargo.toml`: The workspace configuration file, which defines the project's dependencies and structure.
*   `README.md`: The main README file for the project.

## Development

### Running Tests

To run the full test suite for the project, use the following command:

```bash
cargo test --workspace
```

### Code Style and Quality

The project enforces a strict code style and quality standards. Before submitting any changes, please run the following commands to ensure that your code meets these standards:

**Format the code:**

```bash
cargo fmt --all
```

**Run the linter:**

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Contributing

Contributions are welcome! Please refer to the `CONTRIBUTING.md` file for more details on how to contribute to the project.
