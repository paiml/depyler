# Depyler MCP Quickstart - Agentic Python-to-Rust Conversion

**Convert your Python project to Rust with one AI command!**

This guide shows you how to use Depyler's MCP server with Claude Code for agentic Python-to-Rust migration.

---

## 🚀 Quick Setup (2 minutes)

### 1. Install Depyler

```bash
cargo install depyler
```

### 2. Configure Claude Desktop

Add to `~/.config/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "depyler": {
      "command": "depyler",
      "args": ["agent", "start", "--foreground", "--port", "3000"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### 3. Restart Claude Desktop

The Depyler MCP server will start automatically when Claude Code launches.

---

## 💬 Agentic Workflows

### Example 1: "Convert my project to Rust"

```
You: Convert my Python project at /home/user/myproject to Rust

Claude (using MCP tools):
  1. analyze_migration_complexity(/home/user/myproject)
     → Analyzes 1,234 lines, estimates 8h effort, identifies 3 high-risk components

  2. transpile_python(main.py, utils.py, api.py, ...)
     → Generates Rust code for all modules

  3. verify_transpilation(...)
     → Runs property tests, semantic equivalence checks

  4. pmat_quality_check(generated/*.rs)
     → Validates TDG A+ quality, <10 complexity

Result: ✅ Complete Rust project with:
  - Cargo.toml
  - src/*.rs files
  - tests/
  - 75-85% energy reduction
  - Memory safety guaranteed
```

### Example 2: "Analyze migration complexity"

```
You: How hard would it be to migrate my Django app to Rust?

Claude: Let me analyze your project...

[Uses: analyze_migration_complexity(path)]

Claude: Your Django project has:
  - 4,567 Python LOC → ~6,800 Rust LOC
  - Estimated effort: 32 hours
  - Migration strategy:
    1. Phase 1: Models (3h)
    2. Phase 2: Views/Routes (12h)
    3. Phase 3: Business logic (15h)
  - High-risk components:
    - Custom middleware (no direct Rust equivalent)
    - Database ORM (suggest SeaORM migration)
```

### Example 3: "Verify transpilation quality"

```
You: I transpiled my code - verify it's correct

Claude: Running verification...

[Uses: verify_transpilation(python_file, rust_file)]

Claude: ✅ Verification Results:
  - Semantic equivalence: 98.5%
  - All property tests passed (247/247)
  - Memory safety: ✓ Guaranteed
  - Performance: 12.3x faster, 82% less energy
  - Code quality: TDG A+ (99.1/100)
```

---

## 🛠️ Available MCP Tools

Depyler provides 4 production-ready MCP tools:

### 1. `transpile_python`

**Convert Python code to Rust**

```javascript
{
  "source": "def fib(n): return n if n < 2 else fib(n-1) + fib(n-2)",
  "mode": "inline",  // or "file" or "project"
  "options": {
    "optimization_level": "energy",     // size, speed, energy
    "type_inference": "conservative",   // conservative, aggressive, ml_assisted
    "memory_model": "stack_preferred"   // stack_preferred, arena, rc_refcell
  }
}
```

**Returns:**
- Generated Rust code
- Energy reduction estimate
- Memory safety score
- Compilation command

### 2. `analyze_migration_complexity`

**Analyze project migration effort**

```javascript
{
  "project_path": "/path/to/project",
  "analysis_depth": "deep",     // surface, standard, deep
  "include_patterns": ["**/*.py"]
}
```

**Returns:**
- Complexity score
- LOC estimates (Python → Rust)
- Migration strategy with phases
- High-risk components
- Rust crate recommendations
- Effort estimate (hours)

### 3. `verify_transpilation`

**Verify semantic equivalence**

```javascript
{
  "python_source": "...",
  "rust_source": "...",
  "verification_level": "comprehensive"  // basic, comprehensive, formal
}
```

**Returns:**
- Semantic equivalence score
- Property test results
- Safety guarantees (memory, thread)
- Performance comparison
- Optimization suggestions

### 4. `pmat_quality_check`

**Code quality analysis**

```javascript
{
  "file_path": "output.rs",
  "checks": ["complexity", "satd", "tdg"]
}
```

**Returns:**
- TDG score (A+ = 85+)
- Cyclomatic complexity
- Technical debt (SATD)
- Quality recommendations

---

## 📋 Real-World Examples

### Migrate a Flask API

```
You: Convert my Flask REST API to Rust with Axum

