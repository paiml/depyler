# Corpus Convergence Protocol

Run the convergence protocol from `docs/prompts/converge_reprorusted_100.md`.

## Quick Start

Use the built-in converge command with rich progress feedback:

```bash
# Build release binary first
cargo build --release --bin depyler

# Run convergence with rich TUI progress (progress bars, sparklines)
./target/release/depyler converge --input-dir /home/noah/src/reprorusted-python-cli/examples --target-rate 80 --display rich

# CI-friendly minimal output
./target/release/depyler converge --input-dir /home/noah/src/reprorusted-python-cli/examples --target-rate 80 --display minimal
```

## Display Modes

| Mode | Description |
|------|-------------|
| `--display rich` | TUI with progress bars and detailed cluster info (default) |
| `--display minimal` | Single-line CI-friendly output |
| `--display json` | JSON output for automation |
| `--display silent` | No output |

## Alternative: UTOL Automated Loop

```bash
./target/release/depyler utol --corpus /home/noah/src/reprorusted-python-cli/examples --target-rate 0.80 --display rich
```

## Quick Status Check

```bash
./target/release/depyler utol --corpus /home/noah/src/reprorusted-python-cli/examples --status
```

## Key Commands

| Task | Command |
|------|---------|
| Cache Warm | `depyler cache warm --input-dir $CORPUS` |
| Converge (Rich) | `depyler converge --input-dir $CORPUS --target-rate 80 --display rich` |
| Converge (CI) | `depyler converge --input-dir $CORPUS --target-rate 80 --display minimal` |
| UTOL | `depyler utol --corpus $CORPUS --target-rate 0.80 --display rich` |
| Status | `depyler utol --corpus $CORPUS --status` |
| Explain | `depyler explain <file.rs> --trace <trace.json>` |

## Full Protocol

See: `docs/prompts/converge_reprorusted_100.md`