Claude:
1. Analyzes your Flask routes and models
2. Generates Rust code with:
   - Axum for HTTP server
   - SeaORM for database
   - Serde for JSON
3. Verifies API equivalence
4. Provides migration guide

Result: Production-ready Rust API, 10x faster, 85% energy savings
```

### Convert ML Pipeline

```
You: Migrate my scikit-learn pipeline to Rust

Claude:
1. Analyzes your ML workflow
2. Recommends Rust ML crates:
   - linfa (scikit-learn equivalent)
   - ndarray (numpy equivalent)
   - polars (pandas equivalent)
3. Transpiles data preprocessing
4. Verifies numerical accuracy

Result: Type-safe ML pipeline, 15x faster inference
```

### Modernize Legacy Code

```
You: My 10-year-old Python codebase is slow and buggy - help!

Claude:
1. Complexity analysis: 15,000 LOC, 45% high-complexity functions
2. Migration strategy:
   - Phase 1: Core algorithms (highest impact)
   - Phase 2: Data processing
   - Phase 3: UI/API layer
3. Transpiles incrementally
4. Verifies each phase

Result: Gradual migration, immediate performance gains
```

---

## 🎯 Best Practices

### 1. Start with Analysis

Always analyze before transpiling:
```
"Analyze my project at /path/to/project first"
```

### 2. Incremental Migration

Don't convert everything at once:
```
"Convert just the core algorithms first, keep the rest in Python"
```

### 3. Verify Critical Paths

For mission-critical code:
```
"Transpile and verify with comprehensive testing"
```

### 4. Check Quality

Ensure A+ quality:
```
"Run PMAT quality check on the generated Rust code"
```

---

## 🔧 Advanced Configuration

### Custom Transpilation Options

```json
{
  "optimization_level": "speed",
  "type_inference": "aggressive",
  "memory_model": "arena",
  "enable_simd": true,
  "verification_level": "formal"
}
```

### Project-Specific Settings

Create `.depyler.toml`:

```toml
[transpilation]
optimization_level = "energy"
type_inference = "conservative"

[verification]
property_tests = true
fuzz_testing = true
formal_verification = false

[quality]
min_tdg_score = 85.0
max_complexity = 10
allow_satd = false
```

---

## 📊 Performance Impact

**Typical improvements from Python → Rust:**
- ⚡ **10-15x faster** execution
- 🔋 **75-85% less** energy consumption
- 💾 **50-70% less** memory usage
- 🛡️ **100%** memory safety (guaranteed)
- 🧵 **Thread-safe** by default

---

## 🆘 Troubleshooting

### "MCP server not responding"

```bash
# Check if agent is running
depyler agent status

# Restart agent
depyler agent stop && depyler agent start --foreground
```

### "Transpilation failed"

```bash
# Run with debug logging
RUST_LOG=debug depyler agent start --foreground
```

### "Generated code doesn't compile"

The AI will automatically:
1. Fix syntax errors
2. Resolve type mismatches
3. Add missing dependencies
4. Verify with `cargo check`

---

## 🎓 Learning Resources

- **Tutorial**: [Python to Rust Migration Guide](../docs/MIGRATION.md)
- **Examples**: [examples/showcase/](../examples/showcase/)
- **API Docs**: [docs.rs/depyler](https://docs.rs/depyler)
- **Full Agent Docs**: [AGENT.md](../AGENT.md)

---

## 🚀 What's Next?

After successful migration:

1. **Optimize**: Use `--optimization-level speed` for hot paths
2. **Parallelize**: Leverage Rust's fearless concurrency
3. **Profile**: Measure actual energy/performance gains
4. **Deploy**: Ship production-ready Rust binaries

---

## 💡 Pro Tips

### Tip 1: Batch Processing

```
"Convert all Python files in src/ to Rust, keeping the directory structure"
```

### Tip 2: Hybrid Approach

```
"Keep the Python API, convert only the compute-intensive functions to Rust"
```

### Tip 3: Continuous Migration

```
"Monitor my project - automatically transpile any changed Python files"
```

---

## 📞 Support

- **Issues**: https://github.com/paiml/depyler/issues
- **Discord**: https://discord.gg/depyler
- **Email**: support@depyler.dev

---

**Ready to convert your project?**

1. Install Depyler
2. Configure Claude Desktop
3. Say: *"Convert my project to Rust"*

It's that simple! 🎉
